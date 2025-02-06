use crate::spp::asts::ast::Ast;
use crate::spp::asts::literal_ast::LiteralAst;

#[derive(Clone, Debug)]
pub enum PatternVariantLiteralAst {
    Float(LiteralAst),
    Integer(LiteralAst),
    String(LiteralAst),
    Boolean(LiteralAst),
}

impl Ast for PatternVariantLiteralAst {
    fn get_pos(&self) -> usize {
        match self {
            PatternVariantLiteralAst::Float(literal) => literal.get_pos(),
            PatternVariantLiteralAst::Integer(literal) => literal.get_pos(),
            PatternVariantLiteralAst::String(literal) => literal.get_pos(),
            PatternVariantLiteralAst::Boolean(literal) => literal.get_pos(),
        }
    }
}
