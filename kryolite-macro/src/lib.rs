mod write_manifest;

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use write_manifest::write_json;
use proc_macro::TokenTree;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Serialize;
use syn::{
    parse_macro_input, parse_quote,
    visit_mut::{self, VisitMut},
    Expr, ExprLit, ImplItemMethod, Lit, LitInt, Visibility, FnArg,
};

#[proc_macro_derive(State)]
pub fn derive_state_fn(items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    fn ident_name(item: TokenTree) -> String {
        match item {
            TokenTree::Ident(i) => i.to_string(),
            _ => panic!("Not an ident"),
        }
    }

    let item_name = ident_name(items.into_iter().nth(2).unwrap());

    format!(
      "impl State for {} {{
      #[no_mangle]
      fn _export_state(&self) {{
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
  
        unsafe {{
          __export_state(buf.as_ptr(), buf.len());
        }}
      }}
    }}", item_name)
    .parse()
    .unwrap()
}

#[proc_macro_attribute]
pub fn exported(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[no_mangle]
        #input
    };

    output.into()
}

#[proc_macro_attribute]
pub fn smart_contract(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as syn::File);
    syn::visit_mut::visit_file_mut(&mut LiteralReplacer, &mut input);

    let json: String;

    unsafe {
      json = serde_json::to_string_pretty(&CONTRACT).unwrap();
    }

    write_json(&json);

    input.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn smart_contract_state(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(Serialize, State)]
        #input
    };
    output.into()
}

// "visitor" that visits every node in the syntax tree
// we add our own behavior to replace custom literals with proper Rust code
struct LiteralReplacer;

impl VisitMut for LiteralReplacer {
  fn visit_expr_mut(&mut self, i: &mut Expr) {
    if let Expr::Lit(ExprLit { lit, .. }) = i {
      match lit {
        Lit::Int(lit) => {
          // get literal suffix
          let suffix = lit.suffix();
          // get literal without suffix
          let lit_nosuffix = LitInt::new(lit.base10_digits(), lit.span());

          match suffix {
            // replace literal expression with new expression
            "kryo" => *i = parse_quote! { #lit_nosuffix * 1000000 },
            //"y" => *i = parse_quote! { y_literal(#lit_nosuffix) },
            _ => (), // other literal suffix we won't modify
          }
        }

        _ => (), // other literal type we won't modify
      }
    } else {
      // not a literal, use default visitor method
      visit_mut::visit_expr_mut(self, i)
    }
  }

  fn visit_item_impl_mut(&mut self, i: &mut syn::ItemImpl) {
    let name = i.self_ty.to_token_stream().to_string();
    eprintln!("Name: {}", i.self_ty.to_token_stream().to_string());

    unsafe {
      CONTRACT.name = name;
    }

    visit_mut::visit_item_impl_mut(self, i);
  }

  fn visit_impl_item_method_mut(&mut self, i: &mut ImplItemMethod) {
    match &i.vis {

      Visibility::Public(_x) => {
        let name = i.sig.ident.to_string();
        eprintln!("Method: {}", name);

        let mut method = Method {
          name: name,
          method_params: Vec::new()
        };

        i.sig.inputs.iter().for_each(|item| {

          match item {

            FnArg::Typed(y) => {
              let name = y.pat.to_token_stream().to_string();
              let typ = y.ty.to_token_stream().to_string().replace("& ", "");
              eprintln!("Param: {}: {}", name, typ);

              let param = Param {
                name: name,
                param_type: typ
              };

              method.method_params.push(param);
            }
            _ => ()
          }

        });

        unsafe {
          CONTRACT.methods.push(method);
        }
      }
      _ => (),
    }

    visit_mut::visit_impl_item_method_mut(self, i);
  }
}

#[derive(Serialize)]
struct Contract {
  pub name: String,
  pub methods: Vec<Method>
}

#[derive(Serialize)]
struct Method {
  pub name: String,
  pub method_params: Vec<Param>
}

#[derive(Serialize)]
struct Param {
  pub name: String,
  pub param_type: String
}

static mut CONTRACT: Contract = Contract { name: String::new(), methods: Vec::new()};