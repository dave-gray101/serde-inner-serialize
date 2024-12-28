# serde_inner_serialize macro crate
## Helper for custom Serializers

This library is designed to add some helper methods to a struct making it easier to manually serialize its fields.

```{rust}
pub trait InnerSerializableTrait {
    const TYPE_NAME: &'static str;
    fn count_fields() -> usize;
    fn inner_serialize<S>(&self, state: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeStruct;
}

pub trait OuterSerializableTrait<T> where T: InnerSerializableTrait {
    const TYPE_NAME: &'static str;

    fn get_full_type_name(&self) -> &'static str;
}
```

By deriving from `InnerSerializable` and `OuterSerializable` these traits and their implementation can be added to your custom structures, making it easy to represent "alternative versions" of a structure that add additional fields (or computed values that must be serialized) while retaining a "flat" output.

For an example of how to use these types in practice, see [this example file](tests/integration_tests.rs)

The structure of this repo is based on the guidelines found within https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff