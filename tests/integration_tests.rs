// use const_format::{concatcp, formatcp}

use serde::Serialize;
use serde::ser::SerializeStruct;
use serde_inner_serialize::{InnerSerializable, InnerSerializableTrait, OuterSerializable, OuterSerializableTrait};


#[test]
fn demo() {

    pub trait ComputeArtifical {
        fn compute_artifical(&self) -> String;
    }

    #[derive(InnerSerializable)]
    pub struct Test<'a> {
        foo: i32,
        bar: String,
        baz: &'a str,
    }

    #[derive(InnerSerializable)]
    pub struct AltTest {
        other: i32,
    }

    impl<'a> Test<'a> {
        // User supplied impl block
        pub fn new(foo: i32, bar: String, baz: &'a String) -> Self {
            Self { foo, bar, baz }
        }
    }

    impl<'a> ComputeArtifical for Test<'a> {
        fn compute_artifical(&self) -> String {
            format!("{}\\{}", self.foo, self.bar)
        }
    }

    impl ComputeArtifical for AltTest {
        fn compute_artifical(&self) -> String {
            format!("{}\\{}", self.other, "literal")
        }
    }
    
    #[derive(OuterSerializable)]
    pub struct OuterBox<'a, T> where T: InnerSerializableTrait {
        inner: &'a T,
    }

    impl <'a, T> Serialize for OuterBox<'a, T> where T: InnerSerializableTrait + ComputeArtifical {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
    
            let fields = T::count_fields();
    
            let mut state = serializer.serialize_struct(&self.get_full_type_name(), fields+2)?;
            state.serialize_field("artificial_property", &self.inner.compute_artifical())?;
            state.serialize_field("constant_fake", "fake")?;
            self.inner.inner_serialize(&mut state).unwrap();
            state.end()
        }
        
    }

    let baz_string = "baz".to_string();

    let test1 = Test::new(1, "bar2".to_string(), &baz_string);
    let test2 = Test::new(1337, "hax".to_string(), &baz_string);
    let test3 = AltTest { other: 1337 };

    let ob1 = OuterBox { inner: &test1 };
    let pob1 = &ob1.get_full_type_name();
    let ob2 = OuterBox { inner: &test2 };
    let pob2 = &ob2.get_full_type_name();
    let ob3 = OuterBox { inner: &test3 };
    let pob3 = &ob3.get_full_type_name();
    assert_eq!(Test::count_fields(), 3);

    // println!("pob1: {:?} // pob1 {:p}", pob1, pob1.as_ptr());
    // println!("pob2: {:?} // pob2 {:p}", pob2, pob2.as_ptr());
    // println!("pob3: {:?} // pob3 {:p}", pob3, pob3.as_ptr());

    assert_eq!(pob1.as_ptr(), pob2.as_ptr());
    assert_ne!(pob1.as_ptr(), pob3.as_ptr());


    let serialized_result = serde_json::to_string(&ob1).unwrap();
    println!("{}", serialized_result);
    assert_eq!(serialized_result, "{\"artificial_property\":\"1\\\\bar2\",\"constant_fake\":\"fake\",\"foo\":1,\"bar\":\"bar2\",\"baz\":\"baz\"}");

}