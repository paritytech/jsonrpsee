//! Contains implementation of the `syn::parse::Parse` trait. Allows parsing the input tokens
//! stream in a structured way.

/// Multiple `ApiDefinition`s grouped into one struct.
///
/// Represents the entire content of the procedural macro.
#[derive(Debug)]
pub struct ApiDefinitions {
    pub apis: Vec<ApiDefinition>,
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

/// A single API defined by the user.
#[derive(Debug)]
pub struct ApiDefinition {
    /// Visibility of the definition (e.g. `pub`, `pub(crate)`, ...).
    pub visibility: syn::Visibility,
    /// Name of the API. For example `System`.
    pub name: syn::Ident,
    /// List of RPC functions defined for this API.
    pub definitions: Vec<syn::Signature>,
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

/// Implementation detail of `ApiDefinition`.
/// Parses one single block of function definitions.
#[derive(Debug)]
struct ApiMethods {
    definitions: Vec<syn::Signature>,
}

impl syn::parse::Parse for ApiMethods {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut out = ApiMethods {
            definitions: Vec::new(),
        };

        while !input.is_empty() {
            let def: syn::TraitItemMethod = input.parse()?;
            if def.default.is_some() {
                panic!(
                    "It is forbidden to provide a default implementation for methods in the \
                     API definition: {:?}",
                    def
                );
            }

            // TODO: do something with the function attributes?

            out.definitions.push(def.sig);
        }

        Ok(out)
    }
}
