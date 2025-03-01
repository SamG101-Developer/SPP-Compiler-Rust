use crate::spp::analyse::utilities::common_types::CommonTypes;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_identifier_ast::GenericIdentifierAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::type_array_ast::TypeArrayAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::type_binary_expression_ast::TypeBinaryExpressionAst;
use crate::spp::asts::type_parenthesized_expression_ast::TypeParenthesizedExpressionAst;
use crate::spp::asts::type_tuple_ast::TypeTupleAst;
use crate::spp::lexer::token::TokenAstTokenType;

#[derive(Debug, Clone)]
pub struct TypeSingleAst {
    pub pos: usize,
    pub name: GenericIdentifierAst,
}

impl TypeSingleAst {
    pub fn new(pos: usize, name: GenericIdentifierAst) -> Self {
        Self { pos, name }
    }
}

impl From<GenericIdentifierAst> for TypeAst {
    fn from(name: GenericIdentifierAst) -> Self {
        TypeAst::Single(TypeSingleAst::new(name.get_pos(), name))
    }
}

impl From<IdentifierAst> for TypeAst {
    fn from(name: IdentifierAst) -> Self {
        TypeAst::Single(TypeSingleAst::new(name.get_pos(), GenericIdentifierAst::from(name)))
    }
}

impl From<&IdentifierAst> for TypeAst {
    fn from(name: &IdentifierAst) -> Self {
        TypeAst::Single(TypeSingleAst::new(name.get_pos(), GenericIdentifierAst::from(name)))
    }
}

impl From<TypeParenthesizedExpressionAst> for TypeAst {
    fn from(parenthesized_expression: TypeParenthesizedExpressionAst) -> TypeAst {
        *parenthesized_expression.expression
    }
}

impl From<TypeTupleAst> for TypeAst {
    fn from(tuple: TypeTupleAst) -> TypeAst {
        CommonTypes::tuple(tuple.type_list, tuple.pos)
    }
}

impl From<TypeArrayAst> for TypeAst {
    fn from(array: TypeArrayAst) -> TypeAst {
        CommonTypes::array(array.type_, array.size, array.pos)
    }
}

impl From<TypeBinaryExpressionAst> for TypeAst {
    fn from(binary_expression: TypeBinaryExpressionAst) -> TypeAst {
        match binary_expression.op.token_type {
            TokenAstTokenType::KwAnd => CommonTypes::intersection(
                Vec::from([*binary_expression.left, *binary_expression.right]),
                binary_expression.pos,
            ),
            TokenAstTokenType::KwOr => CommonTypes::variant(
                Vec::from([*binary_expression.left, *binary_expression.right]),
                binary_expression.pos,
            ),
            _ => panic!("Invalid binary expression operator"),
        }
    }
}

impl Ast for TypeSingleAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.name.get_final_pos()
    }
}
