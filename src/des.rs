use std::convert::TryInto;
use std::ops::Deref;
use itertools::*;

#[derive(Deref)]
pub struct b64([bool; 64]);

#[derive(Deref)]
pub struct b56([bool; 56]);

#[derive(Deref)]
pub struct b48([bool; 48]);

#[derive(Deref)]
pub struct b32([bool; 32]);

#[derive(Deref)]
pub struct b6([bool; 6]);

#[derive(Deref)]
pub struct b4([bool; 4]);

#[derive(Deref)]
pub struct b2([bool; 2]);

const PI_0: [usize; 64] = [58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4,
                           62, 54, 46, 38, 30, 22, 14, 6, 64, 56, 48, 40, 32, 24, 16, 8,
                           57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3,
                           61, 53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7];

const E: [usize; 48] = [32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 8, 9, 10, 11, 12, 13,
                        12, 13, 14, 15, 16, 17, 16, 17, 18, 19, 20, 21, 20, 21, 22, 23, 24, 25,
                        24, 25, 26, 27, 28, 29, 28, 29, 30, 31, 32, 1];

const P: [usize; 32] = [16, 7, 20, 21, 29, 12, 28, 17, 1, 15, 23, 26, 5, 18, 31, 10,
                        2, 8, 24, 14, 32, 27, 3, 9, 19, 13, 30, 6, 22, 11, 4, 25];

const S: [[usize; 64]; 8] = [[14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7,
                              0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8,
                              4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0,
                              15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
                             [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10,
                              3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5,
                              0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15,
                              13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
                             [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8,
                              13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1,
                              13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7,
                              1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
                             [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15,
                              13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9,
                              10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4,
                              3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
                             [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9,
                              14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6,
                              4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14,
                              11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
                             [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11,
                              10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8,
                              9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6,
                              4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
                             [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1,
                              13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6,
                              1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2,
                              6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
                             [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7,
                              1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2,
                              7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8,
                              2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11]];

const C: [usize; 28] = [57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18,
                        10, 2, 59, 51, 43, 35, 27, 19, 11, 3, 60, 52, 44, 36];
const D: [usize; 28] = [63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22,
                        14, 6, 61, 53, 45, 37, 29, 21, 13, 5, 28, 20, 12, 4];
const SHIFT: [usize; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];
const KEY: [usize; 48] = [14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4,
                          26, 8, 16, 7, 27, 20, 13, 2, 41, 52, 31, 37, 47, 55, 30, 40,
                          5, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32];

pub fn encrypt()
{

}

impl b64 {
    pub fn split(&self) -> (b32, b32)
    {
        (b32(*array_ref![self, 0, 32]), b32(*array_ref![self, 33, 32]))
    }

    pub fn join(left: b32, right: b32) -> b64
    {
        let mut res = [false; 64];

        for (i, b) in left.iter().enumerate() {
            res[i] = *b;
        }
        for (i, b) in right.iter().enumerate() {
            res[i] = *b;
        }
        b64(res)
    }

    pub fn to_bytes(&self) -> [u8; 8]
    {
        let mut res = [0; 8];
        for (i, b) in self.iter().enumerate()
        {
            res[i / 8] = res[i / 8] | ((if *b {1} else {0}) << (i % 8));
        }
        res
    }

    fn permutate(&self, order: &Vec<usize>) -> Self {
        let arr = order.iter().map(|&i| self[i]).collect::<Vec<bool>>();
        b64(*array_ref![arr, 0, 64])
    }
}

impl b32 {
    pub fn join(data: &[b4; 8]) -> b32
    {
        let res: Vec<bool> = data.iter().flat_map(|b| b.iter()).map(|a| *a).collect();
        b32(*array_ref![res, 0, 32])
    }

    pub fn xor(&self, other: &b32) -> b32
    {
        let arr = izip!(self.iter(), other.iter()).map(|(a, b)| a ^ b).collect::<Vec<bool>>();
        b32(*array_ref![arr, 0, 32])
    }

    pub fn expand(&self, order: &Vec<usize>) -> b48
    {
        let arr = order.iter().map(|&i| self[i]).collect::<Vec<bool>>();
        b48(*array_ref![arr, 0, 48])
    }

    fn permutate(&self, order: &Vec<usize>) -> Self {
        let arr = order.iter().map(|&i| self[i]).collect::<Vec<bool>>();
        b32(*array_ref![arr, 0, 32])
    }
}
impl b6
{
    pub fn edges(&self) -> b2
    {
        b2([self[0], self[5]])
    }

    pub fn middle(&self) -> b4
    {
        b4([self[1], self[2], self[3], self[4]])
    }
}

impl b4
{
    pub fn new(i: usize) -> b4
    {
        let _b1 = i % 2 == 1;
        let _b2 = (i >> 1) % 2 == 1;
        let _b3 = (i >> 2) % 2 == 1;
        let _b4 = (i >> 3) % 2 == 1;
        b4([_b4, _b3, _b2, _b1])
    }
}

pub fn to_usize(arr: &[bool]) -> usize
{
    let mut res: usize = 0;
    for &b in arr.iter().rev()
    {
        res <<= 1;
        res += if b {1} else {0};
    }
    res
}

pub fn left_shift(arr: [usize; 28], shift: usize) -> [usize; 28]
{
    let mut p1 = arr.iter().take(shift).collect::<Vec<_>>();
    let p2 = arr.iter().skip(28 - shift);
    p1.extend(p2);

    let res = p1.iter().map(|a| **a).collect::<Vec<usize>>();
    *array_ref![res, 0,  28]
}


fn feistel(r: &b32, k: &b48) -> b32
{
    let _b = izip!(r.expand(&E.to_vec()).iter(), k.iter()).map(|(a, b)| a ^ b).collect::<Vec<bool>>();
    let B: Vec<b6> = vec![b6(*array_ref![_b, 0,  6]), b6(*array_ref![_b, 7,  6]),
                          b6(*array_ref![_b, 13, 6]), b6(*array_ref![_b, 19, 6]),
                          b6(*array_ref![_b, 25, 6]), b6(*array_ref![_b, 31, 6]),
                          b6(*array_ref![_b, 37, 6]), b6(*array_ref![_b, 33, 6])];

    let mut res: Vec<b4> = vec![];
    for (b, s) in izip!(B.iter(), S.iter())
    {
        let i = to_usize(&b.edges()[..]) * 16 + to_usize(&b.middle()[..]);
        res.push(b4::new(s[i]));
    }

    b32::join(array_ref![res, 0, 8]).permutate(&P.to_vec())
}

fn generate_keys(key: &b56) -> &[b48; 16]
{
    let c_i = SHIFT.iter().map(|s| left_shift(C, *s));
    let d_i = SHIFT.iter().map(|s| left_shift(D, *s));

    let res: Vec<Vec<bool>> = izip!(c_i, d_i)
                .map(|(c, d)| c
                    .iter()
                    .chain(d.iter())
                    .map(|i| key[i - i / 8]).collect::<Vec<bool>>())
                .map(|a| KEY.iter().map(|&k| a[k])).collect();
    let r = res.iter().map(|arr| b48(*array_ref![arr, 0,  48])).collect::<Vec<b48>>();
    array_ref![r, 0, 16]
}