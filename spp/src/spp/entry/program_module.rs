use crate::spp::asts::module_prototype_ast::ModulePrototypeAst;
use crate::spp::lexer::token::TokenStream;
use crate::spp::utilities::error_formatter::ErrorFormatter;

#[derive(Clone)]
pub struct ProgramModule {
    pub path: String,
    pub code: String,
    pub tokens: TokenStream,
    pub module_ast: Option<ModulePrototypeAst>,
    pub error_formatter: ErrorFormatter,
}

impl PartialEq for &ProgramModule {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl ProgramModule {
    pub fn new(path: String) -> Self {
        Self {
            path,
            code: String::new(),
            tokens: TokenStream::new(),
            module_ast: None,
            error_formatter: ErrorFormatter::default(),
        }
    }
}
