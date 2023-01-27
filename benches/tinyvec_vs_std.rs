#![feature(portable_simd)]

use criterion::*;
use tinyvec;
use wide::f32x8;

fn tiny_vec_alloc_in_place() {
    let mut v1 = tinyvec::ArrayVec::<[f32x8; 128]>::new();
    let f = f32x8::from([0f32; 8]);
    for _ in 0..100 {
        v1.push(black_box(f + 8.0));
    }
    black_box(v1);
}

fn tiny_vec_ref_alloc(v1: &mut tinyvec::ArrayVec<[f32x8; 128]>) {
    let f = f32x8::from([0f32; 8]);
    for _ in 0..100 {
        v1.push(black_box(f + 8.0));
    }
    v1.clear();
    black_box(v1);
}

fn std_vec_alloc_in_place() {
    let f = f32x8::from([0f32; 8]);
    let mut v2 = Vec::<f32x8>::new();
    for _ in 0..100 {
        v2.push(black_box(f + 8.0));
    }
    black_box(v2);
}

fn std_vec_ref_alloc(v2: &mut Vec<f32x8>) {
    let f = f32x8::from([0f32; 8]);
    for _ in 0..100 {
        v2.push(black_box(f + 8.0));
    }
    v2.clear();
    black_box(v2);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut v1 = tinyvec::ArrayVec::<[f32x8; 128]>::new();
    let mut v2 = Vec::<f32x8>::new();
    c.bench_function("tiny_vec_ref_alloc", |b| {
        b.iter(|| tiny_vec_ref_alloc(&mut v1))
    });
    c.bench_function("tiny_vec_alloc_in_place", |b| {
        b.iter(|| tiny_vec_alloc_in_place())
    });
    c.bench_function("std_vec_ref_alloc", |b| {
        b.iter(|| std_vec_ref_alloc(&mut v2))
    });
    c.bench_function("std_vec_alloc_in_place", |b| {
        b.iter(|| std_vec_alloc_in_place())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
