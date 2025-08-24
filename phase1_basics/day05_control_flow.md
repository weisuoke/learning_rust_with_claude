# Rust入门 - 第5天：控制流 | Rust Introduction - Day 5: Control Flow

## 学习目标 | Learning Objectives
- 掌握条件语句的使用和最佳实践 | Master the usage and best practices of conditional statements
- 理解不同类型循环的应用场景 | Understand the application scenarios of different types of loops
- 学会使用break和continue控制循环流程 | Learn to use break and continue to control loop flow
- 能够组合使用各种控制流结构 | Be able to combine various control flow structures
- 实现复杂的程序逻辑控制 | Implement complex program logic control
- 完成猜数字游戏挑战项目 | Complete the number guessing game challenge project

## 详细内容 | Detailed Content

### 1. 条件语句基础 | Conditional Statements Basics (45分钟 | 45 minutes)

- **if/else 条件语句 | if/else Conditional Statements**
  
  **概念定义 | Concept Definition:**
  条件语句是根据特定条件的真假来决定程序执行路径的控制结构，在Rust中主要使用if表达式来实现条件控制 | Conditional statements are control structures that determine the program execution path based on the truth value of specific conditions, primarily implemented using if expressions in Rust
  
  **核心特征 | Key Characteristics:**
  - if是表达式而非语句，可以返回值用于变量赋值 | if is an expression, not a statement, and can return values for variable assignment
  - 条件必须是bool类型，Rust不会自动转换其他类型 | Conditions must be bool type, Rust won't automatically convert other types
  - 支持多重if-else if-else链式结构 | Supports multiple if-else if-else chain structures
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Rust中的if条件必须是什么类型？ | What type must if conditions be in Rust?
     **答案 | Answer:** bool类型 | bool type - Rust要求条件表达式必须明确返回布尔值
  2. if表达式可以用来给变量赋值吗？ | Can if expressions be used to assign values to variables?
     **答案 | Answer:** 是 | Yes - if是表达式，可以返回值并用于赋值操作
  3. 在Rust中，数字0会被自动转换为false吗？ | In Rust, is the number 0 automatically converted to false?
     **答案 | Answer:** 否 | No - Rust不会自动进行类型转换，必须显式使用bool值
  4. else if可以有多个吗？ | Can there be multiple else if clauses?
     **答案 | Answer:** 是 | Yes - 可以链接任意数量的else if来处理多种条件
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 基本if条件语句 | Basic if conditional statement
  let number = 7;
  
  if number > 5 {
      println!("数字大于5 | Number is greater than 5");
  } else if number > 0 {
      println!("数字大于0但小于等于5 | Number is greater than 0 but less than or equal to 5");
  } else {
      println!("数字小于等于0 | Number is less than or equal to 0");
  }
  
  // if表达式赋值 | if expression assignment
  let condition = true;
  let result = if condition { 5 } else { 6 };
  println!("结果是: {} | Result is: {}", result);
  
  // 复杂条件组合 | Complex condition combinations
  let age = 25;
  let has_license = true;
  
  if age >= 18 && has_license {
      println!("可以开车 | Can drive");
  } else if age >= 18 && !has_license {
      println!("需要考驾照 | Need to get a license");
  } else {
      println!("年龄不够 | Too young");
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码会输出什么？| What will this code output?
    **答案 | Answer:** "数字大于5", "结果是: 5", "可以开车"
  - 如果将condition改为false会发生什么？| What happens if we change condition to false?
    **答案 | Answer:** result的值会变为6
  
  **常见误区检查 | Common Misconception Checks:**
  - 可以直接用整数作为if条件吗？| Can integers be used directly as if conditions?
    **答案 | Answer:** 不可以，必须使用布尔表达式如 `if number != 0`
  - if表达式的各个分支必须返回相同类型吗？| Must all branches of an if expression return the same type?
    **答案 | Answer:** 是的，如果用于赋值，所有分支必须返回相同类型

### 2. 循环结构详解 | Loop Structures Explained (1小时 | 1 hour)

- **loop 无限循环 | loop Infinite Loop**
  
  **概念定义 | Concept Definition:**
  loop关键字创建一个无限循环，除非显式使用break语句跳出，常用于需要持续运行直到满足特定条件的场景 | The loop keyword creates an infinite loop that continues unless explicitly exited with break, commonly used for scenarios requiring continuous execution until specific conditions are met
  
  **核心特征 | Key Characteristics:**
  - 无条件执行，必须手动控制退出 | Executes unconditionally, exit must be manually controlled
  - 可以通过break返回值 | Can return values through break
  - 支持循环标签用于嵌套循环控制 | Supports loop labels for nested loop control
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. loop循环会自动结束吗？| Does a loop automatically end?
     **答案 | Answer:** 否 | No - loop会无限运行直到遇到break语句
  2. 可以从loop循环中返回值吗？| Can values be returned from a loop?
     **答案 | Answer:** 是 | Yes - 可以使用 `break value` 语法返回值
  3. 嵌套的loop循环如何控制内外层？| How to control inner and outer nested loops?
     **答案 | Answer:** 使用循环标签 | Use loop labels like `'outer: loop`
  
- **while 条件循环 | while Conditional Loop**
  
  **概念定义 | Concept Definition:**
  while循环在每次迭代前检查条件，当条件为真时继续执行，为假时停止，适用于条件明确的重复执行场景 | while loops check conditions before each iteration, continuing when true and stopping when false, suitable for repetitive execution with clear conditions
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. while循环什么时候检查条件？| When does a while loop check its condition?
     **答案 | Answer:** 每次迭代开始前 | Before each iteration starts
  2. while循环的条件必须是什么类型？| What type must the while loop condition be?
     **答案 | Answer:** bool类型 | bool type
  3. 如果while条件一开始就是false会怎样？| What happens if the while condition is false from the start?
     **答案 | Answer:** 循环体不会执行 | The loop body won't execute at all
  
- **for 迭代循环 | for Iteration Loop**
  
  **概念定义 | Concept Definition:**
  for循环用于遍历集合或范围，是最常用的循环形式，提供简洁的语法来处理可迭代的数据结构 | for loops are used to iterate over collections or ranges, being the most commonly used loop form, providing concise syntax for handling iterable data structures
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. for循环主要用于什么场景？| What scenarios are for loops mainly used for?
     **答案 | Answer:** 遍历集合或范围 | Iterating over collections or ranges
  2. `for i in 0..5` 会包含5吗？| Does `for i in 0..5` include 5?
     **答案 | Answer:** 否 | No - 范围0..5不包含结束值5
  3. for循环可以遍历数组吗？| Can for loops iterate over arrays?
     **答案 | Answer:** 是 | Yes - 可以直接遍历数组和其他可迭代类型
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // loop无限循环示例 | loop infinite loop example
  let mut counter = 0;
  let result = loop {
      counter += 1;
      
      if counter == 10 {
          break counter * 2; // 返回值20 | Return value 20
      }
  };
  println!("loop结果: {} | loop result: {}", result);
  
  // while条件循环 | while conditional loop
  let mut number = 3;
  while number != 0 {
      println!("{}!", number);
      number -= 1;
  }
  println!("起飞！| Liftoff!");
  
  // for迭代循环 | for iteration loop
  let array = [10, 20, 30, 40, 50];
  
  // 遍历数组元素 | Iterate over array elements
  for element in array {
      println!("值是: {} | Value is: {}", element);
  }
  
  // 遍历范围 | Iterate over range
  for number in 1..4 {
      println!("数字: {} | Number: {}", number);
  }
  
  // 带索引的遍历 | Enumerate iteration
  for (index, value) in array.iter().enumerate() {
      println!("索引{}: {} | Index {}: {}", index, value, index, value);
  }
  ```

### 3. 流程控制语句 | Flow Control Statements (30分钟 | 30 minutes)

- **break 和 continue 语句 | break and continue Statements**
  
  **概念定义 | Concept Definition:**
  break用于立即退出当前循环，continue用于跳过当前迭代继续下一次循环，这两个关键字提供了精确的循环流程控制能力 | break is used to immediately exit the current loop, continue is used to skip the current iteration and proceed to the next, these keywords provide precise loop flow control capabilities
  
  **核心特征 | Key Characteristics:**
  - break可以带标签跳出特定的嵌套循环 | break can use labels to exit specific nested loops
  - continue只跳过当前迭代，循环继续执行 | continue only skips current iteration, loop continues executing
  - 两者都只影响最内层循环，除非使用标签 | Both only affect innermost loop unless labels are used
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. break语句的作用是什么？| What does the break statement do?
     **答案 | Answer:** 立即退出当前循环 | Immediately exits the current loop
  2. continue会终止整个循环吗？| Does continue terminate the entire loop?
     **答案 | Answer:** 否 | No - 只跳过当前迭代，继续下一次循环
  3. 在嵌套循环中，break默认影响哪个循环？| In nested loops, which loop does break affect by default?
     **答案 | Answer:** 最内层循环 | The innermost loop
  4. 如何从外层循环中跳出？| How to break out of an outer loop?
     **答案 | Answer:** 使用循环标签 | Use loop labels like `'outer`
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // break示例 | break example
  for number in 1..10 {
      if number == 5 {
          break; // 在5时退出循环 | Exit loop at 5
      }
      println!("数字: {} | Number: {}", number);
  }
  
  // continue示例 | continue example  
  for number in 1..6 {
      if number % 2 == 0 {
          continue; // 跳过偶数 | Skip even numbers
      }
      println!("奇数: {} | Odd number: {}", number);
  }
  
  // 嵌套循环标签示例 | Nested loop labels example
  'outer: for i in 1..4 {
      'inner: for j in 1..4 {
          if i == 2 && j == 2 {
              break 'outer; // 跳出外层循环 | Break out of outer loop
          }
          println!("i: {}, j: {} | i: {}, j: {}", i, j, i, j);
      }
  }
  ```

### 4. 控制流组合应用 | Combined Control Flow Applications (45分钟 | 45 minutes)

- **复杂控制逻辑设计 | Complex Control Logic Design**
  
  **概念定义 | Concept Definition:**
  在实际程序中，需要将各种控制流结构组合使用来实现复杂的业务逻辑，包括嵌套条件、循环中的条件判断等 | In real programs, various control flow structures need to be combined to implement complex business logic, including nested conditions and conditional judgments within loops
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 可以在循环内部使用if语句吗？| Can if statements be used inside loops?
     **答案 | Answer:** 是 | Yes - 这是常见的组合使用方式
  2. 循环可以嵌套吗？| Can loops be nested?
     **答案 | Answer:** 是 | Yes - 支持任意深度的循环嵌套
  3. 如何在复杂逻辑中保持代码可读性？| How to maintain code readability in complex logic?
     **答案 | Answer:** 使用适当的缩进、注释和函数拆分 | Use proper indentation, comments, and function decomposition
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 复杂控制流示例：查找特定模式 | Complex control flow example: finding specific patterns
  let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let mut found_pairs = Vec::new();
  
  for i in 0..numbers.len() {
      for j in (i + 1)..numbers.len() {
          if numbers[i] + numbers[j] == 10 {
              found_pairs.push((numbers[i], numbers[j]));
              if found_pairs.len() >= 2 {
                  break; // 找到两对就停止 | Stop after finding two pairs
              }
          }
      }
      if found_pairs.len() >= 2 {
          break; // 外层循环也要停止 | Outer loop should also stop
      }
  }
  
  println!("找到的配对: {:?} | Found pairs: {:?}", found_pairs);
  ```

### 5. 最佳实践与性能考虑 | Best Practices and Performance Considerations (20分钟 | 20 minutes)

- **控制流最佳实践 | Control Flow Best Practices**
  
  **关键原则 | Key Principles:**
  - 优先使用for循环遍历集合，避免手动索引管理 | Prefer for loops for collection iteration, avoid manual index management
  - 合理使用early return减少嵌套层级 | Use early return reasonably to reduce nesting levels
  - 为复杂的嵌套循环添加标签和注释 | Add labels and comments for complex nested loops
  
  **实践验证问题 | Practice Verification Questions:**
  1. 什么时候应该使用for循环而不是while？| When should for loops be used instead of while?
     **答案 | Answer:** 遍历已知集合或范围时 | When iterating over known collections or ranges
  2. 如何避免无限循环？| How to avoid infinite loops?
     **答案 | Answer:** 确保循环条件会在某个时刻变为false，或在loop中有明确的break条件
  3. 复杂逻辑应该如何组织？| How should complex logic be organized?
     **答案 | Answer:** 拆分为小函数，使用清晰的变量名和注释

### 6. 知识巩固与检查 | Knowledge Consolidation and Review (10分钟 | 10 minutes)

- **综合控制流理解验证 | Comprehensive Control Flow Understanding Verification**
  
  **综合概念检查 | Comprehensive Concept Check:**
  1. 什么情况下使用loop而不是while循环？| When to use loop instead of while loops?
     **答案 | Answer:** 当需要无限循环或从循环中返回值时 | When infinite loops are needed or returning values from loops
  2. 如何选择合适的循环类型？| How to choose the appropriate loop type?
     **答案 | Answer:** for用于遍历，while用于条件循环，loop用于无限循环
  3. break和continue在不同循环中的行为一致吗？| Do break and continue behave consistently in different loops?
     **答案 | Answer:** 是 | Yes - 行为一致，都作用于当前所在的循环
  4. 如何处理复杂的嵌套控制流？| How to handle complex nested control flow?
     **答案 | Answer:** 使用标签、合理拆分函数、添加清晰注释
  5. 控制流的性能考虑有哪些？| What are the performance considerations for control flow?
     **答案 | Answer:** 避免不必要的嵌套、选择合适的循环类型、减少条件检查复杂度

## 实践项目：猜数字游戏 | Practical Project: Number Guessing Game

### 目标 | Objective
通过实现一个完整的猜数字游戏来综合应用条件语句、循环、流程控制等所有控制流概念 | Implement a complete number guessing game to comprehensively apply all control flow concepts including conditional statements, loops, and flow control

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 如何创建一个持续运行的游戏循环？| How to create a continuously running game loop?
   **答案 | Answer:** 使用loop创建无限循环，根据游戏状态使用break退出
2. 如何根据用户输入做出不同的响应？| How to respond differently based on user input?
   **答案 | Answer:** 使用if-else语句或match表达式处理不同的输入情况
3. 如何让用户可以选择是否继续游戏？| How to let users choose whether to continue the game?
   **答案 | Answer:** 在游戏结束后询问用户，根据回答决定是否break出主循环

### 步骤 | Steps
1. 设计游戏主循环结构，确保能够重复进行游戏
2. 实现随机数生成和用户输入处理
3. 使用条件语句判断猜测结果并给出提示
4. 添加计分系统和游戏统计功能
5. 实现优雅的退出和重新开始机制

### 示例代码 | Example Code
```rust
"""
猜数字游戏 | Number Guessing Game
这是一个综合展示Rust控制流特性的完整游戏项目 | A complete game project demonstrating Rust control flow features

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 条件语句 (if/else) | Conditional statements (if/else)
- 循环结构 (loop, while, for) | Loop structures (loop, while, for)  
- 流程控制 (break, continue) | Flow control (break, continue)
"""

use std::io;
use std::cmp::Ordering;

fn main() {
    println!("🎲 欢迎来到猜数字游戏！| Welcome to the Number Guessing Game! 🎲");
    
    // 主游戏循环 | Main game loop
    'game_loop: loop {
        println!("\n开始新游戏... | Starting new game...");
        
        // 生成1-100的随机数 | Generate random number 1-100
        let secret_number = generate_random_number();
        let mut guess_count = 0;
        let max_guesses = 7;
        
        println!("我已经想好了一个1到100之间的数字，你有{}次机会猜对它！", max_guesses);
        println!("I've thought of a number between 1 and 100, you have {} chances to guess it!", max_guesses);
        
        // 猜测循环 | Guessing loop
        'guess_loop: loop {
            guess_count += 1;
            
            println!("\n第{}次猜测 (剩余{}次) | Guess #{} ({} remaining):", 
                    guess_count, max_guesses - guess_count + 1, guess_count, max_guesses - guess_count + 1);
            print!("请输入你的猜测: | Please input your guess: ");
            
            // 获取用户输入 | Get user input
            let mut guess = String::new();
            io::stdin().read_line(&mut guess)
                .expect("无法读取输入 | Failed to read input");
            
            // 解析输入 | Parse input
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => {
                    // 验证输入范围 | Validate input range
                    if num < 1 || num > 100 {
                        println!("❌ 请输入1到100之间的数字！| Please enter a number between 1 and 100!");
                        continue 'guess_loop; // 跳过本次循环 | Skip this iteration
                    }
                    num
                },
                Err(_) => {
                    println!("❌ 请输入有效的数字！| Please enter a valid number!");
                    continue 'guess_loop; // 跳过本次循环 | Skip this iteration
                },
            };
            
            // 比较猜测结果 | Compare guess result
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("📈 太小了！| Too small!");
                    // 给出更详细的提示 | Give more detailed hints
                    if secret_number - guess > 20 {
                        println!("💡 提示：还差得很远哦！| Hint: You're way off!");
                    } else if secret_number - guess > 10 {
                        println!("💡 提示：有点远 | Hint: Getting warmer");
                    } else {
                        println!("💡 提示：很接近了！| Hint: Very close!");
                    }
                },
                Ordering::Greater => {
                    println!("📉 太大了！| Too big!");
                    // 给出更详细的提示 | Give more detailed hints
                    if guess - secret_number > 20 {
                        println!("💡 提示：还差得很远哦！| Hint: You're way off!");
                    } else if guess - secret_number > 10 {
                        println!("💡 提示：有点远 | Hint: Getting warmer");
                    } else {
                        println!("💡 提示：很接近了！| Hint: Very close!");
                    }
                },
                Ordering::Equal => {
                    // 猜对了！| Correct guess!
                    println!("🎉 恭喜！你猜对了！| Congratulations! You guessed it!");
                    println!("答案就是 {} | The answer was {}", secret_number);
                    
                    // 根据猜测次数给出评价 | Rate performance based on guess count
                    match guess_count {
                        1 => println!("🏆 太厉害了！一次就猜中！| Amazing! Got it in one try!"),
                        2..=3 => println!("🥇 非常棒！| Excellent!"),
                        4..=5 => println!("🥈 很好！| Great job!"),
                        6..=7 => println!("🥉 不错！| Good job!"),
                        _ => println!("🎯 终于猜中了！| Finally got it!"),
                    }
                    
                    break 'guess_loop; // 跳出猜测循环 | Break out of guessing loop
                }
            }
            
            // 检查是否用完所有机会 | Check if all chances are used
            if guess_count >= max_guesses {
                println!("\n💀 游戏结束！你已经用完了所有{}次机会。", max_guesses);
                println!("Game over! You've used all {} chances.", max_guesses);
                println!("答案是: {} | The answer was: {}", secret_number);
                break 'guess_loop; // 跳出猜测循环 | Break out of guessing loop
            }
        }
        
        // 询问是否继续游戏 | Ask if continue playing
        loop {
            println!("\n想再玩一局吗？(y/n) | Want to play again? (y/n)");
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("无法读取输入 | Failed to read input");
            
            match input.trim().to_lowercase().as_str() {
                "y" | "yes" | "是" => {
                    break; // 跳出询问循环，继续主游戏循环 | Break ask loop, continue main game loop
                },
                "n" | "no" | "否" => {
                    println!("👋 谢谢游戏！再见！| Thanks for playing! Goodbye!");
                    break 'game_loop; // 跳出主游戏循环 | Break out of main game loop
                },
                _ => {
                    println!("❌ 请输入 y 或 n | Please enter y or n");
                    continue; // 继续询问 | Continue asking
                }
            }
        }
    }
}

// 生成随机数的辅助函数 | Helper function to generate random number
fn generate_random_number() -> u32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    // 使用当前时间作为种子生成伪随机数 | Use current time as seed for pseudo-random number
    let mut hasher = DefaultHasher::new();
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .hash(&mut hasher);
    
    // 生成1-100范围内的数字 | Generate number in range 1-100
    (hasher.finish() % 100) as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_random_number_generation() {
        // 测试随机数生成在有效范围内 | Test random number generation is in valid range
        for _ in 0..100 {
            let num = generate_random_number();
            assert!(num >= 1 && num <= 100, "生成的随机数应该在1-100范围内");
        }
    }
    
    #[test]
    fn test_control_flow_logic() {
        // 测试比较逻辑 | Test comparison logic
        use std::cmp::Ordering;
        
        assert_eq!(5_u32.cmp(&10), Ordering::Less);
        assert_eq!(10_u32.cmp(&5), Ordering::Greater);
        assert_eq!(5_u32.cmp(&5), Ordering::Equal);
    }
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确使用了loop创建主游戏循环？| Does the project correctly use loop for the main game loop?
2. 条件语句是否正确处理了不同的猜测结果？| Do conditional statements correctly handle different guess results?
3. 代码是否体现了break和continue的正确使用？| Does the code demonstrate correct usage of break and continue?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **条件语句强化练习 | Conditional Statement Reinforcement Exercise**
   - **练习描述 | Exercise Description:** 创建一个成绩等级判断程序，使用多重if-else结构
   - **概念检查 | Concept Check:** if表达式可以返回值并用于变量赋值吗？
   - **学习目标 | Learning Objective:** 深入理解条件表达式的多种用法

2. **循环类型选择练习 | Loop Type Selection Exercise**
   - **练习描述 | Exercise Description:** 实现相同功能的三个版本：分别使用for、while、loop
   - **概念检查 | Concept Check:** 什么情况下应该选择哪种循环类型？
   - **学习目标 | Learning Objective:** 掌握不同循环类型的最佳应用场景

3. **流程控制整合练习 | Flow Control Integration Exercise**
   - **练习描述 | Exercise Description:** 创建一个菜单驱动的计算器程序
   - **概念检查 | Concept Check:** 如何组合使用多种控制流结构？
   - **学习目标 | Learning Objective:** 培养复杂逻辑的设计能力

4. **嵌套循环问题解决练习 | Nested Loop Problem-Solving Exercise**
   - **练习描述 | Exercise Description:** 实现九九乘法表和图形绘制程序
   - **概念检查 | Concept Check:** 如何使用循环标签控制嵌套循环？
   - **学习目标 | Learning Objective:** 掌握复杂嵌套结构的控制技巧

5. **错误处理与控制流练习 | Error Handling and Control Flow Exercise**
   - **练习描述 | Exercise Description:** 创建一个输入验证和重试机制
   - **概念检查 | Concept Check:** 如何在循环中处理错误输入？
   - **学习目标 | Learning Objective:** 结合错误处理设计健壮的程序流程

6. **性能优化练习 | Performance Optimization Exercise**
   - **练习描述 | Exercise Description:** 优化一个查找算法，减少不必要的循环
   - **概念检查 | Concept Check:** 如何使用early return和break优化性能？
   - **学习目标 | Learning Objective:** 理解控制流对程序性能的影响

7. **实际应用场景练习 | Real-world Application Exercise**
   - **练习描述 | Exercise Description:** 实现一个简单的文本处理工具
   - **概念检查 | Concept Check:** 如何将控制流概念应用到实际问题中？
   - **学习目标 | Learning Objective:** 提升解决实际问题的能力

## 学习资源 | Learning Resources
- [Rust官方文档 - 控制流](https://doc.rust-lang.org/book/ch03-05-control-flow.html) | [Rust Official Documentation - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust循环和迭代深入指南](https://doc.rust-lang.org/std/iter/index.html) | [Rust Loops and Iteration In-depth Guide](https://doc.rust-lang.org/std/iter/index.html)
- [控制流最佳实践](https://rust-lang.github.io/api-guidelines/) | [Control Flow Best Practices](https://rust-lang.github.io/api-guidelines/)
- [Rust编程风格指南](https://doc.rust-lang.org/1.0.0/style/) | [Rust Programming Style Guide](https://doc.rust-lang.org/1.0.0/style/)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解if/else条件语句的表达式特性 | Understand if/else conditional statements as expressions
- [ ] 掌握loop、while、for三种循环的使用场景 | Master usage scenarios for loop, while, for
- [ ] 能够正确使用break和continue控制流程 | Correctly use break and continue for flow control
- [ ] 理解循环标签在嵌套结构中的作用 | Understand the role of loop labels in nested structures
- [ ] 完成猜数字游戏实践项目 | Complete the number guessing game practical project
- [ ] 所有CCQs都能正确回答 | All CCQs answered correctly
- [ ] 代码示例理解并能够独立运行 | Code examples understood and can run independently
- [ ] 掌握控制流的最佳实践原则 | Master best practices for control flow
- [ ] 能够识别和避免常见的控制流误区 | Can identify and avoid common control flow misconceptions
- [ ] 至少完成3个扩展练习 | At least 3 extension exercises completed

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释每个核心概念。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain each core concept to others.