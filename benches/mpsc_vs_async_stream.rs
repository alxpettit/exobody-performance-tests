#![allow(dead_code)]

use async_fn_stream::{fn_stream, try_fn_stream};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures::stream::{FusedStream, StreamExt};
use futures::{pin_mut, select, Stream};
use itertools::Itertools;
use std::error::Error;
use std::sync::mpsc::{channel, Receiver, Sender};

// 277 us
fn mpsc_test(tx: &Sender<[f32; 100]>, rx: &mut Receiver<[f32; 100]>) {
    let array = [2f32; 100];
    for _ in 0..1000 {
        let tx_send_data = array.iter().map(|v| black_box(black_box(*v) * 2.));
        let v: [f32; 100] = tx_send_data.collect_vec().try_into().unwrap();
        tx.send(black_box(v)).unwrap();
    }
    //for _ in 0..1000 {
    while let Ok(s) = black_box(rx.recv()) {
        black_box(s);
    }
    //}
}

// 285 us :( Even worse.
fn flume_test(tx: &flume::Sender<[f32; 100]>, rx: &mut flume::Receiver<[f32; 100]>) {
    let array = [2f32; 100];
    for _ in 0..1000 {
        let tx_send_data = array.iter().map(|v| black_box(black_box(*v) * 2.));
        let v: [f32; 100] = tx_send_data.collect_vec().try_into().unwrap();
        tx.send(black_box(v)).unwrap();
    }
    //for _ in 0..1000 {
    while let Ok(s) = black_box(rx.recv()) {
        black_box(s);
    }
    //}
}

async fn tachyonix_test(
    tx: &tachyonix::Sender<[f32; 100]>,
    rx: &mut tachyonix::Receiver<[f32; 100]>,
) {
    let array = [2f32; 100];
    for _ in 0..1000 {
        let tx_send_data = array.iter().map(|v| black_box(black_box(*v) * 2.));
        let v: [f32; 100] = tx_send_data.collect_vec().try_into().unwrap();
        tx.send(black_box(v)).await.unwrap();
    }
    //for _ in 0..1000 {
    while let Ok(s) = black_box(rx.recv().await) {
        black_box(s);
    }
    // }
}

async fn output_stream() -> impl Stream<Item = [f32; 100]> {
    fn_stream(|emitter| async move {
        for _ in 0..1000 {
            emitter.emit(black_box([2f32; 100])).await;
        }
    })
}

// 80ns :(
async fn output_stream_nochunk() -> impl Stream<Item = f32> {
    fn_stream(|emitter| async move {
        for _ in 0..100000 {
            emitter.emit(black_box(0f32)).await;
        }
    })
}
//
// async fn output_stream() -> impl Stream<Item = [f32; 100]> {
//     fn_stream(|emitter| async move {
//         for i in black_box(0..1000) {
//             emitter.emit(black_box([2f32; 100])).await;
//         }
//     })
// }

async fn try_output_stream() -> impl Stream<Item = Result<[f32; 100], Box<dyn Error>>> {
    try_fn_stream(|emitter| async move {
        for _ in black_box(0..1000) {
            emitter.emit(black_box([2f32; 100])).await;
        }
        Ok(())
    })
}

async fn try_input_stream<S: Stream<Item = Result<[f32; 100], Box<dyn Error>>> + Unpin>(
    mut stream: S,
) -> impl Stream<Item = Result<(), Box<dyn Error>>> {
    try_fn_stream(|_emitter| async move {
        while let Some(value) = stream.next().await {
            let value = value?;
            black_box(value);
        }
        Ok(())
    })
}

async fn input_stream<S: Stream<Item = [f32; 100]> + Unpin + FusedStream>(mut stream: S) {
    while let Some(item) = stream.next().await {
        // assert_eq!(item, [2f32; 100]);
        black_box(item);
    }
}

async fn input_stream_nochunk<S: Stream<Item = f32> + Unpin + FusedStream>(mut stream: S) {
    while let Some(item) = stream.next().await {
        // assert_eq!(item, [2f32; 100]);
        black_box(item);
    }
}

async fn switch_streams(
    mut stream_rx: impl Stream<Item = impl Stream<Item = [f32; 100]> + Unpin + FusedStream>
        + Unpin
        + FusedStream,
) {
    let mut current_stream = stream_rx.next().await.unwrap(); // Get the first stream
    loop {
        select! {
            item = current_stream.next() => {
                if let Some(item) = item {
                    black_box(item);
                    //println!("Received item: {:?}", item);
                } else {

                    //println!("Stream ended");
                   // break;
                }
            },
            new_stream = stream_rx.next() => {
                if let Some(new_stream) = new_stream {
                    //println!("Switching to new stream");
                    current_stream = new_stream;
                } else {
                    //println!("All streams ended");
                    break;
                }
            }
        }
    }
}

// 200.72 ns!!!!
async fn async_stream_test() {
    let out = output_stream().await.fuse();
    pin_mut!(out);
    input_stream(out).await;
}

async fn async_stream_nochunk_test() {
    let out = output_stream_nochunk().await.fuse();
    pin_mut!(out);
    input_stream_nochunk(out).await;
}

async fn try_async_stream_test() {
    let out = try_output_stream().await.fuse();
    pin_mut!(out);
    let inp = try_input_stream(out).await.fuse();
    pin_mut!(inp);
    while let Some(v) = inp.next().await {
        black_box(v).unwrap();
    }
}

use futures::stream::iter;

fn test_stream_of_streams(
) -> impl Stream<Item = impl Stream<Item = [f32; 100]> + FusedStream> + FusedStream {
    fn_stream(|emitter| async move {
        emitter.emit(iter(vec![[0.0; 100]; 1000]).fuse()).await;
    })
    .fuse()
}

async fn async_stream_test_switch() {
    let streams = test_stream_of_streams();
    pin_mut!(streams);
    switch_streams(streams).await;
}

fn criterion_benchmark(c: &mut Criterion) {
    let (tac_tx, mut tac_rx) = tachyonix::channel::<[f32; 100]>(10000);
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (flume_tx, mut flume_rx) = flume::bounded::<[f32; 100]>(1000usize);
    //let (tx, mut rx) = mpsc::channel::<f32>();
    //let (tx_chunked, mut rx_chunked) = mpsc::channel::<[f32; 10]>();
    let (tx_big_chunked, mut rx_big_chunked) = channel::<[f32; 100]>();

    c.bench_function("async_stream_nochunk_test", |b| {
        b.iter(|| rt.block_on(async_stream_nochunk_test()))
    });

    c.bench_function("tachyonix_test", |b| {
        b.iter(|| rt.block_on(tachyonix_test(&tac_tx, &mut tac_rx)))
    });

    c.bench_function("try_async_stream_test", |b| {
        b.iter(|| rt.block_on(try_async_stream_test()))
    });

    c.bench_function("async_stream_test_switch", |b| {
        b.iter(|| rt.block_on(async_stream_test_switch()))
    });

    c.bench_function("mpsc_test", |b| {
        b.iter(|| mpsc_test(&tx_big_chunked, &mut rx_big_chunked))
    });

    c.bench_function("flume_test", |b| {
        b.iter(|| flume_test(&flume_tx, &mut flume_rx))
    });

    c.bench_function("async_stream_test", |b| {
        b.iter(|| rt.block_on(async_stream_test()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
