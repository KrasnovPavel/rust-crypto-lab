#[macro_use]
extern crate derive_deref;

#[macro_use]
extern crate arrayref;

use std::{vec, fs};
use std::ops::Add;
use structopt::StructOpt;
use std::path::{Path, PathBuf};

mod des;

#[derive(Debug)]
#[derive(StructOpt)]
enum Command {
    Vertical {
        #[structopt(short)]
        key : Option<i32>
    },
    Random {
        #[structopt(short)]
        a: i32,
        #[structopt(short)]
        c: i32,
        #[structopt(short)]
        t_0: i32,
    },
    Table {
        #[structopt(short)]
        key: String
    },
    DES {
        #[structopt(short)]
        key: String
    },
}

#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    #[structopt(short="P", long="phrase")]
    phrase: Option<String>,
    #[structopt(short="F", long="file", parse(from_os_str))]
    file: Option<PathBuf>,
    #[structopt(short="D", long="decrypt")]
    decrypt: bool,
    #[structopt(subcommand)]
    command: Command,
}

fn int2vec(key: i32) -> Vec<usize>
{
    let mut tmp: Vec<usize> = Vec::new();
    let mut k = key;
    while k > 0 {
        tmp.push((k % 10) as usize);
        k /= 10;
    }

    tmp.reverse();
    tmp
}

fn reverse_key(key: Vec<usize>) -> Vec<usize> {
    let mut res = key.clone();
    let mut index = 0;
    for i in  key {
        res[i] = index;
        index += 1;
    }
    res
}

fn encrypt(s: String, key: Option<Vec<usize>>) -> String {
    let mut res = String::new();
    res.reserve(s.len());

    let k = key.unwrap_or(int2vec(542301));

    let mut height= s.len() / k.len();
    let width = s.len() / height;
    if k.len() % 10 != 0 {
        height += 1;
    }

    for i in 0..height {
        for j in &k {
            res = res.add(match s.chars().nth(i * width + j) {
                Some(ch) => ch.to_string(),
                None => " ".to_string()
            }.as_str());
        }
    }

    res
}

fn decrypt(s: String, key: Option<Vec<usize>>) -> String {
    encrypt(s, Some(reverse_key(key.unwrap_or(int2vec(542301))))).trim().to_string()
}

fn encrypt_table(s: String, key: String) -> Vec<u8>
{
    s.as_bytes().iter().enumerate().map(|(i, b)| (b ^ key.as_bytes()[i % key.len()])).collect()
}

#[derive(Copy, Clone)]
struct RandomGamma {
    a: i32,
    c: i32,
    t_0: i32,
}

impl RandomGamma {
    fn new(a: i32, c: i32, t_0: i32) -> RandomGamma {
        RandomGamma {a, c, t_0}
    }

    fn iter(&self) -> GammaIter {
        GammaIter { next: self.t_0, gamma: *self }
    }
}

struct GammaIter {
    next: i32,
    gamma: RandomGamma,
}

impl Iterator for GammaIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.gamma.a * self.next + self.gamma.c) % 73)
    }
}

fn encrypt_random(s: String, a: i32, c: i32, t_0: i32) -> Vec<u8>
{
    let key = RandomGamma::new(a, c, t_0).iter();
    s.as_bytes().iter().zip(key).map(|(b, k)| b ^ k as u8).collect()
}

fn main() {
    let args: Cli = Cli::from_args();

    let mut phrase: String;

    if let Some(key) = args.phrase {
        phrase = key;
    } else if let Some(path) = &args.file {
        phrase = fs::read_to_string(path.as_path()).unwrap();
    } else {
        panic!("Enter phrase or file for encryption");
    }

    if args.decrypt {
        match args.command {
            Command::Vertical {key} => println!("{}", decrypt(phrase, key.map(int2vec))),
            Command::Random {a, c, t_0} => {},
            Command::Table {key} => {fs::write(args.file.unwrap().as_path(), encrypt_table(phrase, key));},
            Command::DES {key} => {},
        }
    } else {
        match args.command {
            Command::Vertical {key} => println!("{}", encrypt(phrase, key.map(int2vec))),
            Command::Random {a, c, t_0} => {fs::write(args.file.unwrap().as_path(), encrypt_random(phrase, a, c, t_0));},
            Command::Table {key} => {fs::write(args.file.unwrap().as_path(), encrypt_table(phrase, key));},
            Command::DES {key} => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int2vec_test() {
        assert_eq!(int2vec(542301), vec![5, 4, 2, 3, 0, 1]);
        assert_eq!(int2vec(2013), vec![2, 0, 1, 3]);
    }

    #[test]
    fn reverse_key_test() {
        assert_eq!(reverse_key(int2vec(542301)), vec![4, 5, 2, 3, 1, 0]);
        assert_eq!(reverse_key(int2vec(2013)), vec![1, 2, 0, 3]);
    }
}
