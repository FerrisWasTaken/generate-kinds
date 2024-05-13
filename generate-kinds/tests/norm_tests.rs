#![allow(unused)]
use get_kinds::GetKind;

#[derive(Debug, GetKind)]
enum TestEnum {
    T1,
    T2,
}

#[test]
fn norm_test() {
    let tval = TestEnum::T1;
    assert_eq!(tval.kind(), "TestEnum :: T1");
    let tval = TestEnum::T2;
    assert_eq!(tval.kind(), "TestEnum :: T2");
}
