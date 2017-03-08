#![feature(test)]

extern crate encoding;
extern crate encoding_rs;
extern crate test;

use test::{Bencher, black_box};

#[bench]
fn bench_std_validation(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    b.iter(|| {
        let s = ::std::str::from_utf8(black_box(f)).unwrap();
        black_box(s.len())
    })
}

#[bench]
fn bench_encoding_rs_validation(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    b.iter(|| {
        black_box(encoding_rs::Encoding::utf8_valid_up_to(black_box(f)))
    })
}

#[bench]
fn bench_std_str(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    b.iter(|| {
        let s = ::std::str::from_utf8(black_box(f)).unwrap().to_string();
        black_box(s.len())
    })
}

#[bench]
fn bench_std_string(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    b.iter(|| {
        let s = ::std::string::String::from_utf8(black_box(f).to_vec()).unwrap();
        black_box(s.len())
    })
}

#[bench]
fn bench_encoding_rs(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding_rs::UTF_8;
    b.iter(|| {
        let s = encoding.decode(black_box(f)).0.into_owned();
        black_box(s.len())
    })
}

#[bench]
fn bench_encoding_rs_w_bom(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding_rs::UTF_8;
    b.iter(|| {
        let s = encoding.decode_with_bom_removal(black_box(f)).0.into_owned();
        black_box(s.len())
    })
}

#[bench]
fn bench_encoding_rs_wo_bom(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding_rs::UTF_8;
    b.iter(|| {
        let s = encoding.decode_without_bom_handling(black_box(f)).0.into_owned();
        black_box(s.len())
    })
}


#[bench]
fn bench_encoding_rs_wo_bom_and_repl(b: &mut Bencher) {
    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding_rs::UTF_8;
    b.iter(|| {
        let s = encoding.decode_without_bom_handling_and_without_replacement(black_box(f)).unwrap().into_owned();
        black_box(s.len())
    })
}

#[bench]
fn bench_encoding_ignore(b: &mut Bencher) {
    use encoding::Encoding;

    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding::all::UTF_8;
    b.iter(|| {
        let s = encoding.decode(black_box(f), ::encoding::types::DecoderTrap::Ignore).unwrap();
        black_box(s.len())
    })
}

#[bench]
fn bench_encoding_strict(b: &mut Bencher) {
    use encoding::Encoding;

    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding::all::UTF_8;
    b.iter(|| {
        let s = encoding.decode(black_box(f), ::encoding::types::DecoderTrap::Strict).unwrap();
        black_box(s.len())
    })
}

#[bench]
fn bench_encoding_replace(b: &mut Bencher) {
    use encoding::Encoding;

    let f = include_bytes!("../itunes.xml");
    let encoding = ::encoding::all::UTF_8;
    b.iter(|| {
        let s = encoding.decode(black_box(f), ::encoding::types::DecoderTrap::Replace).unwrap();
        black_box(s.len())
    })
}
