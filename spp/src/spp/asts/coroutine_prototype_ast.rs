use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_prototype_ast::FunctionPrototypeBaseAst;

#[derive(Clone, Debug, Default)]
pub struct CoroutinePrototypeAst(pub FunctionPrototypeBaseAst);

impl Ast for CoroutinePrototypeAst {
    fn get_pos(&self) -> usize {
        self.0.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.0.get_final_pos()
    }
}
