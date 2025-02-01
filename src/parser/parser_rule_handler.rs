use crate::lexer::token::TokenType;
use crate::parser::parser::Parser;
use crate::parser::parser_error::SyntaxError;

type ParserRule<T> = dyn Fn() -> Result<T, SyntaxError>;

struct SingleParserRuleHandler<T> {
    rule: ParserRule<T>,
    parser: Box<Parser>,
}

struct AlternativeParserRuleHandler<T> {
    rules: Vec<Box<dyn ParserRuleHandler<T>>>,
    parser: Box<Parser>,
}

pub trait ParserRuleHandler<T> {
    fn get_parser_index(&self) -> usize;

    fn set_parser_index(&mut self, index: usize);

    fn parse_once(&mut self) -> Result<T, SyntaxError>;

    fn parse_optional(&mut self) -> Option<T> {
        let index = self.get_parser_index();
        let result = self.parse_once();

        match result {
            Ok(ast) => Some(ast),
            Err(_) => {
                self.set_parser_index(index);
                None
            }
        }
    }

    fn parse_zero_or_more<U>(
        &mut self,
        mut separator: Box<dyn ParserRuleHandler<U>>,
    ) -> Result<Vec<T>, SyntaxError> {
        let mut result = vec![];

        loop {
            let index = self.get_parser_index();
            let one_result = self.parse_once();

            match one_result {
                Ok(ast) => {
                    result.push(ast);
                    let ast = separator.parse_optional();
                    if ast.is_none() {
                        return Ok(result);
                    }
                }
                Err(err) => {
                    self.set_parser_index(index);
                    return if result.is_empty() {
                        Err(err)
                    } else {
                        Ok(result)
                    };
                }
            }
        }
    }
    fn parse_one_or_more<U>(
        &mut self,
        separator: Box<dyn ParserRuleHandler<U>>,
    ) -> Result<Vec<T>, SyntaxError> {
        let result = self.parse_zero_or_more(separator)?;
        if result.len() < 1 {
            return Err(SyntaxError {});
        }
        Ok(result)
    }

    fn parse_two_or_more<U>(
        &mut self,
        separator: Box<dyn ParserRuleHandler<U>>,
    ) -> Result<Vec<T>, SyntaxError> {
        let result = self.parse_zero_or_more(separator)?;
        if result.len() < 2 {
            return Err(SyntaxError {});
        }
        Ok(result)
    }
}

impl<T> ParserRuleHandler<T> for SingleParserRuleHandler<T> {
    fn get_parser_index(&self) -> usize {
        self.parser.index
    }

    fn set_parser_index(&mut self, index: usize) {
        self.parser.index = index;
    }

    fn parse_once(&mut self) -> Result<T, SyntaxError> {
        self.rule()
    }
}

impl<T> ParserRuleHandler<T> for AlternativeParserRuleHandler<T> {
    fn get_parser_index(&self) -> usize {
        self.parser.index
    }

    fn set_parser_index(&mut self, index: usize) {
        self.parser.index = index;
    }

    fn parse_once(&mut self) -> Result<T, SyntaxError> {
        for rule in &self.rules {
            let index = self.parser.index;
            let result = rule();

            match result {
                Ok(ast) => return Ok(ast),
                Err(_) => self.parser.index = index,
            }
        }

        Err(SyntaxError {})
    }
}

impl std::ops::BitOr for SingleParserRuleHandler<TokenType> {
    type Output = AlternativeParserRuleHandler<TokenType>;

    fn bitor(
        self,
        other: SingleParserRuleHandler<TokenType>,
    ) -> AlternativeParserRuleHandler<TokenType> {
        AlternativeParserRuleHandler {
            rules: vec![Box::new(self), Box::new(other)],
            parser: self.parser,
        }
    }
}

impl std::ops::BitOr for AlternativeParserRuleHandler<TokenType> {
    type Output = AlternativeParserRuleHandler<TokenType>;

    fn bitor(
        mut self,
        other: AlternativeParserRuleHandler<TokenType>,
    ) -> AlternativeParserRuleHandler<TokenType> {
        self.rules.extend(other.rules);
        self
    }
}
