#![feature(test)]

extern crate encoding;
extern crate encoding_rs;
extern crate test;

use test::{Bencher, black_box};

#[bench]
fn bench_std_str(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    b.iter(|| {
        let s = ::std::str::from_utf8(f).unwrap().to_string();
        s.len()
    })
}

#[bench]
fn bench_std_string(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    b.iter(|| {
        let s = ::std::string::String::from_utf8(f.to_vec()).unwrap();
        s.len()
    })
}

#[bench]
fn bench_encoding_rs(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding_rs::UTF_8;
    b.iter(|| {
        let s = encoding.decode(f).0.into_owned();
        s.len()
    })
}

#[bench]
fn bench_encoding_rs_w_bom(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding_rs::UTF_8;
    b.iter(|| {
        let s = encoding.decode_with_bom_removal(f).0.into_owned();
        s.len()
    })
}

#[bench]
fn bench_encoding_rs_wo_bom(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding_rs::UTF_8;
    b.iter(|| {
        let s = encoding.decode_without_bom_handling(f).0.into_owned();
        s.len()
    })
}


#[bench]
fn bench_encoding_rs_wo_bom_and_repl(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding_rs::UTF_8;
    b.iter(|| {
        let s = encoding.decode_without_bom_handling_and_without_replacement(f).unwrap().into_owned();
        s.len()
    })
}

#[bench]
fn bench_encoding_ignore(b: &mut Bencher) {
    use encoding::Encoding;

    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding::all::UTF_8;
    b.iter(|| {
        let s = encoding.decode(f, ::encoding::types::DecoderTrap::Ignore).unwrap();
        s.len()
    })
}

#[bench]
fn bench_encoding_strict(b: &mut Bencher) {
    use encoding::Encoding;

    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding::all::UTF_8;
    b.iter(|| {
        let s = encoding.decode(f, ::encoding::types::DecoderTrap::Strict).unwrap();
        s.len()
    })
}

#[bench]
fn bench_encoding_replace(b: &mut Bencher) {
    use encoding::Encoding;

    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding::all::UTF_8;
    b.iter(|| {
        let s = encoding.decode(f, ::encoding::types::DecoderTrap::Replace).unwrap();
        s.len()
    })
}
