use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_parameter_ast::FunctionParameterAst;
use crate::spp::asts::function_parameter_optional_ast::FunctionParameterOptionalAst;
use crate::spp::asts::function_parameter_required_ast::FunctionParameterRequiredAst;
use crate::spp::asts::function_parameter_self_ast::FunctionParameterSelfAst;
use crate::spp::asts::function_parameter_variadic_ast::FunctionParameterVariadicAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug, Default)]
pub struct FunctionParameterGroupAst {
    pub tok_paren_l: TokenAst,
    pub parameters: Vec<FunctionParameterAst>,
    pub tok_paren_r: TokenAst,
}

impl FunctionParameterGroupAst {
    pub fn new(tok_paren_l: TokenAst, parameters: Vec<FunctionParameterAst>, tok_paren_r: TokenAst) -> Self {
        Self { tok_paren_l, parameters, tok_paren_r }
    }
}

impl FunctionParameterGroupAst {
    pub fn get_self(&mut self) -> Option<&mut FunctionParameterSelfAst> {
        for param in &mut self.parameters {
            if let FunctionParameterAst::Self_(param) = param {
                return Some(param);
            }
        }
        None
    }

    pub fn get_required(&self) -> Vec<&FunctionParameterRequiredAst> {
        let mut required_params = Vec::new();
        for param in &self.parameters {
            if let FunctionParameterAst::Required(param) = param {
                required_params.push(param);
            }
        }
        required_params
    }

    pub fn get_optional(&self) -> Vec<&FunctionParameterOptionalAst> {
        let mut optional_params = Vec::new();
        for param in &self.parameters {
            if let FunctionParameterAst::Optional(param) = param {
                optional_params.push(param);
            }
        }
        optional_params
    }

    pub fn get_variadic(&self) -> Option<&FunctionParameterVariadicAst> {
        for param in &self.parameters {
            if let FunctionParameterAst::Variadic(param) = param {
                return Some(param);
            }
        }
        None
    }
}

impl Ast for FunctionParameterGroupAst {
    fn get_pos(&self) -> usize {
        self.tok_paren_l.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_paren_r.get_final_pos()
    }
}
