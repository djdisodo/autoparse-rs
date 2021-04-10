use proc_macro2::{Ident, Span};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DataEnum, DataStruct, Fields, Generics, ImplItem, ItemImpl, Path, Token, punctuated::Punctuated, token::{Brace, Comma}};



fn generate_code_for_fields<'a>(fields: &'a Fields, self_accesss: TokenStream2) -> (TokenStream2, TokenStream2, Vec<&'a syn::Type>) {
	let mut types = vec![];
	match fields {
		Fields::Named(fields_named) => {
			let mut fields: Punctuated<&Ident, Comma> = Punctuated::new();
			let mut token_stream_write = TokenStream2::new();
			for field in fields_named.named.iter() {
				types.push(&field.ty);
				let field_name = field.ident.as_ref().unwrap();
				fields.push(field_name);
				token_stream_write.extend_one(quote! {
					autoparse::Writable::write(#self_accesss #field_name, buffer);
				});
			}
			return (token_stream_write, quote! { { #fields } }, types);
		},
		Fields::Unnamed(fields_unnamed) => {
			let mut fields: Punctuated<Ident, Comma> = Punctuated::new();
			let mut token_stream_write = TokenStream2::new();
			let mut field_count = 0;

			for field in fields_unnamed.unnamed.iter() {
				types.push(&field.ty);
				let _field_name = Ident::new(&format!("_{}", field_count), Span::call_site());
				fields.push(_field_name);
				let field_name = syn::Index::from(field_count);
				
				token_stream_write.extend_one(quote! {
					autoparse::Writable::write(#self_accesss #field_name, buffer);
				});
				field_count += 1;
			}
			return (token_stream_write, quote! { (#fields,) }, types);

		},
		Fields::Unit => return (quote! {}, quote! {}, types)
	};

}

pub fn derive_writable_for_struct(ident: &Ident, generics: &Generics, data_struct: &DataStruct, autoparse_for: &Path) -> TokenStream2 {
	
	let (token_stream_write, _, /* field_types */_) = generate_code_for_fields(&data_struct.fields, quote! { &self. });

	let items = quote! {
		fn write(&self, buffer: &mut Vec<#autoparse_for>) {
			#token_stream_write
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
		trait_: Some((None, syn::parse2(quote! { autoparse::Writable<#autoparse_for> }).unwrap(), Token![for](span))),
		self_ty: Box::new(syn::parse2(quote! { #ident<#type_args> }).unwrap()),
		brace_token: Brace { span },
		items: vec![ImplItem::Verbatim(items)] 
	};
	quote! {
		#implement
	}
}


pub fn derive_writable_for_enum(ident: &Ident, generics: &Generics, data_enum: &DataEnum, autoparse_for: &Path) -> TokenStream2 {
	let mut token_stream_write_match: Punctuated<TokenStream2, Comma> = Punctuated::new();

	/* let mut field_types = vec![]; */

	for variant in data_enum.variants.iter() {
		let variant_ident = &variant.ident;
		let self_access = if let Fields::Unnamed(_) = variant.fields {
			quote! { self_a. }
		} else {
			quote! {}
		};

		let (token_stream_write_variant, field_bind, /* field_types_variant */_) = generate_code_for_fields(
			&variant.fields,
			self_access,
		);

		/* field_types.extend(field_types_variant); */
		

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
		fn write(&self, buffer: &mut Vec<#autoparse_for>) {
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
		trait_: Some((None, syn::parse2(quote! { autoparse::Writable<#autoparse_for> }).unwrap(), Token![for](span))),
		self_ty: Box::new(syn::parse2(quote! { #ident<#type_args> }).unwrap()),
		brace_token: Brace { span },
		items: vec![ImplItem::Verbatim(items)] 
	};
	quote! {
		#implement
	}
}
