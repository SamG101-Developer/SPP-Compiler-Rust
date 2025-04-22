use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::local_variable_attribute_binding_ast::LocalVariableAttributeBindingAst;
use crate::spp::asts::local_variable_destructure_array_ast::LocalVariableDestructureArrayAst;
use crate::spp::asts::local_variable_destructure_object_ast::LocalVariableDestructureObjectAst;
use crate::spp::asts::local_variable_destructure_skip_1_argument_ast::LocalVariableDestructureSkip1ArgumentAst;
use crate::spp::asts::local_variable_destructure_skip_n_arguments_ast::LocalVariableDestructureSkipNArgumentsAst;
use crate::spp::asts::local_variable_destructure_tuple_ast::LocalVariableDestructureTupleAst;
use crate::spp::asts::local_variable_single_identifier_ast::LocalVariableSingleIdentifierAst;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum LocalVariableAst {
    DestructureArray(LocalVariableDestructureArrayAst),
    DestructureTuple(LocalVariableDestructureTupleAst),
    DestructureObject(LocalVariableDestructureObjectAst),
    SingleIdentifier(LocalVariableSingleIdentifierAst),
}

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum LocalVariableNestedForDestructureArrayAst {
    DestructureArray(LocalVariableDestructureArrayAst),
    DestructureTuple(LocalVariableDestructureTupleAst),
    DestructureObject(LocalVariableDestructureObjectAst),
    SingleIdentifier(LocalVariableSingleIdentifierAst),
    Skip1Args(LocalVariableDestructureSkip1ArgumentAst),
    SkipNArgs(LocalVariableDestructureSkipNArgumentsAst),
}

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum LocalVariableNestedForDestructureTupleAst {
    DestructureArray(LocalVariableDestructureArrayAst),
    DestructureTuple(LocalVariableDestructureTupleAst),
    DestructureObject(LocalVariableDestructureObjectAst),
    SingleIdentifier(LocalVariableSingleIdentifierAst),
    Skip1Args(LocalVariableDestructureSkip1ArgumentAst),
    SkipNArgs(LocalVariableDestructureSkipNArgumentsAst),
}

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum LocalVariableNestedForDestructureObjectAst {
    AttrBind(LocalVariableAttributeBindingAst),
    SkipNArgs(LocalVariableDestructureSkipNArgumentsAst),
    SingleIdentifier(LocalVariableSingleIdentifierAst),
}

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum LocalVariableNestedForAttributeBindingAst {
    DestructureArray(LocalVariableDestructureArrayAst),
    DestructureTuple(LocalVariableDestructureTupleAst),
    DestructureObject(LocalVariableDestructureObjectAst),
    SingleIdentifier(LocalVariableSingleIdentifierAst),
}
