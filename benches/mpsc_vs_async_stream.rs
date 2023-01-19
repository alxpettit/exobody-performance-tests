use async_fn_stream::fn_stream;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures::{pin_mut, Stream, StreamExt};
use itertools::Itertools;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

fn mpsc_test(tx: &Sender<[f32; 100]>, rx: &mut Receiver<[f32; 100]>) {
    let array = [2f32; 100];
    for i in black_box(0..100) {
        let tx_send_data = array.iter().map(|v| black_box(black_box(*v) * 2.));
        let v: [f32; 100] = tx_send_data.collect_vec().try_into().unwrap();
        tx.send(black_box(v)).unwrap();
    }
    for i in black_box(0..100) {
        for s in black_box(rx.recv().unwrap()) {
            black_box(s);
        }
    }
}

async fn output_stream() -> impl Stream<Item = [f32; 100]> {
    fn_stream(|emitter| async move {
        for i in black_box(0..100) {
            emitter.emit(black_box([0f32; 100])).await;
        }
    })
}

async fn input_stream<S: Stream<Item = [f32; 100]> + Unpin>(mut stream: S) {
    for item in stream.next().await {
        black_box(item);
    }
}

async fn async_stream_test() {
    let out = output_stream().await;
    pin_mut!(out);
    input_stream(out).await;
}

fn criterion_benchmark(c: &mut Criterion) {
    let (tx, mut rx) = mpsc::channel::<f32>();
    let (tx_chunked, mut rx_chunked) = mpsc::channel::<[f32; 10]>();
    let (tx_big_chunked, mut rx_big_chunked) = mpsc::channel::<[f32; 100]>();

    c.bench_function("mpsc_test", |b| {
        b.iter(|| mpsc_test(&tx_big_chunked, &mut rx_big_chunked))
    });

    c.bench_function("async_stream_test", |b| b.iter(|| async_stream_test()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
