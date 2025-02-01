use std::iter::Map;

pub enum TokenType {
    TkCharacter(char),
    TkNumber(char),
    TkEqualsSign,
    TkPlus,
    TkHyphenMinus,
    TkAsterisk,
    TkForwardSlash,
    TkPercent,
    TkCaret,
    TkChevronL,
    TkGreaterThanSign,
    TkLeftParenthesis,
    TkRightParenthesis,
    TkLeftSquareBracket,
    TkRightSquareBracket,
    TkLeftCurlyBrace,
    TkRightCurlyBrace,
    TkQuestionMark,
    TkColon,
    TkAmpersand,
    TkVerticalBar,
    TkDot,
    TkComma,
    TkAt,
    TkUnderscore,
    TkSpeechMark,
    TkWhitespace,
    TkNewLine,
    TkDollar,
    TkUnknown(char),
    NoToken,
    Keyword,
}

pub enum Keywords {
    Cls,
    Fun,
    Cor,
    Sup,
    Ext,
    Mut,
    Cmp,
    Where,
    SelfVal_,
    Case,
    Of,
}

pub static KEYWORD_STRINGS: Map<Keywords, &str> = [
    (Keywords::Cls, "cls"),
    (Keywords::Fun, "fun"),
    (Keywords::Cor, "cor"),
    (Keywords::Sup, "sup"),
    (Keywords::Ext, "ext"),
    (Keywords::Mut, "mut"),
    (Keywords::Cmp, "cmp"),
    (Keywords::Where, "where"),
    (Keywords::SelfVal_, "self"),
    (Keywords::Case, "case"),
    (Keywords::Of, "of"),
]
.iter()
.cloned()
.collect();

pub type TokenStream = Vec<TokenType>;
