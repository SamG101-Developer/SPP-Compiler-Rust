use colored::{Colorize};
use crate::spp::asts::ast::Ast;
use crate::spp::lexer::token::{TokenStream, TokenType};

const BAR_CHAR: char = '|';

#[derive(Clone)]
pub struct ErrorFormatter {
    file_path: String,
    tokens: TokenStream,
}

impl ErrorFormatter {
    pub fn new(file_path: String, tokens: TokenStream) -> Self {
        Self { file_path, tokens }
    }

    pub fn update_info(&mut self, file_path: String, tokens: TokenStream) {
        self.file_path = file_path;
        self.tokens = tokens;
    }
}

impl Default for ErrorFormatter {
    fn default() -> Self {
        Self {
            file_path: String::new(),
            tokens: Vec::new(),
        }
    }
}

impl ErrorFormatter {
    fn internal_parse_error_raw_pos(&self, ast_start_pos: usize, ast_size: usize, tag_message: String) -> (String, usize, String, String, String) {
        // Get the tokens at the start and end of the line containing the error. Skip the leading newline.
        let error_line_start_pos = self.tokens[..ast_start_pos].iter().rposition(|x| x == &TokenType::TkNewLine).unwrap() + 1;
        let error_line_end_pos   = self.tokens[ast_start_pos..].iter().position(|x| x == &TokenType::TkNewLine).unwrap_or(self.tokens.len()) + ast_start_pos;
        let error_line_tokens    = &self.tokens[error_line_start_pos..error_line_end_pos];
        let mut error_line_as_string = error_line_tokens.iter().map(|x| x.to_string()).collect::<String>();

        // Get the line number of the error.
        let error_line_number = self.tokens[..ast_start_pos].iter().filter(|x| x == &&TokenType::TkNewLine).count();

        // The number of "^" is the length of the token data where the error is.
        let num_tokens_before_ast_pos = ast_start_pos - error_line_start_pos + 1;
        let mut carets = "^".repeat(ast_size);
        carets.insert_str(0, " ".repeat(num_tokens_before_ast_pos).as_str());

        // Print the preceding spaces before the error line.
        let l1 = error_line_as_string.len();
        error_line_as_string = error_line_as_string.replace("  ", "");
        carets = carets[l1 - error_line_as_string.len()..].to_string();
        carets += (" <- ".to_string() + tag_message.as_str()).bold().white().to_string().as_str();
        let left_padding = " ".repeat(error_line_number.to_string().len());

        // Return the file path, line number, line as string, and carets.
        (self.file_path.clone(), error_line_number, error_line_as_string, left_padding, carets)
    }

    pub fn error(&self, ast: &impl Ast, message: String, tag_message: String) -> String {
        let ast_start_pos = ast.get_pos();
        let ast_size = ast.get_size();
        self.error_raw_pos(ast_start_pos, ast_size, message, tag_message)
    }

    pub fn error_raw_pos(&self, ast_start_pos: usize, ast_size: usize, message: String, tag_message: String) -> String {
        let (file_path, error_line_number, error_line_as_string, left_padding, carets) = self.internal_parse_error_raw_pos(ast_start_pos, ast_size, tag_message);
        let line1 = format!("Error in file '{file_path}', on line {error_line_number}:\n").bold().white();
        let line2 = format!("{left_padding} {BAR_CHAR}\n").bold().white();
        let line3 = format!("{error_line_number} {BAR_CHAR} {error_line_as_string}\n").bold().red();
        let line4 = format!("{left_padding} {BAR_CHAR}").bold().white();
        let line5 = format!("{carets}\n").red();
        let line6 = format!("{message}\n").red();
        line1.input + line2.input.as_str() + line3.input.as_str() + line4.input.as_str() + line5.input.as_str() + line6.input.as_str()
    }

    pub fn error_minimal(&self, ast: &impl Ast, tag_message: String) -> String {
        let ast_start_pos = ast.get_pos();
        let ast_size = ast.get_size();
        self.error_minimal_raw_pos(ast_start_pos, ast_size, tag_message)
    }

    pub fn error_minimal_raw_pos(&self, ast_start_pos: usize, ast_size: usize, tag_message: String) -> String {
        let (file_path, error_line_number, error_line_as_string, left_padding, carets) = self.internal_parse_error_raw_pos(ast_start_pos, ast_size, tag_message);
        let line1 = format!("Context from file '{file_path}', on line {error_line_number}:\n").bold().white();
        let line2 = format!("{left_padding} {BAR_CHAR}\n").bold().white();
        let line3 = format!("{error_line_number} {BAR_CHAR} {error_line_as_string}\n").bold().green();
        let line4 = format!("{left_padding} {BAR_CHAR}").bold().white();
        let line5 = format!("{carets}\n").green();
        line1.input + line2.input.as_str() + line3.input.as_str() + line4.input.as_str() + line5.input.as_str()
    }
}
