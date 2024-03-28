use core::fmt;
use core::marker::PhantomData;

use alloc::vec;
use alloc::vec::Vec;

#[derive(Default, Clone)]
pub struct PextVec<T> {
    inner: Vec<usize>,
    len: usize,
    _datatype: PhantomData<T>,
}

#[derive(Default, Clone)]
pub struct PextMat<T> {
    inner: Vec<usize>,
    pub width: usize,
    pub height: usize,
    _datatype: PhantomData<T>,
}

pub struct PextVecIter<'a, T> {
    inner: &'a PextVec<T>,
    idx: usize,
}

impl<T> PextVec<T> {
    pub const fn new() -> Self {
        Self {
            inner: Vec::new(),
            len: 0,
            _datatype: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get_inner(&self) -> &Vec<usize> {
        &self.inner
    }

    /// # Safety
    /// the len mey be different from the underlying data
    pub unsafe fn get_inner_mut(&mut self) -> &mut Vec<usize> {
        &mut self.inner
    }

    pub fn as_slice(&self) -> &[usize] {
        &self.inner
    }

    pub fn as_mut_slice(&mut self) -> &mut [usize] {
        &mut self.inner
    }

    pub fn iter(&self) -> PextVecIter<T> {
        PextVecIter {
            inner: self,
            idx: 0,
        }
    }

    pub fn into_parts(self) -> (Vec<usize>, usize) {
        (self.inner, self.len)
    }
}

impl<T> PextMat<T> {
    pub unsafe fn get_slice_mut(&mut self) -> &mut [usize] {
        self.inner.as_mut_slice()
    }

    pub fn get_slice(&self) -> &[usize] {
        self.inner.as_slice()
    }
}

pub struct Matrix<T> {
    pub inner: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Matrix<T> {
    pub fn get_mut_at(&mut self, w: usize, h: usize) -> Option<&mut T> {
        self.inner.get_mut(w + h * self.width)
    }

    pub fn get_at(&self, w: usize, h: usize) -> Option<&T> {
        self.inner.get(w + h * self.width)
    }

    pub fn from_iter(width: usize, height: usize, iter: impl Iterator<Item = T>) -> Self {
        Self {
            inner: iter.take(width * height).collect(),
            width,
            height,
        }
    }
}

// impl Matrix<i16> {
//     pub fn zeroed(width: usize, height: usize) -> Self {
//         Self {
//             inner: vec![0; width * height],
//             width,
//             height,
//         }
//     }
// }

impl<T: fmt::Display> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {}x{}", self.width, self.height)?;
        for h in 0..self.height {
            for w in 0..self.width {
                write!(f, "{} ", self.get_at(w, h).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

macro_rules! gen_PextVecType {
    ($t:ty, $align:ty, $next_up:ty) => {
        impl PextVec<$t> {
            pub const DATA_SIZE: usize = ::core::mem::size_of::<$t>();
            pub const RATIO: usize = ::core::mem::size_of::<usize>() / Self::DATA_SIZE;

            pub fn with_capacity(capacity: usize) -> Self {
                Self {
                    inner: Vec::with_capacity(capacity),
                    len: 0,
                    _datatype: PhantomData,
                }
            }

            pub fn push(&mut self, data: $t) {
                // generate a mask that we will use as a mask to get the lower bits of `len`
                // if $t is align 32 mask will be 0x01
                // if $t is align 16 mask will be 0x03
                // if $t is align  8 mask will be 0x07
                let mask = Self::RATIO - 1;
                let idx = self.len & mask;
                if idx == 0 {
                    let t = data as $align as usize;
                    self.inner.push(t);
                } else {
                    unsafe {
                        (self.inner.as_mut_ptr() as *mut $t)
                            .add(self.len())
                            .write(data);
                    }
                }
                self.len += 1;
            }

            pub fn from_parts(inner: Vec<usize>, len: usize) -> Self {
                // if inner.len() == 0 {
                //     assert_eq!(len, 0);
                // } else {
                //     assert!((inner.len() - 1) * Self::RATIO <= len);
                //     assert!(len < inner.len() * Self::RATIO);
                // }
                Self {
                    inner,
                    len,
                    _datatype: PhantomData,
                }
            }

            pub fn filled(value: $t, len: usize) -> Self {
                let mut out = Self::new();
                for _ in 0..len {
                    out.push(value)
                }
                out
            }
        }

        impl ::core::fmt::Debug for PextVec<$t> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_list().entries(self.iter()).finish()
            }
        }

        impl<'a> Iterator for PextVecIter<'a, $t> {
            type Item = $t;

            fn next(&mut self) -> Option<Self::Item> {
                if self.idx < self.inner.len() {
                    let v = unsafe {
                        (self.inner.get_inner().as_ptr() as *const $t)
                            .add(self.idx)
                            .read()
                    };
                    self.idx += 1;
                    Some(v)
                } else {
                    None
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let remaining = self.inner.len() - self.idx;
                (remaining, Some(remaining))
            }
        }

        impl From<&[$t]> for PextVec<$t> {
            fn from(data: &[$t]) -> Self {
                let mut out = Self::new();
                for e in data {
                    out.push(*e);
                }
                out
            }
        }

        impl From<PextVec<$t>> for Vec<$t> {
            fn from(data: PextVec<$t>) -> Self {
                let mut out = Self::new();
                let ratio = PextVec::<$t>::RATIO;
                for e in data.get_inner() {
                    for i in 0..ratio {
                        let t = (e >> (i * 8 * PextVec::<$t>::DATA_SIZE)) as $t;
                        out.push(t);
                    }
                }
                while out.len() != data.len() {
                    out.pop();
                }
                out
            }
        }

        impl PextMat<$t> {
            pub const DATA_SIZE: usize = PextVec::<$t>::DATA_SIZE;
            pub const RATIO: usize = PextVec::<$t>::RATIO;

            pub fn zeroed(width: usize, height: usize) -> Self {
                assert_ne!(width, 0);
                assert_ne!(height, 0);
                let width_alloc = (width - 1) / Self::RATIO + 1;
                let v = vec![0; width_alloc * height];
                Self {
                    inner: v,
                    height,
                    width,
                    _datatype: PhantomData,
                }
            }

            #[inline(always)]
            pub fn alloc_width(&self) -> usize {
                ((self.width - 1) / Self::RATIO + 1)
            }

            /// initialize mat from iterator
            pub fn from_iter(
                width: usize,
                height: usize,
                mut iter: impl Iterator<Item = $t>,
            ) -> Self {
                let mut m = Self::zeroed(width, height);

                let alloc_width = m.alloc_width();
                for h_idx in 0..height {
                    for w_idx in 0..alloc_width {
                        for r_idx in 0..Self::RATIO {
                            if r_idx + w_idx * Self::RATIO < width {
                                m.inner[w_idx + h_idx * alloc_width] |=
                                    (iter.next().unwrap() as $align as usize)
                                        << (r_idx * Self::DATA_SIZE * 8);
                            }
                        }
                    }
                }
                m
            }

            #[inline(always)]
            pub fn get_at(&self, w: usize, h: usize) -> $t {
                unsafe {
                    *((self.inner.as_slice().as_ptr() as *const usize as *const $t)
                        .add(w + h * self.alloc_width() * Self::RATIO))
                }
            }

            #[inline(always)]
            pub fn get_col_at(&self, w: usize, h: usize) -> usize {
                // assert_eq!(h % Self::RATIO, 0);
                let next_width = self.alloc_width() * Self::RATIO;
                unsafe {
                    let ptr = ((self.inner.as_slice().as_ptr() as *const usize as *const $t)
                        .add(w + h * self.alloc_width() * Self::RATIO));
                    let a = (*ptr as usize & ((1 << (Self::DATA_SIZE * 8)) - 1))
                        | ((*ptr.add(next_width) as usize & ((1 << (Self::DATA_SIZE * 8)) - 1))
                            << 16)
                        | ((*ptr.add(next_width * 2) as usize
                            & ((1 << (Self::DATA_SIZE * 8)) - 1))
                            << 32)
                        | ((*ptr.add(next_width * 3) as usize
                            & ((1 << (Self::DATA_SIZE * 8)) - 1))
                            << 48);
                    a
                }
            }

            #[inline(always)]
            pub unsafe fn get_slice_mut_as_type(&mut self) -> &mut [$t] {
                &mut *core::ptr::slice_from_raw_parts_mut(
                    self.inner.as_mut_slice().as_ptr() as *mut usize as *mut $t,
                    self.inner.len() * Self::RATIO,
                )
            }

            pub fn ratio(&self) -> usize {
                Self::RATIO
            }

            pub fn to_matrix(&self) -> Matrix<$t> {
                let mut m = Matrix::<$t>::zeroed(self.width, self.height);
                let self_ptr = self.inner.as_slice().as_ptr() as *mut usize as *mut $t;
                for w in 0..self.width {
                    for h in 0..self.height {
                        *m.get_mut_at(w, h).expect("no value on index?") =
                            unsafe { *self_ptr.add(w + h * self.alloc_width() * Self::RATIO) };
                    }
                }
                m
            }
        }

        impl fmt::Debug for PextMat<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                writeln!(f, "Matrix {}x{}", self.width, self.height)?;
                for h in 0..self.height {
                    for w in 0..self.width {
                        write!(f, "{} ", self.get_at(w, h))?;
                    }
                    writeln!(f)?;
                }
                Ok(())
            }
        }

        impl Matrix<$t> {
            pub fn zeroed(width: usize, height: usize) -> Self {
                Self {
                    inner: vec![0; width * height],
                    width,
                    height,
                }
            }
        }
    };
}

gen_PextVecType!(u32, u32, u64);
gen_PextVecType!(u16, u16, u32);
gen_PextVecType!(u8, u8, u16);
gen_PextVecType!(i32, u32, i64);
gen_PextVecType!(i16, u16, i32);
gen_PextVecType!(i8, u8, i16);

pub const fn transposei16_4x4(m: [usize; 4]) -> [usize; 4] {
    let mut out = [0; 4];
    out[0] = ((m[0] >> 00) & 0xFFFF << 00)
        | ((m[1] >> 00) & 0xFFFF << 16)
        | ((m[2] >> 00) & 0xFFFF << 32)
        | ((m[3] >> 00) & 0xFFFF << 48);
    out[1] = ((m[0] >> 16) & 0xFFFF << 00)
        | ((m[1] >> 16) & 0xFFFF << 16)
        | ((m[2] >> 16) & 0xFFFF << 32)
        | ((m[3] >> 16) & 0xFFFF << 48);
    out[2] = ((m[0] >> 32) & 0xFFFF << 00)
        | ((m[1] >> 32) & 0xFFFF << 16)
        | ((m[2] >> 32) & 0xFFFF << 32)
        | ((m[3] >> 32) & 0xFFFF << 48);
    out[3] = ((m[0] >> 48) & 0xFFFF << 00)
        | ((m[1] >> 48) & 0xFFFF << 16)
        | ((m[2] >> 48) & 0xFFFF << 32)
        | ((m[3] >> 48) & 0xFFFF << 48);
    out
}

pub const fn transposei8_8x8(m: [usize; 8]) -> [usize; 8] {
    let mut out = [0; 8];
    out[0] = ((m[0] >> 00) & 0xFF << 00)
        | ((m[1] >> 00) & 0xFF << 08)
        | ((m[2] >> 00) & 0xFF << 16)
        | ((m[3] >> 00) & 0xFF << 24)
        | ((m[4] >> 00) & 0xFF << 32)
        | ((m[5] >> 00) & 0xFF << 40)
        | ((m[6] >> 00) & 0xFF << 48)
        | ((m[7] >> 00) & 0xFF << 56);
    out[1] = ((m[0] >> 08) & 0xFF << 00)
        | ((m[1] >> 08) & 0xFF << 08)
        | ((m[2] >> 08) & 0xFF << 16)
        | ((m[3] >> 08) & 0xFF << 24)
        | ((m[4] >> 08) & 0xFF << 32)
        | ((m[5] >> 08) & 0xFF << 40)
        | ((m[6] >> 08) & 0xFF << 48)
        | ((m[7] >> 08) & 0xFF << 56);
    out[2] = ((m[0] >> 16) & 0xFF << 00)
        | ((m[1] >> 16) & 0xFF << 08)
        | ((m[2] >> 16) & 0xFF << 16)
        | ((m[3] >> 16) & 0xFF << 24)
        | ((m[4] >> 16) & 0xFF << 32)
        | ((m[5] >> 16) & 0xFF << 40)
        | ((m[6] >> 16) & 0xFF << 48)
        | ((m[7] >> 16) & 0xFF << 56);
    out[3] = ((m[0] >> 24) & 0xFF << 00)
        | ((m[1] >> 24) & 0xFF << 08)
        | ((m[2] >> 24) & 0xFF << 16)
        | ((m[3] >> 24) & 0xFF << 24)
        | ((m[4] >> 24) & 0xFF << 32)
        | ((m[5] >> 24) & 0xFF << 40)
        | ((m[6] >> 24) & 0xFF << 48)
        | ((m[7] >> 24) & 0xFF << 56);
    out[4] = ((m[0] >> 32) & 0xFF << 00)
        | ((m[1] >> 32) & 0xFF << 08)
        | ((m[2] >> 32) & 0xFF << 16)
        | ((m[3] >> 32) & 0xFF << 24)
        | ((m[4] >> 32) & 0xFF << 32)
        | ((m[5] >> 32) & 0xFF << 40)
        | ((m[6] >> 32) & 0xFF << 48)
        | ((m[7] >> 32) & 0xFF << 56);
    out[5] = ((m[0] >> 40) & 0xFF << 00)
        | ((m[1] >> 40) & 0xFF << 08)
        | ((m[2] >> 40) & 0xFF << 16)
        | ((m[3] >> 40) & 0xFF << 24)
        | ((m[4] >> 40) & 0xFF << 32)
        | ((m[5] >> 40) & 0xFF << 40)
        | ((m[6] >> 40) & 0xFF << 48)
        | ((m[7] >> 40) & 0xFF << 56);
    out[6] = ((m[0] >> 48) & 0xFF << 00)
        | ((m[1] >> 48) & 0xFF << 08)
        | ((m[2] >> 48) & 0xFF << 16)
        | ((m[3] >> 48) & 0xFF << 24)
        | ((m[4] >> 48) & 0xFF << 32)
        | ((m[5] >> 48) & 0xFF << 40)
        | ((m[6] >> 48) & 0xFF << 48)
        | ((m[7] >> 48) & 0xFF << 56);
    out[7] = ((m[0] >> 56) & 0xFF << 00)
        | ((m[1] >> 56) & 0xFF << 08)
        | ((m[2] >> 56) & 0xFF << 16)
        | ((m[3] >> 56) & 0xFF << 24)
        | ((m[4] >> 56) & 0xFF << 32)
        | ((m[5] >> 56) & 0xFF << 40)
        | ((m[6] >> 56) & 0xFF << 48)
        | ((m[7] >> 56) & 0xFF << 56);
    out
}
