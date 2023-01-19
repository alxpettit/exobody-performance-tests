use async_fn_stream::fn_stream;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures::executor::block_on;
use futures::{pin_mut, Stream, StreamExt};
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

async fn input_stream<S: Stream<Item = [f32; 100]> + Unpin>(mut stream: S) {
    for item in stream.next().await {
        // assert_eq!(item, [2f32; 100]);
        black_box(item);
    }
}

// async fn switch_streams(rx: mpsc::Receiver<Box<dyn Stream<Item = [f32; 100]> + Unpin>>) {
//     let (tx, thread_rx) = mpsc::channel();
//     let stream = Arc::new(Mutex::new(None));
//
//     thread::spawn(move || {
//         while let Ok(s) = rx.recv() {
//             *stream.lock().unwrap() = Some(s);
//             tx.send(()).unwrap();
//         }
//     });
//
//     let stream = stream.clone();
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     thread::spawn(move || {
//         while let Ok(_) = thread_rx.recv() {
//             let mut s = stream.lock().unwrap();
//             if let Some(mut s) = s.take() {
//                 for item in rt.block_on(s.next()) {
//                     black_box(item);
//                     //println!("received item: {:?}", item);
//                 }
//             }
//         }
//     });
// }

// // this panics...
// // this is stupid, IDK why chatgpt wrote it this way.
// // why not
// async fn switch_streams<S: Stream<Item = [f32; 100]> + Unpin>(
//     mut rx: tokio::sync::mpsc::Receiver<(S, tokio::sync::mpsc::Sender<()>)>,
// ) {
//     let (stream, mut terminate_tx) = rx.recv().await.unwrap();
//     tokio::pin!(stream);
//     loop {
//         tokio::select! {
//             opt_item = stream.next() => {
//                 match opt_item {
//                     Some(item) => assert_eq!(item, [2f32; 100]),
//                     None => terminate_tx.send(()).await.unwrap(),
//                 };
//             },
//             next_stream = rx.recv() => {
//                 terminate_tx.send(()).await.unwrap();
//                 let (next_stream, next_terminate_tx) = next_stream.unwrap();
//                 *stream = next_stream;
//                 //terminate_tx = next_terminate_tx;
//
//             },
//         }
//     }
// }

//
// async fn input_stream_switch<S: Stream<Item = [f32; 100]> + Unpin + Send>(
//     mut streams: Receiver<S>,
// ) {
//     let m = Arc::new(Mutex::new(streams));
//     thread::scope(move |s| {
//         let mut sm: Option<Arc<Mutex<S>>> = None;
//         s.spawn(move || {
//             dbg!("Spawned outer");
//             while let Ok(mut stream) = m.lock().unwrap().recv() {
//                 sm = Some(Arc::new(Mutex::new(stream)));
//             }
//         });
//
//         s.spawn(|| {
//             dbg!("spawned inner");
//             for item in block_on(sm.unwrap().lock().unwrap().borrow_mut().next()).unwrap() {
//                 //assert_eq!(item, [2f32; 100]);
//                 // dbg!(item);
//                 // let s = format!("{:#?}", item);
//                 //f.write(s.as_bytes()).unwrap();
//                 //writeln!(f, "{:#?}", item).unwrap();
//                 black_box(item);
//             }
//         });
//     });
// }

// 200.72 ns!!!!
async fn async_stream_test() {
    let out = output_stream().await;
    pin_mut!(out);
    input_stream(out).await;
}

async fn async_stream_test_switch() {
    let out = output_stream().await;
    pin_mut!(out);
    let (tx_kill, mut rx_kill) = tokio::sync::mpsc::channel::<()>(100);
    let (tx, mut rx) = tokio::sync::mpsc::channel::<(_, tokio::sync::mpsc::Sender<()>)>(1000);
    tx.send((out, tx_kill)).await;
    //switch_streams(rx).await;
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
