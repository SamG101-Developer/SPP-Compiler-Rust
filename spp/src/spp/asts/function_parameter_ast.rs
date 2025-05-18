use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_parameter_optional_ast::FunctionParameterOptionalAst;
use crate::spp::asts::function_parameter_required_ast::FunctionParameterRequiredAst;
use crate::spp::asts::function_parameter_self_ast::FunctionParameterSelfAst;
use crate::spp::asts::function_parameter_variadic_ast::FunctionParameterVariadicAst;
use crate::spp::asts::type_ast::TypeAst;


#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum FunctionParameterAst {
    Self_(FunctionParameterSelfAst),
    Required(FunctionParameterRequiredAst),
    Optional(FunctionParameterOptionalAst),
    Variadic(FunctionParameterVariadicAst),
}

impl FunctionParameterAst {
    pub fn get_type(&mut self) -> &mut TypeAst {
        match self {
            FunctionParameterAst::Self_(param) => &mut param.type_,
            FunctionParameterAst::Required(param) => &mut param.type_,
            FunctionParameterAst::Optional(param) => &mut param.type_,
            FunctionParameterAst::Variadic(param) => &mut param.type_,
        }
    }
}
