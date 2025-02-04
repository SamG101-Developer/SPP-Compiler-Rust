use crate::asts::ast::Ast;
use crate::asts::function_prototype_ast::FunctionPrototypeBaseAst;

#[derive(Clone)]
pub struct CoroutinePrototypeAst(pub FunctionPrototypeBaseAst);

impl Ast for CoroutinePrototypeAst {
    fn get_pos(&self) -> usize {
        self.0.get_pos()
    }
}
