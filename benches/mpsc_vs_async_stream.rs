use async_fn_stream::fn_stream;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures::executor::block_on;
use futures::stream::{FusedStream, StreamExt};
use futures::{pin_mut, select, SinkExt, Stream};
use itertools::Itertools;
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use tokio::sync::oneshot;

// 277 us
fn mpsc_test(tx: &Sender<[f32; 100]>, rx: &mut Receiver<[f32; 100]>) {
    let array = [2f32; 100];
    for i in black_box(0..1000) {
        let tx_send_data = array.iter().map(|v| black_box(black_box(*v) * 2.));
        let v: [f32; 100] = tx_send_data.collect_vec().try_into().unwrap();
        tx.send(black_box(v)).unwrap();
    }
    for i in black_box(0..1000) {
        for s in black_box(rx.recv().unwrap()) {
            black_box(s);
        }
    }
}

// 285 us :( Even worse.
fn flume_test(tx: &flume::Sender<[f32; 100]>, rx: &mut flume::Receiver<[f32; 100]>) {
    let array = [2f32; 100];
    for i in black_box(0..1000) {
        let tx_send_data = array.iter().map(|v| black_box(black_box(*v) * 2.));
        let v: [f32; 100] = tx_send_data.collect_vec().try_into().unwrap();
        tx.send(black_box(v)).unwrap();
    }
    for i in black_box(0..1000) {
        for s in black_box(rx.recv().unwrap()) {
            black_box(s);
        }
    }
}

async fn output_stream() -> impl Stream<Item = [f32; 100]> {
    fn_stream(|emitter| async move {
        for i in black_box(0..1000) {
            emitter.emit(black_box([2f32; 100])).await;
        }
    })
}

async fn input_stream<S: Stream<Item = [f32; 100]> + Unpin + FusedStream>(mut stream: S) {
    for item in stream.next().await {
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

use futures::stream::iter;

fn test_stream_of_streams(
) -> impl Stream<Item = impl Stream<Item = [f32; 100]> + FusedStream> + FusedStream {
    fn_stream(|emitter| async move {
        emitter.emit(iter(vec![[0.0; 100]; 1000]).fuse()).await;
    })
    .fuse()
}

async fn async_stream_test_switch() {
    //let out = output_stream().await.fuse();
    //pin_mut!(out);
    //let (mut tx, mut rx) = futures::channel::mpsc::channel(1000);

    //tx.send(out).await;
    let streams = test_stream_of_streams();
    pin_mut!(streams);
    switch_streams(streams).await;
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (flume_tx, mut flume_rx) = flume::bounded::<[f32; 100]>(1000usize);
    //let (tx, mut rx) = mpsc::channel::<f32>();
    //let (tx_chunked, mut rx_chunked) = mpsc::channel::<[f32; 10]>();
    let (tx_big_chunked, mut rx_big_chunked) = mpsc::channel::<[f32; 100]>();

    c.bench_function("async_stream_test_switch", |b| {
        b.iter(|| rt.block_on(async_stream_test_switch()))
    });

    c.bench_function("mpsc_test", |b| {
        b.iter(|| mpsc_test(&tx_big_chunked, &mut rx_big_chunked))
    });
    //
    // c.bench_function("flume_test", |b| {
    //     b.iter(|| flume_test(&flume_tx, &mut flume_rx))
    // });

    c.bench_function("async_stream_test", |b| {
        b.iter(|| rt.block_on(async_stream_test()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
