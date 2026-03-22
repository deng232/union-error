use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ErrorUnion)]
pub fn derive_union_error(input: TokenStream) -> TokenStream {
    // Parse the incoming Rust item as a `syn::DeriveInput`.
    //
    // This is the standard entry type for derive macros and contains:
    // - the item name
    // - visibility
    // - attributes
    // - generics
    // - and the actual item data (enum / struct / union)
    let input = parse_macro_input!(input as DeriveInput);

    // Save the enum name, e.g. `AppError`.
    let enum_name = input.ident;

    // We only support enums.
    //
    // If the user writes `#[derive(ErrorUnion)]` on a struct or union,
    // emit a compile error.
    let data = match input.data {
        Data::Enum(e) => e,
        _ => {
            return syn::Error::new_spanned(enum_name, "ErrorUnion only supports enums")
                .to_compile_error()
                .into();
        }
    };

    // These hold the generated pieces that will later be assembled into:
    //
    // - `impl From<T> for Enum`
    // - `impl Display for Enum`
    // - `impl Error for Enum`
    let mut from_impls = Vec::new();
    let mut display_arms = Vec::new();
    let mut source_arms = Vec::new();

    // Process each enum variant independently.
    for variant in data.variants {
        let variant_name = variant.ident;

        // Require tuple variants with exactly one field:
        //
        // Good:
        //   Parse(Located<ParseIntError>)
        //
        // Rejected:
        //   Parse
        //   Parse { source: ... }
        //   Parse(A, B)
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

        // Try to extract the inner type from `Located<T>`.
        //
        // Example:
        //   field_ty = Located<std::io::Error>
        //   inner_ty = std::io::Error
        //
        // If the field is not `Located<T>`, we fall back to using the field type
        // directly. That keeps the macro behavior somewhat flexible, though the
        // intended design is to use `Located<T>` for all variants.
        let inner_ty = extract_inner_type(&field_ty).unwrap_or(field_ty.clone());

        // Generate:
        //
        // impl From<T> for Enum {
        //     #[track_caller]
        //     fn from(source: T) -> Self {
        //         Self::Variant(union_error::Located::new(source))
        //     }
        // }
        //
        // This is the critical conversion used by `?`.
        from_impls.push(quote! {
            impl From<#inner_ty> for #enum_name {
                #[track_caller]
                fn from(source: #inner_ty) -> Self {
                    Self::#variant_name(union_error::Located::new(source))
                }
            }
        });

        // Generate a `Display` match arm that delegates to the stored inner value.
        //
        // Example:
        //   Self::Parse(inner) => Display::fmt(inner, f)
        //
        // Since `Located<T>` implements `Display`, this prints both:
        // - the inner source error message
        // - the stored source location
        display_arms.push(quote! {
            Self::#variant_name(inner) => std::fmt::Display::fmt(inner, f),
        });

        // Generate an `Error::source()` match arm.
        //
        // Example:
        //   Self::Parse(inner) => Some(inner as &(dyn Error + 'static))
        //
        // Since `Located<T>` also implements `Error`, the chain becomes:
        //   AppError -> Located<T> -> T
        source_arms.push(quote! {
            Self::#variant_name(inner) => Some(inner as &(dyn std::error::Error + 'static)),
        });
    }

    // Assemble the final generated impls.
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

/// Extract the inner type `T` from `Located<T>`.
///
/// # Example
///
/// Input:
///
/// ```ignore
/// Located<std::io::Error>
/// ```
///
/// Output:
///
/// ```ignore
/// Some(std::io::Error)
/// ```
///
/// If the input is not a path type ending in `Located<...>`, this returns `None`.
///
/// # Why this helper exists
///
/// The derive macro wants to generate:
///
/// ```ignore
/// impl From<T> for AppError
/// ```
///
/// not:
///
/// ```ignore
/// impl From<Located<T>> for AppError
/// ```
///
/// because the `?` operator naturally converts from the original leaf error type.
fn extract_inner_type(ty: &syn::Type) -> Option<syn::Type> {
    // We only care about path types like:
    // - Located<T>
    // - union_error::Located<T>
    if let syn::Type::Path(type_path) = ty {
        // Look at the last path segment.
        //
        // Examples:
        // - Located<T>                  -> last segment = Located
        // - union_error::Located<T>     -> last segment = Located
        let seg = type_path.path.segments.last()?;

        // Only match types whose last path segment is literally `Located`.
        if seg.ident == "Located" {
            // Require angle-bracket generic arguments: `Located<T>`
            if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                // Take the first generic argument if it is a type.
                if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                    return Some(inner.clone());
                }
            }
        }
    }

    None
}
