use crate::spp::asts::annotation_ast::AnnotationAst;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::coroutine_prototype_ast::CoroutinePrototypeAst;
use crate::spp::asts::function_implementation_ast::FunctionImplementationAst;
use crate::spp::asts::function_parameter_group_ast::FunctionParameterGroupAst;
use crate::spp::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::subroutine_prototype_ast::SubroutinePrototypeAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::where_block_ast::WhereBlockAst;

#[derive(Clone, Debug)]
pub struct FunctionPrototypeBaseAst {
    pub pos: usize,
    pub annotations: Vec<AnnotationAst>,
    pub tok_fun: TokenAst,
    pub name: IdentifierAst,
    pub generic_param_group: Option<GenericParameterGroupAst>,
    pub function_param_group: FunctionParameterGroupAst,
    pub tok_arrow: TokenAst,
    pub return_type: TypeAst,
    pub where_block: Option<WhereBlockAst>,
    pub body: FunctionImplementationAst,

    _orig: Option<IdentifierAst>,
    _abstract: Option<AnnotationAst>,
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
        generic_param_group: Option<GenericParameterGroupAst>,
        function_param_group: FunctionParameterGroupAst,
        tok_arrow: TokenAst,
        return_type: TypeAst,
        where_block: Option<WhereBlockAst>,
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

impl Ast for FunctionPrototypeBaseAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.body.get_final_pos()
    }
}

#[derive(Clone, Debug)]
pub enum FunctionPrototypeAst {
    Coroutine(CoroutinePrototypeAst),
    Subroutine(SubroutinePrototypeAst),
}

impl Ast for FunctionPrototypeAst {
    fn get_pos(&self) -> usize {
        match self {
            FunctionPrototypeAst::Coroutine(ast) => ast.get_pos(),
            FunctionPrototypeAst::Subroutine(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            FunctionPrototypeAst::Coroutine(ast) => ast.get_final_pos(),
            FunctionPrototypeAst::Subroutine(ast) => ast.get_final_pos(),
        }
    }
}
