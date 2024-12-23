use serde::Serialize;
use serde::ser::SerializeStruct;
use serde_inner_serialize::{embed_type_name, InnerSerializable, InnerSerializableTrait};


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

    pub struct OuterBox<'a, T> {
        inner: &'a T,
    }

    impl <'a,T>  OuterBox<'a, T> {
        fn boxed_type_name(&self) -> &'static str {
            embed_type_name!(T, "OuterBox<", ">")
        }
    }

    impl <'a, T> Serialize for OuterBox<'a, T> where T: InnerSerializableTrait + ComputeArtifical {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
    
            let fields = T::count_fields();
            let mut state = serializer.serialize_struct(&self.boxed_type_name(), fields+2)?;
            state.serialize_field("artificial_property", &self.inner.compute_artifical())?;
            state.serialize_field("constant_fake", "fake")?;
            self.inner.inner_serialize(&mut state).unwrap();
            state.end()
            // Err(serde::ser::Error::custom(format!("unable to serialize {}", std::any::type_name_of_val(&self.inner))))
        }
        
    }

    let baz_string = "baz".to_string();

    let test = Test::new(1, "bar".to_string(), &baz_string);

    let outerbox = OuterBox { inner: &test };

    assert_eq!(Test::count_fields(), 3);
    let serialized_result = serde_json::to_string(&outerbox).unwrap();
    println!("{}", serialized_result);
    assert_ne!(serialized_result, "");

}