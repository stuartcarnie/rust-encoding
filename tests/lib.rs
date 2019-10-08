extern crate encoding;

use encoding::simple8b::encode_all;
use encoding::simple8b::decode;
use encoding::simple8b::decode_all;

#[test]
fn it_works() {
    let src = vec![0u64; 240];
    let mut dst = vec![0u64; 16];
    //d[120] = 2;
    let res = encode_all(&src, &mut dst).expect("nope");
}

#[test]
fn mixed_sizes() {
    let src = vec![7, 6, 256, 4, 3, 2, 1];
    let mut dst = vec![0u64; 16];
    let v = encode_all(&src, &mut dst).expect("nope");
    let mut dst2 = vec![0u64; 240];
    let res = decode_all(&v, &mut dst2).expect("nope");
    assert_eq!(res.to_vec(), src.to_vec());
}

#[test]
fn it_decodes() {
    let mut dst = vec![0u64; 240];
    let v = decode(2 << 60 | 0x3030_3030_3030, &mut dst);

    for (_, v) in dst[..v].iter().enumerate() {
        print!("{}, ", v)
    }
}

#[test]
fn test_encode_all() {
    let mut src = vec![15u64; 1024];
    let mut dst = vec![0u64; 240];
    let v = encode_all(&src, &mut dst).expect("nope");
}
