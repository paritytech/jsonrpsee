// Copyright 2021 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::collections::HashSet;
use syn::{
	visit::{self, Visit},
	Ident,
};

/// Visitor that parses generic type parameters from `syn::Type`.
// Based on https://github.com/serde-rs/serde/blob/master/serde_derive/src/bound.rs.
#[derive(Default, Debug)]
pub(crate) struct FindSubTyParams {
	pub(crate) generic_sub_params: HashSet<Ident>,
	pub(crate) all_type_params: HashSet<Ident>,
}

/// Visitor for the entire `RPC trait`.
pub struct FindTyParams {
	pub(crate) trait_generics: HashSet<syn::Ident>,
	pub(crate) input_params: HashSet<syn::Ident>,
	pub(crate) ret_params: HashSet<syn::Ident>,
	pub(crate) sub_params: HashSet<syn::Ident>,
	pub(crate) visiting_return_type: bool,
	pub(crate) visiting_fn_arg: bool,
}

impl FindTyParams {
	pub fn new(sub_params: HashSet<syn::Ident>) -> Self {
		Self {
			trait_generics: HashSet::new(),
			input_params: HashSet::new(),
			ret_params: HashSet::new(),
			sub_params,
			visiting_return_type: false,
			visiting_fn_arg: false,
		}
	}
}

impl<'ast> Visit<'ast> for FindTyParams {
	fn visit_type_param(&mut self, ty_param: &'ast syn::TypeParam) {
		self.trait_generics.insert(ty_param.ident.clone());
	}

	fn visit_return_type(&mut self, return_type: &'ast syn::ReturnType) {
		self.visiting_return_type = true;
		visit::visit_return_type(self, return_type);
		self.visiting_return_type = false
	}

	fn visit_ident(&mut self, ident: &'ast syn::Ident) {
		if self.trait_generics.contains(ident) {
			if self.visiting_return_type {
				self.ret_params.insert(ident.clone());
			}
			if self.visiting_fn_arg {
				self.input_params.insert(ident.clone());
			}
		}
	}

	fn visit_fn_arg(&mut self, arg: &'ast syn::FnArg) {
		self.visiting_fn_arg = true;
		visit::visit_fn_arg(self, arg);
		self.visiting_fn_arg = false;
	}
}

impl FindSubTyParams {
	/// Visit all types and returns all generic [`Ident`]'s that are subscriptions.
	pub fn visit(mut self, tys: &[syn::Type]) -> HashSet<Ident> {
		for ty in tys {
			self.visit_type(ty);
		}
		self.generic_sub_params
	}

	pub fn new(all_type_params: HashSet<Ident>) -> Self {
		Self { generic_sub_params: HashSet::new(), all_type_params }
	}

	fn visit_path(&mut self, path: &syn::Path) {
		if path.leading_colon.is_none() && path.segments.len() == 1 {
			let id = &path.segments[0].ident;
			if self.all_type_params.contains(id) {
				self.generic_sub_params.insert(id.clone());
			}
		}
		for segment in &path.segments {
			self.visit_path_segment(segment);
		}
	}

	// Everything below is simply traversing the syntax tree.

	fn visit_type(&mut self, ty: &syn::Type) {
		match ty {
			syn::Type::Array(ty) => self.visit_type(&ty.elem),
			syn::Type::BareFn(ty) => {
				for arg in &ty.inputs {
					self.visit_type(&arg.ty);
				}
				self.visit_return_type(&ty.output);
			}
			syn::Type::Group(ty) => self.visit_type(&ty.elem),
			syn::Type::ImplTrait(ty) => {
				for bound in &ty.bounds {
					self.visit_type_param_bound(bound);
				}
			}
			syn::Type::Macro(ty) => self.visit_macro(&ty.mac),
			syn::Type::Paren(ty) => self.visit_type(&ty.elem),
			syn::Type::Path(ty) => {
				if let Some(qself) = &ty.qself {
					self.visit_type(&qself.ty);
				}
				self.visit_path(&ty.path);
			}
			syn::Type::Ptr(ty) => self.visit_type(&ty.elem),
			syn::Type::Reference(ty) => self.visit_type(&ty.elem),
			syn::Type::Slice(ty) => self.visit_type(&ty.elem),
			syn::Type::TraitObject(ty) => {
				for bound in &ty.bounds {
					self.visit_type_param_bound(bound);
				}
			}
			syn::Type::Tuple(ty) => {
				for elem in &ty.elems {
					self.visit_type(elem);
				}
			}

			syn::Type::Infer(_) | syn::Type::Never(_) | syn::Type::Verbatim(_) => {}

			#[cfg(test)]
			syn::Type::__TestExhaustive(_) => unimplemented!(),
			#[cfg(not(test))]
			_ => {}
		}
	}

	fn visit_path_segment(&mut self, segment: &syn::PathSegment) {
		self.visit_path_arguments(&segment.arguments);
	}

	fn visit_path_arguments(&mut self, arguments: &syn::PathArguments) {
		match arguments {
			syn::PathArguments::None => {}
			syn::PathArguments::AngleBracketed(arguments) => {
				for arg in &arguments.args {
					match arg {
						syn::GenericArgument::Type(arg) => self.visit_type(arg),
						syn::GenericArgument::Binding(arg) => self.visit_type(&arg.ty),
						syn::GenericArgument::Lifetime(_)
						| syn::GenericArgument::Constraint(_)
						| syn::GenericArgument::Const(_) => {}
					}
				}
			}
			syn::PathArguments::Parenthesized(arguments) => {
				for argument in &arguments.inputs {
					self.visit_type(argument);
				}
				self.visit_return_type(&arguments.output);
			}
		}
	}

	fn visit_return_type(&mut self, return_type: &syn::ReturnType) {
		match return_type {
			syn::ReturnType::Default => {}
			syn::ReturnType::Type(_, output) => self.visit_type(output),
		}
	}

	fn visit_type_param_bound(&mut self, bound: &syn::TypeParamBound) {
		match bound {
			syn::TypeParamBound::Trait(bound) => self.visit_path(&bound.path),
			syn::TypeParamBound::Lifetime(_) => {}
		}
	}

	// Type parameter should not be considered used by a macro path.
	//
	//     struct TypeMacro<T> {
	//         mac: T!(),
	//         marker: PhantomData<T>,
	//     }
	fn visit_macro(&mut self, _mac: &syn::Macro) {}
}

#[cfg(test)]
mod tests {
	use super::*;
	use syn::{parse_quote, Type};

	#[test]
	fn it_works() {
		let t: Type = parse_quote!(Vec<T>);
		let id: Ident = parse_quote!(T);

		let mut exp = HashSet::new();
		exp.insert(id);

		assert_eq!(exp, FindSubTyParams::new(exp.clone()).visit(&[t]));
	}

	#[test]
	fn several_type_params() {
		let t: Type = parse_quote!(Vec<(A, B, C)>);

		let mut generics: HashSet<syn::Ident> = HashSet::new();
		let mut exp = HashSet::new();

		generics.insert(parse_quote!(A));
		generics.insert(parse_quote!(B));
		generics.insert(parse_quote!(C));
		generics.insert(parse_quote!(D));

		exp.insert(parse_quote!(A));
		exp.insert(parse_quote!(B));
		exp.insert(parse_quote!(C));

		assert_eq!(exp, FindSubTyParams::new(exp.clone()).visit(&[t]));
	}

	#[test]
	fn nested() {
		let t: Type = parse_quote!(Vec<Foo<A, B>>);

		let mut generics: HashSet<syn::Ident> = HashSet::new();
		let mut exp = HashSet::new();

		generics.insert(parse_quote!(A));
		generics.insert(parse_quote!(B));
		generics.insert(parse_quote!(C));
		generics.insert(parse_quote!(D));

		exp.insert(parse_quote!(A));
		exp.insert(parse_quote!(B));

		assert_eq!(exp, FindSubTyParams::new(exp.clone()).visit(&[t]));
	}
}
