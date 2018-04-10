use webidl;
use webidl::ast;
use std::collections::HashMap;

/// A member in an interface definition.
/// Translates to a Rust function definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
}

/// The main struct that sucks in the webidl AST and generates Rust code.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BindingGenerator {
    modules: HashMap<String, Module>,
    // Augh stateful pattern-matching, sigh.
    current_module_name: String,
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
    }

    fn visit_partial_interface(&mut self, interface: &'ast ast::PartialInterface) {
        println!("Got partial interface {:?}", interface.name);
        if let Some(module) = self.modules.get(&interface.name) {
            self.current_module_name = interface.name.clone();
        } else {
            panic!("Tried to make partial interface without a full interface to go with it");
        }
    }

    fn visit_callback_interface(&mut self, interface: &'ast ast::CallbackInterface) {
        println!("Got callback interface {:?}", interface.name);
    }
}
