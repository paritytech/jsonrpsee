use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Lifetime, Path, Type};

static LIFETIME: &str = "'a";

pub fn replace_lifetimes(ty: &mut Type) -> bool {
	let mut replaced = false;
	traverse_type(ty, &mut replaced, replace_lifetime);
	replaced
}

pub fn replace_lifetime(ty: &mut Type, replaced: &mut bool) {
	use syn::{GenericArgument, PathArguments};

	match ty {
		Type::Path(p) => {
			for segment in p.path.segments.iter_mut() {
				if let PathArguments::AngleBracketed(ref mut ab) = segment.arguments {
					let mut owned = true;

					for arg in ab.args.iter_mut() {
						match arg {
							GenericArgument::Lifetime(lt) => {
								*lt = Lifetime::new(LIFETIME, lt.span());
								*replaced = true;
								owned = false;
							}
							// Stop iterating on first non-lifetime generic argument
							_ => break,
						}
					}

					// Check if the type is a `Cow<_>` with no lifetimes,
					// if so transform it into `Cow<'a, _>`.
					if owned && segment.ident == "Cow" {
						let span = Span::call_site();
						let lt = Lifetime::new(LIFETIME, span);
						ab.args.insert(0, GenericArgument::Lifetime(lt));

						*replaced = true;
					}
				}
			}
		}
		Type::Reference(r) => {
			let span = match r.lifetime.take() {
				Some(lt) => lt.span(),
				None => Span::call_site(),
			};

			r.lifetime = Some(Lifetime::new(LIFETIME, span));
			*replaced = true;
		}
		_ => (),
	}
}

pub fn traverse_type(ty: &mut Type, replaced: &mut bool, f: fn(&mut Type, &mut bool)) {
	f(ty, replaced);

	match ty {
		Type::Array(array) => traverse_type(&mut array.elem, replaced, f),
		Type::BareFn(bare_fn) => {
			for input in &mut bare_fn.inputs {
				traverse_type(&mut input.ty, replaced, f);
			}
			if let syn::ReturnType::Type(_, ty) = &mut bare_fn.output {
				traverse_type(ty, replaced, f);
			}
		}
		Type::Group(group) => traverse_type(&mut group.elem, replaced, f),
		Type::Paren(paren) => traverse_type(&mut paren.elem, replaced, f),
		Type::Path(path) => traverse_path(&mut path.path, replaced, f),
		Type::Ptr(p) => traverse_type(&mut p.elem, replaced, f),
		Type::Reference(r) => traverse_type(&mut r.elem, replaced, f),
		Type::Slice(slice) => traverse_type(&mut slice.elem, replaced, f),
		Type::TraitObject(object) => {
			for bound in object.bounds.iter_mut() {
				if let syn::TypeParamBound::Trait(trait_bound) = bound {
					traverse_path(&mut trait_bound.path, replaced, f);
				}
			}
		}
		Type::Tuple(tuple) => {
			for elem in tuple.elems.iter_mut() {
				traverse_type(elem, replaced, f);
			}
		}
		_ => (),
	}
}

fn traverse_path(path: &mut Path, replaced: &mut bool, f: fn(&mut Type, &mut bool)) {
	for segment in &mut path.segments {
		match &mut segment.arguments {
			syn::PathArguments::None => (),
			syn::PathArguments::AngleBracketed(args) => {
				for arg in &mut args.args {
					match arg {
						syn::GenericArgument::Type(ty) => traverse_type(ty, replaced, f),
						syn::GenericArgument::Binding(bind) => traverse_type(&mut bind.ty, replaced, f),
						_ => (),
					}
				}
			}
			syn::PathArguments::Parenthesized(args) => {
				for arg in &mut args.inputs {
					traverse_type(arg, replaced, f);
				}
				if let syn::ReturnType::Type(_, ty) = &mut args.output {
					traverse_type(ty, replaced, f);
				}
			}
		}
	}
}
