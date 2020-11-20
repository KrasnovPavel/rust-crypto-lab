use std::convert::TryInto;
use itertools::*;

const PI_0: [u8; 64] = [58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4,
                           62, 54, 46, 38, 30, 22, 14, 6, 64, 56, 48, 40, 32, 24, 16, 8,
                           57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3,
                           61, 53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7];

const IP_1: [u8; 64] = [40, 8, 48, 16, 56, 24, 64, 32, 39, 7, 47, 15, 55, 23, 63, 31,
                            38, 6, 46, 14, 54, 22, 62, 30, 37, 5, 45, 13, 53, 21, 61, 29,
                            36, 4, 44, 12, 52, 20, 60, 28, 35, 3, 43, 11, 51, 19, 59, 27,
                            34, 2, 42, 10, 50, 18, 58, 26, 33, 1, 41 ,9, 49, 17, 57, 25];

const E: [u8; 48] = [32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 8, 9, 10, 11, 12, 13,
                        12, 13, 14, 15, 16, 17, 16, 17, 18, 19, 20, 21, 20, 21, 22, 23, 24, 25,
                        24, 25, 26, 27, 28, 29, 28, 29, 30, 31, 32, 1];

const P: [u8; 32] = [16, 7, 20, 21, 29, 12, 28, 17, 1, 15, 23, 26, 5, 18, 31, 10,
                        2, 8, 24, 14, 32, 27, 3, 9, 19, 13, 30, 6, 22, 11, 4, 25];

const S: [[u8; 64]; 8] = [[14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7,
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

const C: [u8; 28] = [57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18,
                        10, 2, 59, 51, 43, 35, 27, 19, 11, 3, 60, 52, 44, 36];
const D: [u8; 28] = [63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22,
                        14, 6, 61, 53, 45, 37, 29, 21, 13, 5, 28, 20, 12, 4];
const SHIFT: [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];
const KEY: [u8; 48] = [14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4,
                          26, 8, 16, 7, 27, 20, 13, 2, 41, 52, 31, 37, 47, 55, 30, 40,
                          5, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32];

pub fn encrypt_text(text: &Vec<u8>, key: &String, decrypt: bool) -> Vec<u8>
{
    let (b_keys, b_text) = prepare_data(text, key);
    b_text.iter()
        .flat_map(|t| if decrypt {decrypt_block(t, &b_keys)} else {encrypt_block(t, &b_keys)}
            .chunks(8)
            .map(to_u8)
            .collect_vec())
        .collect()
}

fn prepare_data(text: &Vec<u8>, key: &String) -> ([[bool; 48]; 16], Vec<[bool; 64]>)
{
    let b_key: [bool; 56] = key.clone()
        .into_bytes()
        .iter()
        .flat_map(|&byte|
            from_u8(byte).iter().cloned().collect_vec())
        .collect::<Vec<bool>>()[..56]
        .try_into().unwrap();

    let keys = generate_keys(&b_key);

    let b_text: Vec<[bool; 64]> = text
        .iter()
        .flat_map(|&byte| from_u8(byte).iter().cloned().collect_vec())
        .collect::<Vec<bool>>()
        .chunks(64)
        .map(|chunk| chunk.try_into().unwrap())
        .collect();

    (keys, b_text)
}

fn encrypt_block(text: &[bool; 64], keys: &[[bool; 48]; 16]) -> [bool; 64]
{
    let mut ip_t = permutate(&text.to_vec(), &PI_0.to_vec());

    let mut r: [bool; 32] = ip_t.split_off(32).try_into().unwrap();
    let mut l: [bool; 32] = ip_t.try_into().unwrap();
    for i in 0..16 {
        let tmp_l = r.clone();
        r = xor_32(&l, &feistel(&l, &keys[i]));
        l = tmp_l;
    }

    permutate(&[l, l].concat(), &IP_1.to_vec()).try_into().unwrap()
}

fn decrypt_block(text: &[bool; 64], keys: &[[bool; 48]; 16]) -> [bool; 64]
{
    let mut ip_t = permutate(&text.to_vec(), &IP_1.to_vec());

    let mut l: [bool; 32] = ip_t.split_off(32).try_into().unwrap();
    let mut r: [bool; 32] = ip_t.try_into().unwrap();
    for i in (0..16).rev() {
        let tmp_r = l.clone();
        l = xor_32(&r, &feistel(&l, &keys[i]));
        r = tmp_r;
    }

    permutate(&[r, r].concat(), &PI_0.to_vec()).try_into().unwrap()
}

fn permutate(arr: &Vec<bool>, order: &Vec<u8>) -> Vec<bool> {
    order.iter()
        .map(|i| i - 1)
        .map(|i| arr[i as usize])
        .collect::<Vec<bool>>()
}

fn xor_32(left: &[bool; 32], right: &[bool; 32]) -> [bool; 32]
{
    izip!(left.iter(), right.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<bool>>()
        .try_into().unwrap()
}

pub fn from_u8(i: u8) -> [bool; 8]
{
    let mut res: Vec<bool> = vec![];
    for j in 0..8
    {
        res.push((i >> j) % 2 == 1)
    }
    res.try_into().unwrap()
}

pub fn to_u8(arr: &[bool]) -> u8
{
    let mut res: u8 = 0;
    for &b in arr.iter().rev()
    {
        res <<= 1;
        res += if b {1} else {0};
    }
    res
}

pub fn left_shift(arr: &[u8; 28], shift: &u8) -> [u8; 28]
{
    let p1 = arr.iter().take(*shift as usize).collect::<Vec<_>>();
    let mut p2 = arr.iter().skip(*shift as usize).collect::<Vec<_>>();
    p2.extend(p1);

    p2.iter().cloned().cloned().collect::<Vec<u8>>().try_into().unwrap()
}


fn feistel(r: &[bool; 32], k: &[bool; 48]) -> [bool; 32]
{
    let arr : Vec<bool> = izip!(permutate(&r.to_vec(), &E.to_vec()).iter(), k.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<bool>>()
        .chunks(6)
        .zip(S.iter())
        .map(|(b, s)|
            from_u8(s[(
                to_u8(&[b[0], b[5]]) * 16 + to_u8(&b[1..5])) as usize])[4..].try_into().unwrap())
        .collect::<Vec<[bool; 4]>>()
        .concat();

    permutate(&arr, &P.to_vec()).try_into().unwrap()
}

fn generate_keys(key: &[bool; 56]) -> [[bool; 48]; 16]
{
    let c_i = SHIFT.iter().map(|s| left_shift(&C, s));
    let d_i = SHIFT.iter().map(|s| left_shift(&D, s));

    izip!(c_i, d_i)
        .map(|(c, d)| c
            .iter()
            .chain(d.iter())
            .map(|i| i - 1)
            .map(|i| key[(i - i / 8) as usize]).collect::<Vec<bool>>())
        .map(|a| permutate(&a, &KEY.to_vec()).try_into().unwrap())
        .collect::<Vec<[bool; 48]>>()
        .try_into().unwrap()
}