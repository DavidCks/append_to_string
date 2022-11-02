extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::{quote};
use syn::{ Expr, parse_macro_input, parse_quote, fold::Fold };

struct Args;

impl syn::fold::Fold for Args {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Struct(e) => {

                // initialize the token stream that is going to represent syn::FieldValue
                let mut field_tokens = proc_macro2::TokenStream::new();

                // iterate over all the fields in the struct
                for (_i, field) in e.fields.iter().enumerate() {

                    // copy the references of the current fields parameters
                    let field_ident = &field.member;
                    let field_punct = &field.colon_token;

                    // recursive call for fields that have nested syn::ExprStruct
                    let field_expr = self.fold_expr(field.expr.clone());

                    // compile the syn::FieldValue TokenStream (member: expr,)
                    let append_tokens: proc_macro2::TokenStream = parse_quote!(
                        #field_ident #field_punct #field_expr,
                    );

                    // append the compiled TokenSteam to the initial empty TokenSteam
                    field_tokens.extend([ append_tokens ].into_iter());
                }

                // copy a reference to the structs identifier
                let struct_ident = &e.path.segments.first().unwrap().ident;

                // returns the parsed struct with all syn::Lit having .to_string() appended to them
                parse_quote!(
                    #struct_ident {
                        #field_tokens
                    }
                )
            }
            Expr::Lit(e) => {
                parse_quote!(
                    #e.to_string()
                )
            }
            _ => syn::fold::fold_expr(self, e),
        }
    }
}

#[proc_macro]
pub fn append_to_string(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as Expr);

    let mut args = Args{};
    
    let output = args.fold_expr(input);

    let expanded = quote! {
        #output
    };

    // returing a simple TokenStream for Struct
    TokenStream::from(expanded)
}