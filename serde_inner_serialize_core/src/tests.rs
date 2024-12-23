#![cfg(test)]

use crate::inner_serializable_core;
use quote::quote;

#[test]
fn test() {

    let before = quote! {
        struct Test {
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