use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Type, Path, Lifetime};

pub fn replace_lifetimes(ty: &mut Type) {
    traverse_type(ty, &mut replace_lifetime)
}

pub fn replace_lifetime(ty: &mut Type) {
    use syn::{GenericArgument, PathArguments};

    match ty {
        Type::Path(p) => {
            p.path
                .segments
                .iter_mut()
                .filter_map(|segment| match &mut segment.arguments {
                    PathArguments::AngleBracketed(ab) => Some(ab),
                    _ => None,
                })
                .flat_map(|ab| ab.args.iter_mut())
                .for_each(|arg| {
                    if let GenericArgument::Lifetime(lt) = arg {
                        *lt = Lifetime::new("'s", lt.span());
                    }
                });
        }
        Type::Reference(r) => {
            let span = match r.lifetime.take() {
                Some(lt) => lt.span(),
                None => Span::call_site(),
            };

            r.lifetime = Some(Lifetime::new("'s", span));
        }
        _ => (),
    }
}

pub fn traverse_type(ty: &mut Type, f: &mut impl FnMut(&mut Type)) {
    f(ty);
    match ty {
        Type::Array(array) => traverse_type(&mut array.elem, f),
        Type::BareFn(bare_fn) => {
            for input in &mut bare_fn.inputs {
                traverse_type(&mut input.ty, f);
            }
            if let syn::ReturnType::Type(_, ty) = &mut bare_fn.output {
                traverse_type(ty, f);
            }
        }
        Type::Group(group) => traverse_type(&mut group.elem, f),
        Type::Paren(paren) => traverse_type(&mut paren.elem, f),
        Type::Path(path) => traverse_path(&mut path.path, f),
        Type::Ptr(p) => traverse_type(&mut p.elem, f),
        Type::Reference(r) => traverse_type(&mut r.elem, f),
        Type::Slice(slice) => traverse_type(&mut slice.elem, f),
        Type::TraitObject(object) => object.bounds.iter_mut().for_each(|bound| {
            if let syn::TypeParamBound::Trait(trait_bound) = bound {
                traverse_path(&mut trait_bound.path, f);
            }
        }),
        Type::Tuple(tuple) => tuple
            .elems
            .iter_mut()
            .for_each(|elem| traverse_type(elem, f)),
        _ => (),
    }
}

fn traverse_path(path: &mut Path, f: &mut impl FnMut(&mut Type)) {
    for segment in &mut path.segments {
        match &mut segment.arguments {
            syn::PathArguments::None => (),
            syn::PathArguments::AngleBracketed(args) => {
                for arg in &mut args.args {
                    match arg {
                        syn::GenericArgument::Type(ty) => {
                            traverse_type(ty, f);
                        }
                        syn::GenericArgument::Binding(bind) => {
                            traverse_type(&mut bind.ty, f);
                        }
                        _ => (),
                    }
                }
            }
            syn::PathArguments::Parenthesized(args) => {
                for arg in &mut args.inputs {
                    traverse_type(arg, f);
                }
                if let syn::ReturnType::Type(_, ty) = &mut args.output {
                    traverse_type(ty, f);
                }
            }
        }
    }
}
