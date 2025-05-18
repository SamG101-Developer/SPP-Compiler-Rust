use crate::spp::asts::ast::Ast;
use crate::spp::asts::object_initializer_argument_group_ast::ObjectInitializerArgumentGroupAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug, Default)]
pub struct ObjectInitializerAst {
    pub type_: TypeAst,
    pub object_args_group: ObjectInitializerArgumentGroupAst,
}

impl ObjectInitializerAst {
    pub fn new(type_: TypeAst, object_args_group: ObjectInitializerArgumentGroupAst) -> Self {
        Self { type_, object_args_group }
    }
}

impl Ast for ObjectInitializerAst {
    fn get_pos(&self) -> usize {
        self.type_.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.object_args_group.get_final_pos()
    }
}
