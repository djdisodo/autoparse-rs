#![feature(extend_one)]
use proc_macro2::{Ident, Span};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Generics, ImplItem, ItemImpl, Meta, NestedMeta, Path, Token, punctuated::Punctuated, token::{Brace, Comma}};

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
                    output.extend_one(match &input.data {
                        Data::Struct(data_struct) =>
                            derive_parsable_for_struct(&input.ident, &input.generics, data_struct, &path),
                        Data::Enum(data_enum) =>
                            derive_parsable_for_enum(&input.ident, &input.generics, data_enum, &path),
                        Data::Union(_) => panic!("can't derive Parsable for Union")
                    })
                }
            }
        }
    }
    //std::fs::write("/home/sodo/debug.rs", output.to_string()).unwrap();
    output.into()
}

fn generate_code_for_fields<'a>(fields: &'a Fields, self_constructor: TokenStream2, self_accesss: TokenStream2, parse: &Path) -> (TokenStream2, TokenStream2, Vec<&'a syn::Type>, TokenStream2) {
    let mut types = vec![];
    match fields {
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
                    crate::autoparse::Parsable::write(#self_accesss #field_name, buffer);
                });
            }
            return (quote! {
                let read = &mut 0;
                (|| {
                    #token_stream_parse
                    Ok((
                        #self_constructor {
                            #fields
                        },
                        *read
                    ))
                })().map_err(| e: crate::autoparse::ParseError<#parse> | e.advance(*read))
            }, token_stream_write, types, quote! { { #fields } });
        },
        Fields::Unnamed(fields_unnamed) => {
            let mut token_stream_parse = TokenStream2::new();
            let mut fields: Punctuated<Ident, Comma> = Punctuated::new();
            let mut token_stream_write = TokenStream2::new();
            let mut field_count = 0;

            for field in fields_unnamed.unnamed.iter() {
                types.push(&field.ty);
                let _field_name = Ident::new(&format!("_{}", field_count), Span::call_site());
                let field_type = &field.ty;
                token_stream_parse.extend_one(quote! {
                    let (#_field_name, r): (#field_type, _) = crate::autoparse::Parsable::try_parse(buffer)?;
                    *read += r;
                });
                fields.push(_field_name);
                let field_name = syn::Index::from(field_count);
                
                token_stream_write.extend_one(quote! {
                    crate::autoparse::Parsable::write(#self_accesss #field_name, buffer);
                });
                field_count += 1;
            }
            return (quote! {
                let read = &mut 0;
                (|| {
                    #token_stream_parse
                    Ok((
                        #self_constructor (
                            #fields
                        ),
                        *read
                    ))
                })().map_err(| e: crate::autoparse::ParseError<#parse> | e.advance(*read))
            }, token_stream_write, types, quote! { (#fields,) });

        },
        Fields::Unit => return (quote! { let result: Result<_, crate::autoparse::ParseError<#parse>> = Ok((#self_constructor, 0)); result }, quote! {}, vec![], quote! {})
    };

}

fn derive_parsable_for_struct(ident: &Ident, generics: &Generics, data_struct: &DataStruct, parse: &Path) -> TokenStream2 {
    
    let (token_stream_parse, token_stream_write, _, _) = generate_code_for_fields(&data_struct.fields, quote! { Self }, quote! { &self. }, parse);

    let items = quote! {
        fn try_parse(buffer: &mut &[#parse]) -> Result<(Self, usize), crate::autoparse::ParseError<#parse>> {
            #token_stream_parse
        }

        fn write(&self, buffer: &mut Vec<#parse>) {
            #token_stream_write
        }
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


fn derive_parsable_for_enum(ident: &Ident, generics: &Generics, data_enum: &DataEnum, parse: &Path) -> TokenStream2 {
    let mut token_stream_parse = quote! {
        let mut error: crate::autoparse::ParseError<#parse> = Default::default();
    };
    let mut token_stream_write_match: Punctuated<TokenStream2, Comma> = Punctuated::new();

    for variant in data_enum.variants.iter() {
        let variant_ident = &variant.ident;
        let self_access = if let Fields::Unnamed(_) = variant.fields {
            quote! { self_a. }
        } else {
            quote! {}
        };

        let (token_stream_parse_variant, token_stream_write_variant, _, field_bind) = generate_code_for_fields(
            &variant.fields,
            quote! { #ident :: #variant_ident },
            self_access,
            parse
        );
        
        token_stream_parse.extend_one(quote! {
            match {
                #token_stream_parse_variant
            } {
                Ok((parsed, read)) => return Ok((parsed, read)),
                Err(e) => (*error).extend(e.expections)
            }
        });

        let rebind = if let Fields::Unnamed(_) = variant.fields {
            quote! {
                let self_a = #field_bind;
            }
        } else {
            quote! {}
        };

        token_stream_write_match.push(quote! {
            #ident :: #variant_ident #field_bind => {
                #rebind
                #token_stream_write_variant
            }
        });
    }
    let items = quote! {
        fn try_parse(buffer: &mut &[#parse]) -> Result<(Self, usize), crate::autoparse::ParseError<#parse>> {
            #token_stream_parse
            Err(error)
        }

        fn write(&self, buffer: &mut Vec<#parse>) {
            match self {
                #token_stream_write_match
            }
        }
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
