use std::fs;
use std::io::Read;


extern crate webidl;

use webidl::visitor::ImmutableVisitor;

fn pretty_print(ast: &webidl::ast::AST) {
    let mut visitor = webidl::visitor::PrettyPrintVisitor::new();
    visitor.visit(ast);
    println!("{}", visitor.get_output());
}

fn parse_firefox_webidls() {
    let source_dir = format!("{}/webidl_src/firefox_webidl", env!("CARGO_MANIFEST_DIR"));
    let mut successes = 0;
    let mut failures = 0;
    for entry in fs::read_dir(&source_dir).unwrap() {
        // Read each file in the webidl dir
        let file_path = entry.unwrap().path();
        let f = &mut fs::File::open(&file_path).unwrap();
        let file_contents = &mut String::new();
        f.read_to_string(file_contents).unwrap();

        // Create parser and parse file
        let parser = webidl::Parser::new();
        if let Err(e) = parser.parse_string(file_contents) {
            println!("Could not parse {:?}: {:?}", file_path, e);
            failures += 1;
        } else {
            println!("Parsed {:?} successfully", file_path);
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
    parse_firefox_webidls();
}
