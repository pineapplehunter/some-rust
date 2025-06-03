#![no_std]
#![no_main]

extern crate alloc;

use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use rust_riscv_benches::metrics::{Metrics, MetricsCSV, get_metrics};
use rust_riscv_benches::pxet::asm::{ucmplt8, ucmplt16};
use rust_riscv_benches::pxet::structure::PextVec;
use rust_riscv_benches::thread::{event_loop, event_loop_until_empty, spawn};
use rust_riscv_benches::{get_thread_count, println};

#[unsafe(no_mangle)]
#[inline(never)]
fn median_i16(input: &[i16], result: &mut [i16]) {
    assert_eq!(input.len(), result.len());
    result[0] = 0;
    result[result.len() - 1] = 0;

    for (win, res) in input.windows(3).zip(result.iter_mut().skip(1)) {
        let a = win[0];
        let b = win[1];
        let c = win[2];

        if a < b {
            if b < c {
                *res = b;
            } else if c < a {
                *res = a;
            } else {
                *res = c;
            }
        } else {
            if a < c {
                *res = a;
            } else if c < b {
                *res = b;
            } else {
                *res = c;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[inline(never)]
fn median_i8(input: &[i8], result: &mut [i8]) {
    assert_eq!(input.len(), result.len());
    result[0] = 0;
    result[result.len() - 1] = 0;

    for (win, res) in input.windows(3).zip(result.iter_mut().skip(1)) {
        let a = win[0];
        let b = win[1];
        let c = win[2];

        if a < b {
            if b < c {
                *res = b;
            } else if c < a {
                *res = a;
            } else {
                *res = c;
            }
        } else {
            if a < c {
                *res = a;
            } else if c < b {
                *res = b;
            } else {
                *res = c;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[inline(never)]
fn median_p_i16(input: &PextVec<i16>, result: &mut PextVec<i16>) {
    for (win, out) in input
        .as_slice()
        .windows(2)
        .chain([&[*input.as_slice().last().unwrap(), 0][..]]) // last element as exception
        .zip(result.as_mut_slice())
    {
        let v1 = win[0];
        let vt = win[1];

        let v2 = (v1 >> 16) | (vt << 48);
        let v3 = (v1 >> 32) | (vt << 32);

        let c1 = ucmplt16(v1, v2);
        let c2 = ucmplt16(v2, v3);
        let c3 = ucmplt16(v3, v1);

        let m1 = !(c1 ^ c3);
        let m2 = !(c1 ^ c2);
        let m3 = !(c2 ^ c3);

        let o1 = v1 & m1;
        let o2 = v2 & m2;
        let o3 = v3 & m3;

        let res = o1 | o2 | o3;
        *out = res;
    }
}

#[unsafe(no_mangle)]
#[inline(never)]
fn median_p_i8(input: &PextVec<i8>, result: &mut PextVec<i8>) {
    for (win, out) in input
        .as_slice()
        .windows(2)
        .chain([&[*input.as_slice().last().unwrap(), 0][..]]) // last element as exception
        .zip(result.as_mut_slice())
    {
        let v1 = win[0];
        let vt = win[1];

        let v2 = (v1 >> 8) | (vt << 56);
        let v3 = (v1 >> 16) | (vt << 48);

        let c1 = ucmplt8(v1, v2);
        let c2 = ucmplt8(v2, v3);
        let c3 = ucmplt8(v3, v1);

        let m1 = !(c1 ^ c3);
        let m2 = !(c1 ^ c2);
        let m3 = !(c2 ^ c3);

        let o1 = v1 & m1;
        let o2 = v2 & m2;
        let o3 = v3 & m3;

        let res = o1 | o2 | o3;
        *out = res;
    }
}

const TEST_DATA_A_I16: &[i16] = &[
    12206, -19181, -19180, -13177, -8985, 16826, -13831, -10524, 20724, -18190, 26366, -32454,
    -1569, 1556, -21268, 20407, 6439, -19077, 5492, 27923, -4467, -12965, -27812, -29628, 14609,
    -16203, 31382, -12490, 12623, 4996, 20750, 17954, -18527, -9420, 20412, -3505, 2641, 22606,
    -2715, 12224, 22502, 16984, -16806, -22038, -18797, 32733, 21229, 3553, -11190, -17255, -11713,
    -2799, 16136, -15855, -62, -19478, -15975, 12384, -16882, -13909, -4334, -11591, 6686, -8634,
    -6601, -3137, 25471, 3157, 14112, -12427, -9259, -32650, 14280, -15548, 26959, -26331, 18033,
    2214, -9177, 25745, -25870, -18920, -30785, 29514, 13466, -19600, -24369, 12399, -21776, 13909,
    -15719, -11721, 4904, -5286, 2477, 27120, -26589, 870, 15416, 5909, -14138, 10438, 5934, 18238,
    27125, -4212, -16420, -10567, -30339, 26143, -25742, 30929, -1548, -10460, -32042, -30855,
    21748, 25416, -487, 5179, -4404, -12130, 24995, -14515, -7768, 32303, -23078, 3392, -14195,
    -19011, -2716, -25004, 14190, -13570, -30804, -3979, -1549, 13477, 17505, 7514, 7084, 5094,
    -16567, 25577, 9468, 23785, 4253, -26583, 24947, -20389, -1642, -7817, 13914, -19369, -23335,
    29744, 20838, -16142, -16429, 28786, -24671, 17623, 5342, -18388, -15962, 5420, -30684, -2362,
    -9237, -48, -15246, 19872, -12873, -6784, 3896, -29563, -18835, 31004, -2126, 21642, -8850,
    -12655, 2243, 14987, 10022, -22197, 23527, -8331, 6176, 25696, -7286, 3455, -7334, -25608,
    -30018, 13239, -13208, 12333, 31261, -14736, -22879, 17688, 5455, 14828, -13001, 20953, -15967,
    -27116, 30443, 20439, 22581, 29436, 25579, 11452, 29117, -1680, -21765, 22174, -26826, -11693,
    15617, 8611, 12707, 13790, 7679, -5809, 11668, -16160, 12496, -23913, 23263, -19127, -18955,
    48, 6369, 15380, -28597, 2999, -19365, 10494, -2404, 18789, -6529, -30986, 26102, 23955,
    -15534, 8792, 18872, 1907, 8920, -32701, -26695, -25634, -30923, 7920, 12206, -19181, -19180,
    -13177, -8985, 16826, -13831, -10524, 20724, -18190, 26366, -32454, -1569, 1556, -21268, 20407,
    6439, -19077, 5492, 27923, -4467, -12965, -27812, -29628, 14609, -16203, 31382, -12490, 12623,
    4996, 20750, 17954, -18527, -9420, 20412, -3505, 2641, 22606, -2715, 12224, 22502, 16984,
    -16806, -22038, -18797, 32733, 21229, 3553, -11190, -17255, -11713, -2799, 16136, -15855, -62,
    -19478, -15975, 12384, -16882, -13909, -4334, -11591, 6686, -8634, -6601, -3137, 25471, 3157,
    14112, -12427, -9259, -32650, 14280, -15548, 26959, -26331, 18033, 2214, -9177, 25745, -25870,
    -18920, -30785, 29514, 13466, -19600, -24369, 12399, -21776, 13909, -15719, -11721, 4904,
    -5286, 2477, 27120, -26589, 870, 15416, 5909, -14138, 10438, 5934, 18238, 27125, -4212, -16420,
    -10567, -30339, 26143, -25742, 30929, -1548, -10460, -32042, -30855, 21748, 25416, -487, 5179,
    -4404, -12130, 24995, -14515, -7768, 32303, -23078, 3392, -14195, -19011, -2716, -25004, 14190,
    -13570, -30804, -3979, -1549, 13477, 17505, 7514, 7084, 5094, -16567, 25577, 9468, 23785, 4253,
    -26583, 24947, -20389, -1642, -7817, 13914, -19369, -23335, 29744, 20838, -16142, -16429,
    28786, -24671, 17623, 5342, -18388, -15962, 5420, -30684, -2362, -9237, -48, -15246, 19872,
    -12873, -6784, 3896, -29563, -18835, 31004, -2126, 21642, -8850, -12655, 2243, 14987, 10022,
    -22197, 23527, -8331, 6176, 25696, -7286, 3455, -7334, -25608, -30018, 13239, -13208, 12333,
    31261, -14736, -22879, 17688, 5455, 14828, -13001, 20953, -15967, -27116, 30443, 20439, 22581,
    29436, 25579, 11452, 29117, -1680, -21765, 22174, -26826, -11693, 15617, 8611, 12707, 13790,
    7679, -5809, 11668, -16160, 12496, -23913, 23263, -19127, -18955, 48, 6369, 15380, -28597,
    2999, -19365, 10494, -2404, 18789, -6529, -30986, 26102, 23955, -15534, 8792, 18872, 1907,
    8920, -32701, -26695, -25634, -30923, 7920, 12206, -19181, -19180, -13177, -8985, 16826,
    -13831, -10524, 20724, -18190, 26366, -32454, -1569, 1556, -21268, 20407, 6439, -19077, 5492,
    27923, -4467, -12965, -27812, -29628, 14609, -16203, 31382, -12490, 12623, 4996, 20750, 17954,
    -18527, -9420, 20412, -3505, 2641, 22606, -2715, 12224, 22502, 16984, -16806, -22038, -18797,
    32733, 21229, 3553, -11190, -17255, -11713, -2799, 16136, -15855, -62, -19478, -15975, 12384,
    -16882, -13909, -4334, -11591, 6686, -8634, -6601, -3137, 25471, 3157, 14112, -12427, -9259,
    -32650, 14280, -15548, 26959, -26331, 18033, 2214, -9177, 25745, -25870, -18920, -30785, 29514,
    13466, -19600, -24369, 12399, -21776, 13909, -15719, -11721, 4904, -5286, 2477, 27120, -26589,
    870, 15416, 5909, -14138, 10438, 5934, 18238, 27125, -4212, -16420, -10567, -30339, 26143,
    -25742, 30929, -1548, -10460, -32042, -30855, 21748, 25416, -487, 5179, -4404, -12130, 24995,
    -14515, -7768, 32303, -23078, 3392, -14195, -19011, -2716, -25004, 14190, -13570, -30804,
    -3979, -1549, 13477, 17505, 7514, 7084, 5094, -16567, 25577, 9468, 23785, 4253, -26583, 24947,
    -20389, -1642, -7817, 13914, -19369, -23335, 29744, 20838, -16142, -16429, 28786, -24671,
    17623, 5342, -18388, -15962, 5420, -30684, -2362, -9237, -48, -15246, 19872, -12873, -6784,
    3896, -29563, -18835, 31004, -2126, 21642, -8850, -12655, 2243, 14987, 10022, -22197, 23527,
    -8331, 6176, 25696, -7286, 3455, -7334, -25608, -30018, 13239, -13208, 12333, 31261, -14736,
    -22879, 17688, 5455, 14828, -13001, 20953, -15967, -27116, 30443, 20439, 22581, 29436, 25579,
    11452, 29117, -1680, -21765, 22174, -26826, -11693, 15617, 8611, 12707, 13790, 7679, -5809,
    11668, -16160, 12496, -23913, 23263, -19127, -18955, 48, 6369, 15380, -28597, 2999, -19365,
    10494, -2404, 18789, -6529, -30986, 26102, 23955, -15534, 8792, 18872, 1907, 8920, -32701,
    -26695, -25634, -30923, 7920, 12206, -19181, -19180, -13177, -8985, 16826, -13831, -10524,
    20724, -18190, 26366, -32454, -1569, 1556, -21268, 20407, 6439, -19077, 5492, 27923, -4467,
    -12965, -27812, -29628, 14609, -16203, 31382, -12490, 12623, 4996, 20750, 17954, -18527, -9420,
    20412, -3505, 2641, 22606, -2715, 12224, 22502, 16984, -16806, -22038, -18797, 32733, 21229,
    3553, -11190, -17255, -11713, -2799, 16136, -15855, -62, -19478, -15975, 12384, -16882, -13909,
    -4334, -11591, 6686, -8634, -6601, -3137, 25471, 3157, 14112, -12427, -9259, -32650, 14280,
    -15548, 26959, -26331, 18033, 2214, -9177, 25745, -25870, -18920, -30785, 29514, 13466, -19600,
    -24369, 12399, -21776, 13909, -15719, -11721, 4904, -5286, 2477, 27120, -26589, 870, 15416,
    5909, -14138, 10438, 5934, 18238, 27125, -4212, -16420, -10567, -30339, 26143, -25742, 30929,
    -1548, -10460, -32042, -30855, 21748, 25416, -487, 5179, -4404, -12130, 24995, -14515, -7768,
    32303, -23078, 3392, -14195, -19011, -2716, -25004, 14190, -13570, -30804, -3979, -1549, 13477,
    17505, 7514, 7084, 5094, -16567, 25577, 9468, 23785, 4253, -26583, 24947, -20389, -1642, -7817,
    13914, -19369, -23335, 29744, 20838, -16142, -16429, 28786, -24671, 17623, 5342, -18388,
    -15962, 5420, -30684, -2362, -9237, -48, -15246, 19872, -12873, -6784, 3896, -29563, -18835,
    31004, -2126, 21642, -8850, -12655, 2243, 14987, 10022, -22197, 23527, -8331, 6176, 25696,
    -7286, 3455, -7334, -25608, -30018, 13239, -13208, 12333, 31261, -14736, -22879, 17688, 5455,
    14828, -13001, 20953, -15967, -27116, 30443, 20439, 22581, 29436, 25579, 11452, 29117, -1680,
    -21765, 22174, -26826, -11693, 15617, 8611, 12707, 13790, 7679, -5809, 11668, -16160, 12496,
    -23913, 23263, -19127, -18955, 48, 6369, 15380, -28597, 2999, -19365, 10494, -2404, 18789,
    -6529, -30986, 26102, 23955, -15534, 8792, 18872, 1907, 8920, -32701, -26695, -25634, -30923,
    7920,
];

fn threaded_bench(threads: usize) {
    {
        // get teady for threading
        let mut handles = Vec::with_capacity(threads);
        // this is totaly unsafe, so becareful to what index to use
        let a = TEST_DATA_A_I16
            .iter()
            .cloned()
            .take(1024)
            .collect::<Vec<_>>();
        let a_arc = Arc::new(a);

        let (group_metric, individual_metrics) = get_metrics(|| {
            for _ in 0..threads {
                let a_arc = a_arc.clone();
                let handle = spawn(move || {
                    let mut out = vec![0; 1024];
                    let m = get_metrics(|| {
                        median_i16(&a_arc, &mut out);
                    })
                    .0;
                    // assert!(out.inner.iter().zip(CORRECT_OUTPUT).all(|(a, b)| a == b));
                    m
                });
                handles.push(handle);
            }
            event_loop_until_empty();
            let metrics: Vec<Metrics> = handles.into_iter().map(|h| *h.join()).collect();
            metrics
        });

        for m in individual_metrics {
            println!("i,16,{},baseline,{}", threads, m.csv());
        }
        println!("g,16,{},baseline,{}", threads, group_metric.csv());
    }

    // pext array

    {
        // get teady for threading
        let mut handles: Vec<_> = Vec::with_capacity(threads);

        let a = PextVec::from(TEST_DATA_A_I16.iter().cloned().take(1024));
        let a_arc = Arc::new(a);

        let (group_metric, individual_metrics) = get_metrics(|| {
            for _ in 0..threads {
                let a_arc = a_arc.clone();
                let handle = spawn(move || {
                    let mut out = PextVec::<i16>::filled(0, 1024);
                    let m = get_metrics(|| {
                        median_p_i16(&a_arc, &mut out);
                    })
                    .0;
                    core::hint::black_box(out);
                    // assert!(out.inner.iter().zip(CORRECT_OUTPUT).all(|(a, b)| a == b));
                    m
                });
                handles.push(handle);
            }
            event_loop_until_empty();
            let metrics: Vec<Metrics> = handles.into_iter().map(|h| *h.join()).collect();
            metrics
        });

        for m in individual_metrics {
            println!("i,16,{},pext16,{}", threads, m.csv());
        }
        println!("g,16,{},pext16,{}", threads, group_metric.csv());
    }
    // baseline 8
    {
        // get teady for threading
        let mut handles = Vec::with_capacity(threads);
        // this is totaly unsafe, so becareful to what index to use
        let a = TEST_DATA_A_I16
            .iter()
            .cloned()
            .take(1024)
            .map(|x| x as i8)
            .collect::<Vec<_>>();
        let a_arc = Arc::new(a);

        let (group_metric, individual_metrics) = get_metrics(|| {
            for _ in 0..threads {
                let a_arc = a_arc.clone();
                let handle = spawn(move || {
                    let mut out = vec![0; 1024];
                    let m = get_metrics(|| {
                        median_i8(&a_arc, &mut out);
                    })
                    .0;
                    // assert!(out.inner.iter().zip(CORRECT_OUTPUT).all(|(a, b)| a == b));
                    m
                });
                handles.push(handle);
            }
            event_loop_until_empty();
            let metrics: Vec<Metrics> = handles.into_iter().map(|h| *h.join()).collect();
            metrics
        });

        for m in individual_metrics {
            println!("i,8,{},baseline,{}", threads, m.csv());
        }
        println!("g,8,{},baseline,{}", threads, group_metric.csv());
    }

    // pext array

    {
        // get teady for threading
        let mut handles: Vec<_> = Vec::with_capacity(threads);

        let a = PextVec::from(TEST_DATA_A_I16.iter().cloned().map(|x| x as i8).take(1024));
        let a_arc = Arc::new(a);

        let (group_metric, individual_metrics) = get_metrics(|| {
            for _ in 0..threads {
                let a_arc = a_arc.clone();
                let handle = spawn(move || {
                    let mut out = PextVec::<i8>::filled(0, 1024);
                    let m = get_metrics(|| {
                        median_p_i8(&a_arc, &mut out);
                    })
                    .0;
                    // assert!(out.inner.iter().zip(CORRECT_OUTPUT).all(|(a, b)| a == b));
                    m
                });
                handles.push(handle);
            }
            event_loop_until_empty();
            let metrics: Vec<Metrics> = handles.into_iter().map(|h| *h.join()).collect();
            metrics
        });

        for m in individual_metrics {
            println!("i,8,{},pext8,{}", threads, m.csv());
        }
        println!("g,8,{},pext8,{}", threads, group_metric.csv());
    }
}

#[inline(never)]
fn first_hart_entry() {
    println!("B4SMT evaluation program");
    println!(
        "This program measures the performance difference of normal and pext matrix multiplication of arrays."
    );
    println!("START");

    println!(
        "individual-group,elemnt,threads,type,{}",
        MetricsCSV::HEADER
    );
    let thread_count = get_thread_count();
    for i in (1..=thread_count).rev() {
        threaded_bench(i);
        // spawn(|| loop {});
        // i8_bench(tasks, i);
    }

    println!("END")
}

#[inline(never)]
fn other_hart_entry() {
    event_loop()
}

#[inline(never)]
#[unsafe(no_mangle)]
fn main(thread_id: usize) {
    if thread_id == 0 {
        first_hart_entry()
    } else {
        other_hart_entry()
    }
}
