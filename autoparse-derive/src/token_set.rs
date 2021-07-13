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
					output.extend_one(quote! {
						fn try_parse(parse_stream: &mut autoparse::ParseStream) -> Result<Self, autoparse::ParseError> {
							return if let #ident::#variant_ident(value) = parse_stream.next() {
								value
							} else {
								Err(
									ParseError::new
								)
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
