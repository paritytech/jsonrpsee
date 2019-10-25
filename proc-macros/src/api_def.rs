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
    /// Attributes on the method.
    pub attributes: ApiMethodAttrs,
}

/// List of attributes applied to a method.
#[derive(Debug, Default)]
pub struct ApiMethodAttrs {
    /// Name of the RPC method, if specified.
    pub method: Option<String>,
}

impl ApiMethod {
    /// Returns true if this method has a `()` return type.
    ///
    /// This is used to determine whether this should be a notification or a method call.
    pub fn is_void_ret_type(&self) -> bool {
        let ret_ty = match &self.signature.output {
            syn::ReturnType::Default => return true,
            syn::ReturnType::Type(_, ty) => ty,
        };

        let tuple_ret_ty = match &**ret_ty {
            syn::Type::Tuple(tuple) => tuple,
            _ => return false,
        };

        tuple_ret_ty.elems.is_empty()
    }
}

/// Implementation detail of `ApiDefinition`.
/// Parses one single block of function definitions.
#[derive(Debug)]
struct ApiMethods {
    definitions: Vec<ApiMethod>,
}

/// Implementation detail of `ApiMethodAttrs`.
/// Parses a single attribute.
enum ApiMethodAttr {
    Method(proc_macro2::Literal),
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

        let mut attributes = ApiMethodAttrs::default();
        for attribute in &item.attrs {
            if attribute.path.is_ident("rpc") {
                let attrs = attribute.parse_args()?;
                attributes.try_merge(attrs)?;
            } else {
                // TODO: do we copy the attributes somewhere in the output?
            }
        }

        Ok(ApiMethod {
            signature: item.sig,
            attributes,
        })
    }
}

impl ApiMethodAttrs {
    /// Tries to merge another `ApiMethodAttrs` within this one. Returns an error if there is an
    /// overlap in the attributes.
    // TODO: span
    fn try_merge(&mut self, other: ApiMethodAttrs) -> syn::parse::Result<()> {
        if let Some(method) = other.method {
            if self.method.is_some() {
                // TODO: return Err(())
            }
            self.method = Some(method);
        }

        Ok(())
    }
}

impl syn::parse::Parse for ApiMethodAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut out = ApiMethodAttrs::default();

        let list = input
            .parse_terminated::<_, syn::token::Comma>(|input| input.parse::<ApiMethodAttr>())?;
        for attr in list {
            match attr {
                ApiMethodAttr::Method(method) => {
                    if out.method.is_some() {
                        return Err(syn::Error::new(
                            method.span(),
                            "Duplicate method attribute found",
                        ));
                    }
                    out.method = Some(method.to_string());
                }
            }
        }
        Ok(out)
    }
}

impl syn::parse::Parse for ApiMethodAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let attr: syn::Ident = input.parse()?;

        if attr == "method" {
            let _: syn::token::Eq = input.parse()?;
            let val = input.parse()?;
            Ok(ApiMethodAttr::Method(val))
        } else {
            Err(syn::Error::new(
                attr.span(),
                &format!("Unknown attribute: {}", attr.to_string()),
            ))
        }
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
