//! 汉语拼音转换工具 Rust 版。
//! [![Build Status](https://img.shields.io/travis/mozillazg/rust-pinyin/master.svg)](https://travis-ci.org/mozillazg/rust-pinyin)
//! [![Coverage Status](https://img.shields.io/coveralls/mozillazg/rust-pinyin/master.svg)](https://coveralls.io/github/mozillazg/rust-pinyin)
//! [![Crates.io Version](https://img.shields.io/crates/v/pinyin.svg)](https://crates.io/crates/pinyin)
//! [![GitHub
//! stars](https://img.shields.io/github/stars/mozillazg/rust-pinyin.svg?style=social&label=Star)](https://github.com/mozillazg/rust-pinyin)
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/pinyin) and can be
//! used by adding `pinyin` to your dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! pinyin = "*"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate pinyin;
//! ```
//!
//! ```
extern crate phf;
use std::collections::HashSet;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn single_pinyin<'a>(c: char) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    let n: u32 = c as u32;

    match PINYIN_MAP.get(&n) {
        Some(&pys) => {
            let x: HashSet<_> = pys.split(',').map(|x| x.to_string()).collect();
            for s in x {
                ret.push(s);
            };
        },
        None => {
            ret = vec![];
        }
    };

    ret
}

/// 汉字转拼音
///
/// ```
/// let hans = "中国人";
///
/// // 默认输出 [["zhong"] ["guo"] ["ren"]]
/// println!("{:?}",  pinyin::pinyin(hans));
/// ```
pub fn pinyin<'a>(s: &'a str) -> Vec<Vec<String>> {
    let mut ret: Vec<Vec<String>> = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    for c in chars {
        ret.push(single_pinyin(c));
    }

    return ret
}
