use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_parameter_ast::FunctionParameterAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct FunctionParameterGroupAst {
    pub pos: usize,
    pub tok_paren_l: TokenAst,
    pub parameters: Vec<FunctionParameterAst>,
    pub tok_paren_r: TokenAst,
}

impl FunctionParameterGroupAst {
    // pub fn get_self_param(&self) -> Option<&FunctionParameterAst> {
    //     let self_params = self
    //         .parameters
    //         .iter()
    //         .filter(|param| match param {
    //             FunctionParameterAst::Self_ { .. } => true,
    //             _ => false,
    //         })
    //         .collect::<Vec<&FunctionParameterAst>>();
    //
    //     if self_params.len() == 1 {
    //         Some(self_params[0])
    //     } else {
    //         None
    //     }
    // }
    //
    // pub fn get_required_params(&self) -> &[&FunctionParameterAst] {
    //     let required_params = self.parameters.iter().filter(|param| match param {
    //         FunctionParameterAst::Required { .. } => true,
    //         _ => false,
    //     });
    //     required_params
    //         .collect::<Vec<&FunctionParameterAst>>()
    //         .as_slice()
    // }
    //
    // pub fn get_optional_params(&self) -> &[&FunctionParameterAst] {
    //     let optional_params = self.parameters.iter().filter(|param| match param {
    //         FunctionParameterAst::Optional { .. } => true,
    //         _ => false,
    //     });
    //     optional_params
    //         .collect::<Vec<&FunctionParameterAst>>()
    //         .as_slice()
    // }
    //
    // pub fn get_variadic_param(&self) -> Option<FunctionParameterAst> {
    //     let variadic_params = self.parameters.iter().filter(|param| match param {
    //         FunctionParameterAst::Variadic { .. } => true,
    //         _ => false,
    //     });
    //
    //     let variadic_params = variadic_params.collect::<Vec<&FunctionParameterAst>>();
    //     if variadic_params.len() == 1 {
    //         Some(variadic_params[0].clone())
    //     } else {
    //         None
    //     }
    // }

    pub fn param_types(&self) -> Vec<TypeAst> {
        self.parameters.iter().map(|param| match param {
            FunctionParameterAst::Self_ { _type, .. } => _type.clone(),
            FunctionParameterAst::Required { type_, .. } => type_.clone(),
            FunctionParameterAst::Optional { type_, .. } => type_.clone(),
            FunctionParameterAst::Variadic { type_, .. } => type_.clone(),
        }).collect()
    }
}

impl FunctionParameterGroupAst {
    pub fn new(
        pos: usize,
        tok_paren_l: TokenAst,
        parameters: Vec<FunctionParameterAst>,
        tok_paren_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_paren_l,
            parameters,
            tok_paren_r,
        }
    }
}

impl Ast for FunctionParameterGroupAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_paren_r.get_final_pos()
    }
}
