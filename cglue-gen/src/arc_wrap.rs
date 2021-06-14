use proc_macro2::TokenStream;

use std::collections::BTreeMap;

use super::func::WrappedType;
use super::generics::ParsedGenerics;

use quote::*;
use syn::*;

pub fn gen_wrap(tr: ItemTrait, ext_path: Option<TokenStream>) -> TokenStream {
    let crate_path = crate::util::crate_path();

    let mut types = BTreeMap::new();

    types.insert(
        format_ident!("Self"),
        WrappedType {
            ty: parse2(quote!(Self)).unwrap(),
            ty_static: None,
            return_conv: None,
            lifetime_bound: None,
            lifetime_type_bound: None,
            other_bounds: None,
            other_bounds_simple: None,
            impl_return_conv: None,
            inject_ret_tmp: false,
            unbounded_hrtb: false,
        },
    );

    let mut wrapped_types = TokenStream::new();

    let needs_send = tr.supertraits.iter().any(|s| {
        if let TypeParamBound::Trait(tr) = s {
            tr.path.get_ident().map(|i| i == "Send") == Some(true)
        } else {
            false
        }
    });

    let send_bound = if needs_send {
        quote!(+ Send + Sync)
    } else {
        quote!()
    };

    let (funcs, generics, _) =
        super::traits::parse_trait(&tr, &crate_path, |ty, _, _, _, types, _| {
            let mut has_wrapped = false;
            let ident = &ty.ident;

            for attr in &ty.attrs {
                let s = attr.path.to_token_stream().to_string();

                if s.as_str() == "arc_wrap" {
                    let new_ty =
                        parse2(quote!(#crate_path::arc::ArcWrapped<CGlueT::#ident, CGlueA>))
                            .unwrap();
                    wrapped_types.extend(quote!(type #ident = #new_ty;));

                    types.insert(
                        ident.clone(),
                        WrappedType {
                            ty: new_ty,
                            ty_static: None,
                            return_conv: None,
                            lifetime_bound: None,
                            lifetime_type_bound: None,
                            other_bounds: None,
                            other_bounds_simple: None,
                            impl_return_conv: None,
                            inject_ret_tmp: false,
                            unbounded_hrtb: false,
                        },
                    );

                    has_wrapped = true;
                }
            }

            if !has_wrapped {
                wrapped_types.extend(quote!(type #ident = CGlueT::#ident;));
            }
        });

    let ParsedGenerics {
        life_declare,
        life_use,
        gen_declare,
        gen_use,
        gen_where_bounds,
        ..
    } = &generics;

    let trait_name = &tr.ident;

    let mut impls = TokenStream::new();

    for func in &funcs {
        func.arc_wrapped_trait_impl(&mut impls);
    }

    let tr_impl = if ext_path.is_some() {
        quote!()
    } else {
        quote!(#tr)
    };

    quote! {
        #tr_impl

        impl<#life_declare CGlueT, CGlueA: 'static #send_bound, #gen_declare> #ext_path #trait_name<#life_use #gen_use> for #crate_path::arc::ArcWrapped<CGlueT, CGlueA> where CGlueT: #ext_path #trait_name<#life_use #gen_use>, #gen_where_bounds {
            #wrapped_types
            #impls
        }
    }
}
