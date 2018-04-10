use std::collections::HashMap;

use syn;
use quote;
use webidl;
use webidl::ast;

/// A member in an interface definition.
/// Translates to a Rust function definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Member {
    name: String,
}

/// This is created from a webidl interface, and
/// contains all the members of it.  You can add
/// partial interfaces to it, and they accumulate
/// up until this contains all the function signatures
/// etc. we need.
///
/// This then gets turned into a Rust module that
/// creates a struct that implements that interface.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    name: String,
    class_members: Vec<Member>,
    instance_members: Vec<Member>,
}

impl Module {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            class_members: vec![],
            instance_members: vec![],
        }
    }

    fn add_instance_member(&mut self, name: &str) {
        // TODO:
        // We need to generate multiple implementations for
        // different function signatures, but we don't do that
        // yet so for now we just dedup them.
        let mem = Member {
            name: name.to_owned(),
        };
        if !self.instance_members.contains(&mem) {
            self.instance_members.push(mem);
        }
    }

    fn add_class_member(&mut self, name: &str) {
        // TODO:
        // We need to generate multiple implementations for
        // different function signatures, but we don't do that
        // yet so for now we just dedup them.
        let mem = Member {
            name: name.to_owned(),
        };
        if !self.class_members.contains(&mem) {
            self.class_members.push(mem);
        }
    }

    fn to_rust(&self) -> quote::Tokens {
        let mut output = quote::Tokens::new();
        let modulename = syn::Ident::from(self.name.clone());
        
        let mut impl_tokens = quote::Tokens::new();        
        let type_tokens = quote! {
            type #modulename;
        };
        impl_tokens.append_all(type_tokens);

        for member in &self.class_members {
            let member_name = syn::Ident::from(member.name.clone());
            let tokens = quote! {
                pub fn #member_name();
            };
            impl_tokens.append_all(tokens);
        }
        
        for member in &self.instance_members {
            let member_name = syn::Ident::from(member.name.clone());
            let tokens = quote! {
                #[wasm_bindgen(method)]
                pub fn #member_name(this: &#modulename);
            };
            impl_tokens.append_all(tokens);
        }

        let full_impl = quote! {
            #[wasm_bindgen]
            extern {
                #impl_tokens
            }
        };
        output.append_all(full_impl);

        output
    }
}

/// The main struct that sucks in the webidl AST and generates Rust code.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BindingGenerator {
    modules: HashMap<String, Module>,
    // Augh stateful pattern-matching, sigh.
    current_module_name: String,
}

impl BindingGenerator {
    pub fn to_rust_module(&self) -> quote::Tokens {
        let mut output = quote::Tokens::new();
        let header_tokens = quote! {
            
            use wasm_bindgen::prelude::*;
        };
        output.append_all(header_tokens);
        output.append_all(
            self.modules.values()
                .map(Module::to_rust)
        );
        output
    }

    fn current_module(&mut self) -> &mut Module {
        self.modules.get_mut(&self.current_module_name)
            .expect("Current module does not exist?")
    }
}


impl<'ast> webidl::visitor::ImmutableVisitor<'ast> for BindingGenerator {
    fn visit_non_partial_interface(&mut self, interface: &'ast ast::NonPartialInterface) {
        println!("Got full interface {:?}", interface.name);

        // Create a new module with the given name.
        let module = Module::new(&interface.name);
        if let Some(existing_module) = self.modules.insert(interface.name.clone(), module) {
            panic!(format!(
                "Error: duplicate full interface {}",
                &existing_module.name
            ));
        }

        self.current_module_name = interface.name.clone();


        self.walk_non_partial_interface(interface);
    }

    fn visit_partial_interface(&mut self, interface: &'ast ast::PartialInterface) {
        println!("Got partial interface {:?}", interface.name);
        if let Some(_module) = self.modules.get(&interface.name) {
            self.current_module_name = interface.name.clone();
        } else {
            panic!("Tried to make partial interface without a full interface to go with it");
        }
        self.walk_partial_interface(interface);
    }

    fn visit_callback_interface(&mut self, interface: &'ast ast::CallbackInterface) {
        println!("Got callback interface {:?}", interface.name);
        self.walk_callback_interface(interface);
        unimplemented!();
    }

    // Interface operations, ie method
    fn visit_regular_operation(&mut self, op: &'ast ast::RegularOperation) {
        println!("  Got regular operation {:?}", op.name);
        {
            let module = self.current_module();
            module.add_instance_member(op.name.as_ref().unwrap());
        }
        self.walk_regular_operation(op);
    }

    fn visit_special_operation(&mut self, op: &'ast ast::SpecialOperation) {
        println!("  Got special operation {:?}", op.name);
        // https://heycam.github.io/webidl/#dfn-special-operation
        self.walk_special_operation(op);
    }

    fn visit_static_operation(&mut self, op: &'ast ast::StaticOperation) {
        println!("  Got static operation {:?}", op.name);
        {
            let module = self.current_module();
            module.add_class_member(&op.name.as_ref().unwrap());
        }
        self.walk_static_operation(op);
    }
    
    fn visit_stringifier_operation(&mut self, op: &'ast ast::StringifierOperation) {
        println!("  Got stringifier operation {:?}", op);
        // https://heycam.github.io/webidl/#dfn-special-operation too
        // I think
        self.walk_stringifier_operation(op);
    }
}
