#![feature(test)]

extern crate encoding;
extern crate test;

use encoding::simple8b::encode_all;
use encoding::simple8b::decode;
use test::Bencher;

#[bench]
fn bench_encode_all_4_bits(b: &mut Bencher) {
    let src = vec![15u64; 1024];
    let mut dst = vec![0u64; 240];
    b.iter(|| {
        encode_all(&src, &mut dst).expect("nope");
    })
}

#[bench]
fn bench_encode_all_1_bits(b: &mut Bencher) {
    let src = vec![1u64; 1024];
    let mut dst = vec![0u64; 240];
    b.iter(|| {
        encode_all(&src, &mut dst).expect("nope");
    })
}

macro_rules! bench_decode {
    ($name:ident, $sel:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut dst = vec![0u64; 240];
            b.iter(|| {
                decode($sel << 60, &mut dst);
            })
        }
    };
}

bench_decode!(decode_240, 0);
bench_decode!(decode_120, 1);
bench_decode!(decode_060, 2);
bench_decode!(decode_030, 3);
bench_decode!(decode_020, 4);
bench_decode!(decode_015, 5);
bench_decode!(decode_012, 6);
bench_decode!(decode_010, 7);
bench_decode!(decode_008, 8);
bench_decode!(decode_007, 9);
bench_decode!(decode_006, 10);
bench_decode!(decode_005, 11);
bench_decode!(decode_004, 12);
bench_decode!(decode_003, 13);
bench_decode!(decode_002, 14);
bench_decode!(decode_001, 15);
