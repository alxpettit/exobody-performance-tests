#![feature(portable_simd)]
use duration_human::DurationHuman;
use itertools::Itertools;
use std::hint::black_box;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;
use tinyvec::ArrayVec;

// #[inline(never)]
// fn mpsc_test_big_chunked_10x_with_iteration(
//     tx: &Sender<[f32; 100]>,
//     rx: &mut Receiver<[f32; 100]>,
// ) {
//     let array = [2f32; 100];
//     for i in black_box(0..100) {
//         let tx_send_data = array.iter().map(|v| black_box(black_box(*v) * 2.));
//         let v: [f32; 100] = tx_send_data.collect_vec().try_into().unwrap();
//         tx.send(black_box(v)).unwrap();
//     }
//     for i in black_box(0..100) {
//         for s in black_box(rx.recv().unwrap()) {
//             black_box(s);
//         }
//     }
// }

#[inline(never)]
fn mpsc() {
    let (tx, mut rx) = mpsc::channel::<f32>();
    tx.send(black_box(5.)).unwrap();

    black_box(rx.recv().unwrap());
}

#[inline(never)]
fn five_plus_six() {
    let a = black_box(5);
    let b = black_box(6);
    // Hardcodes 11 instead of doing calculation during runtime
    black_box(5 + 6);
}

#[inline(never)]
fn main() {
    mpsc();
    //let mut a = 5;

    //let (tx, mut rx) = mpsc::channel::<[f32; 100]>();

    //mpsc_test_big_chunked_10x_with_iteration(&tx, &mut rx);
}
