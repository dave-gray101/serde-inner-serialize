#![cfg(test)]

use crate::inner_serializable_core;
use quote::quote;

#[test]
fn simple_test() {

    let before = quote! {
        pub struct Test {
            foo: i32,
            bar: String
        }
    };
    let after = inner_serializable_core(before);
    assert_eq!(
        after.to_string(),
        "impl InnerSerializableTrait for Test { fn count_fields () -> usize { 2usize } fn inner_serialize < S > (& self , state : & mut S) -> Result < () , S :: Error > where S : serde :: ser :: SerializeStruct , { state . serialize_field (\"foo\" , & self . foo) ? ; state . serialize_field (\"bar\" , & self . bar) ? ; Ok (()) } }"
    );
}

#[test]
fn lifetime_test() {

    let before = quote! {
        pub struct Test<'a> {
            foo: i32,
            bar: String
        }
    };
    let after = inner_serializable_core(before);
    assert_eq!(
        after.to_string(),
        "impl < 'a > InnerSerializableTrait for Test < 'a > { fn count_fields () -> usize { 2usize } fn inner_serialize < S > (& self , state : & mut S) -> Result < () , S :: Error > where S : serde :: ser :: SerializeStruct , { state . serialize_field (\"foo\" , & self . foo) ? ; state . serialize_field (\"bar\" , & self . bar) ? ; Ok (()) } }"
    );
}

#[test]
fn complicated_test() {

    let before = quote! {
        pub struct Test<'a, T> where T: FakeTrait {
            foo: i32,
            bar: String
        }
    };
    let after = inner_serializable_core(before);
    assert_eq!(
        after.to_string(),
        "impl < 'a , T > InnerSerializableTrait for Test < 'a , T > where T : FakeTrait { fn count_fields () -> usize { 2usize } fn inner_serialize < S > (& self , state : & mut S) -> Result < () , S :: Error > where S : serde :: ser :: SerializeStruct , { state . serialize_field (\"foo\" , & self . foo) ? ; state . serialize_field (\"bar\" , & self . bar) ? ; Ok (()) } }"
    );
}