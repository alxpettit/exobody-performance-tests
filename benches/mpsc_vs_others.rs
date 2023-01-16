#![feature(portable_simd)]

use criterion::*;
use std::ops::Add;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use tinyvec;
use wide::f32x8;

// Result was like 22 microseconds
fn mpsc_test_f32x8(tx: &Sender<f32x8>, rx: &mut Receiver<f32x8>) {
    for i in 0..1000 {
        tx.send(black_box(f32x8::from(0f32))).unwrap();
    }
    for i in 0..1000 {
        black_box(rx.recv().unwrap());
    }
}

// 15us
fn mpsc_test(tx: &Sender<f32>, rx: &mut Receiver<f32>) {
    for i in 0..1000 {
        tx.send(black_box(0f32)).unwrap();
    }
    for i in 0..1000 {
        black_box(rx.recv().unwrap());
    }
}

// 2.8us!!!! :O
fn mpsc_test_chunked(tx: &Sender<[f32; 10]>, rx: &mut Receiver<[f32; 10]>) {
    for i in 0..100 {
        tx.send(black_box([0f32; 10])).unwrap();
    }
    for i in 0..100 {
        black_box(rx.recv().unwrap());
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let (tx, mut rx) = mpsc::channel::<f32>();
    let (tx_chunked, mut rx_chunked) = mpsc::channel::<[f32; 10]>();
    c.bench_function("mpsc_test", |b| b.iter(|| mpsc_test(&tx, &mut rx)));
    c.bench_function("mpsc_test_chunked", |b| {
        b.iter(|| mpsc_test_chunked(&tx_chunked, &mut rx_chunked))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);