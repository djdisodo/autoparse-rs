#![feature(extend_one)]
use proc_macro2::{Ident, Span};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Generics, ImplItem, ItemImpl, Meta, NestedMeta, Path, Token, punctuated::Punctuated, token::{Brace, Comma}};

extern crate syn;

#[proc_macro_derive(Parsable, attributes(parse))]
pub fn derive_parsable(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<DeriveInput>(item).expect("`Parsable` macro can only be used for struct or enum");
    let mut output = TokenStream2::new();
    for attr in input.attrs.iter() {
        let meta = attr.parse_meta();
        if meta.is_err() {
            continue;
        }
        let meta = meta.unwrap();
        if let Meta::List(list) = meta {
            let name = if let Some(last) = attr.path.segments.last() {
                last
            } else {
                continue
            };
            if name.ident.to_string() != "parse" {
                continue;
            }

            for nested in list.nested.iter() {
                if let NestedMeta::Meta(Meta::Path(path)) = nested {
                    match input.data {
                        Data::Struct(ref data_struct) => output.extend_one(
                            derive_parsable_for_struct(&input.ident, &input.generics, data_struct, path.clone())
                        ),
                        _ => {},
                    }
                }
            }
        }
    }
    //std::fs::write("/home/sodo/debug.rs", output.to_string()).unwrap();
    output.into()
}

fn derive_parsable_for_struct(ident: &Ident, generics: &Generics, data_struct: &DataStruct, parse: Path) -> TokenStream2 {
    
    let mut types = Vec::new();
    let items = match &data_struct.fields {
        Fields::Named(fields_named) => {
            let mut token_stream_parse = TokenStream2::new();
            let mut fields: Punctuated<&Ident, Comma> = Punctuated::new();
            let mut token_stream_write = TokenStream2::new();
            for field in fields_named.named.iter() {
                types.push(&field.ty);
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                token_stream_parse.extend_one(quote! {
                    let (#field_name, r): (#field_type, _) = crate::autoparse::Parsable::try_parse(buffer)?;
                    *read += r;
                });
                fields.push(field_name);
                token_stream_write.extend_one(quote! {
                    crate::autoparse::Parsable::write(&self.#field_name, buffer);
                });
            }
            quote! {
                fn try_parse(buffer: &mut &[#parse]) -> Result<(Self, usize), crate::autoparse::ParseError<#parse>> {
                    let read = &mut 0;
                    (|| {
                        #token_stream_parse
                        Ok((
                            Self {
                                #fields
                            },
                            *read
                        ))
                    })().map_err(| e: crate::autoparse::ParseError<#parse> | e.advance(*read))
                }

                fn write(&self, buffer: &mut Vec<#parse>) {
                    #token_stream_write
                }
            }
        },
        _ => TokenStream2::new() //todo
    };
    
    
    let span = Span::call_site();
    
    let mut type_args: Punctuated<_, Comma> = Punctuated::new();
    for param in generics.type_params() {
        type_args.push(param.ident.clone());
    }
    let implement = ItemImpl {
        attrs: vec![],
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: generics.clone(),
        trait_: Some((None, syn::parse2(quote! { crate::autoparse::Parsable<#parse> }).unwrap(), Token![for](span))),
        self_ty: Box::new(syn::parse2(quote! { #ident<#type_args> }).unwrap()),
        brace_token: Brace { span },
        items: vec![ImplItem::Verbatim(items)] 
    };
    quote! {
        #implement
    }
}
