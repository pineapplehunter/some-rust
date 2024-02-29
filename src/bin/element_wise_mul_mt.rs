#![no_std]
#![no_main]

extern crate alloc;

use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use rust_riscv_benches::metrics::{get_metrics, Metrics, MetricsCSV};
use rust_riscv_benches::sync::UnsafeSyncCell;
use rust_riscv_benches::thread::{event_loop, event_loop_until_empty, spawn};
use rust_riscv_benches::{
    get_thread_count, println,
    pxet::{
        asm::{smul16, smul8},
        structure::PextVec,
    },
};

#[no_mangle]
#[inline(never)]
fn element_wise_mul_i16(a: &[i16], b: &[i16], result: &mut [i32], threads: usize, index: usize) {
    for i in (0..result.len()).skip(index).step_by(threads) {
        result[i] = a[i] as i32 * b[i] as i32;
    }
}

#[no_mangle]
#[inline(never)]
fn element_wise_mul_i8(a: &[i8], b: &[i8], result: &mut [i16], threads: usize, index: usize) {
    for i in (0..result.len()).skip(index).step_by(threads) {
        result[i] = a[i] as i16 * b[i] as i16;
    }
}

#[no_mangle]
#[inline(never)]
#[allow(dead_code)]
fn element_wise_mul_i16_simd(
    a: &[usize],
    b: &[usize],
    result: &mut [usize],
    threads: usize,
    index: usize,
) {
    for i in (0..result.len()).skip(index).step_by(threads) {
        unsafe {
            let val_a = a.get_unchecked(i);
            let val_b = b.get_unchecked(i);
            *result.get_unchecked_mut(i * 2) = smul16(*val_a, *val_b);
            *result.get_unchecked_mut(i * 2 + 1) = smul16(*val_a >> 32, *val_b >> 32);
        }
    }
}

#[no_mangle]
#[inline(never)]
#[allow(dead_code)]
fn element_wise_mul_i8_simd(
    a: &[usize],
    b: &[usize],
    result: &mut [usize],
    threads: usize,
    index: usize,
) {
    for i in (0..result.len()).skip(index).step_by(threads) {
        unsafe {
            let val_a = a.get_unchecked(i);
            let val_b = b.get_unchecked(i);
            *result.get_unchecked_mut(i * 2) = smul8(*val_a, *val_b);
            *result.get_unchecked_mut(i * 2 + 1) = smul8(*val_a >> 32, *val_b >> 32);
        }
    }
}

static TEST_DATA_A_I16: &[i16] = &[
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
    8920, -32701, -26695, -25634, -30923, 7920,
];

static TEST_DATA_B_I16: &[i16] = &[
    27965, 21325, -20166, 20583, -6355, 11500, -13379, -12881, -20977, 22291, -22814, 23399, 19414,
    9728, -23628, -18570, -24113, 15854, 12438, -9205, 13612, -27364, -23106, -16927, -5284, -827,
    28238, 22085, -6244, -18656, 27303, -14389, 6117, 17068, 31000, 31920, -26293, -21720, -7070,
    2116, 28751, -6170, 7292, 19007, 15906, -1410, -18772, 19035, 30461, 21578, 21742, -5502,
    -12916, 32173, 22949, -29378, 27273, -10465, -31654, -4951, -2173, -727, 22530, -32701, -27760,
    -4433, 261, -12942, 28993, -28582, 12007, 17673, -7576, 10185, 24255, -17313, -30777, 9256,
    -11250, 700, -5600, 907, -5052, 17, -25771, 3645, 29006, 26969, -3925, -82, -3012, 3745, -5150,
    -2186, 10410, 7433, -19844, 28948, 22566, -27330, 7477, -17880, 4645, 30797, 24661, 25790, 448,
    -32186, 17012, 16921, 18285, 6101, 25874, -7127, 32513, -19943, 21264, -988, -17097, 30782,
    20891, 22650, 6004, -11778, -5879, -6173, -4320, 20982, 20940, 23868, 32365, -21246, -29436,
    21864, -15512, 9265, 26123, 9361, 21321, -20574, -25511, 26227, -15012, -24659, 20007, -17760,
    20502, 31789, 18769, -6597, -30324, 18408, -30129, -8583, 19787, 11288, 12367, 20003, -18114,
    30167, 18533, 25834, 26840, -20710, 11560, -7264, -15348, -15621, -25853, 9419, 32662, 27480,
    9691, 26570, 26602, -21326, 8706, -24367, -23495, 21973, 29148, 16235, -15175, 10768, 2239,
    30534, 31909, -28764, -21928, 4738, 29705, -26194, 1596, 22692, -26188, -14956, -23253, 7927,
    -2412, 2469, -23483, -760, 19465, 5362, 8367, 17454, -25126, 6417, -28690, -30119, -5048,
    -3077, 21108, -18611, 26691, -26719, 16003, -32676, 6329, -31582, 15019, 11497, 32101, -26641,
    23643, 3974, 22960, 29167, 6848, -16962, -17778, 31648, -3930, 3887, 15786, -4353, 2645,
    -12435, -31042, 2030, -26385, -16648, 7300, 27404, -6193, 26442, 31378, -1111, -16736, -32418,
    -4396, 10951, 26414, 21041, 4702, -11641, 27965, 21325, -20166, 20583, -6355, 11500, -13379,
    -12881, -20977, 22291, -22814, 23399, 19414, 9728, -23628, -18570, -24113, 15854, 12438, -9205,
    13612, -27364, -23106, -16927, -5284, -827, 28238, 22085, -6244, -18656, 27303, -14389, 6117,
    17068, 31000, 31920, -26293, -21720, -7070, 2116, 28751, -6170, 7292, 19007, 15906, -1410,
    -18772, 19035, 30461, 21578, 21742, -5502, -12916, 32173, 22949, -29378, 27273, -10465, -31654,
    -4951, -2173, -727, 22530, -32701, -27760, -4433, 261, -12942, 28993, -28582, 12007, 17673,
    -7576, 10185, 24255, -17313, -30777, 9256, -11250, 700, -5600, 907, -5052, 17, -25771, 3645,
    29006, 26969, -3925, -82, -3012, 3745, -5150, -2186, 10410, 7433, -19844, 28948, 22566, -27330,
    7477, -17880, 4645, 30797, 24661, 25790, 448, -32186, 17012, 16921, 18285, 6101, 25874, -7127,
    32513, -19943, 21264, -988, -17097, 30782, 20891, 22650, 6004, -11778, -5879, -6173, -4320,
    20982, 20940, 23868, 32365, -21246, -29436, 21864, -15512, 9265, 26123, 9361, 21321, -20574,
    -25511, 26227, -15012, -24659, 20007, -17760, 20502, 31789, 18769, -6597, -30324, 18408,
    -30129, -8583, 19787, 11288, 12367, 20003, -18114, 30167, 18533, 25834, 26840, -20710, 11560,
    -7264, -15348, -15621, -25853, 9419, 32662, 27480, 9691, 26570, 26602, -21326, 8706, -24367,
    -23495, 21973, 29148, 16235, -15175, 10768, 2239, 30534, 31909, -28764, -21928, 4738, 29705,
    -26194, 1596, 22692, -26188, -14956, -23253, 7927, -2412, 2469, -23483, -760, 19465, 5362,
    8367, 17454, -25126, 6417, -28690, -30119, -5048, -3077, 21108, -18611, 26691, -26719, 16003,
    -32676, 6329, -31582, 15019, 11497, 32101, -26641, 23643, 3974, 22960, 29167, 6848, -16962,
    -17778, 31648, -3930, 3887, 15786, -4353, 2645, -12435, -31042, 2030, -26385, -16648, 7300,
    27404, -6193, 26442, 31378, -1111, -16736, -32418, -4396, 10951, 26414, 21041, 4702, -11641,
];

static TEST_DATA_A_I8: &[i8] = &[
    -120, 64, -91, -79, -51, 96, 85, 126, 116, 121, -12, 29, -25, -25, -93, 9, -63, 92, 13, -67,
    -102, 107, -65, -36, 50, -16, -72, 94, 11, 80, 25, 27, 105, -2, 55, -9, 117, -87, 105, -70,
    -104, -15, -117, 6, 1, 83, 61, 61, -83, 59, 5, 90, 44, -18, -69, 93, 25, -128, 84, -56, -19,
    116, 108, 83, -34, -60, 12, 83, 88, -15, 43, 111, 52, -12, -37, 99, 5, 33, 102, -106, -34, -90,
    27, 123, -70, 3, 12, 44, -71, 110, 4, -9, 52, -64, 33, 121, 90, 18, 66, -126, 105, 7, 3, -85,
    -107, -31, 22, -76, -82, 27, -62, -51, -30, -113, 83, 118, 64, -4, 12, 93, -55, -73, -114, -33,
    113, 34, -12, -71, 16, -15, -91, 125, -126, -31, -21, 119, -100, 125, -80, 31, 43, -50, -76,
    75, 68, 22, -53, -31, 64, -98, 22, 24, 109, -57, 26, -97, 85, 98, -60, -90, -7, 3, 82, 92, 93,
    -100, -113, -83, 88, 12, 44, -121, -34, -88, 11, -48, 26, 66, -127, 73, -105, -128, -90, -121,
    49, -27, -14, 66, -96, -46, -113, -86, -123, 91, 1, -65, 79, -1, -60, -97, -48, 127, 124, -69,
    -95, -24, 7, 124, 66, 36, 23, -72, 106, -101, 90, -95, 61, 102, 40, -99, 32, -60, 13, 41, 117,
    -30, -52, 114, -29, -52, -2, 4, -14, -72, 83, -91, -15, -11, 11, -27, -124, -109, -90, 24, 63,
    -92, 13, -20, -72, -16, -49, -3, -42, -122, -104, -125, -120, 64, -91, -79, -51, 96, 85, 126,
    116, 121, -12, 29, -25, -25, -93, 9, -63, 92, 13, -67, -102, 107, -65, -36, 50, -16, -72, 94,
    11, 80, 25, 27, 105, -2, 55, -9, 117, -87, 105, -70, -104, -15, -117, 6, 1, 83, 61, 61, -83,
    59, 5, 90, 44, -18, -69, 93, 25, -128, 84, -56, -19, 116, 108, 83, -34, -60, 12, 83, 88, -15,
    43, 111, 52, -12, -37, 99, 5, 33, 102, -106, -34, -90, 27, 123, -70, 3, 12, 44, -71, 110, 4,
    -9, 52, -64, 33, 121, 90, 18, 66, -126, 105, 7, 3, -85, -107, -31, 22, -76, -82, 27, -62, -51,
    -30, -113, 83, 118, 64, -4, 12, 93, -55, -73, -114, -33, 113, 34, -12, -71, 16, -15, -91, 125,
    -126, -31, -21, 119, -100, 125, -80, 31, 43, -50, -76, 75, 68, 22, -53, -31, 64, -98, 22, 24,
    109, -57, 26, -97, 85, 98, -60, -90, -7, 3, 82, 92, 93, -100, -113, -83, 88, 12, 44, -121, -34,
    -88, 11, -48, 26, 66, -127, 73, -105, -128, -90, -121, 49, -27, -14, 66, -96, -46, -113, -86,
    -123, 91, 1, -65, 79, -1, -60, -97, -48, 127, 124, -69, -95, -24, 7, 124, 66, 36, 23, -72, 106,
    -101, 90, -95, 61, 102, 40, -99, 32, -60, 13, 41, 117, -30, -52, 114, -29, -52, -2, 4, -14,
    -72, 83, -91, -15, -11, 11, -27, -124, -109, -90, 24, 63, -92, 13, -20, -72, -16, -49, -3, -42,
    -122, -104, -125,
];

static TEST_DATA_B_I8: &[i8] = &[
    25, 127, -78, -75, -127, 67, 112, 30, 67, 41, -120, 115, 21, -32, -57, -82, -5, 67, 48, 58,
    -53, -43, 98, -65, 53, 106, 92, 124, -62, -112, 116, 103, -124, 62, -97, -7, -127, 81, -66,
    -102, 36, -80, 49, 67, 1, 127, -16, -37, 107, -120, -125, 76, 16, -75, 90, -38, 127, -39, 51,
    -113, -90, -113, 29, -49, -115, 4, -119, -23, 6, -115, 95, -70, 87, -55, -75, -49, 13, -63, 98,
    105, 28, 117, -38, -10, -27, 72, 10, 95, -82, 100, 113, -41, -101, 93, 6, 65, 36, 123, 47, 69,
    73, 0, -6, 99, 62, -90, -21, -3, -112, 125, 31, 22, 114, -21, 1, -80, 52, 126, 65, -78, 94,
    108, -64, -82, 114, -82, 51, -101, -58, 122, -98, -93, -15, -56, -24, -108, 71, 119, -126, -98,
    -84, 102, 33, 118, 37, -29, -102, -107, 44, -62, -24, -65, -9, 20, -59, -45, -53, 17, -123,
    -89, 14, 1, 14, -49, -21, 45, 38, -79, -114, -114, 61, 118, -82, -18, -124, -63, -70, 91, 57,
    -45, 22, 93, -4, 105, -77, -72, 17, -1, 81, -34, -126, -30, 117, 30, 40, 37, -67, -49, 62, 60,
    114, -40, -72, -18, -72, -10, 31, 77, 103, -23, -11, -36, -10, 68, 114, -95, 56, 104, -81, 28,
    84, -46, -12, 71, -26, -26, 62, 84, -54, 95, 114, 69, -122, 53, 115, -16, 108, -115, -48, -11,
    -128, 58, -68, 87, 68, 26, -15, -16, -13, -31, 33, 42, 86, 25, 90, -53, 25, 127, -78, -75,
    -127, 67, 112, 30, 67, 41, -120, 115, 21, -32, -57, -82, -5, 67, 48, 58, -53, -43, 98, -65, 53,
    106, 92, 124, -62, -112, 116, 103, -124, 62, -97, -7, -127, 81, -66, -102, 36, -80, 49, 67, 1,
    127, -16, -37, 107, -120, -125, 76, 16, -75, 90, -38, 127, -39, 51, -113, -90, -113, 29, -49,
    -115, 4, -119, -23, 6, -115, 95, -70, 87, -55, -75, -49, 13, -63, 98, 105, 28, 117, -38, -10,
    -27, 72, 10, 95, -82, 100, 113, -41, -101, 93, 6, 65, 36, 123, 47, 69, 73, 0, -6, 99, 62, -90,
    -21, -3, -112, 125, 31, 22, 114, -21, 1, -80, 52, 126, 65, -78, 94, 108, -64, -82, 114, -82,
    51, -101, -58, 122, -98, -93, -15, -56, -24, -108, 71, 119, -126, -98, -84, 102, 33, 118, 37,
    -29, -102, -107, 44, -62, -24, -65, -9, 20, -59, -45, -53, 17, -123, -89, 14, 1, 14, -49, -21,
    45, 38, -79, -114, -114, 61, 118, -82, -18, -124, -63, -70, 91, 57, -45, 22, 93, -4, 105, -77,
    -72, 17, -1, 81, -34, -126, -30, 117, 30, 40, 37, -67, -49, 62, 60, 114, -40, -72, -18, -72,
    -10, 31, 77, 103, -23, -11, -36, -10, 68, 114, -95, 56, 104, -81, 28, 84, -46, -12, 71, -26,
    -26, 62, 84, -54, 95, 114, 69, -122, 53, 115, -16, 108, -115, -48, -11, -128, 58, -68, 87, 68,
    26, -15, -16, -13, -31, 33, 42, 86, 25, 90, -53,
];

fn i16_exec(thread_count: usize) {
    let mut group_metrics = Vec::with_capacity(thread_count);

    let data_a = Arc::new(PextVec::from(TEST_DATA_A_I16));
    let data_b = Arc::new(PextVec::from(TEST_DATA_B_I16));

    for threads in (1..=thread_count).rev() {
        // normal array
        let normal_result = {
            // get teady for threading
            let mut handles = Vec::with_capacity(threads);

            let result = Arc::new(UnsafeSyncCell::new(vec![0; TEST_DATA_A_I16.len()]));

            let (group_metric, individual_metrics) = get_metrics(|| {
                for i in 0..threads {
                    let result = result.clone();

                    let handle = spawn(move || {
                        get_metrics(|| {
                            // this is totaly unsafe, so becareful to what index to use
                            let result_ref = unsafe { &mut *result.get_mut() };
                            let a = TEST_DATA_A_I16;
                            let b = TEST_DATA_B_I16;
                            element_wise_mul_i16(a, b, result_ref, threads, i);
                        })
                        .0
                    });
                    handles.push(handle);
                }
                event_loop_until_empty();
                let metrics: Vec<Metrics> = handles.into_iter().map(|h| *h.join()).collect();
                metrics
            });

            group_metrics.push((threads, "normal", group_metric));
            for m in individual_metrics {
                println!("16,{},normal,{}", threads, m.csv());
            }
            result
        };

        // pext array
        let pext_result = {
            // get teady for threading
            let mut handles = Vec::with_capacity(threads);

            let result = Arc::new(UnsafeSyncCell::new(PextVec::<i32>::filled(0, data_a.len())));

            let (group_metric, individual_metrics) = get_metrics(|| {
                for i in 0..threads {
                    let result = result.clone();
                    let data_a = data_a.clone();
                    let data_b = data_b.clone();

                    let handle = spawn(move || {
                        get_metrics(|| {
                            // this is totaly unsafe, so becareful to what index to use
                            let result_ref = unsafe { &mut *result.get_mut() };
                            element_wise_mul_i16_simd(
                                data_a.get_inner(),
                                data_b.get_inner(),
                                result_ref.as_mut_slice(),
                                threads,
                                i,
                            );
                        })
                        .0
                    });
                    handles.push(handle);
                }
                event_loop_until_empty();
                let metrics: Vec<Metrics> = handles.into_iter().map(|h| *h.join()).collect();
                metrics
            });

            group_metrics.push((threads, "pext", group_metric));
            for m in individual_metrics {
                println!("16,{},pext,{}", threads, m.csv());
            }
            result
        };

        if pext_result
            .get()
            .iter()
            .zip(normal_result.get().iter())
            .all(|(a, &b)| a == b)
        {
            println!("16,{} OK", threads);
        } else {
            println!("16,{} NG", threads);
            println!("{:?}", normal_result.get());
            println!("{:?}", pext_result.get());
        }
    }
    for (threads, ty, metric) in group_metrics {
        println!("16,{},{},{}", threads, ty, metric.csv())
    }
    println!()
}

fn i8_exec(thread_count: usize) {
    let mut group_metrics = Vec::with_capacity(thread_count);

    let data_a = Arc::new(PextVec::from(TEST_DATA_A_I8));
    let data_b = Arc::new(PextVec::from(TEST_DATA_B_I8));

    for threads in (1..=thread_count).rev() {
        // normal array
        let normal_result = {
            // get teady for threading
            let mut handles = Vec::with_capacity(threads);

            let result = Arc::new(UnsafeSyncCell::new(vec![0; TEST_DATA_A_I8.len()]));

            let (group_metric, individual_metrics) = get_metrics(|| {
                for i in 0..threads {
                    let result = result.clone();

                    let handle = spawn(move || {
                        get_metrics(|| {
                            // this is totaly unsafe, so becareful to what index to use
                            let result_ref = unsafe { &mut *result.get_mut() };
                            let a = TEST_DATA_A_I8;
                            let b = TEST_DATA_B_I8;
                            element_wise_mul_i8(a, b, result_ref, threads, i);
                        })
                        .0
                    });
                    handles.push(handle);
                }
                event_loop_until_empty();
                let metrics: Vec<Metrics> = handles.into_iter().map(|h| *h.join()).collect();
                metrics
            });

            group_metrics.push((threads, "normal", group_metric));
            for m in individual_metrics {
                println!("8,{},normal,{}", threads, m.csv());
            }
            result
        };

        // pext array
        let pext_result = {
            // get teady for threading
            let mut handles = Vec::with_capacity(threads);

            let result = Arc::new(UnsafeSyncCell::new(PextVec::<i16>::filled(0, data_a.len())));

            let (group_metric, individual_metrics) = get_metrics(|| {
                for i in 0..threads {
                    let result = result.clone();
                    let data_a = data_a.clone();
                    let data_b = data_b.clone();

                    let handle = spawn(move || {
                        get_metrics(|| {
                            // this is totaly unsafe, so becareful to what index to use
                            let result_ref = unsafe { &mut *result.get_mut() };
                            element_wise_mul_i8_simd(
                                data_a.get_inner(),
                                data_b.get_inner(),
                                result_ref.as_mut_slice(),
                                threads,
                                i,
                            );
                        })
                        .0
                    });
                    handles.push(handle);
                }
                event_loop_until_empty();
                let metrics: Vec<Metrics> = handles.into_iter().map(|h| *h.join()).collect();
                metrics
            });

            group_metrics.push((threads, "pext", group_metric));
            for m in individual_metrics {
                println!("8,{},pext,{}", threads, m.csv());
            }
            result
        };

        if pext_result
            .get()
            .iter()
            .zip(normal_result.get().iter())
            .all(|(a, &b)| a == b)
        {
            println!("8,{} OK", threads);
        } else {
            println!("8,{} NG", threads);
            println!("{:?}", normal_result.get());
            println!("{:?}", pext_result.get());
        }
    }
    for (threads, ty, metric) in group_metrics {
        println!("8,{},{},{}", threads, ty, metric.csv())
    }
    println!()
}

#[inline(never)]
fn first_hart_entry() {
    println!("elemnt,threads,type,{}", MetricsCSV::HEADER);
    let thread_count = get_thread_count();
    i16_exec(thread_count);
    i8_exec(thread_count);
}

#[inline(never)]
fn other_hart_entry() {
    event_loop()
}

#[inline(never)]
#[no_mangle]
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
