use crate::lexer::token::TokenType;

pub struct ClassPrototypeAst {
    pub pos: usize,
    pub annotations: Vec<AnnotationAst>,
    pub tok_cls: TokenAst,
    pub name: IdentifierAst,
    pub generic_param_group: GenericParameterGroupAst,
    pub where_block: WhereBlockAst,
    pub body: ClassImplementationAst,
    pub is_alias: bool,
}

impl ClassPrototypeAst {
    pub(crate) fn new(
        pos: usize,
        annotations: Vec<AnnotationAst>,
        tok_cls: TokenAst,
        name: IdentifierAst,
        generic_param_group: GenericParameterGroupAst,
        where_block: WhereBlockAst,
        body: ClassImplementationAst,
    ) -> Self {
        Self {
            pos,
            annotations,
            tok_cls,
            name,
            generic_param_group,
            where_block,
            body,
            is_alias: false,
        }
    }
}
