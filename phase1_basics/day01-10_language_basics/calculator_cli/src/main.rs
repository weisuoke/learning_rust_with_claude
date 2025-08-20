// 命令行计算器 - Rust学习项目
// 学习目标：掌握Rust基础语法、所有权系统、错误处理

use std::io::{self, Write};

mod calculator;
mod errors;
mod parser;

use calculator::Calculator;
use errors::CalculatorError;

fn main() -> Result<(), CalculatorError> {
    println!("=== Rust 命令行计算器 ===");
    println!("输入数学表达式，或输入 'quit' 退出");
    println!("支持：+, -, *, /, (), 幂运算(^)");
    
    let mut calculator = Calculator::new();
    
    loop {
        print!("calc> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input == "quit" || input == "exit" {
            println!("再见!");
            break;
        }
        
        if input == "history" {
            calculator.show_history();
            continue;
        }
        
        if input == "clear" {
            calculator.clear_history();
            println!("历史记录已清空");
            continue;
        }
        
        match calculator.evaluate(input) {
            Ok(result) => println!("结果: {}", result),
            Err(e) => println!("错误: {}", e),
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_creation() {
        let calc = Calculator::new();
        assert_eq!(calc.history_count(), 0);
    }
}
