use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, AngleBracketedGenericArguments, Data, DeriveInput, GenericArgument, Turbofish, Type };

mod tests;

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
    fn _get_full_type_name(&self) -> String;
}

pub fn inner_serializable_core(input: TokenStream) -> TokenStream {

    let input = parse2::<DeriveInput>(input).unwrap();
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = &generics.split_for_impl();

    let type_name_impl = if cfg!(feature = "const_type_name") {
        quote! {
            const TYPE_NAME: &'static str = std::any::type_name::<#name>();
        }
    } else {
        quote! {
            const TYPE_NAME: &'static str = stringify!(#name);
        }
    };

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
    
            #type_name_impl

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

fn extract_types_from_turbofish(turbofish: &Turbofish) -> Vec<Type> {
    let ts = turbofish.to_token_stream();
    let abga = parse2::<AngleBracketedGenericArguments>(ts).unwrap();
    abga.args.iter()
        .filter_map(|arg| match arg {
            // Only look at type arguments, ignore lifetimes and const params
            GenericArgument::Type(ty) => Some(ty.clone()),
            _ => None,
        })
        .collect()
}


pub fn outer_serializable_core(input: TokenStream) -> TokenStream {
    let input = parse2::<DeriveInput>(input).unwrap();
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = &generics.split_for_impl();
    let turbofish = ty_generics.as_turbofish();
    let ty_inner_type = extract_types_from_turbofish(&turbofish)[0].clone();

    let type_name_impl = if cfg!(feature = "const_type_name") {
        quote! {
            const TYPE_NAME: &'static str = std::any::type_name::<#name>();
        }
    } else {
        quote! {
            const TYPE_NAME: &'static str = stringify!(#name);
        }
    };

    let expanded = quote! {
        impl #impl_generics OuterSerializableTrait<#ty_inner_type> for #name #ty_generics #where_clause {
    
            #type_name_impl

            fn _get_full_type_name(&self) -> String {
                let mut s = String::from(#name #turbofish :: TYPE_NAME);
                s.push_str("->");
                s.push_str(#ty_inner_type :: TYPE_NAME);
                s
            }

            fn get_full_type_name(&self) -> &'static str {
                static TYPEMAP: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<&'static str, &'static str>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::<&'static str, &'static str>::new()));
                let mut jtn = TYPEMAP.lock().unwrap();
                let e = jtn.entry(#ty_inner_type :: TYPE_NAME);
                e.or_insert_with(|| {
                    let s : &str = &self._get_full_type_name();
                    let leaked_str: &'static str = s.to_string().leak();
                    // println!("LEAKING: {} @ {:p}", leaked_str, leaked_str);
                    leaked_str
                })
            }
        }
    };
    TokenStream::from(expanded)
}

