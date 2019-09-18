//! Contains implementation of the `syn::parse::Parse` trait. Allows parsing the input tokens
//! stream in a structured way.

use syn::spanned::Spanned as _;

/// Multiple `ApiDefinition`s grouped into one struct.
///
/// Represents the entire content of the procedural macro.
#[derive(Debug)]
pub struct ApiDefinitions {
    pub apis: Vec<ApiDefinition>,
}

/// A single API defined by the user.
#[derive(Debug)]
pub struct ApiDefinition {
    /// Visibility of the definition (e.g. `pub`, `pub(crate)`, ...).
    pub visibility: syn::Visibility,
    /// Name of the API. For example `System`.
    pub name: syn::Ident,
    /// List of RPC functions defined for this API.
    pub definitions: Vec<ApiMethod>,
}

/// A single JSON-RPC method definition.
#[derive(Debug)]
pub struct ApiMethod {
    /// Signature of the method.
    pub signature: syn::Signature,
}

/// Implementation detail of `ApiDefinition`.
/// Parses one single block of function definitions.
#[derive(Debug)]
struct ApiMethods {
    definitions: Vec<ApiMethod>,
}

impl syn::parse::Parse for ApiDefinitions {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut out = ApiDefinitions { apis: Vec::new() };

        while !input.is_empty() {
            out.apis.push(input.parse()?);
        }

        Ok(out)
    }
}

impl syn::parse::Parse for ApiDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let visibility = input.parse()?;
        let name = input.parse()?;
        let group: proc_macro2::Group = input.parse()?;
        assert_eq!(group.delimiter(), proc_macro2::Delimiter::Brace);
        let defs: ApiMethods = syn::parse2(group.stream())?;

        Ok(ApiDefinition {
            visibility,
            name,
            definitions: defs.definitions,
        })
    }
}

impl syn::parse::Parse for ApiMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let item: syn::TraitItemMethod = input.parse()?;
        if item.default.is_some() {
            return Err(syn::Error::new(item.default.span(),
                "It is forbidden to provide a default implementation for methods in the API definition"));
        }
        Ok(ApiMethod {
            signature: item.sig,
        })
    }
}

impl syn::parse::Parse for ApiMethods {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut out = ApiMethods {
            definitions: Vec::new(),
        };

        while !input.is_empty() {
            let method: ApiMethod = input.parse()?;
            out.definitions.push(method);
        }

        Ok(out)
    }
}
