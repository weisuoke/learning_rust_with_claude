//! 计算器核心逻辑
//! 学习目标：结构体、方法、所有权系统

use crate::errors::{CalculatorError, CalculatorResult};
use crate::parser::Parser;

#[derive(Debug)]
pub struct Calculator {
    history: Vec<String>,
    max_history: usize,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            max_history: 100,
        }
    }

    pub fn with_max_history(max_history: usize) -> Self {
        Self {
            history: Vec::new(),
            max_history,
        }
    }

    pub fn evaluate(&mut self, expression: &str) -> CalculatorResult<f64> {
        if expression.trim().is_empty() {
            return Err(CalculatorError::EmptyExpression);
        }

        let mut parser = Parser::new(expression);
        let result = parser.parse()?;
        
        // 添加到历史记录
        self.add_to_history(expression, result);
        
        Ok(result)
    }

    fn add_to_history(&mut self, expression: &str, result: f64) {
        let entry = format!("{} = {}", expression, result);
        
        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }
        
        self.history.push(entry);
    }

    pub fn show_history(&self) {
        if self.history.is_empty() {
            println!("历史记录为空");
            return;
        }

        println!("=== 历史记录 ===");
        for (i, entry) in self.history.iter().enumerate() {
            println!("{}: {}", i + 1, entry);
        }
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_new() {
        let calc = Calculator::new();
        assert_eq!(calc.history_count(), 0);
        assert_eq!(calc.max_history, 100);
    }

    #[test]
    fn test_calculator_with_max_history() {
        let calc = Calculator::with_max_history(50);
        assert_eq!(calc.max_history, 50);
    }

    #[test]
    fn test_empty_expression() {
        let mut calc = Calculator::new();
        let result = calc.evaluate("");
        assert!(matches!(result, Err(CalculatorError::EmptyExpression)));
    }

    #[test]
    fn test_history_management() {
        let mut calc = Calculator::with_max_history(2);
        
        // 添加历史记录不会报错，但需要valid的表达式
        // 这里先测试历史记录容量管理
        calc.add_to_history("1+1", 2.0);
        calc.add_to_history("2+2", 4.0);
        calc.add_to_history("3+3", 6.0);
        
        assert_eq!(calc.history_count(), 2);
        assert_eq!(calc.get_history()[0], "2+2 = 4");
        assert_eq!(calc.get_history()[1], "3+3 = 6");
    }
}