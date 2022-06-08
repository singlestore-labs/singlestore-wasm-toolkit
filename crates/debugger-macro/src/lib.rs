use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, visit::Visit};

struct Visitor {}

impl Visit<'_> for Visitor {
    fn visit_item_fn(&mut self, node: &'_ syn::ItemFn) {
        let sig = &node.sig;
        let name = &sig.ident;
        /*
            sig.inputs.iter().for_each(|input| {
                if let syn::FnArg::Typed(x) = input {
                    self.source.push_str(quote! {#x}.to_string().as_str());
                }
            });

            self.source.push_str(") -> ");

            if let syn::ReturnType::Type(_, ref ty) = sig.output {
                let type_name = quote! {#ty}.to_string();
                let type_name = type_name.replace("Vec", "list");
                self.source.push_str(type_name.as_str());
                self.source.push('\n');
            }
        */
    }
}

#[proc_macro_attribute]
pub fn debugger(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::Item);
    let mut visitor = Visitor {};
    visitor.visit_item(&input);

    let x = quote! {
        #input

        #[derive(Serialize, Deserialize)]
        struct Payload(i32, i32);

        struct Debugger;
        wit_bindgen_rust::export!("../../../crates/debugger/debugger.wit");
        impl debugger::Debugger for Debugger {
            fn handle_json(_name: String, json: Vec<u8>) -> Vec<u8> {
                let payload: Payload = serde_json::from_slice(&json).unwrap();
                let result = <Power as power::Power>::power_of(payload.0, payload.1);
                serde_json::to_vec(&vec![result]).unwrap()
            }
        }
    };
    x.into()
}
