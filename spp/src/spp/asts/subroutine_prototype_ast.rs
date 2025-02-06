use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_prototype_ast::FunctionPrototypeBaseAst;

#[derive(Clone, Debug)]
pub struct SubroutinePrototypeAst(pub FunctionPrototypeBaseAst);

impl Ast for SubroutinePrototypeAst {
    fn get_pos(&self) -> usize {
        self.0.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.0.get_final_pos()
    }
}
