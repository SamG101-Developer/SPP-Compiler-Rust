pub enum GenericArgumentAst {
    CompNamed(GenericCompNamedArgumentAst),
    CompUnnamed(GenericCompUnnamedArgumentAst),
    TypeNamed(GenericTypeNamedArgumentAst),
    TypeUnnamed(GenericTypeUnnamedArgumentAst),
}
