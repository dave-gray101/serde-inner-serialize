use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Data, DeriveInput };

mod tests;

pub trait InnerSerializableTrait {
    fn count_fields() -> usize;
    fn inner_serialize<S>(&self, state: &mut S) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeStruct;
}


pub fn inner_serializable_core(input: TokenStream) -> TokenStream {

    let input = parse2::<DeriveInput>(input).unwrap();
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = &generics.split_for_impl();

    let count = if let syn::Data::Struct(data) = &input.data {
        data.fields.iter().count()
    } else {
        0
    };

    let serialize_fields = if let Data::Struct(data) = &input.data {
        data.fields.iter().map(|field| {
            let field_name = &field.ident;
            let field_name_str = field_name.as_ref().unwrap().to_string();
            quote! {
                state.serialize_field(#field_name_str, &self.#field_name)?;
            }
        }).collect::<Vec<_>>().into_iter()
    } else {
        Vec::new().into_iter()
    };

    let expanded = quote! {
        impl #impl_generics InnerSerializableTrait for #name #ty_generics #where_clause {
            fn count_fields() -> usize {
                #count
            }

            fn inner_serialize<S>(&self, state: &mut S) -> Result<(), S::Error>
            where
                S: serde::ser::SerializeStruct,
            {
                #(#serialize_fields)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)

}