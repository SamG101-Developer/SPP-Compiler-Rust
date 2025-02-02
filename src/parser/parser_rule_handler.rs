use crate::asts::ast::Ast;
use crate::parser::parser::Parser;
use crate::parser::parser_error::SyntaxError;

type ParserRule<T> = Box<dyn FnMut() -> Result<T, SyntaxError>>;

pub struct SingleParserRuleHandler<'a, T> {
    pub rule: ParserRule<T>,
    pub parser: Box<&'a mut Parser>,
}

pub struct AlternativeParserRuleHandler<'a, T> {
    pub rules: Vec<Box<dyn ParserRuleHandler<T>>>,
    pub parser: Box<&'a mut Parser>,
}

impl <'a, T> SingleParserRuleHandler<'a, T> {
    pub fn enum_wrapper<U, Arg>(self, rule: Box<fn(Arg) -> U>) -> SingleParserRuleHandler<'a, U> {
        SingleParserRuleHandler {
            rule,
            parser: Box::new(*self.parser),
        }
    }
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
        mut separator: Box<impl ParserRuleHandler<U>>,
    ) -> Vec<T> {
        let mut result = vec![];

        loop {
            let index = self.get_parser_index();
            let one_result = self.parse_once();

            match one_result {
                Ok(ast) => {
                    result.push(ast);
                    let ast = separator.parse_optional();
                    if ast.is_none() {
                        return result;
                    }
                }
                Err(_) => {
                    self.set_parser_index(index);
                    return result;
                }
            }
        }
    }

    fn parse_one_or_more<U>(
        &mut self,
        separator: Box<impl ParserRuleHandler<U>>,
    ) -> Result<Vec<T>, SyntaxError> {
        let result = self.parse_zero_or_more(separator);
        if result.len() < 1 {
            return Err(SyntaxError {});
        }
        Ok(result)
    }

    fn parse_two_or_more<U>(
        &mut self,
        separator: Box<impl ParserRuleHandler<U>>,
    ) -> Result<Vec<T>, SyntaxError> {
        let result = self.parse_zero_or_more(separator);
        if result.len() < 2 {
            return Err(SyntaxError {});
        }
        Ok(result)
    }

    fn or<'a>(
        self,
        other: SingleParserRuleHandler<T>) -> AlternativeParserRuleHandler<'a, T>;
}

impl<'a, T> ParserRuleHandler<T> for SingleParserRuleHandler<'a, T> {
    fn get_parser_index(&self) -> usize {
        self.parser.index
    }

    fn set_parser_index(&mut self, index: usize) {
        self.parser.index = index;
    }

    fn parse_once(&mut self) -> Result<T, SyntaxError> {
        self.rule()
    }

    fn or<'b>(self, other: SingleParserRuleHandler<T>) -> AlternativeParserRuleHandler<'b, T> {
        AlternativeParserRuleHandler {
            rules: vec![Box::new(self), Box::new(other)],
            parser: self.parser,
        }
    }
}

impl<'a, T> ParserRuleHandler<T> for AlternativeParserRuleHandler<'a, T> {
    fn get_parser_index(&self) -> usize {
        self.parser.index
    }

    fn set_parser_index(&mut self, index: usize) {
        self.parser.index = index;
    }

    fn parse_once(&mut self) -> Result<T, SyntaxError> {
        for rule in &mut self.rules {
            let index = self.parser.index;
            let result = rule.parse_once();

            match result {
                Ok(ast) => return Ok(ast),
                Err(_) => self.parser.index = index,
            }
        }

        Err(SyntaxError {})
    }

    fn or<'b>(mut self, other: SingleParserRuleHandler<T>) -> AlternativeParserRuleHandler<'b, T> {
        self.rules.push(Box::new(other));
        self
    }
}
