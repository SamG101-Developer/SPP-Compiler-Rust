use crate::spp::asts::ast::Ast;
use crate::spp::asts::local_variable_attribute_binding_ast::LocalVariableAttributeBindingAst;
use crate::spp::asts::local_variable_destructure_array_ast::LocalVariableDestructureArrayAst;
use crate::spp::asts::local_variable_destructure_object_ast::LocalVariableDestructureObjectAst;
use crate::spp::asts::local_variable_destructure_skip_1_argument_ast::LocalVariableDestructureSkip1ArgumentAst;
use crate::spp::asts::local_variable_destructure_skip_n_arguments_ast::LocalVariableDestructureSkipNArgumentsAst;
use crate::spp::asts::local_variable_destructure_tuple_ast::LocalVariableDestructureTupleAst;
use crate::spp::asts::local_variable_single_identifier_ast::LocalVariableSingleIdentifierAst;

#[derive(Clone, Debug)]
pub enum LocalVariableAst {
    DestructureArray(LocalVariableDestructureArrayAst),
    DestructureTuple(LocalVariableDestructureTupleAst),
    DestructureObject(LocalVariableDestructureObjectAst),
    SingleIdentifier(LocalVariableSingleIdentifierAst),
}

#[derive(Clone, Debug)]
pub enum LocalVariableNestedForDestructureArrayAst {
    DestructureArray(LocalVariableDestructureArrayAst),
    DestructureTuple(LocalVariableDestructureTupleAst),
    DestructureObject(LocalVariableDestructureObjectAst),
    SingleIdentifier(LocalVariableSingleIdentifierAst),
    Skip1Args(LocalVariableDestructureSkip1ArgumentAst),
    SkipNArgs(LocalVariableDestructureSkipNArgumentsAst),
}

#[derive(Clone, Debug)]
pub enum LocalVariableNestedForDestructureTupleAst {
    DestructureArray(LocalVariableDestructureArrayAst),
    DestructureTuple(LocalVariableDestructureTupleAst),
    DestructureObject(LocalVariableDestructureObjectAst),
    SingleIdentifier(LocalVariableSingleIdentifierAst),
    Skip1Args(LocalVariableDestructureSkip1ArgumentAst),
    SkipNArgs(LocalVariableDestructureSkipNArgumentsAst),
}

#[derive(Clone, Debug)]
pub enum LocalVariableNestedForDestructureObjectAst {
    AttrBind(LocalVariableAttributeBindingAst),
    SkipNArgs(LocalVariableDestructureSkipNArgumentsAst),
    SingleIdentifier(LocalVariableSingleIdentifierAst),
}

#[derive(Clone, Debug)]
pub enum LocalVariableNestedForAttributeBindingAst {
    DestructureArray(LocalVariableDestructureArrayAst),
    DestructureTuple(LocalVariableDestructureTupleAst),
    DestructureObject(LocalVariableDestructureObjectAst),
    SingleIdentifier(LocalVariableSingleIdentifierAst),
}

impl Ast for LocalVariableAst {
    fn get_pos(&self) -> usize {
        match self {
            LocalVariableAst::DestructureArray(ast) => ast.get_pos(),
            LocalVariableAst::DestructureTuple(ast) => ast.get_pos(),
            LocalVariableAst::DestructureObject(ast) => ast.get_pos(),
            LocalVariableAst::SingleIdentifier(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            LocalVariableAst::DestructureArray(ast) => ast.get_final_pos(),
            LocalVariableAst::DestructureTuple(ast) => ast.get_final_pos(),
            LocalVariableAst::DestructureObject(ast) => ast.get_final_pos(),
            LocalVariableAst::SingleIdentifier(ast) => ast.get_final_pos(),
        }
    }
}

impl Ast for LocalVariableNestedForDestructureArrayAst {
    fn get_pos(&self) -> usize {
        match self {
            LocalVariableNestedForDestructureArrayAst::DestructureArray(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureArrayAst::DestructureTuple(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureArrayAst::DestructureObject(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureArrayAst::SingleIdentifier(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureArrayAst::Skip1Args(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureArrayAst::SkipNArgs(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            LocalVariableNestedForDestructureArrayAst::DestructureArray(ast) => ast.get_final_pos(),
            LocalVariableNestedForDestructureArrayAst::DestructureTuple(ast) => ast.get_final_pos(),
            LocalVariableNestedForDestructureArrayAst::DestructureObject(ast) => {
                ast.get_final_pos()
            }
            LocalVariableNestedForDestructureArrayAst::SingleIdentifier(ast) => ast.get_final_pos(),
            LocalVariableNestedForDestructureArrayAst::Skip1Args(ast) => ast.get_final_pos(),
            LocalVariableNestedForDestructureArrayAst::SkipNArgs(ast) => ast.get_final_pos(),
        }
    }
}

impl Ast for LocalVariableNestedForDestructureTupleAst {
    fn get_pos(&self) -> usize {
        match self {
            LocalVariableNestedForDestructureTupleAst::DestructureArray(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureTupleAst::DestructureTuple(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureTupleAst::DestructureObject(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureTupleAst::SingleIdentifier(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureTupleAst::Skip1Args(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureTupleAst::SkipNArgs(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            LocalVariableNestedForDestructureTupleAst::DestructureArray(ast) => ast.get_final_pos(),
            LocalVariableNestedForDestructureTupleAst::DestructureTuple(ast) => ast.get_final_pos(),
            LocalVariableNestedForDestructureTupleAst::DestructureObject(ast) => {
                ast.get_final_pos()
            }
            LocalVariableNestedForDestructureTupleAst::SingleIdentifier(ast) => ast.get_final_pos(),
            LocalVariableNestedForDestructureTupleAst::Skip1Args(ast) => ast.get_final_pos(),
            LocalVariableNestedForDestructureTupleAst::SkipNArgs(ast) => ast.get_final_pos(),
        }
    }
}

impl Ast for LocalVariableNestedForDestructureObjectAst {
    fn get_pos(&self) -> usize {
        match self {
            LocalVariableNestedForDestructureObjectAst::AttrBind(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureObjectAst::SkipNArgs(ast) => ast.get_pos(),
            LocalVariableNestedForDestructureObjectAst::SingleIdentifier(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            LocalVariableNestedForDestructureObjectAst::AttrBind(ast) => ast.get_final_pos(),
            LocalVariableNestedForDestructureObjectAst::SkipNArgs(ast) => ast.get_final_pos(),
            LocalVariableNestedForDestructureObjectAst::SingleIdentifier(ast) => {
                ast.get_final_pos()
            }
        }
    }
}

impl Ast for LocalVariableNestedForAttributeBindingAst {
    fn get_pos(&self) -> usize {
        match self {
            LocalVariableNestedForAttributeBindingAst::DestructureArray(ast) => ast.get_pos(),
            LocalVariableNestedForAttributeBindingAst::DestructureTuple(ast) => ast.get_pos(),
            LocalVariableNestedForAttributeBindingAst::DestructureObject(ast) => ast.get_pos(),
            LocalVariableNestedForAttributeBindingAst::SingleIdentifier(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            LocalVariableNestedForAttributeBindingAst::DestructureArray(ast) => ast.get_final_pos(),
            LocalVariableNestedForAttributeBindingAst::DestructureTuple(ast) => ast.get_final_pos(),
            LocalVariableNestedForAttributeBindingAst::DestructureObject(ast) => {
                ast.get_final_pos()
            }
            LocalVariableNestedForAttributeBindingAst::SingleIdentifier(ast) => ast.get_final_pos(),
        }
    }
}
