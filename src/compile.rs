use crate::chunk::{Chunk, OpCode};
use crate::scanner::{Scanner, Token, TokenType};
use std::collections::HashMap;
use std::str::FromStr;

pub struct Parser {
    previous: Option<Token>,
    current: Option<Token>,
    scanner: Scanner,
    had_error: bool,
    chunk: Chunk,
    parse_rules: HashMap<TokenType, ParseRule>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    fn next(&self) -> Self {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

#[derive(Debug, Clone)]
enum ParseFn {
    Unary,
    Grouping,
    Binary,
    Number,
}

#[derive(Debug, Clone)]
struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        Self {
            previous: None,
            current: None,
            scanner: Scanner::new(source),
            had_error: false,
            chunk: Chunk::new(),
            parse_rules: Self::build_parse_rules(),
        }
    }

    fn build_parse_rules() -> HashMap<TokenType, ParseRule> {
        HashMap::from([
            (
                TokenType::LeftParen,
                ParseRule {
                    prefix: Some(ParseFn::Grouping),
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::RightParen,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::LeftBrace,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::RightBrace,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Comma,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Dot,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Minus,
                ParseRule {
                    prefix: Some(ParseFn::Unary),
                    infix: Some(ParseFn::Binary),
                    precedence: Precedence::Term,
                },
            ),
            (
                TokenType::Plus,
                ParseRule {
                    prefix: None,
                    infix: Some(ParseFn::Binary),
                    precedence: Precedence::Term,
                },
            ),
            (
                TokenType::Semicolon,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Slash,
                ParseRule {
                    prefix: None,
                    infix: Some(ParseFn::Binary),
                    precedence: Precedence::Factor,
                },
            ),
            (
                TokenType::Star,
                ParseRule {
                    prefix: None,
                    infix: Some(ParseFn::Binary),
                    precedence: Precedence::Factor,
                },
            ),
            (
                TokenType::Bang,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::BangEqual,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Equal,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::EqualEqual,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Greater,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::GreaterEqual,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Less,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::LessEqual,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Identifier,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::String,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Number,
                ParseRule {
                    prefix: Some(ParseFn::Number),
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::And,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Class,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Else,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::False,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::For,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Fun,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::If,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Nil,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Or,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Print,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Return,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Super,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::This,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::True,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Var,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::While,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Error,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
            (
                TokenType::Eof,
                ParseRule {
                    prefix: None,
                    infix: None,
                    precedence: Precedence::None,
                },
            ),
        ])
    }

    fn get_rule(&self, t_type: &TokenType) -> &ParseRule {
        self.parse_rules.get(t_type).unwrap()
    }

    fn dispatch_parse_fn(&mut self, parse_fn: &ParseFn) {
        match parse_fn {
            ParseFn::Unary => self.unary(),
            ParseFn::Grouping => self.grouping(),
            ParseFn::Binary => self.binary(),
            ParseFn::Number => self.number(),
        }
    }

    pub fn compile(&mut self) -> Chunk {
        self.advance();
        self.expression();
        self.consume(&TokenType::Eof, "Expect end of expression.");
        self.end_compile();

        // let mut line = 0;
        // loop {
        //     let token = scanner.scan_token();
        //     if token.line != line {
        //         print!("{:4} ", token.line);
        //         line = token.line;
        //     } else {
        //         print!("   | ");
        //     }
        //
        //     println!("{:?} '{}'", token.t_type, token.token);
        //
        //     if (token.t_type == TokenType::Eof) {
        //         break;
        //     }
        // }

        self.chunk.clone()
    }

    fn end_compile(&mut self) {
        self.emit_return();
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) {
        if &self.current.as_ref().unwrap().t_type == token_type {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = Some(self.scanner.scan_token());
            if self.current.as_ref().unwrap().t_type != TokenType::Error {
                break;
            }

            let current = &self.current.clone().unwrap();
            self.error_at_current(&current.token);
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn number(&mut self) {
        let token = &self.previous.clone().unwrap();
        let value = f64::from_str(&token.token).unwrap();
        self.emit_constant(value);
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(&TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator_type = &self.previous.clone().unwrap().t_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate),
            _ => return,
        }
    }

    fn binary(&mut self) {
        let operator_type = &self.previous.clone().unwrap().t_type;
        let rule = self.get_rule(&operator_type);
        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide),
            _ => return,
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let rule = self.get_rule(&self.previous.clone().unwrap().t_type);
        let prefix_rule = &rule.prefix.clone();
        if let Some(prefix_parse_fn) = prefix_rule {
            self.dispatch_parse_fn(prefix_parse_fn);

            while precedence
                <= self
                    .get_rule(&self.current.clone().unwrap().t_type)
                    .precedence
            {
                self.advance();
                let infix_rule = self
                    .get_rule(&self.previous.clone().unwrap().t_type)
                    .clone();
                if let Some(infix_parse_fn) = &infix_rule.infix {
                    self.dispatch_parse_fn(infix_parse_fn);
                }
            }
            return;
        }

        self.error("Expect expression.");
    }

    fn emit_byte(&mut self, op_code: OpCode) {
        let token = &self.previous.clone().unwrap();
        self.chunk.write_chunk(op_code, token.line);
    }

    fn emit_constant(&mut self, value: f64) {
        let index = self.chunk.add_constant(value);
        self.emit_byte(OpCode::OpConstant { index });
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }

    fn error_at_current(&mut self, message: &str) {
        let token = &self.current.clone().unwrap();
        self.error_at(token, message);
    }

    fn error(&mut self, message: &str) {
        let token = &self.previous.clone().unwrap();
        self.error_at(token, message);
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.had_error {
            return;
        }

        eprint!("[line {}] Error", token.line);

        if token.t_type == TokenType::Eof {
            eprint!(" at end")
        } else if token.t_type == TokenType::Error {
            // nothing
        } else {
            eprint!(" at '{}'", token.token);
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }
}
