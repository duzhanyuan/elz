use crate::ast::{Import, TopAst};
use crate::codegen::llvm::LLVMValue;
use crate::codegen::CodeGenerator;
use crate::diagnostic::Reporter;
use crate::lexer::Location;
use crate::parser::{parse_prelude, Parser};
use crate::semantic::SemanticChecker;

pub const CMD_NAME: &'static str = "compile";

pub fn compile(files: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut reporter = Reporter::new();
    // FIXME: comment out code generator for now to focus on semantic checking
    let program = check(&mut reporter, files)?;
    let code_generator = CodeGenerator::new();
    let module = code_generator.generate_module(&program);
    println!("{}", module.llvm_represent());
    Ok(())
}

fn check(
    reporter: &mut Reporter,
    files: Vec<&str>,
) -> Result<Vec<TopAst>, Box<dyn std::error::Error>> {
    // FIXME: for now to make code simple we only handle the first input file.
    let code = std::fs::read_to_string(files[0])?;
    let mut file_reporter = reporter.for_file(files[0], &code);
    let mut module = match Parser::parse_program(files[0], &code) {
        Ok(p) => p,
        Err(err) => {
            file_reporter.add_diagnostic(err.location(), format!("{}", err), err.message());
            file_reporter.report(reporter);
            return Err(err.into());
        }
    };
    // insert import prelude
    module.top_list.push(TopAst::Import(Import {
        location: Location::none(),
        import_path: "prelude".to_string(),
        imported_component: vec![
            "int".to_string(),
            "void".to_string(),
            "f64".to_string(),
            "bool".to_string(),
            "string".to_string(),
            "List".to_string(),
            "println".to_string(),
        ],
    }));

    let prelude = parse_prelude();
    let mut l = prelude.top_list.clone();
    l.append(module.top_list.as_mut());
    let program = vec![prelude, module];
    // check program
    let mut semantic_checker = SemanticChecker::new();
    match semantic_checker.check_program(&program) {
        Ok(..) => Ok(l),
        Err(err) => {
            file_reporter.add_diagnostic(err.location(), format!("{}", err), err.message());
            file_reporter.report(reporter);
            Err(err.into())
        }
    }
}
