use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::Read;


extern crate webidl;
use webidl::ast;
use webidl::visitor::ImmutableVisitor;

fn pretty_print(ast: &webidl::ast::AST) {
    let mut visitor = webidl::visitor::PrettyPrintVisitor::new();
    visitor.visit(ast);
    println!("{}", visitor.get_output());
}

/// A member in an interface definition.
/// Translates to a Rust function definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Member {
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
struct Module {
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
struct BindingGenerator {
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
            panic!(format!("Error: duplicate full interface {}", &existing_module.name));
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

/// To start off we're just going to try to read the
/// Window definition file and generate a Rust file that
/// contains bindings to `alert()`.
fn window_test() {
    let src_file = &format!("{}/webidl_src/servo_webidl/Window.webidl", env!("CARGO_MANIFEST_DIR"));
    let dst_file = &format!("{}/webidl_dst/servo/window.rs", env!("CARGO_MANIFEST_DIR"));

    let ast = {
        let f = &mut fs::File::open(&src_file)
            .expect("Could not open source file");
        let file_contents = &mut String::new();
        f.read_to_string(file_contents)
            .expect("Could not read source file");
                                                         
        webidl::parse_string(file_contents)
            .expect("Could not parse source file")
    };

    let gen = &mut BindingGenerator::default();
    gen.visit(&ast);
}

fn parse_webidls(platform_name: &str) {
    let source_dir = format!("{}/webidl_src/{}_webidl", env!("CARGO_MANIFEST_DIR"), platform_name);
    let mut successes = 0;
    let mut failures = 0;
    for entry in fs::read_dir(&source_dir).unwrap() {
        // Read each file in the webidl dir
        let file_path = entry.unwrap().path();
        if file_path.extension() != Some(OsStr::new("webidl")) {
            continue;
        }
        let f = &mut fs::File::open(&file_path).unwrap();
        let file_contents = &mut String::new();
        f.read_to_string(file_contents).unwrap();

        // Create parser and parse file
        if let Err(e) = webidl::parse_string(file_contents) {
            failures += 1;
            if let Some(filename) = file_path.file_name() {
                println!("Could not parse {:?}: {}", filename, e);
            } else {
                unreachable!();
            }
            
        } else {
            //println!("Parsed {:?} successfully", file_path);
            successes += 1;
        }
    }

    println!("Total results: Parsed {}/{} files successfully", successes, successes+failures);
}

fn main() {
    /*
    let testcode = r#"/* Example taken from emscripten site */
                        enum EnumClass_EnumWithinClass {
                            "EnumClass::e_val"
                        };"#;
    let parser = webidl::Parser::new();
    let res = parser.parse_string(testcode).unwrap();
    pretty_print(&res);
    */
    //parse_webidls("servo");

    window_test();
}
