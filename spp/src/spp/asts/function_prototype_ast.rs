use crate::spp::analyse::scopes::scope::Scope;
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
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
pub struct FunctionPrototypeBaseAst {
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
            annotations: Vec<AnnotationAst>, tok_fun: TokenAst, name: IdentifierAst,
            generic_param_group: Option<GenericParameterGroupAst>, function_param_group:
            FunctionParameterGroupAst, tok_arrow: TokenAst, return_type: TypeAst,
            where_block: Option<WhereBlockAst>, body: FunctionImplementationAst,
            is_free: bool) -> Self {
        
        Self {
            annotations, tok_fun, name, generic_param_group, function_param_group, tok_arrow, return_type, where_block, body,
            _orig: None, _abstract: None, _virtual: None, _non_implemented: None, _cold: None, _hot: None, _is_free: is_free, _scope: None }
    }
}

impl Ast for FunctionPrototypeBaseAst {
    fn get_pos(&self) -> usize {
        self.annotations.first().map_or(self.tok_fun.get_pos(), |a| a.get_pos())
    }

    fn get_final_pos(&self) -> usize {
        self.return_type.get_final_pos()
    }

    // fn stage_1_preprocess_asts<T: Any + Clone>(&mut self, meta_data: &mut HashMap<String, Box<T>>) -> Result<(), SemanticError> {
    //
    //     // Substitute the "Self" parameter's type with the name of the method.
    //     if self.function_param_group.get_self().is_some() {
    //         if meta_data["ctx_type"].downcast::<String>() == "extension" {
    //             let ctx_name = meta_data["ctx_name"].downcast::<TypeAst>().clone();
    //             let gen_self_sub = GenericArgumentTypeNamedAst { name: CommonTypes::self_(0), value: ctx_name.clone(), ..GenericArgumentTypeNamedAst::default() };
    //             let gen_self_sub = vec![GenericArgumentAst::TypeNamed(gen_self_sub)];
    //
    //             self.function_param_group.get_self().unwrap().set_true_self_type_from_context(ctx_name);
    //             self.function_param_group.get_self().unwrap().type_ = self.function_param_group.get_self().unwrap().type_.sub_generics(gen_self_sub.clone());
    //             for mut p in self.function_param_group.parameters {
    //                 *p.get_type() = p.get_type().sub_generics(gen_self_sub.clone());
    //             }
    //             self.return_type = self.return_type.sub_generics(gen_self_sub);
    //         }
    //     }
    //
    //     // Pre-process the annotations.
    //     for mut a in self.annotations {
    //         a.stage_1_preprocess_asts(meta_data)?;
    //     }
    //
    //     // Convert the "fun" function to a "sup" superimposition of a "Fun[Mov|Mut|Ref]" type over a mock type.
    //     let mock_class_name = TypeAst::from(self.name.as_function_identifier());
    //     let function_type = self._deduce_function_class_type();
    //     let function_call = IdentifierAst::new(self.name.pos, "call".to_string());
    //
    //     // If this is the first overload being converted, then the class needs to be made for the type.
    //     if !meta_data["class_list"].downcast::<Vec<TypeAst>>().contains(&function_type) {
    //         let mock_class_ast = ClassPrototypeAst{ name: mock_class_name.clone(), ..ClassPrototypeAst::default() };
    //         let mock_constant_ast = CmpStatementAst{ name: self.name.clone(), type_: mock_class_name, value: ObjectInitializerAst{ type_: mock_class_name, ..ObjectInitializerAst::default() }, ..CmpStatementAst::default() };
    //     }
    // }
}

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum FunctionPrototypeAst {
    Cor(CoroutinePrototypeAst),
    Sub(SubroutinePrototypeAst),
}
