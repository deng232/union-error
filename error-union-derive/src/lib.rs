use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ErrorUnion)]
pub fn derive_error_union(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = input.ident;

    let data = match input.data {
        Data::Enum(e) => e,
        _ => {
            return syn::Error::new_spanned(enum_name, "ErrorUnion only supports enums")
                .to_compile_error()
                .into();
        }
    };

    let mut from_impls = Vec::new();
    let mut display_arms = Vec::new();
    let mut source_arms = Vec::new();

    for variant in data.variants {
        let variant_name = variant.ident;

        let field_ty = match variant.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => f.unnamed.first().unwrap().ty.clone(),
            _ => {
                return syn::Error::new_spanned(
                    variant_name,
                    "Each variant must have exactly one unnamed field",
                )
                .to_compile_error()
                .into();
            }
        };

        // Expect Located<T>
        // Extract inner T if possible
        let inner_ty = extract_inner_type(&field_ty).unwrap_or(field_ty.clone());

        from_impls.push(quote! {
            impl From<#inner_ty> for #enum_name {
                #[track_caller]
                fn from(source: #inner_ty) -> Self {
                    Self::#variant_name(error_union::Located::new(source))
                }
            }
        });

        display_arms.push(quote! {
            Self::#variant_name(inner) => std::fmt::Display::fmt(inner, f),
        });

        source_arms.push(quote! {
            Self::#variant_name(inner) => Some(inner as &(dyn std::error::Error + 'static)),
        });
    }

    let expanded = quote! {
        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms)*
                }
            }
        }

        impl std::error::Error for #enum_name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    #(#source_arms)*
                }
            }
        }

        #(#from_impls)*
    };

    expanded.into()
}

/// Extract T from Located<T>
fn extract_inner_type(ty: &syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        let seg = type_path.path.segments.last()?;
        if seg.ident == "Located" {
            if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                    return Some(inner.clone());
                }
            }
        }
    }
    None
}
