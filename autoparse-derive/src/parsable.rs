use proc_macro2::{Ident, Span};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DataEnum, DataStruct, Fields, Generics, ImplItem, ItemImpl, Path, Token, punctuated::Punctuated, token::{Brace, Comma}};


fn generate_code_for_fields<'a>(fields: &'a Fields, self_constructor: TokenStream2, autoparse_for: &Path) -> (TokenStream2, Vec<&'a syn::Type>) {
	let mut types = vec![];
	match fields {
		Fields::Named(fields_named) => {
			let mut token_stream_parse = TokenStream2::new();
			let mut fields: Punctuated<&Ident, Comma> = Punctuated::new();
			for field in fields_named.named.iter() {
				types.push(&field.ty);
				let field_name = field.ident.as_ref().unwrap();
				let field_type = &field.ty;
				token_stream_parse.extend_one(quote! {
					let (#field_name, r): (#field_type, _) = autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
					read += r;
				});
				fields.push(field_name);
			}
			return (quote! {
				let mut read = 0;
				#token_stream_parse
				Ok((
					#self_constructor {
						#fields
					},
					read
				))
			}, types);
		},
		Fields::Unnamed(fields_unnamed) => {
			let mut token_stream_parse = TokenStream2::new();
			let mut fields: Punctuated<Ident, Comma> = Punctuated::new();
			let mut field_count = 0;

			for field in fields_unnamed.unnamed.iter() {
				let field_name = Ident::new(&format!("_{}", field_count), Span::call_site());
				let field_type = &field.ty;
				token_stream_parse.extend_one(quote! {
					let (#field_name, r): (#field_type, _) = autoparse::Parsable::try_parse_no_rewind(stream, position + read)?;
					read += r;
				});
				fields.push(field_name);
				field_count += 1;
			}
			return (quote! {
				let mut read = 0;
				#token_stream_parse
				Ok((
					#self_constructor (
						#fields
					),
					read
				))	
			}, types);

		},
		Fields::Unit => return (quote! { let result: Result<_, autoparse::ParseError<#autoparse_for>> = Ok((#self_constructor, 0)); result }, types)
	};

}

pub fn derive_parsable_for_struct(ident: &Ident, generics: &Generics, data_struct: &DataStruct, autoparse_for: &Path) -> TokenStream2 {
	
	let (token_stream_parse, /* field_type */_ ) = generate_code_for_fields(&data_struct.fields, quote! { Self }, autoparse_for);

	let items = quote! {
		fn try_parse_no_rewind(stream: &mut autoparse::ParseStream<#autoparse_for, impl Iterator<Item=#autoparse_for>>, position: usize) -> Result<(Self, usize), autoparse::ParseError<#autoparse_for>> {
			#token_stream_parse
		}
	};

	let span = Span::call_site();
	
	let mut type_args: Punctuated<_, Comma> = Punctuated::new();
	for param in generics.type_params() {
		type_args.push(param.ident.clone());
	}

	let generics = generics.clone();
	/*
	for field_type in field_types {
		generics.params.push(syn::GenericParam::Type(syn::parse2(quote! { #field_type: autoparse::Parsable }).unwrap()));
	}
	*/

	let implement = ItemImpl {
		attrs: vec![],
		defaultness: None,
		unsafety: None,
		impl_token: Token![impl](span),
		generics,
		trait_: Some((None, syn::parse2(quote! { autoparse::Parsable<#autoparse_for> }).unwrap(), Token![for](span))),
		self_ty: Box::new(syn::parse2(quote! { #ident<#type_args> }).unwrap()),
		brace_token: Brace { span },
		items: vec![ImplItem::Verbatim(items)] 
	};
	quote! {
		#implement
	}
}


pub fn derive_parsable_for_enum(ident: &Ident, generics: &Generics, data_enum: &DataEnum, autoparse_for: &Path) -> TokenStream2 {
	let mut token_stream_parse = quote! {
		let mut error: autoparse::ParseError<#autoparse_for> = Default::default();
		stream.set_rewind_point();
	};

	/* let mut field_types = vec![]; */

	for variant in data_enum.variants.iter() {
		let variant_ident = &variant.ident;

		let (token_stream_parse_variant, /* field_types_variant */_) = generate_code_for_fields(
			&variant.fields,
			quote! { #ident :: #variant_ident },
			autoparse_for
		);
		/* field_types.extend(field_types_variant); */

		token_stream_parse.extend_one(quote! {
			match {
				let mut a = stream.fork();
				(| stream | { #token_stream_parse_variant })(&mut a)
			} {
				Ok((parsed, read)) => return Ok((parsed, read)),
				Err(e) => {
					let e: autoparse::ParseError<#autoparse_for> = e;
					(*error).extend(e.expections);
					stream.rewind();
				}
			}
		});
	}
	let items = quote! {
		fn try_parse_no_rewind(mut stream: &mut autoparse::ParseStream<#autoparse_for, impl Iterator<Item=#autoparse_for>>, position: usize) -> Result<(Self, usize), autoparse::ParseError<#autoparse_for>> {
			#token_stream_parse
			Err(error)
		}
	};

	let span = Span::call_site();
	
	let mut type_args: Punctuated<_, Comma> = Punctuated::new();
	for param in generics.type_params() {
		type_args.push(param.ident.clone());
	}

	let generics = generics.clone();
	/*
	for field_type in field_types {
		generics.params.push(syn::GenericParam::Type(syn::parse2(quote! { #field_type: autoparse::Parsable }).unwrap()));
	}
	*/

	let implement = ItemImpl {
		attrs: vec![],
		defaultness: None,
		unsafety: None,
		impl_token: Token![impl](span),
		generics,
		trait_: Some((None, syn::parse2(quote! { autoparse::Parsable<#autoparse_for> }).unwrap(), Token![for](span))),
		self_ty: Box::new(syn::parse2(quote! { #ident<#type_args> }).unwrap()),
		brace_token: Brace { span },
		items: vec![ImplItem::Verbatim(items)] 
	};
	quote! {
		#implement
	}
}
