#![feature(extend_one)]

extern crate autoparse as autoparse;

mod parsable;
mod writable;
mod token_set;

use parsable::*;
use writable::*;

use proc_macro2::TokenStream as TokenStream2;
use syn::{Data, DeriveInput, Meta, NestedMeta};



#[proc_macro_derive(Parsable, attributes(autoparse_for))]
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
			if name.ident.to_string() != "autoparse_for" {
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

#[proc_macro_derive(Writable, attributes(autoparse_for))]
pub fn derive_writable(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse::<DeriveInput>(item).expect("`Writable` macro can only be used for struct or enum");
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
			if name.ident.to_string() != "autoparse_for" {
				continue;
			}

			for nested in list.nested.iter() {
				if let NestedMeta::Meta(Meta::Path(path)) = nested {
					output.extend_one(match &input.data {
						Data::Struct(data_struct) =>
							derive_writable_for_struct(&input.ident, &input.generics, data_struct, &path),
						Data::Enum(data_enum) =>
							derive_writable_for_enum(&input.ident, &input.generics, data_enum, &path),
						Data::Union(_) => panic!("can't derive Writable for Union")
					})
				}
			}
		}
	}
	output.into()
}

#[proc_macro_derive(TokenSet, attributes(autoparse_for))]
pub fn derive_token_set(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse::<DeriveInput>(item.clone()).expect("`TokenSet` macro can only be used for enum");
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
			if name.ident.to_string() != "autoparse_for" {
				continue;
			}

			for _nested in list.nested.iter() {
				output.extend_one(token_set::derive_token_set(item.clone().into()))
			}
		}
	}
	//std::fs::write("/home/sodo/debug.rs", output.to_string()).unwrap();
	output.into()
}
