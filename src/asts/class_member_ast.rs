use crate::asts::ast::Ast;
use crate::asts::class_attribute_ast::ClassAttributeAst;

pub enum ClassMemberAst {
    Attr(ClassAttributeAst),
}

impl Ast for ClassMemberAst {
    fn get_pos(&self) -> usize {
        match self {
            ClassMemberAst::Attr(attr) => attr.get_pos(),
        }
    }
}
