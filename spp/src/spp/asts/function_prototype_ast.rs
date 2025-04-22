use crate::spp::analyse::scopes::scope::Scope;
use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::annotation_ast::AnnotationAst;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::coroutine_prototype_ast::CoroutinePrototypeAst;
use crate::spp::asts::function_implementation_ast::FunctionImplementationAst;
use crate::spp::asts::function_parameter_group_ast::FunctionParameterGroupAst;
use crate::spp::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::subroutine_prototype_ast::SubroutinePrototypeAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::where_block_ast::WhereBlockAst;
use enum_dispatch::enum_dispatch;
use std::cell::RefCell;
use std::rc::Rc;

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

    _is_free: bool,
    _scope: Option<Rc<RefCell<Scope>>>,
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
        is_free: bool,
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
            _is_free: is_free,
            _scope: None,
        }
    }
}

impl Ast for FunctionPrototypeBaseAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.return_type.get_final_pos()
    }
}

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum FunctionPrototypeAst {
    Cor(CoroutinePrototypeAst),
    Sub(SubroutinePrototypeAst),
}
