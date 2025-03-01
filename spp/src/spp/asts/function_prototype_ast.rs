use crate::spp::analyse::scopes::scope::Scope;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::annotation_ast::AnnotationAst;
use crate::spp::asts::ast::{Ast, PreProcessingContext};
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
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;

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

impl FunctionPrototypeBaseAst {
    // fn deduce_mock_class_type(&self) -> TypeAst {
    //     // Module level functions are always FunRef.
    //     if self._is_free || self.function_param_group.get_self_param().is_none() {
    //         return CommonTypes::function_ref(CommonTypes::tuple(self.function_param_group.param_types(), self.pos), self.return_type.clone(), self.pos);
    //     }
    //
    //     // Determine the method function type based off the self-parameter convention.
    //     match self.function_param_group.get_self_param().unwrap().get_convention() {
    //         ConventionAst::Mov { .. } => {
    //             CommonTypes::function_mov(CommonTypes::tuple(self.function_param_group.param_types(), self.pos), self.return_type.clone(), self.pos)
    //         }
    //         ConventionAst::Mut { .. } => {
    //             CommonTypes::function_mut(CommonTypes::tuple(self.function_param_group.param_types(), self.pos), self.return_type.clone(), self.pos)
    //         }
    //         ConventionAst::Ref { .. } => {
    //             CommonTypes::function_ref(CommonTypes::tuple(self.function_param_group.param_types(), self.pos), self.return_type.clone(), self.pos)
    //         }
    //     }
    // }
    //
    // fn deduce_mock_class_call(&self, function_type: &TypeAst) -> IdentifierAst {
    //     IdentifierAst::new(function_type.get_pos(), format!("call_{}", IdentifierAst::from(function_type).value))
    // }
}

impl Ast for FunctionPrototypeBaseAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.body.get_final_pos()
    }

    fn stage_1_preprocess(&mut self, context: PreProcessingContext) -> Result<(), SemanticError> {
        Ok(())
        // for mut a in self.borrow_mut().annotations {
        //     a.stage_1_preprocess(context.clone())?;
        // }
        //
        // // Convert the "fun" to a "sup" of a "Fun[Mov|Mut|Ref]" type over a mock type.
        // let mock_class_name = TypeAst::from_function_identifier(self.borrow_mut().name.clone());
        // let function_type = self.borrow().deduce_mock_class_type();
        // let function_call = self.borrow().deduce_mock_class_call(&function_type);
        //
        // // If this is the first appearance of a function with this name, make the class for it.
        // // if context.
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
