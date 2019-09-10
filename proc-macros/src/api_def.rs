

#[derive(Debug)]
pub struct ApiDefinitions {
    pub apis: Vec<ApiDefinition>,
}

impl syn::parse::Parse for ApiDefinitions {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut out = ApiDefinitions {
            apis: Vec::new(),
        };

        while !input.is_empty() {
            out.apis.push(input.parse()?);
        }

        Ok(out)
    }
}


#[derive(Debug)]
pub struct ApiDefinition {
    pub name: syn::Ident,
    pub definitions: Vec<syn::Signature>,
}

impl syn::parse::Parse for ApiDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let name = input.parse()?;
        let group: proc_macro2::Group = input.parse()?;
        assert_eq!(group.delimiter(), proc_macro2::Delimiter::Brace);
        let defs: ApiMethods = syn::parse2(group.stream())?;

        Ok(ApiDefinition {
            name,
            definitions: defs.definitions,
        })
    }
}

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
                panic!("It is forbidden to provide a default implementation for methods in the \
                        API definition: {:?}", def);
            }

            // TODO: do something with the function attributes?

            out.definitions.push(def.sig);
        }

        Ok(out)
    }
}
