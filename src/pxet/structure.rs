use core::marker::PhantomData;

use alloc::vec::Vec;

#[derive(Default, Clone)]
pub struct PextVec<T> {
    inner: Vec<usize>,
    len: usize,
    _datatype: PhantomData<T>,
}

pub struct PextVecIter<'a, T> {
    inner: &'a PextVec<T>,
    idx: usize,
}

macro_rules! gen_PextVecType {
    ($t:ty, $align:ty) => {
        impl PextVec<$t> {
            pub const DATA_SIZE: usize = ::core::mem::size_of::<$t>();
            pub const RATIO: usize = ::core::mem::size_of::<usize>() / Self::DATA_SIZE;

            pub fn new() -> Self {
                Self::default()
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

            pub fn len(&self) -> usize {
                self.len
            }

            pub fn is_empty(&self) -> bool {
                self.len == 0
            }

            pub fn get_inner(&self) -> &Vec<usize> {
                &self.inner
            }

            pub fn iter(&self) -> PextVecIter<$t> {
                PextVecIter {
                    inner: self,
                    idx: 0,
                }
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
    };
}
gen_PextVecType!(u32, u32);
gen_PextVecType!(u16, u16);
gen_PextVecType!(u8, u8);
gen_PextVecType!(i32, u32);
gen_PextVecType!(i16, u16);
gen_PextVecType!(i8, u8);
