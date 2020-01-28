//!
//! hacspec consists of three parts:
//! * hacspec library
//! * syntax checker
//! * compiler
//!
//! # The hacspec library
//!
//! The hacspec library implements a comprehensive set of functions to implement
//! succinct cryptographic specs that can be compiled to formal languages such
//! as [F*](https://www.fstar-lang.org/).
//!
//! # The syntax checker
//! TODO:
//! * describe clippy extension
//! * add `cargo hacspec check` command
//!
//! # The compiler
//! TODO:
//! * define compiler interface
//! * add `cargo hacspec fstar` command
//!

use rand;
use std::convert::AsMut;
use std::num::ParseIntError;
use std::ops::{Index, IndexMut, Range, RangeFull};

pub mod prelude;

use crate::prelude::*;

fn hex_string_to_bytes(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0);
    let b: Result<Vec<u8>, ParseIntError> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();
    b.expect("Error parsing hex string")
}

/// Common trait for all byte arrays.
pub trait SeqTrait<T: Copy> {
    fn raw<'a>(&'a self) -> &'a [T];
    fn len(&self) -> usize;
    fn iter(&self) -> std::slice::Iter<T>;
}

// ======================== Variable length arrays ========================== //

/// Variable length byte arrays.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Seq<T: Copy> {
    b: Vec<T>,
}

pub type ByteSeq = Seq<U8>;

impl<T: Copy + Default> Seq<T> {
    pub fn new_len(l: usize) -> Self {
        Self {
            b: vec![T::default(); l],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.b.is_empty()
    }
    pub fn from_array(v: &[T]) -> Self {
        Self { b: v.to_vec() }
    }
    pub fn len(&self) -> usize {
        self.b.len()
    }
    pub fn update<A: SeqTrait<T>>(mut self, start: usize, v: A) -> Self {
        assert!(self.len() >= start + v.len());
        for (i, b) in v.iter().enumerate() {
            self[start + i] = *b;
        }
        self
    }
    pub fn update_sub<A: SeqTrait<T>>(
        mut self,
        start_out: usize,
        v: A,
        start_in: usize,
        len: usize,
    ) -> Self {
        assert!(self.len() >= start_out + len);
        assert!(v.len() >= start_in + len);
        for (i, b) in v.iter().skip(start_in).take(len).enumerate() {
            self[start_out + i] = *b;
        }
        self
    }

    pub fn from_sub<A: SeqTrait<T>>(input: A, r: Range<usize>) -> Self {
        let mut a = Self::default();
        for (i, v) in r
            .clone()
            .zip(input.iter().skip(r.start).take(r.end - r.start))
        {
            a[i] = *v;
        }
        a
    }
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
        &self.b[i as usize]
    }
}

impl<T: Copy> IndexMut<u32> for Seq<T> {
    fn index_mut(&mut self, i: u32) -> &mut T {
        &mut self.b[i as usize]
    }
}

impl<T: Copy> Index<i32> for Seq<T> {
    type Output = T;
    fn index(&self, i: i32) -> &T {
        &self.b[i as usize]
    }
}

impl<T: Copy> IndexMut<i32> for Seq<T> {
    fn index_mut(&mut self, i: i32) -> &mut T {
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

impl<T: Copy> From<Vec<T>> for Seq<T> {
    fn from(x: Vec<T>) -> Seq<T> {
        Self { b: x.clone() }
    }
}
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

// ========================== Fixed length arrays =========================== //

#[macro_export]
macro_rules! bytes {
    ($name:ident, $l:expr) => {
        array!($name, $l, U8, u8);
    };
}

#[macro_export]
macro_rules! array_base {
    ($name:ident,$l:expr,$t:ty, $tbase:ty) => {
        /// Fixed length byte array.
        /// Because Rust requires fixed length arrays to have a known size at
        /// compile time there's no generic fixed length byte array here.
        /// Use this to define the fixed length byte arrays needed in your code.
        #[derive(Clone, Copy)]
        pub struct $name(pub [$t; $l]);

        impl From<[$t; $l]> for $name {
            fn from(v: [$t; $l]) -> Self {
                Self(v.clone())
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self([<$t>::default(); $l])
            }

            pub fn from_sub_pad<A: SeqTrait<$t>>(input: A, r: Range<usize>) -> Self {
                let mut a = Self::default();
                for (i, v) in r
                    .clone()
                    .zip(input.iter().skip(r.start).take(r.end - r.start))
                {
                    a[i - r.start] = *v;
                }
                a
            }

            pub fn from_sub<A: SeqTrait<$t>>(input: A, r: Range<usize>) -> Self {
                assert!(
                    $l == r.end - r.start,
                    "sub range is not the length of the output type "
                );
                $name::from_sub_pad(input, r)
            }

            pub fn copy_pad<A: SeqTrait<$t>>(v: A) -> Self {
                assert!(v.len() <= $l);
                let mut tmp = [<$t>::default(); $l];
                for (i, x) in v.iter().enumerate() {
                    tmp[i] = *x;
                }
                Self(tmp.clone())
            }
            pub fn copy<A: SeqTrait<$t>>(v: A) -> Self {
                assert!(v.len() == $l);
                let mut tmp = [<$t>::default(); $l];
                for (i, x) in v.iter().enumerate() {
                    tmp[i] = *x;
                }
                Self(tmp.clone())
            }
            pub fn update<A: SeqTrait<$t>>(mut self, start: usize, v: A) -> Self {
                assert!(self.len() >= start + v.len());
                for (i, b) in v.iter().enumerate() {
                    self[start + i] = *b;
                }
                self
            }
            pub fn update_sub<A: SeqTrait<$t>>(
                mut self,
                start_out: usize,
                v: A,
                start_in: usize,
                len: usize,
            ) -> Self {
                assert!(self.len() >= start_out + len);
                assert!(v.len() >= start_in + len);
                for (i, b) in v.iter().skip(start_in).take(len).enumerate() {
                    self[start_out + i] = *b;
                }
                self
            }
            pub fn len(&self) -> usize {
                $l
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name::new()
            }
        }
        impl AsMut<[$t]> for $name {
            fn as_mut(&mut self) -> &mut [$t] {
                &mut self.0
            }
        }
        impl SeqTrait<$t> for $name {
            fn raw<'a>(&'a self) -> &'a [$t] {
                &self.0
            }
            fn len(&self) -> usize {
                $l
            }
            fn iter(&self) -> std::slice::Iter<$t> {
                self.0.iter()
            }
        }

        impl Index<usize> for $name {
            type Output = $t;
            fn index(&self, i: usize) -> &$t {
                &self.0[i]
            }
        }
        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, i: usize) -> &mut $t {
                &mut self.0[i]
            }
        }

        impl Index<u8> for $name {
            type Output = $t;
            fn index(&self, i: u8) -> &$t {
                &self.0[i as usize]
            }
        }
        impl IndexMut<u8> for $name {
            fn index_mut(&mut self, i: u8) -> &mut $t {
                &mut self.0[i as usize]
            }
        }
        impl Index<u32> for $name {
            type Output = $t;
            fn index(&self, i: u32) -> &$t {
                &self.0[i as usize]
            }
        }
        impl IndexMut<u32> for $name {
            fn index_mut(&mut self, i: u32) -> &mut $t {
                &mut self.0[i as usize]
            }
        }
        impl Index<i32> for $name {
            type Output = $t;
            fn index(&self, i: i32) -> &$t {
                &self.0[i as usize]
            }
        }
        impl IndexMut<i32> for $name {
            fn index_mut(&mut self, i: i32) -> &mut $t {
                &mut self.0[i as usize]
            }
        }
        impl Index<RangeFull> for $name {
            type Output = [$t];
            fn index(&self, r: RangeFull) -> &[$t] {
                &self.0[r]
            }
        }
        impl From<Vec<$t>> for $name {
            fn from(x: Vec<$t>) -> $name {
                assert!(x.len() <= $l);
                let mut tmp = [<$t>::default(); $l];
                for (i, e) in x.iter().enumerate() {
                    tmp[i] = *e;
                }
                $name(tmp.clone())
            }
        }

        impl $name {
            pub fn random() -> $name {
                let mut tmp = [<$t>::default(); $l];
                tmp.copy_from_slice(&$name::get_random_vec($l)[..$l]);
                Self(tmp.clone())
            }
        }

        /// Read hex string to Bytes.
        impl From<&str> for $name {
            fn from(s: &str) -> $name {
                let v = $name::hex_string_to_vec(s);
                let mut o = $name::new();
                assert!(v.len() == $l);
                for i in 0..$l {
                    o[i] = v[i]
                }
                o
            }
        }
    };
}

#[macro_export]
macro_rules! array {
    ($name:ident,$l:expr,$t:ty, $tbase:ty) => {
        array_base!($name, $l, $t, $tbase);

        impl $name {
            fn hex_string_to_vec(s: &str) -> Vec<$t> {
                assert!(s.len() % std::mem::size_of::<$t>() == 0);
                let b: Result<Vec<$t>, ParseIntError> = (0..s.len())
                    .step_by(2)
                    .map(|i| <$tbase>::from_str_radix(&s[i..i + 2], 16).map(<$t>::classify))
                    .collect();
                b.expect("Error parsing hex string")
            }

            pub fn get_random_vec(l: usize) -> Vec<$t> {
                (0..l)
                    .map(|_| <$t>::classify(rand::random::<$tbase>()))
                    .collect()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0[..]
                    .iter()
                    .map(|x| <$t>::declassify(*x))
                    .collect::<Vec<_>>()
                    .fmt(f)
            }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0[..]
                    .iter()
                    .map(|x| <$t>::declassify(*x))
                    .collect::<Vec<_>>()
                    == other.0[..]
                        .iter()
                        .map(|x| <$t>::declassify(*x))
                        .collect::<Vec<_>>()
            }
        }
    };
}

#[macro_export]
macro_rules! public_array {
    ($name:ident,$l:expr,$t:ty) => {
        array_base!($name, $l, $t, $t);
        impl $name {
            fn hex_string_to_vec(s: &str) -> Vec<$t> {
                assert!(s.len() % std::mem::size_of::<$t>() == 0);
                let b: Result<Vec<$t>, ParseIntError> = (0..s.len())
                    .step_by(2)
                    .map(|i| <$t>::from_str_radix(&s[i..i + 2], 16))
                    .collect();
                b.expect("Error parsing hex string")
            }

            pub fn get_random_vec(l: usize) -> Vec<$t> {
                (0..l).map(|_| rand::random::<$t>()).collect()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0[..].fmt(f)
            }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0[..] == other.0[..]
            }
        }
    };
}

bytes!(U32Word, 4);
bytes!(U128Word, 16);
bytes!(U64Word, 8);
public_array!(Counter, 2, usize);

pub fn u32_to_le_bytes(x: U32) -> U32Word {
    U32Word([
        U8::from((x & U32::classify(0xFF000000u32)) >> 24),
        U8::from((x & U32::classify(0xFF0000u32)) >> 16),
        U8::from((x & U32::classify(0xFF00u32)) >> 8),
        U8::from(x & U32::classify(0xFFu32)),
    ])
}

pub fn u32_from_le_bytes(s: U32Word) -> U32 {
    U32::from_bytes_le(&s.0)[0]
}

pub fn u32_to_be_bytes(x: U32) -> U32Word {
    U32Word::from(U32::to_bytes_be(&[x]))
}

pub fn u128_from_le_bytes(s: U128Word) -> U128 {
    U128::from_bytes_le(&s.0)[0]
}

pub fn u128_from_be_bytes(s: U128Word) -> U128 {
    U128::from_bytes_be(&s.0)[0]
}

pub fn u128_to_be_bytes(x: U128) -> U128Word {
    U128Word::from(U128::to_bytes_be(&[x]))
}

pub fn u64_to_be_bytes(x: U64) -> U64Word {
    U64Word::from(U64::to_bytes_be(&[x]))
}

pub fn u64_to_le_bytes(x: U64) -> U64Word {
    U64Word::from(U64::to_bytes_le(&[x]))
}

pub fn u64_slice_to_le_u8s(x: &dyn SeqTrait<U64>) -> ByteSeq {
    let mut result = ByteSeq::new_len(x.len() * 8);
    for (i, v) in x.iter().enumerate().rev() {
        result[0 + (i * 8)] = U8::from(*v & U64::classify(0xFFu64));
        result[1 + (i * 8)] = U8::from((*v & U64::classify(0xFF00u64)) >> 8);
        result[2 + (i * 8)] = U8::from((*v & U64::classify(0xFF0000u64)) >> 16);
        result[3 + (i * 8)] = U8::from((*v & U64::classify(0xFF000000u64)) >> 24);
        result[4 + (i * 8)] = U8::from((*v & U64::classify(0xFF00000000u64)) >> 32);
        result[5 + (i * 8)] = U8::from((*v & U64::classify(0xFF0000000000u64)) >> 40);
        result[6 + (i * 8)] = U8::from((*v & U64::classify(0xFF000000000000u64)) >> 48);
        result[7 + (i * 8)] = U8::from((*v & U64::classify(0xFF00000000000000u64)) >> 56);
    }
    result
}

#[macro_export]
macro_rules! secret_array {
    ( $int_type: ident, [ $( $x:expr ),+ ] ) => {
        [
            $(
                $int_type($x)
            ),+
        ]
    }
}

#[macro_export]
macro_rules! secret_bytes {
    ([ $( $x:expr ),+ ] ) => {
        secret_array!(U8, [$($x),+])
    }
}

#[macro_export]
macro_rules! assert_secret_array_eq {
    ( $a1: expr, $a2: expr, $si: ident) => {
        assert_eq!(
            $a1.iter().map(|x| $si::declassify(*x)).collect::<Vec<_>>(),
            $a2.iter().map(|x| $si::declassify(*x)).collect::<Vec<_>>()
        );
    };
}

#[macro_export]
macro_rules! assert_bytes_eq {
    ( $a1: expr, $a2: expr) => {
        assert_secret_array_eq!($a1, $a2, U8)
    };
}

#[macro_export]
macro_rules! unsigned_integer {
    ($name:ident, $bits:literal) => {
        define_abstract_integer_checked!($name, $bits);
    };
}

#[macro_export]
macro_rules! field_integer {
    ($name:ident, $base:ident, $max:expr) => {
        define_refined_modular_integer!($name, $base, $max);

        impl $name {
            pub fn from_byte_seq_le<A: SeqTrait<U8>>(s: A) -> $name {
                $name::from_bytes_le(
                    s.iter()
                        .map(|x| U8::declassify(*x))
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
            }

            pub fn to_byte_seq_le(self) -> Seq<U8> {
                Seq::from(
                    self.to_bytes_le()
                        .iter()
                        .map(|x| U8::classify(*x))
                        .collect::<Vec<U8>>(),
                )
            }

            pub fn from_secret_literal(x: U128) -> $name {
                $name::from_literal(U128::declassify(x))
            }
        }
    };
}
