use std::ffi::OsStr;
use std::fs;
use std::io::Read;


extern crate webidl;

use webidl::visitor::ImmutableVisitor;

fn pretty_print(ast: &webidl::ast::AST) {
    let mut visitor = webidl::visitor::PrettyPrintVisitor::new();
    visitor.visit(ast);
    println!("{}", visitor.get_output());
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
    parse_webidls("servo");
}
