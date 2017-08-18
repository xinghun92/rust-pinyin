extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut builder = phf_codegen::Map::new();
    let data = include!("data");
    for i in 0..data.len() {
        let (k, v) = data[i];
        builder.entry(k, &escape_str(v));
    }
    // 拼音库
    write!(&mut file, "static PINYIN_MAP: ::phf::Map<u32, &'static str> = ").unwrap();
    builder.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}

fn escape_str(s: &str) -> String {
    let mut res = String::new();
    res.push('"');
    for ch in s.chars() {
        res.push_str(&format!("\\u{{{:x}}}", ch as u32));
    }
    res.push('"');
    res
}
