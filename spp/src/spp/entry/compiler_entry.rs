use indicatif::MultiProgress;
use crate::spp::entry::program::Program;
use crate::spp::entry::program_module_tree::ProgramModuleTree;
use crate::spp::lexer::lexer::Lexer;
use crate::spp::parser::parser::Parser;
use crate::spp::utilities::progress_bar::new_progress_bar;

pub struct CompilerEntry {
    src_path: String,
    module_tree: ProgramModuleTree,
    program_ast: Program,
}

impl CompilerEntry {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        Self {
            src_path: cwd.clone(),
            module_tree: ProgramModuleTree::new(cwd),
            program_ast: Program::new(),
        }
    }

    pub fn compile(&mut self, mode: &String) -> Result<(), Box<dyn std::error::Error>> {
        self.module_tree.scan_modules();

        // Create the progress bars for each stage of the compilation process.
        let mut mpb = MultiProgress::new();
        let pb_1 = new_progress_bar(&mut mpb, "Lexing".to_string(), self.module_tree.modules().len() as u64);
        let pb_2 = new_progress_bar(&mut mpb, "Parsing".to_string(), self.module_tree.modules().len() as u64);
        // let pb_3 = new_progress_bar(&mut mpb, "Pre-processing".to_string(), self.module_tree.modules().len() as u64);
        // let pb_4 = new_progress_bar(&mut mpb, "Generating top-level scopes".to_string(), self.module_tree.modules().len() as u64);
        // let pb_5 = new_progress_bar(&mut mpb, "Generating top-level aliases".to_string(), self.module_tree.modules().len() as u64);
        // let pb_6 = new_progress_bar(&mut mpb, "Loading super scopes".to_string(), self.module_tree.modules().len() as u64);
        // let pb_7 = new_progress_bar(&mut mpb, "Preprocessing super scopes".to_string(), self.module_tree.modules().len() as u64);
        // let pb_8 = new_progress_bar(&mut mpb, "Regenerating generic aliases".to_string(), self.module_tree.modules().len() as u64);
        // let pb_9 = new_progress_bar(&mut mpb, "Regenerating generic types".to_string(), self.module_tree.modules().len() as u64);
        // let pb_10 = new_progress_bar(&mut mpb, "Analysing semantics".to_string(), self.module_tree.modules().len() as u64);
        mpb.println("Compiling...").unwrap();

        // Run the lexing steps for each module.
        for mut module in self.module_tree.modules() {
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

        // Remove the "main.spp" files from the vcs imported libraries.
        for module in self.module_tree.modules_clone() {
            let ns = module.path.split("/").skip_while(|x| x != &"src");
            if module.path.starts_with("/vcs") && ns.count() == 0 {
                self.module_tree.remove_module(&module);
            }
        }

        Ok(())
    }
}
