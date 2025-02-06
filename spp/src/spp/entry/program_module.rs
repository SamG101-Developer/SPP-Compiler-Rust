use crate::spp::asts::module_prototype_ast::ModulePrototypeAst;
use crate::spp::lexer::token::TokenStream;
use crate::spp::utilities::error_formatter::ErrorFormatter;

pub struct ProgramModule {
    path: String,
    code: String,
    tokens: TokenStream,
    module_ast: Option<ModulePrototypeAst>,
    error_formatter: Option<ErrorFormatter>,
}

pub struct ProgramModuleTree {
    src_path: String,
    modules: Vec<ProgramModule>,
}
