use super::token::{TokenStream, TokenType};

pub struct Lexer {
    pub code: String,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer { code }
    }

    pub fn lex(self) -> TokenStream {
        let mut tokens = vec![];
        let mut in_string = false;

        for c in self.code.chars() {
            if in_string && c != '"' {
                tokens.push(TokenType::TkCharacter(c));
                continue;
            }

            tokens.push(match c {
                c if 'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' => TokenType::TkCharacter(c),
                c if '0' <= c && c <= '9' => TokenType::TkNumber(c),
                '=' => TokenType::TkEqualsSign,
                '+' => TokenType::TkPlusSign,
                '-' => TokenType::TkMinusSign,
                '*' => TokenType::TkAsterisk,
                '/' => TokenType::TkForwardSlash,
                '%' => TokenType::TkPercentSign,
                '^' => TokenType::TkCaret,
                '<' => TokenType::TkLessThanSign,
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
                '"' => {
                    in_string = !in_string;
                    TokenType::TkSpeechMark
                }
                '$' => TokenType::TkDollar,
                ' ' => TokenType::TkWhitespace,
                '\n' => TokenType::TkNewLine,
                '\r' => continue,
                c => TokenType::TkUnknown(c),
            });
        }

        tokens.push(TokenType::EndOfFile);
        tokens
    }
}
