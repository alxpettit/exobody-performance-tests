#![feature(portable_simd)]
use duration_human::DurationHuman;
use std::hint::black_box;
use std::time::Instant;
use tinyvec::ArrayVec;

fn main() {
    let mut a = 5;
    a = 10;
    println!("{}", a);
    // black_box(a);
    // let t = || {
    //     println!("hi");
    // };
    // let v1 = Instant::now();
    // // as closure: 27754ns
    // // as closure: 17765ns
    // // as closure: 17765ns
    // //t();
    //
    // println!("hi");
    // //println!("Hello, world!");
    // // let a = ArrayVec<f32x8>::new();
    // // dbg!("hi");
    //
    // let v1_elapsed = DurationHuman::from(v1.elapsed());
    // println!("{}", v1_elapsed);
}
