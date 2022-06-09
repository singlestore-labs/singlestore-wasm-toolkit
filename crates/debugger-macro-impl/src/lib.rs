use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, visit::Visit};

struct Handler {
    name: String,
    src: TokenStream2,
}

struct HandleVisitor {
    impl_trait: syn::Path,
    impl_type: syn::Type,
    handlers: Vec<Handler>,
}

impl<'ast> Visit<'ast> for HandleVisitor {
    fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
        let sig = &node.sig;
        let name = &sig.ident;

        let typed_args = sig.inputs.iter().filter_map(|input| match input {
            syn::FnArg::Typed(x) => Some((*x.ty).clone()),
            _ => None,
        });
        let indexes = (0..typed_args.clone().count()).map(syn::Index::from);

        let args = quote! { (#(#typed_args,)*) };
        let args_splat = quote! { #(args.#indexes),* };

        let impl_trait = &self.impl_trait;
        let impl_type = &self.impl_type;

        self.handlers.push(Handler {
            name: name.to_string(),
            src: quote! {
                let args: #args = serde_json::from_slice(&json).unwrap();
                let result = <#impl_type as #impl_trait>::#name(#args_splat);
                serde_json::to_vec(&vec![result]).unwrap()
            },
        });

        syn::visit::visit_impl_item_method(self, node);
    }
}

#[proc_macro_attribute]
pub fn export_debug_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemImpl);

    let (_, path, _) = input.trait_.as_ref().expect("expected trait");

    let mut visitor = HandleVisitor {
        impl_trait: path.clone(),
        impl_type: (*input.self_ty).clone(),
        handlers: vec![],
    };
    visitor.visit_item_impl(&input);

    let handle_names = visitor.handlers.iter().map(|h| &h.name);
    let handle_srcs = visitor.handlers.iter().map(|h| &h.src);

    let debugger_wit = include_str!("../../debugger/debugger.wit");

    quote! {
        #input

        use ::debugger_macro::serde_json;

        struct DebuggerImpl;
        wit_bindgen_rust::export!({
            src["debugger_impl"]: #debugger_wit
        });
        impl debugger_impl::DebuggerImpl for DebuggerImpl {
            fn handle_json(name: String, json: Vec<u8>) -> Vec<u8> {
                match name.as_str() {
                    #(#handle_names => {#handle_srcs})*
                    _ => panic!("unknown handler")
                }
            }
        }
    }
    .into()
}
