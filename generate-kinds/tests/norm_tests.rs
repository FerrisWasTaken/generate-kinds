#![allow(unused)]
use generate_kinds::kinds;
use get_kinds::Kind;

#[kinds]
#[derive(Debug)]
enum TestEnum {
    T1,
    T2
}

#[test]
fn norm_test() {
    let tval = TestEnum::T1;
    assert_eq!(tval.kind(), "TestEnum :: T1");
}