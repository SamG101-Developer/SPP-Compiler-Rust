use crate::spp::asts::ast::Ast;
use crate::spp::entry::program_module_tree::ProgramModuleTree;
use crate::spp::lexer::lexer::Lexer;
use crate::spp::parser::parser::Parser;
use crate::spp::utilities::progress_bar::new_pb;
use indicatif::MultiProgress;

pub struct CompilerEntry {
    src_path: String,
    module_tree: ProgramModuleTree,
}


impl CompilerEntry {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        Self {
            src_path: cwd.clone(),
            module_tree: ProgramModuleTree::new(cwd),
        }
    }

    pub fn compile(mut self, mode: &String) -> Result<(), Box<dyn std::error::Error>> {
        self.module_tree.scan_modules();

        // Create the progress bars for each stage of the compilation process.
        let module_count = self.module_tree.modules().len() as u64;
        let mut mpb = MultiProgress::new();
        let pb_1 = new_pb(&mut mpb, "Lexing".to_string(), module_count);
        let pb_2 = new_pb(&mut mpb, "Parsing".to_string(), module_count);
        mpb.println("\nCompiling...\n").unwrap();

        // Run the lexing steps for each module.
        for module in self.module_tree.modules() {
            let full_path = self.src_path.clone() + &module.path;
            module.code = std::fs::read_to_string(&full_path).expect("Failed to read the module file.");
            module.tokens = Lexer::new(module.code.clone()).lex();
            module.error_formatter.update_info(module.path.clone(), module.tokens.clone());
            pb_1.set_message(module.path.clone());
            pb_1.inc(1);
        }
        pb_1.finish_with_message("Lexing complete.");

        // Run the parsing steps for each module.
        for module in self.module_tree.modules() {
            let module_ast = Parser::new(module.tokens.clone()).parse();
            if let Ok(module_ast) = module_ast {
                module.module_ast = Some(module_ast);
                pb_2.set_message(module.path.clone());
                pb_2.inc(1);
            }
            else {
                let error = module_ast.unwrap_err();
                println!("{}", module.error_formatter.error_raw_pos(error.get_pos(), 1, error.get_msg(), "Syntax Error".to_string()));
                return Err(Box::new(error));
            }
        }
        pb_2.finish_with_message("Parsing complete.");

        Ok(())
    }
}
