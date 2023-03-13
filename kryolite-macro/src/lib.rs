mod write_manifest;

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use write_manifest::write_json;
use proc_macro2::TokenStream;
use quote::{ToTokens};
use serde::Serialize;
use syn::{
    parse_macro_input, parse_quote,
    visit_mut::{self, VisitMut},
    Expr, ExprLit, ImplItemMethod, Lit, LitInt, Visibility, FnArg, ReturnType, ImplItem,
};

#[proc_macro_attribute]
pub fn smart_contract(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as syn::File);
    let walker = &mut Walker { items: Vec::new() };

    syn::visit_mut::visit_file_mut(walker, &mut input);

    let json: String;

    unsafe {
      json = serde_json::to_string_pretty(&CONTRACT).unwrap();
    }

    write_json(&json);

    // uncomment to see outputs
    // eprintln!("{}", input.to_token_stream().to_string());

    input.to_token_stream().into()
}

// "visitor" that visits every node in the syntax tree
// we add our own behavior to replace custom literals with proper Rust code
struct Walker {
  items: Vec<ImplItem>
}

impl VisitMut for Walker {
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
      CONTRACT.name = name.clone();
    }

    let export: TokenStream = format!("#[export_name = \"__init\"]")
    .parse()
    .unwrap();

    let init: TokenStream = format!("
      pub fn __init() -> *mut u8 {{
        let instance = {}::new();
        Box::into_raw(Box::new(instance)) as *mut u8
      }}", name)
      .parse()
      .unwrap();

    let initfn: ImplItemMethod = parse_quote!{
      #export
      #init
    };

    i.items.push(syn::ImplItem::Method(initfn));
    visit_mut::visit_item_impl_mut(self, i);

    for ele in &self.items {
      i.items.push(ele.clone());
    }

    visit_mut::visit_item_impl_mut(self, i);
  }

  fn visit_impl_item_method_mut(&mut self, i: &mut ImplItemMethod) {
    match &i.vis {

      Visibility::Public(_x) => {
        let name = i.sig.ident.to_string();

        if name == "new" || name == "__init" {
          visit_mut::visit_impl_item_method_mut(self, i);
          return;
        }

        eprintln!("Method: {}", name);

        let mut method = Method {
          name: name.clone(),
          readonly: false,
          method_params: Vec::new(),
          return_value: ReturnValue {
            value_type: "void".to_string()
          }
        };

        let export: TokenStream = format!("#[export_name = \"{}\"]", name)
          .parse()
          .unwrap();

        let mut param_names: Vec<String> = Vec::new();

        i.sig.inputs.iter().for_each(|item| {
          match item {
            FnArg::Receiver(rec) => {
              method.readonly = rec.mutability.is_none();
            },

            FnArg::Typed(y) => {
              let name = y.pat.to_token_stream().to_string();
              let typ = y.ty.to_token_stream().to_string().replace("&", "").replace(" ", "");
              eprintln!("Param: {}: {}", name, typ);

              let param = Param {
                name: name.clone(),
                param_type: typ
              };

              method.method_params.push(param);
              param_names.push(name);
            }
          }

        });

        let input: TokenStream = i.to_token_stream();

        match &i.sig.output {
          ReturnType::Type(_arrow, type_arg) => {
            let type_str = type_arg.to_token_stream().to_string();
            let value_type = type_str.replace("&", "").replace(" ", "").to_string();

            eprintln!("Returns: {}", value_type);

            method.return_value = ReturnValue {
              value_type,
            };

            let wrapper: TokenStream = format!(
              "pub fn {}_json({}) {{
                let result = self.{}({});
                let json = serde_json::to_string(&result).unwrap();
                push_return(json.as_str());
              }}", name, i.sig.inputs.to_token_stream().to_string(), name, param_names.join(", "))
            .parse()
            .unwrap();

            let wrapfn: ImplItemMethod = parse_quote!(
              #export
              #wrapper
            );
        
            self.items.push(syn::ImplItem::Method(wrapfn));

            *i = parse_quote! {
                #input
            };
          }
          _ => {

            *i = parse_quote! {
                #export
                #input
            };
          }
        }

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
  pub readonly: bool,
  pub method_params: Vec<Param>,
  pub return_value: ReturnValue
}

#[derive(Serialize)]
struct Param {
  pub name: String,
  pub param_type: String
}

#[derive(Serialize)]
struct ReturnValue {
  pub value_type: String
}

static mut CONTRACT: Contract = Contract { name: String::new(), methods: Vec::new()};