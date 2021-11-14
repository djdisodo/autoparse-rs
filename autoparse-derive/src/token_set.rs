use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput, Fields};
use quote::quote;

pub fn derive_token_set(input: TokenStream2) -> TokenStream2 {
	let derive_input: DeriveInput = syn::parse2(input).expect("TokenSet macro can only be used on enum");
	let ident = derive_input.ident;
	match derive_input.data {
		syn::Data::Enum(data_enum) => {
			let mut output = TokenStream2::new();
			for variant in data_enum.variants {
				if let Fields::Unnamed(fields_unnamed) = variant.fields {
					let mut fields = fields_unnamed.unnamed.iter();
					let _field = fields.next().expect("enum deriving TokenSet cannot have variant with no field");
					let variant_ident = variant.ident;
					let field_ty = &_field.ty;
					let expected_dbgmsg = format!("{}::{}", ident.to_string(), variant_ident.to_string());
					let expected = syn::LitStr::new(&expected_dbgmsg, Span::call_site());
					output.extend_one(quote! {
						impl Parsable<#ident> for #field_ty {
							fn try_parse_no_rewind<'a>(parse_stream: &mut autoparse::ParseStream<'a, #ident, impl ParseStreamReference<#ident> + ?Sized + 'a>, position: usize) -> Result<(Self, usize), autoparse::ParseError<#ident>> {
								return if let Some(#ident::#variant_ident(value)) = parse_stream.next() {
									Ok((value, 1))
								} else {
									Err(
										ParseError::new(vec![autoparse::ExpectedValue::String(#expected.to_string())], position)
									)
								}
							}
						}

						impl Writable<#ident> for #field_ty {
							fn write(&self, stream: &mut Vec<#ident>) {
								stream.push(#ident::#variant_ident(self.clone()));
							}
						}
					});
				} else {
					panic!("TokenSet macro can only be used with enum with unnamed fields");
				}
			}
			output
		},
		_ => panic!("TokenSet macro can only be used with enum")	
	}
}
