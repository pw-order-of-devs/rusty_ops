use rstest::rstest;

use commons::hashing::sha::{hmac512, sha512};

#[rstest]
#[case("test")]
#[case("password")]
#[case("some_test_string")]
fn hmac512_test(#[case] key: &str) {
    let hash = hmac512(key);
    assert!(hash.is_ok());
}

#[rstest]
#[case("test", "ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db27ac185f8a0e1d5f84f88bc887fd67b143732c304cc5fa9ad8e6f57f50028a8ff")]
#[case("password", "b109f3bbbc244eb82441917ed06d618b9008dd09b3befd1b5e07394c706a8bb980b1d7785e5976ec049b46df5f1326af5a2ea6d103fd07c95385ffab0cacbc86")]
#[case("some_test_string", "28994e7dba67174ef81b31f56c650c63204bb1c0dc9d162ffef3f1f7f1179b22e0be73d8a9a7b21966fbdd7fdce1faa06e76813ea84976434b0fa344e6bb3b0d")]
fn sha512_test(#[case] input: &str, #[case] output: &str) {
    assert_eq!(output.to_string(), sha512(input))
}
