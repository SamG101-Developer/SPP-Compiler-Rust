use crate::spp::asts::ast::Ast;
use crate::spp::asts::class_attribute_ast::ClassAttributeAst;

#[derive(Clone, Debug)]
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
