use crate::asts::object_initializer_argument_group_ast::ObjectInitializerArgumentGroupAst;
use crate::asts::type_ast::TypeAst;

pub struct ObjectInitializerAst {
    pub pos: usize,
    pub type_: TypeAst,
    pub object_args_group: ObjectInitializerArgumentGroupAst,
}

impl ObjectInitializerAst {
    pub fn new(
        pos: usize,
        type_: TypeAst,
        object_args_group: ObjectInitializerArgumentGroupAst,
    ) -> Self {
        Self {
            pos,
            type_,
            object_args_group,
        }
    }
}
