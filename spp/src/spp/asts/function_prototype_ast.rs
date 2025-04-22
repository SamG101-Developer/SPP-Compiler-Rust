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
use std::cell::RefCell;
use std::rc::Rc;
use crate::spp::analyse::utilities::common_types::CommonTypes;
use crate::spp::asts::generic_argument_type_named_ast::GenericArgumentTypeNamedAst;

#[derive(Clone, Debug, Default)]
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
    _ctx: Option<PreProcessingContext>,
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
            _ctx: None,
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

    // fn stage_1_preprocess_asts(&mut self, context: PreProcessingContext) -> Result<(), SemanticError> {
    //     self._ctx = Some(context.clone());
    // 
    //     if self.function_param_group.get_self().is_some() {
    //         if let PreProcessingContext::Ext(ext_block) = context {
    //             let ctx_name = ext_block.borrow().name.clone();
    //             let gen_self_sub = GenericArgumentTypeNamedAst { name: CommonTypes::self_(0), value: ctx_name.clone(), ..GenericArgumentTypeNamedAst::default() };
    //             let gen_self_sub = vec![gen_self_sub];
    // 
    //             self.function_param_group.get_self().unwrap().set_true_self_type_from_context(ctx_name);
    //             self.function_param_group.get_self().unwrap().type_ = self.function_param_group.get_self().unwrap().type_.sub_generics(gen_self_sub);
    //             for p in self.function_param_group.parameters {
    //                 
    //             }
    //         }
    //     }
    // 
    // 
    // }
}

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum FunctionPrototypeAst {
    Cor(CoroutinePrototypeAst),
    Sub(SubroutinePrototypeAst),
}
