use crate::asts::coroutine_prototype_ast::CoroutinePrototypeAst;
use crate::asts::subroutine_prototype_ast::SubroutinePrototypeAst;

pub struct FunctionPrototypeBaseAst {
    pub pos: usize,
    pub annotations: Vec<AnnotationAst>,
    pub tok_fun: TokenAst,
    pub name: IdentifierAst,
    pub generic_param_group: GenericParameterGroupAst,
    pub function_param_group: FunctionParameterGroupAst,
    pub tok_arrow: TokenAst,
    pub return_type: TypeAst,
    pub where_block: WhereBlockAst,
    pub body: FunctionImplementationAst,

    _orig: Option<IdentifierAst>,
    _abstract: Optional<AnnotationAst>,
    _virtual: Option<AnnotationAst>,
    _non_implemented: Option<AnnotationAst>,
    _cold: Option<AnnotationAst>,
    _hot: Option<AnnotationAst>,
}

impl FunctionPrototypeBaseAst {
    pub fn new(
        pos: usize,
        annotations: Vec<AnnotationAst>,
        tok_fun: TokenAst,
        name: IdentifierAst,
        generic_param_group: GenericParameterGroupAst,
        function_param_group: FunctionParameterGroupAst,
        tok_arrow: TokenAst,
        return_type: TypeAst,
        where_block: WhereBlockAst,
        body: FunctionImplementationAst,
    ) -> Self {
        Self {
            pos,
            annotations,
            tok_fun,
            name,
            generic_param_group,
            function_param_group,
            tok_arrow,
            return_type,
            where_block,
            body,

            _orig: None,
            _abstract: None,
            _virtual: None,
            _non_implemented: None,
            _cold: None,
            _hot: None,
        }
    }
}


pub enum FunctionPrototypeAst {
    Coroutine(CoroutinePrototypeAst),
    Subroutine(SubroutinePrototypeAst),
}
