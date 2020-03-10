//!
//! # Sequences
//!
//! This module implements variable-length sequences and utility functions for it.
//!

use crate::prelude::*;

/// Variable length byte arrays.
#[derive(Debug, Clone, Default)]
pub struct Seq<T: Copy> {
    pub(crate) b: Vec<T>,
    l: usize,
}

pub type ByteSeq = Seq<U8>;
pub type PublicByteSeq = Seq<u8>;

impl<T: Copy + Default> Seq<T> {
    /// Get a new sequence of capacity `l`.
    pub fn new(l: usize) -> Self {
        Self {
            b: vec![T::default(); l],
        }
    }
    /// Get the size of this sequence.
    pub fn len(&self) -> usize {
        self.b.len()
    }
    /// Update this sequence with `v` starting at `start`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hacspec::prelude::*;
    ///
    /// let mut s = Seq::<u8>::new(5);
    /// let tmp = Seq::<u8>::from_array(&[2, 3]);
    /// s = s.update(2, tmp);
    /// assert_eq!(s, Seq::<u8>::from_array(&[0, 0, 2, 3, 0]));
    /// ```
    pub fn update<A: SeqTrait<T>>(self, start: usize, v: A) -> Self {
        println!("{:?} >= {:?} + {:?}", self.len(), start, v.len());
        debug_assert!(self.len() >= start + v.len());
        let mut self_copy = self;
        for (i, b) in v.iter().enumerate() {
            self_copy[start + i] = *b;
        }
        self_copy
    }
    /// Update this sequence with `l` elements of `v`, starting at `start_in`,
    /// at `start_out`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hacspec::prelude::*;
    ///
    /// let mut s = Seq::<u8>::new(5);
    /// let tmp = Seq::<u8>::from(&[2, 3]);
    /// s = s.update_slice(2, tmp, 1..2);
    /// assert_eq!(s, Seq::<u8>::from(&[0, 0, 3, 0, 0]));
    /// ```
    pub fn update_sub<A: SeqTrait<T>>(
        &self,
        start_out: usize,
        v: A,
        start_in: usize,
        len: usize,
    ) -> Self {
        debug_assert!(self.len() >= start_out + len);
        debug_assert!(v.len() >= start_in + len);
        let mut self_copy = self;
        for (i, b) in v.iter().skip(start_in).take(len).enumerate() {
            self_copy[start_out + i] = *b;
        }
        self_copy
    }
    /// sub(self, start..end)
    pub fn sub(self, start_out: usize, len: usize) -> Self {
        Self::from(
            self.b
                .iter()
                .skip(start_out)
                .map(|x| *x)
                .take(len)
                .collect::<Vec<T>>(),
        )
    }
    // s.get_chunk(i: usize, block_size: usize)
    // s.update_chunk(i: usize, block_size: usize, v: Seq<T>)
    // s.chunks(block_size: usize)
}

impl Seq<U8> {
    fn get_random_vec(l: usize) -> Vec<U8> {
        (0..l)
            .map(|_| rand::random::<u8>())
            .map(|x| U8::classify(x))
            .collect()
    }

    pub fn random(l: usize) -> Self {
        Self {
            b: Seq::get_random_vec(l),
        }
    }
}

// TODO: move to test crate (have everything declassified, so this works for U8 as well)
impl Seq<u8> {
    pub fn to_hex(&self) -> String {
        let strs: Vec<String> = self.iter().map(|b| format!("{:02x}", b)).collect();
        strs.join("")
    }
}

impl<T: Copy> SeqTrait<T> for Seq<T> {
    fn raw<'a>(&'a self) -> &'a [T] {
        &self.b
    }
    fn len(&self) -> usize {
        self.b.len()
    }
    fn iter(&self) -> std::slice::Iter<T> {
        self.b.iter()
    }
}

impl<T: Copy> Index<u8> for Seq<T> {
    type Output = T;
    fn index(&self, i: u8) -> &T {
        &self.b[i as usize]
    }
}

impl<T: Copy> IndexMut<u8> for Seq<T> {
    fn index_mut(&mut self, i: u8) -> &mut T {
        &mut self.b[i as usize]
    }
}

impl<T: Copy> Index<u32> for Seq<T> {
    type Output = T;
    fn index(&self, i: u32) -> &T {
        debug_assert!(i <= usize::MAX);
        &self.b[i as usize]
    }
}

impl<T: Copy> IndexMut<u32> for Seq<T> {
    fn index_mut(&mut self, i: u32) -> &mut T {
        debug_assert!(i <= usize::MAX);
        &mut self.b[i as usize]
    }
}

impl<T: Copy> Index<i32> for Seq<T> {
    type Output = T;
    fn index(&self, i: i32) -> &T {
        debug_assert!(i <= usize::MAX && i >= 0);
        &self.b[i as usize]
    }
}

impl<T: Copy> IndexMut<i32> for Seq<T> {
    fn index_mut(&mut self, i: i32) -> &mut T {
        debug_assert!(i <= usize::MAX && i >= 0);
        &mut self.b[i as usize]
    }
}

impl<T: Copy> Index<usize> for Seq<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.b[i]
    }
}

impl<T: Copy> IndexMut<usize> for Seq<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.b[i]
    }
}

impl<T: Copy> Index<Range<usize>> for Seq<T> {
    type Output = [T];
    fn index(&self, r: Range<usize>) -> &[T] {
        &self.b[r]
    }
}

impl<T: Copy> From<Vec<T>> for Seq<T> {
    fn from(x: Vec<T>) -> Seq<T> {
        Self {
            b: x.clone(),
        }
    }
}

impl<T: Copy> From<&[T]> for Seq<T> {
    fn from(x: &[T]) -> Seq<T> {
        Self {
            b: x.to_vec(),
        }
    }
}

// macro_rules! from_int_vec {
//     ($t:ty) => {
//         impl<T: Copy> From<Vec<$t>> for Seq<T> {
//             fn from(x: Vec<$t>) -> Seq<T> {
//                 Self {
//                     b: x.iter().map(|&x| T::from(x)).collect::<Vec<T>>(),
//                     idx: 0,
//                 }
//             }
//         }
//         TODO: add From<&[$t]>
//     };
// }

// from_int_vec!(u8);
// from_int_vec!(u16);
// from_int_vec!(u32);
// from_int_vec!(u64);
// from_int_vec!(u128);

// TODO: add From<&str> with s.into_bytes()
/// Read hex string to Bytes.
impl From<&str> for Seq<U8> {
    fn from(s: &str) -> Seq<U8> {
        Seq::from(
            hex_string_to_bytes(s)
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        )
    }
}
impl From<String> for Seq<U8> {
    fn from(s: String) -> Seq<U8> {
        Seq::<U8>::from(
            hex_string_to_bytes(&s)
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        )
    }
}
// TODO: duplicate code...
impl From<&str> for Seq<u8> {
    fn from(s: &str) -> Seq<u8> {
        Seq::<u8>::from(hex_string_to_bytes(s))
    }
}
impl From<String> for Seq<u8> {
    fn from(s: String) -> Seq<u8> {
        Seq::<u8>::from(hex_string_to_bytes(&s))
    }
}

/// Two sequences are equal if the underlying vector is equal.
/// The idx field is ignored.
impl<T: Copy + PartialEq> std::cmp::PartialEq<Seq<T>> for Seq<T> {
    fn eq(&self, other: &Self) -> bool {
        self.b == other.b
    }
}
