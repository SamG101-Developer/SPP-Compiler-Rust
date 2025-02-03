use crate::parser::parser::Parser;
use crate::parser::parser_error::SyntaxError;
use std::cell::RefCell;
use std::rc::Rc;

type ParserRule<T> = Box<dyn FnMut() -> Result<T, SyntaxError>>;

pub struct SingleParserRuleHandler<T> {
    pub rule: ParserRule<T>,
    pub parser: Rc<RefCell<Parser>>,
}

pub struct AlternativeParserRuleHandler<T> {
    pub rules: Vec<Rc<RefCell<SingleParserRuleHandler<T>>>>,
    pub parser: Rc<RefCell<Parser>>,
}

impl<T: 'static> SingleParserRuleHandler<T> {
    pub fn enum_wrapper<U: 'static>(mut self, mut rule: Box<dyn FnMut(T) -> U>) -> SingleParserRuleHandler<U> {
        SingleParserRuleHandler {
            rule: Box::new(move || { let arg = self.rule.as_mut()()?; Ok(rule(arg)) }),
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

    fn parse_zero_or_more<U>(&mut self, mut separator: Box<impl ParserRuleHandler<U>>) -> Vec<T> {
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
            return Err(SyntaxError::new(
                self.get_parser_index(),
                "Expected one or more".to_string(),
            ));
        }
        Ok(result)
    }

    fn parse_two_or_more<U>(
        &mut self,
        separator: Box<impl ParserRuleHandler<U>>,
    ) -> Result<Vec<T>, SyntaxError> {
        let result = self.parse_zero_or_more(separator);
        if result.len() < 2 {
            return Err(SyntaxError::new(
                self.get_parser_index(),
                "Expected two or more".to_string(),
            ));
        }
        Ok(result)
    }

    fn or(self, other: SingleParserRuleHandler<T>) -> AlternativeParserRuleHandler<T>;
}

impl<T> ParserRuleHandler<T> for SingleParserRuleHandler<T> {
    fn get_parser_index(&self) -> usize {
        self.parser.borrow().index
    }

    fn set_parser_index(&mut self, index: usize) {
        self.parser.borrow_mut().index = index;
    }

    fn parse_once(&mut self) -> Result<T, SyntaxError> {
        self.rule.as_mut()()
    }

    fn or(self, other: SingleParserRuleHandler<T>) -> AlternativeParserRuleHandler<T> {
        let self_parser = Rc::clone(&self.parser);
        AlternativeParserRuleHandler {
            rules: vec![Rc::new(RefCell::new(self)), Rc::new(RefCell::new(other))],
            parser: self_parser,
        }
    }
}

impl<T> ParserRuleHandler<T> for AlternativeParserRuleHandler<T> {
    fn get_parser_index(&self) -> usize {
        self.parser.borrow().index
    }

    fn set_parser_index(&mut self, index: usize) {
        self.parser.borrow_mut().index = index;
    }

    fn parse_once(&mut self) -> Result<T, SyntaxError> {
        for rule in &mut self.rules {
            let index = self.parser.borrow_mut().index;
            let result = rule.borrow_mut().parse_once();

            match result {
                Ok(ast) => return Ok(ast),
                Err(_) => self.parser.borrow_mut().index = index,
            }
        }

        Err(SyntaxError::new(
            self.get_parser_index(),
            "Expected one of the alternatives".to_string(),
        ))
    }

    fn or(mut self, other: SingleParserRuleHandler<T>) -> AlternativeParserRuleHandler<T> {
        self.rules.push(Rc::new(RefCell::new(other)));
        self
    }
}
