use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident, ToTokens};
use syn::Token;
use syn::parse::Parser;
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) fn content(ast: syn::ItemImpl) -> TokenStream {
    let clone = ast.clone();
    let ty = match clone.trait_ {
        Some((_, p, _)) => p,
        None => panic!("must be impl for something")
    };
    let name = clone.self_ty;
    (quote! {
        pub fn new() -> Box<dyn #ty> {
            Box::new( #name {
                ..Default::default()
            } )
        }
        #[typetag::serde]
        #ast
    }).into()
}

pub(crate) fn stages(input: TokenStream, stage: &'static str) -> TokenStream2 {
    let ast = match syn::punctuated::Punctuated::<syn::Expr, Token![;]>::parse_terminated
        .parse(input.clone()) {
        Ok(ast) => ast.into_iter(),
        Err(..) => panic!("input to resolution macros must be expressions separated by semicolons.")
    };

    let mut tag: Option<u64> = None;

    let request_stage = format_ident!("request_{}", stage);
    let confirm_stage = format_ident!("confirm_{}", stage);
    let mut acc: TokenStream2 = quote! {};
    for seg in ast {
        let id = next_id();
        let (id_expr, make_hash) = match tag {
            Some(t) => {
                let hash_ident = format_ident!("dndcent_stage_hash_{}", id);
                let hasher_ident = format_ident!("dndcent_stage_hasher_{}", t);
                (
                    quote! {#hash_ident},
                    quote! {
                        #id.hash(&mut #hasher_ident);
                        let #hash_ident = #hasher_ident.finish();
                    }
                )
            }
            None => (quote! { #id }, quote! {})
        };
        match seg {
            syn::Expr::Assign(
                syn::ExprAssign {
                    left,
                    right,
                    ..
                }
            ) => {
                let expanded_right = expand_carriers(right.to_token_stream());
                acc = quote! {
                    #acc
                    #make_hash
                    if (#left).#request_stage(#id_expr) {
                        match (|| -> Result<_, ()> {Ok(#expanded_right)})() {
                            Ok(v) => {
                                *#left = v;
                                (#left).#confirm_stage(#id_expr);
                            }
                            _ => {}
                        }
                    }
                }
            },
            syn::Expr::AssignOp(
                syn::ExprAssignOp {
                    left,
                    right,
                    op,
                    ..
                }
            ) => {
                let func = format_ident!("{}", match op {
                    syn::BinOp::AddEq(..) => "add_assign",
                    syn::BinOp::SubEq(..) => "sub_assign",
                    syn::BinOp::ShlEq(..) => "push",
                    syn::BinOp::ShrEq(..) => "extend",
                    _ => ""
                });
                let expanded_right = expand_carriers(right.to_token_stream());
                acc = quote! {
                    #acc
                    #make_hash
                    if (#left).#request_stage(#id_expr) {
                        match (|| -> Result<_, ()> {Ok(#expanded_right)})() {
                            Ok(v) => {
                                use std::ops::{AddAssign, SubAssign};
                                (*#left).#func(v);
                                (#left).#confirm_stage(#id_expr);
                            }
                            _ => {}
                        }
                    }
                };
            },
            expr => {
                tag = Some(id);
                let hasher_ident = format_ident!("dndcent_stage_hasher_{}", id);
                acc = quote! {
                    #acc
                    let mut #hasher_ident = std::collections::hash_map::DefaultHasher::new();
                    (#expr).hash(&mut #hasher_ident);
                }
            }
        }
    }
    quote! {
        {
            use std::hash::{Hash, Hasher};
            #acc
        }
    }
}

static ID: AtomicU64 = AtomicU64::new(0);
pub fn next_id() -> u64 {
    ID.fetch_add(1, Ordering::SeqCst)
}

fn expand_carriers(stream: TokenStream2) -> TokenStream2 {
    use proc_macro2::TokenTree;

    let done: TokenStream2 = ". r#final ( ) ?".parse().unwrap();

    let res: TokenStream2 = stream.into_iter().flat_map(
        |token| {
            match token.clone() {
                TokenTree::Punct(p) => match p.as_char() {
                    '?' => done.clone(),
                    _ => p.to_token_stream()
                }
                TokenTree::Group(g) => {
                    let (open, close) = match g.delimiter() {
                        proc_macro2::Delimiter::Parenthesis => ('(', ')'),
                        proc_macro2::Delimiter::Brace => ('{', '}'),
                        proc_macro2::Delimiter::Bracket => ('[', ']'),
                        proc_macro2::Delimiter::None => panic!("how")
                    };
                    format!("{}{}{}",
                            open,
                            expand_carriers(g.stream()),
                            close
                    ).parse().expect("expand parse failed")
                },
                t => t.to_token_stream()
            }
        }
    ).collect();
    return res
}