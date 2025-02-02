use std::cell::RefCell;
use std::rc::Rc;
use crate::parser::parser::Parser;
use crate::parser::parser_error::SyntaxError;

type ParserRule<T> = Box<dyn FnMut() -> Result<T, SyntaxError>>;

pub struct SingleParserRuleHandler<T> {
    pub rule: ParserRule<T>,
    pub parser: RefCell<Rc<Parser>>,
}

pub struct AlternativeParserRuleHandler<T> {
    pub rules: Vec<Box<dyn ParserRuleHandler<T>>>,
    pub parser: RefCell<Rc<Parser>>,
}

impl <T> SingleParserRuleHandler<T> {
    pub fn enum_wrapper<U, Arg>(self, rule: Box<fn(Arg) -> U>) -> SingleParserRuleHandler<U> {
        SingleParserRuleHandler {
            rule,
            parser: self.parser,
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

    fn or(
        self,
        other: SingleParserRuleHandler<T>) -> AlternativeParserRuleHandler<T>;
}

impl<T> ParserRuleHandler<T> for SingleParserRuleHandler<T> {
    fn get_parser_index(&mut self) -> usize {
        self.parser.borrow_mut().index
    }

    fn set_parser_index(&mut self, index: usize) {
        self.parser.borrow_mut().index = index;
    }

    fn parse_once(&mut self) -> Result<T, SyntaxError> {
        self.rule()
    }

    fn or(self, other: SingleParserRuleHandler<T>) -> AlternativeParserRuleHandler<T> {
        AlternativeParserRuleHandler {
            rules: vec![Box::new(self), Box::new(other)],
            parser: self.parser,
        }
    }
}

impl<T> ParserRuleHandler<T> for AlternativeParserRuleHandler<T> {
    fn get_parser_index(&mut self) -> usize {
        self.parser.borrow_mut().index
    }

    fn set_parser_index(&mut self, index: usize) {
        self.parser.borrow_mut().index = index;
    }

    fn parse_once(&mut self) -> Result<T, SyntaxError> {
        for rule in &mut self.rules {
            let index = self.parser.borrow_mut().index;
            let result = rule.parse_once();

            match result {
                Ok(ast) => return Ok(ast),
                Err(_) => self.parser.get_mut().index = index,
            }
        }

        Err(SyntaxError {})
    }

    fn or(mut self, other: SingleParserRuleHandler<T>) -> AlternativeParserRuleHandler<T> {
        self.rules.push(Box::new(other));
        self
    }
}
