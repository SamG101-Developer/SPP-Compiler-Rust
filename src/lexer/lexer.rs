use super::token::{TokenStream, TokenType};

pub struct Lexer {
    pub(crate) code: String,
}

impl Lexer {
    pub fn lex(mut self) -> TokenStream {
        let mut tokens = vec![];

        for c in self.code.chars() {
            tokens.push(match c {
                c if 'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' => TokenType::TkCharacter(c),
                c if '0' <= c && c <= '9' => TokenType::TkNumber(c),
                '=' => TokenType::TkEqualsSign,
                '+' => TokenType::TkPlus,
                '-' => TokenType::TkHyphenMinus,
                '*' => TokenType::TkAsterisk,
                '/' => TokenType::TkForwardSlash,
                '%' => TokenType::TkPercent,
                '^' => TokenType::TkCaret,
                '<' => TokenType::TkChevronL,
                '>' => TokenType::TkGreaterThanSign,
                '(' => TokenType::TkLeftParenthesis,
                ')' => TokenType::TkRightParenthesis,
                '[' => TokenType::TkLeftSquareBracket,
                ']' => TokenType::TkRightSquareBracket,
                '{' => TokenType::TkLeftCurlyBrace,
                '}' => TokenType::TkRightCurlyBrace,
                '?' => TokenType::TkQuestionMark,
                ':' => TokenType::TkColon,
                '&' => TokenType::TkAmpersand,
                '|' => TokenType::TkVerticalBar,
                '.' => TokenType::TkDot,
                ',' => TokenType::TkComma,
                '@' => TokenType::TkAt,
                '_' => TokenType::TkUnderscore,
                '"' => TokenType::TkSpeechMark,
                '$' => TokenType::TkDollar,
                '\n' => TokenType::TkNewLine,
                ' ' => continue,
                c => TokenType::TkUnknown(c),
            });
        }

        tokens
    }
}
