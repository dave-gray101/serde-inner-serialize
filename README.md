# serde_inner_serialize macro crate
## Helper for custom Serializers

This library is designed to add some helper methods to a struct making it easier to manually serialize its fields.

```{rust}
InnerSerializableTraitpub trait InnerSerializableTrait {
    fn count_fields() -> usize;
    fn inner_serialize<S>(&self, state: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeStruct;
}
```

A proper example will follow soon.

This repo is based on the guidelines found within https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff