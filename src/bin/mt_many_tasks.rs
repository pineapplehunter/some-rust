#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use rust_riscv_benches::dbg;
use rust_riscv_benches::metrics::get_metrics;
use rust_riscv_benches::thread::{event_loop, spawn};
use rust_riscv_benches::{
    get_thread_id,
    pxet::{
        asm::{smul8, smul16},
        structure::PextVec,
    },
};

#[unsafe(no_mangle)]
#[inline(never)]
fn element_wise_mul_i16(a: &[i16], b: &[i16], result: &mut [i32], threads: usize) {
    assert_eq!(a.len(), result.len());
    // change end if last thread
    for i in (get_thread_id()..result.len()).step_by(threads) {
        result[i] = a[i] as i32 * b[i] as i32
    }
}

#[unsafe(no_mangle)]
#[inline(never)]
fn element_wise_mul_i8(a: &[i8], b: &[i8]) -> Vec<i16> {
    a.iter()
        .zip(b)
        .map(|(a, b)| *a as i16 * *b as i16)
        .collect()
}

#[unsafe(no_mangle)]
#[inline(never)]
#[allow(dead_code)]
fn element_wise_mul_i16_simd(
    a: &PextVec<i16>,
    b: &PextVec<i16>,
    result: &mut PextVec<i32>,
    threads: usize,
) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), result.len());
    let a = a.get_inner();
    let b = b.get_inner();
    let res = unsafe { result.get_inner_mut() };
    let len = a.len();
    for i in (get_thread_id()..len).step_by(threads) {
        unsafe {
            *res.get_unchecked_mut(i * 2) = smul16(*a.get_unchecked(i), *b.get_unchecked(i));
            *res.get_unchecked_mut(i * 2 + 1) =
                smul16(a.get_unchecked(i) >> 32, b.get_unchecked(i) >> 32);
        }
    }
}

#[unsafe(no_mangle)]
#[inline(never)]
fn element_wise_mul_i8_simd(a: &PextVec<i8>, b: &PextVec<i8>) -> PextVec<i16> {
    let a = a.get_inner();
    let b = b.get_inner();
    let out_vec: Vec<usize> = a
        .iter()
        .zip(b)
        .flat_map(|(a, b)| {
            let a_shifted = a >> 32;
            let b_shifted = b >> 32;
            [smul8(*a, *b), smul8(a_shifted, b_shifted)]
        })
        .collect();
    PextVec::<i16>::from_parts(out_vec, a.len() * 2)
}

#[inline(never)]
fn first_hart_entry() {
    let (m, _) = get_metrics(|| {
        for i in 0..50 {
            spawn(move || dbg!("hello" = i));
        }
    });
    dbg!(m);
    event_loop()
}

#[inline(never)]
fn other_hart_entry() {
    event_loop()
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C" fn main(thread_id: usize) {
    if thread_id == 0 {
        first_hart_entry()
    } else {
        other_hart_entry()
    }
    // if get_thread_id() == 0 {
    //     println!("Rust on B4SMT");
    //     unsafe {
    //         println!(
    //             "Heap from = {:?} to {:?} size = {}",
    //             addr_of!(PROGRAM_END),
    //             addr_of!(HEAP_END),
    //             addr_of!(HEAP_END) as usize - addr_of!(PROGRAM_END) as usize
    //         );
    //     }

    //     let (met, _) = get_metrics(|| ());
    //     println!("sample metrics = {:?}", met);

    //     let sample = PextVec::<u16>::from(&[1, 2, 3, 4, 5][..]);
    //     println!("i16 sample = {:?}", sample);

    //     let sample = PextVec::<i8>::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9][..]);
    //     println!("i8  sample = {:?}", sample);

    //     // initialize tasks
    //     {
    //         *I16_TASKS.write() = Some(vec![
    //             TaskStatus {
    //                 ready_count: 0,
    //                 start: false,
    //                 normal_result: None,
    //                 pext_result: None,
    //                 done_count: 0,
    //             };
    //             get_thread_count()
    //         ]);
    //     }

    //     // let guard = i8_tasks.write();
    //     // *guard = Some(vec![
    //     //     TaskStatus {
    //     //         ready_count: 0,
    //     //         start: false,
    //     //         normal_result: None,
    //     //         pext_result: None,
    //     //         done_count: 0,
    //     //     };
    //     //     get_thread_count()
    //     // ]);
    // }

    // // wait for task initialize
    // while I16_TASKS.read().is_none() {
    //     delay(30);
    // }

    // dbg!("calc start");

    // for threads in (0..get_thread_count()).rev() {
    //     // stop higher threads
    //     if get_thread_id() > threads {
    //         return;
    //     }

    //     // let before_instant = Metrics::get_instant();
    //     // tell main thread that thread is ready
    //     {
    //         let mut guard = I16_TASKS.write();
    //         guard.as_mut().unwrap()[threads].ready_count += 1;
    //         // dbg!("threads inc", guard.as_mut().unwrap()[threads].ready_count);
    //     }
    //     // get
    //     if get_thread_id() == 0 {
    //         {
    //             let mut guard = I16_TASKS.write();
    //             guard.as_mut().unwrap()[threads].pext_result =
    //                 Some(PextVec::<i32>::filled(0, TEST_DATA_A_I16.len()));
    //             guard.as_mut().unwrap()[threads].normal_result =
    //                 Some(vec![0; TEST_DATA_A_I16.len()]);
    //         }
    //         // dbg!("wait for all ready");
    //         while I16_TASKS.read().as_ref().unwrap()[threads].ready_count != threads + 1 {
    //             delay(30);
    //         }
    //         I16_TASKS.write().as_mut().unwrap()[threads].start = true;
    //         // dbg!("set start");
    //     } else {
    //         while !I16_TASKS.read().as_ref().unwrap()[threads].start {
    //             delay(30);
    //         }
    //     };

    //     dbg!("start");

    //     // i16
    //     let res1: &mut [i32] = unsafe {
    //         let guard = I16_TASKS.read();
    //         let len = guard.as_ref().unwrap()[threads]
    //             .normal_result
    //             .as_ref()
    //             .unwrap()
    //             .len();
    //         let ptr = guard.as_ref().unwrap()[threads]
    //             .normal_result
    //             .as_ref()
    //             .unwrap()
    //             .as_ptr();
    //         &mut *slice_from_raw_parts_mut(ptr as *mut _, len)
    //     };
    //     let (metrics1, _) = get_metrics(|| {
    //         element_wise_mul_i16(TEST_DATA_A_I16, TEST_DATA_B_I16, res1, threads + 1)
    //     });

    //     println!("{},norm,{}", threads + 1, metrics1.csv());

    //     let a2 = PextVec::from(TEST_DATA_A_I16);
    //     let b2 = PextVec::from(TEST_DATA_B_I16);
    //     let res2 = unsafe {
    //         let guard = I16_TASKS.read();
    //         let pv = guard.as_ref().unwrap()[threads]
    //             .pext_result
    //             .as_ref()
    //             .unwrap();
    //         let addr = addr_of!(*pv) as *mut _;
    //         &mut *addr
    //     };
    //     let (metrics2, _) = get_metrics(|| element_wise_mul_i16_simd(&a2, &b2, res2, threads + 1));

    //     println!("{},pext,{}", threads + 1, metrics2.csv());

    //     I16_TASKS.write().as_mut().unwrap()[threads].done_count += 1;
    //     while I16_TASKS.read().as_ref().unwrap()[threads].done_count != threads + 1 {
    //         delay(30);
    //     }
    //     if thread_id == 0 {
    //         let mut guard = I16_TASKS.write();

    //         let res1 = guard.as_ref().unwrap()[threads]
    //             .normal_result
    //             .as_ref()
    //             .unwrap();
    //         let res2 = guard.as_ref().unwrap()[threads]
    //             .pext_result
    //             .as_ref()
    //             .unwrap();

    //         if res1.iter().zip(res2.iter()).all(|(&a, b)| a == b) {
    //             println!("OK");
    //             // println!("{} metric1={:?}", thread_id, cycle1);
    //             // println!("{} metric2={:?}", thread_id, cycle2);
    //         } else {
    //             println!("NG");
    //             println!("res1 = {:?}", res1);
    //             println!("res2 = {:?}", res2);
    //         }

    //         guard.as_mut().unwrap()[threads].normal_result = None;
    //         guard.as_mut().unwrap()[threads].pext_result = None;
    //     }

    //     // let after_instant = Metrics::get_instant();
    //     // println!(
    //     //     "{} {} total metrics {:?}",
    //     //     threads + 1,
    //     //     thread_id,
    //     //     after_instant - before_instant
    //     // );

    //     // // i8
    //     // let (cycle1, res1) = get_metrics(|| element_wise_mul_i8(TEST_DATA_A_I8, TEST_DATA_B_I8));

    //     // let a2 = PextVec::from(TEST_DATA_A_I8);
    //     // let b2 = PextVec::from(TEST_DATA_B_I8);
    //     // let (cycle2, res2) = get_metrics(|| element_wise_mul_i8_simd(&a2, &b2));

    //     // if res1.iter().zip(res2.iter()).all(|(&a, b)| a == b) {
    //     //     println!("{} OK", thread_id);
    //     //     println!("{} metric1={:?}", thread_id, cycle1);
    //     //     println!("{} metric2={:?}", thread_id, cycle2);
    //     // } else {
    //     //     println!("{} NG", thread_id);
    //     //     println!("{} res1 = {:?}", thread_id, res1);
    //     //     println!("{} res2 = {:?}", thread_id, res2);
    //     // }
    // }
}
