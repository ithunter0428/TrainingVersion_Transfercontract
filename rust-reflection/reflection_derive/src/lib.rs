#![recursion_limit = "128"]
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate serde_derive_internals;
extern crate syn;

use proc_macro::TokenStream;

use serde_derive_internals::attr::get_serde_meta_items;
use serde_derive_internals::Ctxt;
use syn::Meta::NameValue;
use syn::NestedMeta::Meta;
use syn::{Data, DeriveInput, Fields, GenericParam, Generics, Type};

#[proc_macro_derive(Reflection)]
pub fn derive_reflection(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let members = aggregate(&input.data);
    let schema = generate_schema(&input.data);
    let ty = generate_ty(&input.data);

    let expanded = quote! {
        impl #impl_generics ::reflection::Reflection for #name #ty_generics #where_clause {
            fn ty() -> ::reflection::Type { #ty }
            fn name() -> ::reflection::Name { Some( String::from( stringify!( #name )))}

            fn schema( id: ::reflection::Id ) -> ::reflection::Schema {
                let mut tree = {#schema};
                match &mut tree.root_mut().data_mut() {
                    ::reflection::Member::Field( ref mut field ) => {
                        field.id = id;
                        field.tyname = Self::name();
                        field.expander = Some( <#name #ty_generics as Reflection>::members );
                    },
                    ::reflection::Member::Variant( ref mut variant ) => {
                        variant.id = id;
                    },
                }
                tree
            }

            fn members() -> ::reflection::Schemas { #members }
        }
    };

    expanded.into()
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            let bound = syn::parse_str("::reflection::Reflection").unwrap();
            type_param.bounds.push(bound);
        }
    }
    generics
}

fn get_serde_meta_items2(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    let cx: Ctxt = Ctxt::new();
    match get_serde_meta_items(&cx, attr) {
        Ok(val) => {
            cx.check().unwrap();
            Some(val)
        }
        _ => None,
    }
}

fn is_skipped(attrs: &Vec<syn::Attribute>) -> bool {
    let map = attrs.iter().filter_map(get_serde_meta_items2);
    for meta_items in map {
        for meta_item in meta_items {
            match meta_item {
                // Parse `#[serde(skip_serializing)]`
                Meta(NameValue(word)) => {
                    if let syn::Lit::Str(ref lit) = *&word.lit {
                        return lit.value() == "skip_serializing";
                    } else {
                        continue;
                    };
                }
                _ => continue,
            }
        }
    }
    false
}

fn serde_rename(field: &syn::Field) -> Option<String> {
    for meta_items in field.attrs.iter().filter_map(get_serde_meta_items2) {
        for nested in meta_items {
            match nested {
                Meta(NameValue(h)) if h.path.is_ident("rename") => {
                    if let syn::Lit::Str(ref lit) = *&h.lit {
                        return Some(lit.value());
                    }
                }
                _ => continue,
            }
        }
    }
    None
}

fn generate_ty(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(_) => {
            quote!(::reflection::Type::Struct)
        }
        Data::Enum(_) => {
            quote!(::reflection::Type::Enum)
        }
        Data::Union(_) => unimplemented!(),
    }
}

fn generate_schema(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(_) => {
            quote!(::reflection::field(
                "_",
                ::reflection::Type::Struct,
                None,
                None
            ))
        }
        Data::Enum(_) => {
            quote!(::reflection::field(
                "_",
                ::reflection::Type::Enum,
                None,
                None
            ))
        }
        Data::Union(_) => unimplemented!(),
    }
}

fn aggregate(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let fnames: Vec<String> = fields
                    .named
                    .iter()
                    .filter(|f| !is_skipped(&f.attrs))
                    .map(|f| match serde_rename(f) {
                        Some(name) => name,
                        None => f.clone().ident.unwrap().to_string(),
                    })
                    .collect();

                let ftypes1 = fields.named.iter().map(|f| f.ty.clone());
                let ftypes2 = fields.named.iter().map(|f| f.ty.clone());
                let ftypes3 = fields.named.iter().map(|f| f.ty.clone());
                quote! {
                    #(
                        -( ::reflection::field(
                                #fnames,
                                <#ftypes1 as ::reflection::Reflection>::ty(),
                                <#ftypes2 as ::reflection::Reflection>::name(),
                                Some( <#ftypes3 as ::reflection::Reflection>::members )))
                    )*
                }
            }
            Fields::Unnamed(ref fields) => {
                let mut i = 0;
                let indices = fields
                    .unnamed
                    .iter()
                    .filter(|f| !is_skipped(&f.attrs))
                    .map(|f| match serde_rename(f) {
                        Some(name) => name,
                        None => {
                            i += 1;
                            return (i - 1).to_string();
                        }
                    });
                let ftypes1 = fields.unnamed.iter().map(|f| f.ty.clone());
                let ftypes2 = fields.unnamed.iter().map(|f| f.ty.clone());
                let ftypes3 = fields.unnamed.iter().map(|f| f.ty.clone());
                quote! {
                    #(
                        -( ::reflection::field(
                                #indices,
                                <#ftypes1 as ::reflection::Reflection>::ty(),
                                <#ftypes2 as ::reflection::Reflection>::name(),
                                Some( <#ftypes3 as ::reflection::Reflection>::members )))
                    )*
                }
            }
            Fields::Unit => {
                quote!(::reflection::Schemas::new())
            }
        },
        Data::Enum(ref data) => {
            let vnames = data.variants.iter().map(|v| v.ident.clone());
            let fnames: Vec<Vec<String>> = data
                .variants
                .iter()
                .map(|v| {
                    let mut i = 0;
                    v.fields
                        .clone()
                        .into_iter()
                        // .filter(|f| !is_skipped(&f.attrs))
                        .map(move |f| match serde_rename(&f) {
                            Some(name) => name,
                            None => {
                                if let Some(ident) = f.ident {
                                    ident.to_string()
                                } else {
                                    i += 1;
                                    (i - 1).to_string()
                                }
                            }
                        })
                        .collect()
                })
                .collect();

            let ftypes1 = data.variants.iter().map(|variant| {
                let map = variant.fields.iter().map(|fields| fields.ty.clone());
                let field_types: Vec<Type> = map.collect();
                field_types
            });

            let ftypes2 = ftypes1.clone();
            let ftypes3 = ftypes1.clone();

            quote! {
                #(
                    -( ::reflection::variant( stringify!( #vnames ))
                        /(
                            #(
                                -( ::reflection::field(
                                        #fnames,
                                        <#ftypes1 as ::reflection::Reflection>::ty(),
                                        <#ftypes2 as ::reflection::Reflection>::name(),
                                        Some( <#ftypes3 as ::reflection::Reflection>::members )))
                            )*
                        )
                    )
                )*
            }
        }
        Data::Union(_) => unimplemented!(),
    }
}
