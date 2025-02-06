use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

pub trait Ast {
    fn get_pos(&self) -> usize;

    fn get_final_pos(&self) -> usize;

    fn get_size(&self) -> usize {
        self.get_final_pos() - self.get_pos()
    }

    /// Preprocessor stage of the compiler.
    ///
    /// The preprocessor stage performs mutations on ASTs, introduces new ASTs, and removes some
    /// ASTs. This allows for single-method processing of multiple ASTs, such as functions vs types
    /// with function classes superimposed over them. This stage directly affects what symbols are
    /// generated.
    fn stage_1_preprocess(&mut self, context: &mut dyn Ast) -> Result<(), SemanticError> {
        Ok(())
    }

    /// Top level scope generator stage of the compiler.
    ///
    /// The generate top-level scopes stage generates all module and superimposition level scopes
    /// and symbols. This includes classes, attributes, functions, sup-methods, aliases and global
    /// constants. No generation is done for symbols inside functions. The symbols are generated
    /// here so that they can be used in any module, allowing for circular imports.
    fn stage_2_generate_top_level_scopes(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    /// Top level alias generator stage of the compiler.
    ///
    /// The generate top-level aliases stage generates all aliases at the module/sup level. This
    /// must come after the symbol generation stage, as it requires symbol knowledge to attach the
    /// correct "old types". It must also come before the load sup scopes stage, because
    /// superimposing over aliases requires the alias to exist beforehand, in any order of
    /// compilation.
    fn stage_3_generate_top_level_aliases(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    /// Super scope loader stage of the compiler.
    ///
    /// The load super scopes stage links all super scopes to classes. This allows a type to know
    /// what attributes and methods are on its superclasses, and is requires for symbol resolution.
    fn stage_4_load_super_scopes(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    /// Super scope post-processor stage of the compiler.
    ///
    /// The postprocess super scopes stage performs checks that must happen after the super scopes
    /// have been injected, but that are separate from the next stage (type-regeneration). This
    /// includes things that require knowledge of all the super scopes.
    fn stage_5_postprocess_super_scopes(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    /// Generic alias regenerator stage of the compiler.
    ///
    /// The regenerate generic aliases stage is the generic type regeneration stage exclusive to
    /// type-aliases. This is to ensure the aliases' old type has been generically substituted
    /// correctly, and is required before the rest of the regular type's generic substitution is
    /// regenerated. This is because regular type regeneration may rely on aliased types.
    fn stage_6_regenerate_generic_aliases(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    /// Generic type regenerator stage of the compiler.
    ///
    /// The regenerate generic types stage takes all the pruned generic types, and regenerated them
    /// with full knowledge of substitutions and inference. This is required as the generic types
    /// were placeholders earlier in the compilation stages.
    fn stage_7_regenerate_generic_types(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    /// Semantic analyser stage of the compiler.
    ///
    /// The semantic analysis stage is the most complex, and final analysis, stage of the semantic
    /// pipeline. This stage performs all the semantic checks, type inference, and type checking.
    /// This stage requires all symbols to be generated, and all types to be aliased, loaded, and
    /// post-processed. All functions scopes are inspected (function contents).
    fn stage_8_analyse_semantics(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }
}

pub trait ToBinaryExpression {
    fn to_binary_expression(
        pos: usize,
        lhs: ExpressionAst,
        op: TokenAst,
        rhs: Self,
    ) -> ExpressionAst;
}
