//! 错误类型定义
//! 学习目标：理解Rust的错误处理机制

use std::fmt;

#[derive(Debug, Clone)]
pub enum CalculatorError {
    ParseError(String),
    DivisionByZero,
    InvalidOperation(String),
    UnbalancedParentheses,
    EmptyExpression,
    InvalidNumber(String),
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            CalculatorError::DivisionByZero => write!(f, "除零错误"),
            CalculatorError::InvalidOperation(op) => write!(f, "无效操作: {}", op),
            CalculatorError::UnbalancedParentheses => write!(f, "括号不匹配"),
            CalculatorError::EmptyExpression => write!(f, "表达式为空"),
            CalculatorError::InvalidNumber(num) => write!(f, "无效数字: {}", num),
        }
    }
}

impl std::error::Error for CalculatorError {}

impl From<std::num::ParseFloatError> for CalculatorError {
    fn from(err: std::num::ParseFloatError) -> Self {
        CalculatorError::InvalidNumber(err.to_string())
    }
}

pub type CalculatorResult<T> = Result<T, CalculatorError>;