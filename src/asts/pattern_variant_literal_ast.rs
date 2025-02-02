use crate::asts::literal_ast::LiteralAst;

pub enum PatternVariantLiteralAst {
    Float(LiteralAst),
    Integer(LiteralAst),
    String(LiteralAst),
    Boolean(LiteralAst),
}
