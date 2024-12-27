#![cfg(test)]

use crate::{inner_serializable_core, outer_serializable_core};
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
    // println!("simple_test:\n{}\n", after.to_string());
    assert_eq!(
        after.to_string(),
        "impl InnerSerializableTrait for Test { # [cfg (feature = \"const_type_name\")] const TYPE_NAME : & 'static str = std :: any :: type_name :: < Test > () ; # [cfg (not (feature = \"const_type_name\"))] const TYPE_NAME : & 'static str = stringify ! (Test) ; fn count_fields () -> usize { 2usize } fn inner_serialize < S > (& self , state : & mut S) -> Result < () , S :: Error > where S : serde :: ser :: SerializeStruct , { state . serialize_field (\"foo\" , & self . foo) ? ; state . serialize_field (\"bar\" , & self . bar) ? ; Ok (()) } }"
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
    // println!("lifetime_test:\n{}\n", after.to_string());
    assert_eq!(
        after.to_string(),
        "impl < 'a > InnerSerializableTrait for Test < 'a > { # [cfg (feature = \"const_type_name\")] const TYPE_NAME : & 'static str = std :: any :: type_name :: < Test > () ; # [cfg (not (feature = \"const_type_name\"))] const TYPE_NAME : & 'static str = stringify ! (Test) ; fn count_fields () -> usize { 2usize } fn inner_serialize < S > (& self , state : & mut S) -> Result < () , S :: Error > where S : serde :: ser :: SerializeStruct , { state . serialize_field (\"foo\" , & self . foo) ? ; state . serialize_field (\"bar\" , & self . bar) ? ; Ok (()) } }"
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
    // println!("complicated_test:\n{}\n", after.to_string());
    assert_eq!(
        after.to_string(),
        "impl < 'a , T > InnerSerializableTrait for Test < 'a , T > where T : FakeTrait { # [cfg (feature = \"const_type_name\")] const TYPE_NAME : & 'static str = std :: any :: type_name :: < Test > () ; # [cfg (not (feature = \"const_type_name\"))] const TYPE_NAME : & 'static str = stringify ! (Test) ; fn count_fields () -> usize { 2usize } fn inner_serialize < S > (& self , state : & mut S) -> Result < () , S :: Error > where S : serde :: ser :: SerializeStruct , { state . serialize_field (\"foo\" , & self . foo) ? ; state . serialize_field (\"bar\" , & self . bar) ? ; Ok (()) } }"
    );
}

#[test]
fn outer_test() {
    let before = quote! {
        pub struct OuterBox<'a, T> where T: InnerSerializableTrait {
            inner: &'a T,
        }
    };
    let after = outer_serializable_core(before);
    println!("outer_test:\n{}\n", after.to_string());
    assert_eq!(
        after.to_string(),
        "impl < 'a , T > OuterSerializableTrait < T > for OuterBox < 'a , T > where T : InnerSerializableTrait { # [cfg (feature = \"const_type_name\")] const TYPE_NAME : & 'static str = std :: any :: type_name :: < OuterBox > () ; # [cfg (not (feature = \"const_type_name\"))] const TYPE_NAME : & 'static str = stringify ! (OuterBox) ; fn _get_full_type_name (& self) -> String { let mut s = String :: from (OuterBox :: < 'a , T > :: TYPE_NAME) ; s . push_str (\"->\") ; s . push_str (T :: TYPE_NAME) ; s } fn get_full_type_name (& self) -> & 'static str { static TYPEMAP : std :: sync :: LazyLock < std :: sync :: Mutex < std :: collections :: HashMap < & 'static str , & 'static str >> > = std :: sync :: LazyLock :: new (|| std :: sync :: Mutex :: new (std :: collections :: HashMap :: < & 'static str , & 'static str > :: new ())) ; let mut jtn = TYPEMAP . lock () . unwrap () ; let e = jtn . entry (T :: TYPE_NAME) ; e . or_insert_with (|| { let s : & str = & self . _get_full_type_name () ; let leaked_str : & 'static str = s . to_string () . leak () ; leaked_str }) } }"
    );
}