use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{braced, parse_macro_input, Arm, DeriveInput, Ident, LitStr, Result};

#[proc_macro_derive(NamedClass)]
pub fn bff_named_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let class_name = LitStr::new(format!("{}_Z", name).as_str(), name.span());

    quote! {
        impl crate::traits::NamedClass for #name {
            const NAME: crate::names::Name = crate::names::Name::new(crate::crc32::asobo(#class_name.as_bytes()));
            const NAME_STR: &'static str = #class_name;
        }
    }
    .into()
}

struct BffClassMacroInput {
    class: Ident,
    forms: Vec<Arm>,
}

impl Parse for BffClassMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let class = input.parse()?;
        let content;
        braced!(content in input);
        let mut forms = Vec::new();
        while !content.is_empty() {
            forms.push(content.parse()?);
        }
        Ok(BffClassMacroInput { class, forms })
    }
}

#[proc_macro]
pub fn bff_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BffClassMacroInput);

    let enum_class = impl_enum_class(&input);

    let from_object_to_shadow_class = impl_from_object_to_shadow_class(&input);
    let from_shadow_class_to_object = impl_from_shadow_class_to_object(&input);

    quote! {
        #enum_class
        #from_object_to_shadow_class
        #from_shadow_class_to_object
    }
    .into()
}

fn impl_enum_class(input: &BffClassMacroInput) -> proc_macro2::TokenStream {
    let class = &input.class;

    let variants = input
        .forms
        .iter()
        .map(|form| {
            let body = &form.body;
            quote! { #body(#body) }
        })
        .collect::<Vec<_>>();

    quote! {
        #[derive(serde::Serialize, serde::Deserialize, Debug, bff_derive::NamedClass, derive_more::From, derive_more::IsVariant)]
        pub enum #class {
            #(#variants),*
        }
    }
}

fn impl_from_object_to_shadow_class(input: &BffClassMacroInput) -> proc_macro2::TokenStream {
    let class = &input.class;

    let arms = input.forms.iter().map(|form| {
        let attrs = &form.attrs;
        let pat = &form.pat;
        let guard = match &form.guard {
            Some((_, guard)) => quote! { #guard },
            None => quote! {},
        };
        let body = &form.body;
        quote! {
            #(#attrs)*
            #pat #guard => {
                let shadow_class: #body = <&crate::object::Object as crate::traits::TryIntoVersionPlatform<#body>>::try_into_version_platform(object, version, platform)?;
                Ok(shadow_class.into())
            }
        }
    }).collect::<Vec<_>>();

    quote! {
        impl crate::traits::TryFromVersionPlatform<&crate::object::Object> for #class {
            type Error = crate::error::Error;

            fn try_from_version_platform(
                object: &crate::object::Object,
                version: crate::versions::Version,
                platform: crate::platforms::Platform,
            ) -> crate::BffResult<#class> {
                use crate::versions::Version::*;
                use crate::platforms::Platform::*;
                match (version.clone(), platform) {
                    #(#arms)*
                    _ => Err(
                        crate::error::UnimplementedClassError::new(object.name(), <Self as crate::traits::NamedClass>::NAME, version, platform).into(),
                    ),
                }
            }
        }
    }
}

fn impl_from_shadow_class_to_object(input: &BffClassMacroInput) -> proc_macro2::TokenStream {
    let class = &input.class;

    let arms = input.forms.iter().map(|form| {
        let attrs = &form.attrs;
        let body = &form.body;
        quote! {
            #(#attrs)*
            #class::#body(class) => {
                let object: crate::object::Object = <&#body as crate::traits::TryIntoVersionPlatform<crate::object::Object>>::try_into_version_platform(class, version, platform)?;
                Ok(object)
            }
        }
    }).collect::<Vec<_>>();

    quote! {
        impl crate::traits::TryFromVersionPlatform<&#class> for crate::object::Object {
            type Error = crate::error::Error;

            fn try_from_version_platform(
                class: &#class,
                version: crate::versions::Version,
                platform: crate::platforms::Platform,
            ) -> crate::BffResult<crate::object::Object> {
                use crate::versions::Version::*;
                use crate::platforms::Platform::*;
                match class {
                    #(#arms)*
                }
            }
        }
    }
}
