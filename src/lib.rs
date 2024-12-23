pub use serde_inner_serialize_derive::InnerSerializable;
pub use serde_inner_serialize_core::InnerSerializableTrait;

#[macro_export]
macro_rules! embed_type_name {
    ($t:ty, $prefix:expr, $suffix:expr) => {
        concat!($prefix, stringify!($t), $suffix)
    };
}