# Rust入门 - 第24天：闭包与迭代器 | Rust Introduction - Day 24: Closures and Iterators

## 学习目标 | Learning Objectives
- 理解闭包的概念和语法 | Understand closure concepts and syntax
- 掌握闭包环境捕获机制 | Master closure environment capturing mechanisms
- 学会使用Iterator trait及其方法 | Learn to use Iterator trait and its methods
- 熟练运用map、filter、collect等迭代器适配器 | Proficiently use iterator adapters like map, filter, collect
- 掌握函数式编程风格的数据处理 | Master functional programming style data processing
- 理解惰性求值与迭代器性能优化 | Understand lazy evaluation and iterator performance optimization

## 详细内容 | Detailed Content

### 1. 闭包基础概念 | Closure Fundamentals (1.5小时 | 1.5 hours)

- **闭包定义与特性 | Closure Definition and Characteristics**
  
  **概念定义 | Concept Definition:**
  闭包是可以捕获其环境的匿名函数，它能够访问定义时作用域内的变量，并且可以赋值给变量或作为参数传递 | Closures are anonymous functions that can capture their environment, accessing variables from the scope where they're defined and can be assigned to variables or passed as parameters
  
  **核心特征 | Key Characteristics:**
  - 匿名性：无需命名即可定义和使用 | Anonymity: can be defined and used without naming
  - 环境捕获：可以访问定义时作用域内的变量 | Environment capture: can access variables from defining scope
  - 类型推断：编译器自动推断参数和返回值类型 | Type inference: compiler automatically infers parameter and return types
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 闭包可以访问定义它的作用域中的变量吗？| Can closures access variables from the scope where they are defined?
     **答案 | Answer:** 是 | Yes - 这是闭包的核心特性，称为环境捕获 | This is the core feature of closures, called environment capture
  2. 闭包必须显式声明参数类型吗？| Must closures explicitly declare parameter types?  
     **答案 | Answer:** 否 | No - 编译器可以通过上下文推断类型 | The compiler can infer types from context
  3. 闭包和普通函数在内存中的存储方式相同吗？| Do closures and regular functions have the same memory storage?
     **答案 | Answer:** 否 | No - 闭包可能需要额外存储捕获的环境变量 | Closures may need additional storage for captured environment variables
  4. 每个闭包都有唯一的类型吗？| Does each closure have a unique type?
     **答案 | Answer:** 是 | Yes - 即使语法相同的闭包也有不同的类型 | Even syntactically identical closures have different types
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      // 基本闭包语法 | Basic closure syntax
      let add_one = |x| x + 1;
      println!("5 + 1 = {}", add_one(5)); // 输出：6 | Output: 6
      
      // 环境捕获示例 | Environment capture example
      let multiplier = 3;
      let multiply_by_three = |x| x * multiplier; // 捕获multiplier变量 | Captures multiplier variable
      println!("4 * 3 = {}", multiply_by_three(4)); // 输出：12 | Output: 12
      
      // 显式类型注解 | Explicit type annotation
      let divide: fn(i32, i32) -> i32 = |a, b| a / b;
      println!("10 / 2 = {}", divide(10, 2)); // 输出：5 | Output: 5
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码会输出什么？| What will this code output?
    **答案 | Answer:** 6, 12, 5
  - 如果改变multiplier的值会发生什么？| What happens if we change the value of multiplier?
    **答案 | Answer:** 闭包会使用定义时捕获的值，而非运行时的值 | The closure uses the value captured at definition time, not runtime
  
  **常见误区检查 | Common Misconception Checks:**
  - 闭包会自动复制所有外部变量吗？| Do closures automatically copy all external variables?
    **答案 | Answer:** 否，只捕获实际使用的变量 | No, they only capture variables that are actually used
  - 闭包的类型可以显式声明为fn类型吗？| Can closure types be explicitly declared as fn types?
    **答案 | Answer:** 只有不捕获环境的闭包可以，否则需要使用Fn traits | Only non-capturing closures can, otherwise Fn traits are needed

### 2. 闭包环境捕获机制 | Closure Environment Capture Mechanisms (1.5小时 | 1.5 hours)

- **捕获模式详解 | Capture Modes Explained**
  
  **概念定义 | Concept Definition:**
  Rust中的闭包有三种捕获模式：FnOnce（获取所有权）、FnMut（可变借用）、Fn（不可变借用），编译器会自动选择最适合的模式 | Rust closures have three capture modes: FnOnce (taking ownership), FnMut (mutable borrow), Fn (immutable borrow), with the compiler automatically selecting the most appropriate mode
  
  **与基础概念的关系 | Relationship to Basic Concepts:**
  建立在Rust所有权系统的基础上，将借用检查器的规则应用到闭包的环境捕获中 | Built on Rust's ownership system, applying borrow checker rules to closure environment capture
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. FnOnce trait表示闭包可以调用多少次？| How many times can a closure implementing FnOnce trait be called?
     **答案 | Answer:** 最多一次 | At most once - 因为它获取了环境的所有权 | Because it takes ownership of the environment
  2. 如果闭包只读取外部变量，它实现哪个trait？| If a closure only reads external variables, which trait does it implement?
     **答案 | Answer:** Fn - 因为它只需要不可变借用 | Because it only needs immutable borrow
  3. move关键字会改变闭包的捕获模式吗？| Does the move keyword change the closure's capture mode?
     **答案 | Answer:** 是 | Yes - 强制闭包获取所有使用变量的所有权 | Forces the closure to take ownership of all used variables
  4. 实现Fn的闭包也自动实现FnMut和FnOnce吗？| Do closures implementing Fn automatically implement FnMut and FnOnce?
     **答案 | Answer:** 是 | Yes - 这是trait继承关系的结果 | This is the result of trait inheritance relationships
  
  **复杂代码示例 | Complex Code Examples:**
  ```rust
  fn main() {
      // Fn: 只读取环境 | Only reads environment
      let name = String::from("Rust");
      let greet = || println!("Hello, {}!", name); // 不可变借用name | Immutable borrow of name
      greet(); // 可以多次调用 | Can be called multiple times
      greet();
      println!("Original name: {}", name); // name仍然有效 | name is still valid
      
      // FnMut: 修改环境 | Modifies environment  
      let mut counter = 0;
      let mut increment = || {
          counter += 1; // 可变借用counter | Mutable borrow of counter
          println!("Count: {}", counter);
      };
      increment(); // 输出：Count: 1 | Output: Count: 1
      increment(); // 输出：Count: 2 | Output: Count: 2
      println!("Final counter: {}", counter); // counter被修改了 | counter was modified
      
      // FnOnce: 移动环境 | Moves environment
      let data = vec![1, 2, 3, 4, 5];
      let consume = move || {
          println!("Data: {:?}", data); // 获取data的所有权 | Takes ownership of data
          data // 返回data，消费了它 | Returns data, consuming it
      };
      let _result = consume(); // 只能调用一次 | Can only be called once
      // consume(); // 编译错误！闭包已被消费 | Compile error! Closure already consumed
      
      // move强制获取所有权 | move forces taking ownership
      let value = 42;
      let moved_closure = move || println!("Value: {}", value); // 即使只读取，也获取所有权 | Takes ownership even though only reading
      moved_closure();
      // println!("{}", value); // 编译错误！value已被移动 | Compile error! value has been moved
  }
  ```
  
  **综合应用检查 | Comprehensive Application Check:**
  - 如何根据使用需求选择合适的捕获模式？| How to choose appropriate capture mode based on usage needs?
  - 什么情况下需要使用move关键字？| When is the move keyword needed?
  - 如何处理闭包生命周期问题？| How to handle closure lifetime issues?

### 3. Iterator Trait详解 | Iterator Trait Detailed Explanation (1小时 | 1 hour)

- **Iterator核心概念 | Iterator Core Concepts**
  
  **概念定义 | Concept Definition:**
  Iterator是Rust中用于遍历集合的标准trait，提供了惰性求值的迭代方式，只在需要时才计算下一个元素 | Iterator is Rust's standard trait for traversing collections, providing lazy evaluation that only computes the next element when needed
  
  **解决的问题 | Problems It Solves:**
  - 统一的遍历接口：为不同类型的集合提供一致的遍历方式 | Unified traversal interface: provides consistent traversal for different collection types
  - 内存效率：惰性求值避免不必要的计算和内存分配 | Memory efficiency: lazy evaluation avoids unnecessary computation and memory allocation
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Iterator是惰性求值的吗？| Is Iterator lazy evaluated?
     **答案 | Answer:** 是 | Yes - 只在调用消费适配器时才执行计算 | Only performs computation when consuming adaptors are called
  2. 所有实现Iterator的类型都必须实现next方法吗？| Must all types implementing Iterator implement the next method?
     **答案 | Answer:** 是 | Yes - next是Iterator trait的唯一必需方法 | next is the only required method of Iterator trait
  3. 迭代器适配器会立即执行吗？| Do iterator adaptors execute immediately?
     **答案 | Answer:** 否 | No - 它们返回新的迭代器，需要消费适配器触发执行 | They return new iterators, requiring consuming adaptors to trigger execution
  4. for循环可以用于任何实现Iterator的类型吗？| Can for loops work with any type implementing Iterator?
     **答案 | Answer:** 是 | Yes - for循环会自动调用into_iter() | for loops automatically call into_iter()
  
  **实际应用示例 | Real-world Application Examples:**
  ```rust
  fn main() {
      // 创建迭代器的几种方式 | Different ways to create iterators
      let vec = vec![1, 2, 3, 4, 5];
      
      // iter(): 产生&T类型的迭代器 | Produces iterator of &T
      for item in vec.iter() {
          println!("Reference: {}", item); // item是&i32类型 | item is of type &i32
      }
      
      // into_iter(): 产生T类型的迭代器（获取所有权）| Produces iterator of T (takes ownership)
      let vec2 = vec![10, 20, 30];
      for item in vec2.into_iter() {
          println!("Owned: {}", item); // item是i32类型 | item is of type i32
      }
      // println!("{:?}", vec2); // 编译错误！vec2已被移动 | Compile error! vec2 has been moved
      
      // iter_mut(): 产生&mut T类型的迭代器 | Produces iterator of &mut T
      let mut vec3 = vec![100, 200, 300];
      for item in vec3.iter_mut() {
          *item *= 2; // 修改原始值 | Modify original value
      }
      println!("Modified: {:?}", vec3); // 输出：[200, 400, 600] | Output: [200, 400, 600]
      
      // 手动实现迭代器 | Manual iterator implementation
      struct Counter {
          current: i32,
          max: i32,
      }
      
      impl Counter {
          fn new(max: i32) -> Counter {
              Counter { current: 0, max }
          }
      }
      
      impl Iterator for Counter {
          type Item = i32;
          
          fn next(&mut self) -> Option<Self::Item> {
              if self.current < self.max {
                  let current = self.current;
                  self.current += 1;
                  Some(current)
              } else {
                  None
              }
          }
      }
      
      // 使用自定义迭代器 | Using custom iterator
      let counter = Counter::new(3);
      for num in counter {
          println!("Count: {}", num); // 输出：0, 1, 2 | Output: 0, 1, 2
      }
  }
  ```

### 4. 迭代器适配器与消费适配器 | Iterator Adaptors and Consuming Adaptors (1.5小时 | 1.5 hours)

- **适配器类型与应用 | Adaptor Types and Applications**
  
  **多概念整合 | Multi-concept Integration:**
  将迭代器与闭包结合，利用函数式编程思想处理数据集合，实现链式操作和数据转换管道 | Combines iterators with closures, using functional programming concepts to process data collections and implement chained operations and data transformation pipelines
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. map()是迭代器适配器还是消费适配器？| Is map() an iterator adaptor or consuming adaptor?
     **答案 | Answer:** 迭代器适配器 | Iterator adaptor - 它返回新的迭代器而不消费原始迭代器 | It returns a new iterator without consuming the original
  2. collect()会触发惰性迭代器的执行吗？| Does collect() trigger execution of lazy iterators?
     **答案 | Answer:** 是 | Yes - collect()是消费适配器，会消费整个迭代器 | collect() is a consuming adaptor that consumes the entire iterator
  3. 可以在同一个迭代器链中使用多个适配器吗？| Can multiple adaptors be used in the same iterator chain?
     **答案 | Answer:** 是 | Yes - 适配器可以链式组合，只有最后的消费适配器会触发执行 | Adaptors can be chained, with only the final consuming adaptor triggering execution
  4. filter()和map()的执行顺序重要吗？| Does the order of filter() and map() matter?
     **答案 | Answer:** 是 | Yes - 不同顺序会影响性能和结果 | Different orders affect performance and results
  
  **复杂代码示例 | Complex Code Examples:**
  ```rust
  fn main() {
      let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
      
      // 链式操作：过滤、映射、收集 | Chained operations: filter, map, collect
      let processed: Vec<String> = numbers
          .iter()                              // 创建迭代器 | Create iterator
          .filter(|&x| x % 2 == 0)            // 过滤偶数 | Filter even numbers
          .map(|x| x * x)                     // 平方运算 | Square operation
          .map(|x| format!("{}²", x))         // 格式化为字符串 | Format as string
          .collect();                         // 收集结果 | Collect results
      
      println!("Processed: {:?}", processed); // 输出：["4²", "16²", "36²", "64²", "100²"] | Output: ["4²", "16²", "36²", "64²", "100²"]
      
      // 使用enumerate获取索引 | Using enumerate to get indices
      let words = vec!["hello", "world", "rust", "programming"];
      let indexed: Vec<(usize, &str)> = words
          .iter()
          .enumerate()
          .filter(|(i, _)| i % 2 == 0)        // 只保留偶数索引 | Keep only even indices
          .map(|(i, &word)| (i, word))
          .collect();
      
      println!("Indexed: {:?}", indexed);     // 输出：[(0, "hello"), (2, "rust")] | Output: [(0, "hello"), (2, "rust")]
      
      // 复杂数据结构处理 | Complex data structure processing
      #[derive(Debug)]
      struct Person {
          name: String,
          age: u32,
          city: String,
      }
      
      let people = vec![
          Person { name: "Alice".to_string(), age: 25, city: "New York".to_string() },
          Person { name: "Bob".to_string(), age: 30, city: "San Francisco".to_string() },
          Person { name: "Charlie".to_string(), age: 22, city: "New York".to_string() },
          Person { name: "Diana".to_string(), age: 28, city: "Los Angeles".to_string() },
      ];
      
      // 查找年龄大于25且来自纽约的人的姓名 | Find names of people older than 25 from New York
      let ny_adults: Vec<String> = people
          .iter()
          .filter(|person| person.age > 25)
          .filter(|person| person.city == "New York")
          .map(|person| person.name.clone())
          .collect();
      
      println!("NY Adults: {:?}", ny_adults); // 输出：["Alice"] | Output: ["Alice"]
      
      // 使用fold进行累积操作 | Using fold for accumulation
      let sum = (1..=100)
          .filter(|&x| x % 3 == 0)            // 3的倍数 | Multiples of 3
          .fold(0, |acc, x| acc + x);         // 累加 | Accumulate
      
      println!("Sum of multiples of 3 (1-100): {}", sum); // 输出：1683 | Output: 1683
      
      // 使用reduce进行归约操作 | Using reduce for reduction
      let max_value = vec![45, 23, 78, 12, 89, 34]
          .into_iter()
          .reduce(|acc, x| if x > acc { x } else { acc });
      
      match max_value {
          Some(max) => println!("Maximum value: {}", max), // 输出：89 | Output: 89
          None => println!("No maximum found"),
      }
  }
  ```
  
  **综合应用检查 | Comprehensive Application Check:**
  - 如何优化迭代器链的性能？| How to optimize iterator chain performance?
  - 何时使用for_each而不是collect？| When to use for_each instead of collect?
  - 如何处理迭代器中的错误传播？| How to handle error propagation in iterators?

### 5. 性能优化与最佳实践 | Performance Optimization and Best Practices (30分钟 | 30 minutes)

- **迭代器性能优化 | Iterator Performance Optimization**
  
  **关键原则 | Key Principles:**
  - 零成本抽象：Rust的迭代器在编译时优化为等效的循环代码 | Zero-cost abstraction: Rust iterators are optimized to equivalent loop code at compile time
  - 惰性求值：避免不必要的中间集合分配 | Lazy evaluation: avoid unnecessary intermediate collection allocation
  - 类型专化：利用编译器优化实现最佳性能 | Type specialization: leverage compiler optimizations for best performance
  
  **实践验证问题 | Practice Verification Questions:**
  1. 迭代器链比手写循环慢吗？| Are iterator chains slower than hand-written loops?
     **答案 | Answer:** 否 | No - 编译器会将迭代器优化为等效的循环代码 | The compiler optimizes iterators to equivalent loop code
  2. 何时应该避免使用collect()？| When should collect() be avoided?
     **答案 | Answer:** 当不需要拥有完整集合时 | When you don't need to own the complete collection
  3. filter在map之前还是之后更高效？| Is filter more efficient before or after map?
     **答案 | Answer:** 通常在map之前，可以减少map操作的次数 | Usually before map, to reduce the number of map operations

### 6. 函数式编程模式 | Functional Programming Patterns (30分钟 | 30 minutes)

- **函数式数据处理管道 | Functional Data Processing Pipeline**
  
  **综合概念检查 | Comprehensive Concept Check:**
  1. 如何将命令式循环转换为函数式迭代器链？| How to convert imperative loops to functional iterator chains?
     **答案 | Answer:** 识别操作类型（过滤、映射、归约）并使用相应的迭代器方法 | Identify operation types (filter, map, reduce) and use corresponding iterator methods
  2. 闭包在函数式编程中扮演什么角色？| What role do closures play in functional programming?
     **答案 | Answer:** 作为高阶函数的参数，定义数据转换逻辑 | Serve as arguments to higher-order functions, defining data transformation logic
  3. 如何在迭代器中处理副作用？| How to handle side effects in iterators?
     **答案 | Answer:** 使用for_each或在闭包中执行副作用操作 | Use for_each or execute side effects within closures
  4. 迭代器适配器的组合顺序如何影响性能？| How does the order of iterator adaptor composition affect performance?
     **答案 | Answer:** 早期过滤可以减少后续操作的数据量 | Early filtering can reduce data volume for subsequent operations
  5. 何时选择迭代器而不是传统循环？| When to choose iterators over traditional loops?
     **答案 | Answer:** 当需要链式操作、函数式风格或更好的可读性时 | When chained operations, functional style, or better readability is needed

## 实践项目：数据处理管道 | Practical Project: Data Processing Pipeline

### 目标 | Objective
构建一个综合的数据处理系统，演示闭包和迭代器在实际场景中的应用，包括文件处理、数据分析和结果输出 | Build a comprehensive data processing system demonstrating closures and iterators in real scenarios, including file processing, data analysis, and result output

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 闭包的三种捕获模式有什么区别？| What are the differences between the three closure capture modes?
   **答案 | Answer:** Fn（不可变借用，可多次调用）、FnMut（可变借用，可多次调用）、FnOnce（获取所有权，只能调用一次）| Fn (immutable borrow, callable multiple times), FnMut (mutable borrow, callable multiple times), FnOnce (takes ownership, callable only once)
2. 迭代器适配器和消费适配器的区别是什么？| What's the difference between iterator adaptors and consuming adaptors?
   **答案 | Answer:** 适配器返回新的迭代器（惰性），消费适配器消费迭代器并产生最终结果 | Adaptors return new iterators (lazy), consuming adaptors consume iterators and produce final results
3. 何时使用move闭包？| When to use move closures?
   **答案 | Answer:** 当闭包需要获取环境变量的所有权，特别是在多线程或异步场景中 | When closures need to take ownership of environment variables, especially in multithreading or async scenarios

### 步骤 | Steps
1. **设计数据结构**：定义学生成绩记录结构体 | Design data structures: define student grade record struct
2. **实现文件读取**：使用迭代器处理CSV格式数据 | Implement file reading: use iterators to process CSV format data
3. **数据过滤与转换**：应用各种迭代器适配器 | Data filtering and transformation: apply various iterator adaptors
4. **统计计算**：使用闭包实现自定义统计函数 | Statistical calculation: use closures to implement custom statistical functions
5. **结果输出**：格式化并输出处理结果 | Result output: format and output processing results

### 示例代码 | Example Code
```rust
"""
数据处理管道系统 | Data Processing Pipeline System
一个演示闭包和迭代器综合应用的项目 | A project demonstrating comprehensive application of closures and iterators

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 闭包的环境捕获和不同trait实现 | Closure environment capture and different trait implementations
- 迭代器适配器的链式组合 | Chained composition of iterator adaptors
- 函数式编程的数据处理模式 | Functional programming data processing patterns
"""

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug, Clone)]
struct StudentRecord {
    name: String,
    subject: String,
    score: f64,
    grade_level: u32,
}

impl StudentRecord {
    fn new(name: String, subject: String, score: f64, grade_level: u32) -> Self {
        StudentRecord { name, subject, score, grade_level }
    }
    
    // 闭包示例：创建分级函数 | Closure example: create grading function
    fn create_grader() -> impl Fn(f64) -> String {
        |score| {
            match score {
                s if s >= 90.0 => "A".to_string(),
                s if s >= 80.0 => "B".to_string(),
                s if s >= 70.0 => "C".to_string(),
                s if s >= 60.0 => "D".to_string(),
                _ => "F".to_string(),
            }
        }
    }
}

struct DataProcessor {
    records: Vec<StudentRecord>,
}

impl DataProcessor {
    fn new() -> Self {
        DataProcessor { records: Vec::new() }
    }
    
    // 从文件加载数据，演示错误处理与迭代器 | Load data from file, demonstrate error handling with iterators
    fn load_from_file(&mut self, filename: &str) -> Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        
        // 使用迭代器处理文件行 | Use iterator to process file lines
        self.records = reader
            .lines()
            .skip(1)  // 跳过标题行 | Skip header line
            .filter_map(|line| {
                line.ok().and_then(|l| {
                    let parts: Vec<&str> = l.split(',').collect();
                    if parts.len() == 4 {
                        let name = parts[0].trim().to_string();
                        let subject = parts[1].trim().to_string();
                        let score = parts[2].trim().parse().ok()?;
                        let grade_level = parts[3].trim().parse().ok()?;
                        Some(StudentRecord::new(name, subject, score, grade_level))
                    } else {
                        None
                    }
                })
            })
            .collect();
        
        Ok(())
    }
    
    // 添加测试数据 | Add test data
    fn add_test_data(&mut self) {
        let test_data = vec![
            ("Alice Johnson", "Mathematics", 92.5, 10),
            ("Bob Smith", "Mathematics", 78.0, 10),
            ("Charlie Brown", "Science", 85.5, 11),
            ("Diana Wilson", "Mathematics", 95.0, 11),
            ("Eve Davis", "Science", 72.0, 10),
            ("Frank Miller", "English", 88.0, 10),
            ("Grace Lee", "English", 91.5, 11),
            ("Henry Kim", "Science", 79.5, 11),
        ];
        
        self.records = test_data
            .into_iter()
            .map(|(name, subject, score, grade)| {
                StudentRecord::new(name.to_string(), subject.to_string(), score, grade)
            })
            .collect();
    }
    
    // 使用闭包进行条件过滤 | Use closures for conditional filtering
    fn filter_records<F>(&self, predicate: F) -> Vec<&StudentRecord>
    where
        F: Fn(&StudentRecord) -> bool,
    {
        self.records.iter().filter(|record| predicate(record)).collect()
    }
    
    // 高阶函数：返回自定义过滤闭包 | Higher-order function: return custom filter closure
    fn create_subject_filter(subject: &str) -> impl Fn(&StudentRecord) -> bool + '_ {
        move |record: &StudentRecord| record.subject == subject
    }
    
    // 复杂的数据分析：使用多个迭代器适配器 | Complex data analysis: using multiple iterator adaptors
    fn analyze_performance(&self) -> HashMap<String, (f64, usize, f64, f64)> {
        // 分组统计：平均分、学生数量、最高分、最低分 | Group statistics: average, student count, highest, lowest
        let mut subject_stats = HashMap::new();
        
        // 按科目分组 | Group by subject
        for subject in self.records.iter().map(|r| &r.subject).collect::<std::collections::HashSet<_>>() {
            let subject_records: Vec<&StudentRecord> = self.records
                .iter()
                .filter(|record| &record.subject == subject)
                .collect();
            
            if !subject_records.is_empty() {
                // 计算统计信息 | Calculate statistics
                let scores: Vec<f64> = subject_records.iter().map(|r| r.score).collect();
                let avg = scores.iter().sum::<f64>() / scores.len() as f64;
                let count = scores.len();
                let max = scores.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let min = scores.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                
                subject_stats.insert(subject.clone(), (avg, count, max, min));
            }
        }
        
        subject_stats
    }
    
    // 函数式编程风格的成绩分析 | Functional programming style grade analysis
    fn functional_analysis(&self) -> Vec<(String, String, f64)> {
        let grader = StudentRecord::create_grader();
        
        self.records
            .iter()
            .map(|record| {
                let grade = grader(record.score);
                (record.name.clone(), record.subject.clone(), record.score)
            })
            .filter(|(_, _, score)| *score >= 80.0)  // 只保留B级以上成绩 | Keep only B grade and above
            .collect()
    }
    
    // 使用fold进行复杂聚合 | Use fold for complex aggregation
    fn calculate_gpa(&self, student_name: &str) -> Option<f64> {
        let student_scores: Vec<f64> = self.records
            .iter()
            .filter(|record| record.name == student_name)
            .map(|record| record.score)
            .collect();
        
        if student_scores.is_empty() {
            None
        } else {
            let total_points = student_scores
                .iter()
                .fold(0.0, |acc, &score| {
                    acc + match score {
                        s if s >= 90.0 => 4.0,
                        s if s >= 80.0 => 3.0,
                        s if s >= 70.0 => 2.0,
                        s if s >= 60.0 => 1.0,
                        _ => 0.0,
                    }
                });
            Some(total_points / student_scores.len() as f64)
        }
    }
    
    // 演示不同类型的闭包 | Demonstrate different types of closures
    fn demonstrate_closure_types(&self) {
        println!("\n=== 闭包类型演示 | Closure Types Demonstration ===");
        
        // Fn闭包：只读环境 | Fn closure: read-only environment
        let threshold = 85.0;
        let high_performers = self.records
            .iter()
            .filter(|record| record.score > threshold)  // 捕获threshold但不修改 | Captures threshold but doesn't modify
            .count();
        println!("High performers (>{}): {}", threshold, high_performers);
        
        // FnMut闭包：修改环境 | FnMut closure: modifies environment
        let mut total_processed = 0;
        let _averages: Vec<f64> = self.records
            .iter()
            .map(|record| {
                total_processed += 1;  // 修改外部变量 | Modifies external variable
                record.score
            })
            .collect();
        println!("Total records processed: {}", total_processed);
        
        // FnOnce闭包：获取所有权 | FnOnce closure: takes ownership
        let subject_filter = String::from("Mathematics");
        let math_records = self.records
            .iter()
            .filter(move |record| record.subject == subject_filter)  // 获取subject_filter的所有权 | Takes ownership of subject_filter
            .count();
        println!("Mathematics records: {}", math_records);
        // println!("{}", subject_filter); // 编译错误！已被移动 | Compile error! Has been moved
    }
    
    // 链式迭代器操作展示 | Chained iterator operations showcase
    fn showcase_iterator_chains(&self) {
        println!("\n=== 迭代器链式操作展示 | Iterator Chains Showcase ===");
        
        // 复杂链式操作：多级过滤、转换、聚合 | Complex chained operations: multi-level filtering, transformation, aggregation
        let top_students: Vec<String> = self.records
            .iter()
            .filter(|record| record.grade_level >= 11)     // 过滤11年级以上 | Filter grade 11 and above
            .filter(|record| record.score >= 85.0)        // 过滤高分学生 | Filter high-scoring students
            .map(|record| &record.name)                    // 提取姓名 | Extract names
            .collect::<std::collections::HashSet<_>>()     // 去重 | Deduplicate
            .into_iter()
            .collect();
        
        println!("Top students (Grade 11+, Score 85+): {:?}", top_students);
        
        // 使用enumerate和复杂条件 | Using enumerate with complex conditions
        let indexed_high_scores: Vec<(usize, &str, f64)> = self.records
            .iter()
            .enumerate()
            .filter(|(_, record)| record.score > 90.0)
            .map(|(i, record)| (i, record.name.as_str(), record.score))
            .collect();
        
        println!("Indexed high scores: {:?}", indexed_high_scores);
    }
}

fn main() -> Result<()> {
    println!("=== 数据处理管道系统 | Data Processing Pipeline System ===\n");
    
    let mut processor = DataProcessor::new();
    
    // 添加测试数据 | Add test data
    processor.add_test_data();
    
    // 演示不同的过滤方式 | Demonstrate different filtering methods
    println!("1. 基础过滤操作 | Basic Filtering Operations");
    
    // 使用内联闭包 | Using inline closures
    let high_scorers = processor.filter_records(|record| record.score >= 90.0);
    println!("High scorers (>=90): {:?}", 
        high_scorers.iter().map(|r| &r.name).collect::<Vec<_>>());
    
    // 使用预定义闭包 | Using predefined closures
    let math_filter = DataProcessor::create_subject_filter("Mathematics");
    let math_students = processor.filter_records(math_filter);
    println!("Math students: {:?}", 
        math_students.iter().map(|r| &r.name).collect::<Vec<_>>());
    
    // 复合条件过滤 | Compound condition filtering
    let senior_high_performers = processor.filter_records(|record| {
        record.grade_level >= 11 && record.score >= 85.0
    });
    println!("Senior high performers: {:?}", 
        senior_high_performers.iter().map(|r| (&r.name, r.score)).collect::<Vec<_>>());
    
    // 性能分析 | Performance analysis
    println!("\n2. 科目统计分析 | Subject Statistical Analysis");
    let stats = processor.analyze_performance();
    for (subject, (avg, count, max, min)) in &stats {
        println!("{}: 平均分{:.1}, 学生{}人, 最高{:.1}, 最低{:.1} | Avg {:.1}, {} students, Max {:.1}, Min {:.1}", 
            subject, avg, count, max, min);
    }
    
    // 函数式编程分析 | Functional programming analysis
    println!("\n3. 优秀学生分析 | Excellent Students Analysis");
    let excellent_students = processor.functional_analysis();
    for (name, subject, score) in excellent_students {
        println!("{}: {} - {:.1}分 | {:.1} points", name, subject, score);
    }
    
    // GPA计算 | GPA calculation
    println!("\n4. GPA计算 | GPA Calculation");
    let students: Vec<&str> = processor.records
        .iter()
        .map(|r| r.name.as_str())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    
    for student in students {
        if let Some(gpa) = processor.calculate_gpa(student) {
            println!("{}: GPA {:.2}", student, gpa);
        }
    }
    
    // 闭包类型演示 | Closure types demonstration
    processor.demonstrate_closure_types();
    
    // 迭代器链展示 | Iterator chains showcase
    processor.showcase_iterator_chains();
    
    println!("\n=== 项目完成 | Project Completed ===");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_capture_modes() {
        let mut processor = DataProcessor::new();
        processor.add_test_data();
        
        // 测试Fn闭包 | Test Fn closure
        let threshold = 80.0;
        let count = processor.filter_records(|record| record.score > threshold).len();
        assert!(count > 0);
        
        // 测试FnMut闭包（通过计数器）| Test FnMut closure (via counter)
        let mut counter = 0;
        let _results: Vec<_> = processor.records
            .iter()
            .map(|record| {
                counter += 1;
                record.score
            })
            .collect();
        assert_eq!(counter, processor.records.len());
    }
    
    #[test]
    fn test_iterator_adaptors() {
        let mut processor = DataProcessor::new();
        processor.add_test_data();
        
        // 测试链式操作 | Test chained operations
        let high_score_names: Vec<String> = processor.records
            .iter()
            .filter(|record| record.score >= 90.0)
            .map(|record| record.name.clone())
            .collect();
        
        assert!(!high_score_names.is_empty());
    }
    
    #[test]
    fn test_gpa_calculation() {
        let mut processor = DataProcessor::new();
        processor.add_test_data();
        
        let gpa = processor.calculate_gpa("Diana Wilson");
        assert!(gpa.is_some());
        assert!(gpa.unwrap() > 3.5); // Diana has 95.0, should have high GPA
    }
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确使用了三种不同的闭包类型？| Does the project correctly use three different closure types?
2. 迭代器适配器的链式组合是否实现了预期功能？| Do the chained iterator adaptors achieve expected functionality?
3. 代码是否体现了函数式编程的优势？| Does the code demonstrate functional programming advantages?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **闭包生命周期练习 | Closure Lifetime Exercise**
   - **练习描述 | Exercise Description:** 创建返回闭包的函数，处理生命周期问题
   - **概念检查 | Concept Check:** 闭包如何捕获不同生命周期的变量？
   - **学习目标 | Learning Objective:** 深入理解闭包与生命周期的关系

2. **自定义迭代器实现 | Custom Iterator Implementation**
   - **练习描述 | Exercise Description:** 为自定义数据结构实现Iterator trait
   - **概念检查 | Concept Check:** Iterator trait的核心方法是什么？
   - **学习目标 | Learning Objective:** 掌握迭代器的内部工作原理

3. **高阶函数设计 | Higher-Order Function Design**
   - **练习描述 | Exercise Description:** 设计接受和返回闭包的函数
   - **概念检查 | Concept Check:** 如何处理不同类型的闭包参数？
   - **学习目标 | Learning Objective:** 提高函数式编程设计能力

4. **性能对比分析 | Performance Comparison Analysis**
   - **练习描述 | Exercise Description:** 比较迭代器和传统循环的性能
   - **概念检查 | Concept Check:** 什么是零成本抽象？
   - **学习目标 | Learning Objective:** 理解Rust迭代器的性能优化

5. **并行迭代器探索 | Parallel Iterator Exploration**
   - **练习描述 | Exercise Description:** 使用rayon库实现并行数据处理
   - **概念检查 | Concept Check:** 并行迭代器如何保证线程安全？
   - **学习目标 | Learning Objective:** 了解并行计算与迭代器的结合

6. **错误处理与迭代器 | Error Handling with Iterators**
   - **练习描述 | Exercise Description:** 在迭代器链中优雅处理错误
   - **概念检查 | Concept Check:** 如何在map中处理可能失败的操作？
   - **学习目标 | Learning Objective:** 掌握函数式错误处理模式

7. **流式数据处理 | Stream Data Processing**
   - **练习描述 | Exercise Description:** 实现实时数据流处理系统
   - **概念检查 | Concept Check:** 如何处理无限长度的数据流？
   - **学习目标 | Learning Objective:** 应用迭代器处理实时数据

## 学习资源 | Learning Resources
- [Rust官方文档 - 闭包](https://doc.rust-lang.org/book/ch13-01-closures.html) | [Rust Official Documentation - Closures]
- [Rust官方文档 - 迭代器](https://doc.rust-lang.org/book/ch13-02-iterators.html) | [Rust Official Documentation - Iterators]
- [函数式编程模式指南](https://doc.rust-lang.org/book/ch13-00-functional-features.html) | [Functional Programming Patterns Guide]
- [性能优化最佳实践](https://doc.rust-lang.org/book/ch13-04-performance.html) | [Performance Optimization Best Practices]

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解闭包的定义和基本语法 | Understand closure definition and basic syntax
- [ ] 掌握三种闭包捕获模式的区别 | Master differences between three closure capture modes
- [ ] 熟练使用Iterator trait和常用方法 | Proficiently use Iterator trait and common methods
- [ ] 能够链式组合迭代器适配器 | Able to chain iterator adaptors
- [ ] 理解惰性求值的概念和优势 | Understand lazy evaluation concepts and advantages
- [ ] 掌握函数式编程的数据处理模式 | Master functional programming data processing patterns
- [ ] 完成数据处理管道实践项目 | Complete data processing pipeline practical project
- [ ] 能够优化迭代器链的性能 | Able to optimize iterator chain performance
- [ ] 理解零成本抽象原理 | Understand zero-cost abstraction principles
- [ ] 扩展练习至少完成3个 | At least 3 extension exercises completed

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释闭包捕获机制、迭代器适配器的工作原理以及函数式编程的优势。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain closure capture mechanisms, how iterator adaptors work, and the advantages of functional programming to others.