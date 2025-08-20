//! 表达式解析器
//! 学习目标：枚举、模式匹配、递归

use crate::errors::{CalculatorError, CalculatorResult};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LeftParen,
    RightParen,
    End,
}

pub struct Parser {
    input: Vec<char>,
    position: usize,
    current_token: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut parser = Self {
            input: input.chars().collect(),
            position: 0,
            current_token: Token::End,
        };
        parser.next_token().unwrap_or(());
        parser
    }

    pub fn parse(&mut self) -> CalculatorResult<f64> {
        let result = self.parse_expression()?;
        
        if self.current_token != Token::End {
            return Err(CalculatorError::ParseError("表达式未完全解析".to_string()));
        }
        
        Ok(result)
    }

    fn parse_expression(&mut self) -> CalculatorResult<f64> {
        let mut result = self.parse_term()?;

        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = self.current_token.clone();
            self.next_token()?;
            let right = self.parse_term()?;

            match op {
                Token::Plus => result += right,
                Token::Minus => result -= right,
                _ => unreachable!(),
            }
        }

        Ok(result)
    }

    fn parse_term(&mut self) -> CalculatorResult<f64> {
        let mut result = self.parse_power()?;

        while matches!(self.current_token, Token::Multiply | Token::Divide) {
            let op = self.current_token.clone();
            self.next_token()?;
            let right = self.parse_power()?;

            match op {
                Token::Multiply => result *= right,
                Token::Divide => {
                    if right == 0.0 {
                        return Err(CalculatorError::DivisionByZero);
                    }
                    result /= right;
                }
                _ => unreachable!(),
            }
        }

        Ok(result)
    }

    fn parse_power(&mut self) -> CalculatorResult<f64> {
        let mut result = self.parse_factor()?;

        if matches!(self.current_token, Token::Power) {
            self.next_token()?;
            let right = self.parse_power()?; // 右结合
            result = result.powf(right);
        }

        Ok(result)
    }

    fn parse_factor(&mut self) -> CalculatorResult<f64> {
        match &self.current_token {
            Token::Number(value) => {
                let result = *value;
                self.next_token()?;
                Ok(result)
            }
            Token::Minus => {
                self.next_token()?;
                let result = self.parse_factor()?;
                Ok(-result)
            }
            Token::Plus => {
                self.next_token()?;
                self.parse_factor()
            }
            Token::LeftParen => {
                self.next_token()?;
                let result = self.parse_expression()?;
                
                if self.current_token != Token::RightParen {
                    return Err(CalculatorError::UnbalancedParentheses);
                }
                
                self.next_token()?;
                Ok(result)
            }
            _ => Err(CalculatorError::ParseError("期望数字或左括号".to_string())),
        }
    }

    fn next_token(&mut self) -> CalculatorResult<()> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            self.current_token = Token::End;
            return Ok(());
        }

        let ch = self.input[self.position];
        self.position += 1;

        self.current_token = match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '^' => Token::Power,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '0'..='9' | '.' => {
                self.position -= 1; // 回退一个字符
                self.parse_number()?
            }
            _ => return Err(CalculatorError::ParseError(format!("无效字符: {}", ch))),
        };

        Ok(())
    }

    fn parse_number(&mut self) -> CalculatorResult<Token> {
        let start = self.position;
        let mut has_dot = false;

        while self.position < self.input.len() {
            match self.input[self.position] {
                '0'..='9' => self.position += 1,
                '.' if !has_dot => {
                    has_dot = true;
                    self.position += 1;
                }
                _ => break,
            }
        }

        let number_str: String = self.input[start..self.position].iter().collect();
        let value = number_str.parse::<f64>()?;
        
        Ok(Token::Number(value))
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let mut parser = Parser::new("1 + 2");
        assert_eq!(parser.parse().unwrap(), 3.0);
    }

    #[test]
    fn test_multiplication_precedence() {
        let mut parser = Parser::new("2 + 3 * 4");
        assert_eq!(parser.parse().unwrap(), 14.0);
    }

    #[test]
    fn test_parentheses() {
        let mut parser = Parser::new("(2 + 3) * 4");
        assert_eq!(parser.parse().unwrap(), 20.0);
    }

    #[test]
    fn test_power() {
        let mut parser = Parser::new("2 ^ 3");
        assert_eq!(parser.parse().unwrap(), 8.0);
    }

    #[test]
    fn test_division_by_zero() {
        let mut parser = Parser::new("1 / 0");
        assert!(matches!(parser.parse(), Err(CalculatorError::DivisionByZero)));
    }

    #[test]
    fn test_unbalanced_parentheses() {
        let mut parser = Parser::new("(1 + 2");
        assert!(matches!(parser.parse(), Err(CalculatorError::UnbalancedParentheses)));
    }

    #[test]
    fn test_negative_numbers() {
        let mut parser = Parser::new("-5 + 3");
        assert_eq!(parser.parse().unwrap(), -2.0);
    }

    #[test]
    fn test_floating_point() {
        let mut parser = Parser::new("3.14 + 2.86");
        assert!((parser.parse().unwrap() - 6.0).abs() < 1e-10);
    }
}