/*
Implement basic function to split some generic computational work between threads.
Split should occur only on some threshold - if computational work (input length) is shorter than
this threshold, no splitting should occur and no threads should be created.

You get as input:

1. Vec<T>
2. Function f(t: T) -> R


Threshold can be just constant.

You should return:
   1. Up to you, but probably some Vec of the same length as input(1)

Code should be published on github.
 */

use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;


fn main() {
    let result1 = run1(
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25].to_vec(),
        |t: u8| {
            println!("Ft: {}", t);
            thread::sleep(Duration::from_millis(200));
            t*2+1
        }
    );

    let result2 = run2(
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25].to_vec(),
        |t: u8| {
            println!("Ft: {}", t);
            thread::sleep(Duration::from_millis(200));
            t*2+1
        }
    );

    println!("Result1: {:?}", result1);
    println!("Result2: {:?}", result2);
}

fn run2<T, F, R>(mut vec: Vec<T>, f: F) -> Vec<bool>
where
    F: Fn(T) -> R + Send + Sync,
    T: Send
{
    const THRESHOLD: usize = 15;
    const CHUNK_SIZE: usize = 8;

    let vec_len = vec.len();

    if vec.len() > THRESHOLD {
        crossbeam::thread::scope(|s| {
            let arc_f = Arc::new(f);
            while vec.len() > 0 {
                let at = if vec.len() > CHUNK_SIZE { vec.len() - CHUNK_SIZE } else { 0 };

                let chunk = vec.split_off(at);
                let thread_f = arc_f.clone();

                s.spawn(move |_| {
                    for t in chunk {
                        thread_f(t);
                    }
                });
            }
        }).expect("Failed at scope");

        vec![true; vec_len]
    } else {
        let mut result_vec = Vec::with_capacity(vec.len());
        for t in vec {
            f(t);
            result_vec.push(true);
        }
        result_vec
    }
}

fn run1<T, F, R>(mut vec: Vec<T>, f: F) -> Vec<R>
where
    F: Fn(T) -> R + Send + Clone + Sync + 'static,
    T: Send + Clone + 'static,
    R: Send + 'static,
{
    const THRESHOLD: usize = 15;
    const CHUNK_SIZE: usize = 8;

    if vec.len() > THRESHOLD {
        let mut handles: Vec<JoinHandle<Vec<R>>> = Vec::new();
        let mut result = Vec::with_capacity(vec.len());

        while vec.len() > 0 {
            let at = if vec.len() > CHUNK_SIZE { vec.len() - CHUNK_SIZE } else { 0 };
            println!("L: {}, {}", vec.len(), at);
            let chunk = vec.split_off(at);
            let thread_f = f.clone();
            handles.push(thread::spawn(move || {
                let mut result_chunk = Vec::with_capacity(chunk.len());
                for t in chunk {
                    result_chunk.push(thread_f(t));
                }
                result_chunk
            }));
        }

        handles.reverse();
        for handle in handles {
            let mut chunk = handle.join().expect("Failed join thread");
            result.append(&mut chunk);
        }

        result
    } else {
        let mut result_vec = Vec::with_capacity(vec.len());
        for t in vec {
            result_vec.push(f(t));
        }
        result_vec
    }
}
