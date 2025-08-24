# Rust入门 - 第6天：匹配模式基础 | Rust Introduction - Day 6: Pattern Matching Basics

## 学习目标 | Learning Objectives
- 理解模式匹配的核心概念和作用 | Understand the core concepts and purpose of pattern matching
- 掌握match表达式的基本语法和使用场景 | Master basic syntax and use cases of match expressions
- 学会使用各种模式进行值的匹配和解构 | Learn to use various patterns for value matching and destructuring
- 理解if let语法糖及其适用场景 | Understand if let syntax sugar and its applicable scenarios
- 掌握模式匹配中的通配符和绑定变量 | Master wildcards and binding variables in pattern matching
- 能够运用模式匹配解决实际编程问题 | Apply pattern matching to solve real programming problems

## 详细内容 | Detailed Content

### 1. match表达式基础 | Basic match Expressions (1小时 | 1 hour)

- **match表达式语法 | match Expression Syntax**
  
  **概念定义 | Concept Definition:**
  match表达式是Rust中用于模式匹配的强大控制流构造，它将一个值与一系列模式进行比较，并执行匹配的第一个模式对应的代码块。与其他语言的switch语句不同，match必须处理所有可能的情况，确保代码的完整性和安全性。| match expressions are powerful control flow constructs in Rust used for pattern matching, comparing a value against a series of patterns and executing the code block corresponding to the first matching pattern. Unlike switch statements in other languages, match must handle all possible cases, ensuring code completeness and safety.
  
  **核心特征 | Key Characteristics:**
  - 必须穷尽所有可能的模式 (exhaustiveness) | Must exhaustively cover all possible patterns
  - 每个分支必须返回相同类型的值 | Each branch must return values of the same type
  - 模式匹配按顺序进行，第一个匹配的模式会被执行 | Pattern matching occurs in order, first matching pattern is executed
  - 支持复杂的模式解构和值绑定 | Supports complex pattern destructuring and value binding
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. match表达式必须处理所有可能的情况吗？| Must match expressions handle all possible cases?
     **答案 | Answer:** 是 | Yes - Rust编译器强制要求穷尽性，确保没有遗漏的情况 | Rust compiler enforces exhaustiveness to ensure no cases are missed
  2. match的所有分支可以返回不同类型的值吗？| Can all branches of a match return different types of values?  
     **答案 | Answer:** 否 | No - 所有分支必须返回相同类型，确保类型安全 | All branches must return the same type to ensure type safety
  3. 如果多个模式都匹配，会执行哪一个？| If multiple patterns match, which one will be executed?
     **答案 | Answer:** 第一个匹配的模式 | The first matching pattern - 模式匹配按从上到下的顺序进行 | Pattern matching proceeds from top to bottom
  4. 可以在match中使用变量绑定吗？| Can variable binding be used in match?
     **答案 | Answer:** 是 | Yes - 可以通过模式绑定从匹配的值中提取数据 | Can extract data from matched values through pattern binding
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 基本match表达式 | Basic match expression
  fn describe_number(n: i32) -> String {
      match n {
          0 => String::from("零"), // 精确匹配 | Exact match
          1 => String::from("一"), // 另一个精确匹配 | Another exact match
          2..=10 => String::from("小数"), // 范围匹配 | Range match
          _ => String::from("其他数字"), // 通配符，捕获所有其他情况 | Wildcard, catches all other cases
      }
  }

  fn main() {
      println!("{}", describe_number(0));    // 输出：零 | Output: 零
      println!("{}", describe_number(5));    // 输出：小数 | Output: 小数
      println!("{}", describe_number(100));  // 输出：其他数字 | Output: 其他数字
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码会输出什么？| What will this code output?
    **答案 | Answer:** 零\n小数\n其他数字 | 零\n小数\n其他数字
  - 如果去掉 `_ =>` 分支会发生什么？| What happens if we remove the `_ =>` branch?
    **答案 | Answer:** 编译错误 | Compilation error - 因为没有覆盖所有可能的i32值 | because not all possible i32 values are covered
  
  **常见误区检查 | Common Misconception Checks:**
  - match分支可以像其他语言的switch一样"落穿"到下一个分支吗？| Can match branches "fall through" to the next branch like switch in other languages?
    **答案 | Answer:** 否 | No - Rust的match分支是独立的，不会自动执行下一个分支 | Rust match branches are independent and won't automatically execute the next branch
  - 是否可以省略某些可能的情况？| Can some possible cases be omitted?
    **答案 | Answer:** 否 | No - 除非使用通配符_，否则必须显式处理所有情况 | Unless using wildcard _, all cases must be explicitly handled

- **值绑定与解构 | Value Binding and Destructuring**
  
  **概念定义 | Concept Definition:**
  在模式匹配中，可以通过变量绑定从匹配的值中提取数据，同时可以对复合类型进行解构，将其内部的值分别绑定到不同的变量中。这使得数据处理更加灵活和直观。| In pattern matching, data can be extracted from matched values through variable binding, and compound types can be destructured to bind their internal values to different variables. This makes data processing more flexible and intuitive.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 模式匹配中的变量绑定会创建新的变量吗？| Does variable binding in pattern matching create new variables?
     **答案 | Answer:** 是 | Yes - 绑定的变量只在该分支作用域内有效 | Bound variables are only valid within that branch's scope
  2. 可以同时匹配值和绑定变量吗？| Can we both match values and bind variables simultaneously?
     **答案 | Answer:** 是 | Yes - 可以使用 `@` 操作符进行模式绑定 | Can use `@` operator for pattern binding
  3. 解构时必须使用所有字段吗？| Must all fields be used when destructuring?
     **答案 | Answer:** 否 | No - 可以使用 `..` 忽略剩余字段 | Can use `..` to ignore remaining fields
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 元组解构 | Tuple destructuring
  fn analyze_point(point: (i32, i32)) -> String {
      match point {
          (0, 0) => String::from("原点"), // 匹配特定值 | Match specific values
          (x, 0) => format!("在x轴上，x={}", x), // 绑定x值 | Bind x value
          (0, y) => format!("在y轴上，y={}", y), // 绑定y值 | Bind y value
          (x, y) => format!("点({}, {})", x, y), // 绑定两个值 | Bind both values
      }
  }

  // 结构体解构 | Struct destructuring
  struct Person {
      name: String,
      age: u32,
  }

  fn greet_person(person: Person) -> String {
      match person {
          Person { name, age: 0 } => format!("婴儿 {}", name), // 部分匹配 | Partial match
          Person { name, age } if age >= 18 => format!("成年人 {}", name), // 带条件 | With condition
          Person { name, .. } => format!("未成年人 {}", name), // 忽略其他字段 | Ignore other fields
      }
  }
  ```

### 2. 枚举匹配 | Enum Matching (1小时 | 1 hour)

- **枚举值匹配 | Enum Value Matching**
  
  **概念定义 | Concept Definition:**
  枚举匹配是Rust中最强大的模式匹配应用之一，允许对枚举的不同变体进行匹配，并可以从带有数据的枚举变体中提取值。这种机制使得状态管理和错误处理变得极其安全和直观。| Enum matching is one of the most powerful pattern matching applications in Rust, allowing matching against different enum variants and extracting values from enum variants that carry data. This mechanism makes state management and error handling extremely safe and intuitive.
  
  **核心特征 | Key Characteristics:**
  - 必须处理枚举的所有变体 | Must handle all enum variants
  - 可以从数据承载的枚举变体中提取值 | Can extract values from data-carrying enum variants
  - 支持嵌套枚举的复杂匹配 | Supports complex matching of nested enums
  - 与Option和Result类型完美配合 | Works perfectly with Option and Result types
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 匹配枚举时必须处理所有变体吗？| Must all variants be handled when matching enums?
     **答案 | Answer:** 是 | Yes - 除非使用通配符，否则编译器要求穷尽性 | Unless using wildcards, compiler requires exhaustiveness
  2. 可以从枚举变体中提取数据吗？| Can data be extracted from enum variants?  
     **答案 | Answer:** 是 | Yes - 通过模式匹配可以提取枚举变体携带的数据 | Data carried by enum variants can be extracted through pattern matching
  3. Option<T>是特殊的枚举类型吗？| Is Option<T> a special enum type?
     **答案 | Answer:** 是 | Yes - 它是标准库中定义的枚举，有Some(T)和None两个变体 | It's an enum defined in standard library with Some(T) and None variants
  4. 匹配Some时可以直接使用里面的值吗？| When matching Some, can the inner value be used directly?
     **答案 | Answer:** 是 | Yes - 通过模式绑定可以直接提取Some中的值 | The value inside Some can be directly extracted through pattern binding
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 自定义枚举匹配 | Custom enum matching
  #[derive(Debug)]
  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(i32, i32, i32),
  }

  fn process_message(msg: Message) -> String {
      match msg {
          Message::Quit => String::from("程序退出"), // 无数据变体 | Variant without data
          Message::Move { x, y } => { // 结构体风格变体解构 | Struct-style variant destructuring
              format!("移动到坐标 ({}, {})", x, y)
          }
          Message::Write(text) => { // 元组风格变体解构 | Tuple-style variant destructuring
              format!("写入文本: {}", text)
          }
          Message::ChangeColor(r, g, b) => { // 多值元组解构 | Multi-value tuple destructuring
              format!("改变颜色为 RGB({}, {}, {})", r, g, b)
          }
      }
  }

  // Option<T> 匹配 | Option<T> matching
  fn safe_divide(x: f64, y: f64) -> Option<f64> {
      if y != 0.0 {
          Some(x / y)
      } else {
          None
      }
  }

  fn handle_division_result(result: Option<f64>) -> String {
      match result {
          Some(value) => format!("结果: {:.2}", value), // 提取Some中的值 | Extract value from Some
          None => String::from("错误: 除零操作"), // 处理None情况 | Handle None case
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 当处理 `Message::Write(String::from("Hello"))` 时输出什么？| What is the output when processing `Message::Write(String::from("Hello"))`?
    **答案 | Answer:** "写入文本: Hello" | "写入文本: Hello"
  - Option::None匹配时需要提供类型参数吗？| Does Option::None require type parameters when matching?
    **答案 | Answer:** 否 | No - None是无参数变体，类型可以推断 | None is a parameterless variant, type can be inferred

### 3. if let语法糖 | if let Syntax Sugar (45分钟 | 45 minutes)

- **if let基础用法 | Basic if let Usage**
  
  **概念定义 | Concept Definition:**
  if let是match表达式的语法糖，专门用于只关心一个特定模式匹配情况的场景。它简化了只需要处理一种匹配情况而忽略其他所有情况的代码，使代码更加简洁和可读。| if let is syntactic sugar for match expressions, specifically designed for scenarios where only one specific pattern matching case is of concern. It simplifies code that only needs to handle one matching case while ignoring all other cases, making code more concise and readable.
  
  **核心特征 | Key Characteristics:**
  - 只处理一个感兴趣的模式 | Only handles one pattern of interest
  - 可以配合else使用处理其他情况 | Can be used with else to handle other cases
  - 语法更简洁，减少嵌套 | More concise syntax with less nesting
  - 适用于Option、Result等常见枚举 | Suitable for common enums like Option and Result
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. if let只能处理一种模式吗？| Can if let only handle one pattern?
     **答案 | Answer:** 是 | Yes - if let专门设计用于单一模式匹配 | if let is specifically designed for single pattern matching
  2. if let可以替代所有的match表达式吗？| Can if let replace all match expressions?  
     **答案 | Answer:** 否 | No - 只适用于只关心一个模式的情况 | Only suitable when only one pattern is of concern
  3. if let后面可以跟else分支吗？| Can if let be followed by an else branch?
     **答案 | Answer:** 是 | Yes - else分支处理不匹配的所有其他情况 | else branch handles all other non-matching cases
  4. if let中可以进行值绑定吗？| Can value binding be performed in if let?
     **答案 | Answer:** 是 | Yes - 可以从匹配的模式中提取和绑定值 | Values can be extracted and bound from the matched pattern
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // if let与Option的使用 | Using if let with Option
  fn process_optional_value(opt: Option<i32>) {
      // 使用if let简化Option处理 | Simplify Option handling with if let
      if let Some(value) = opt {
          println!("找到值: {}", value); // 只处理Some情况 | Only handle Some case
      } else {
          println!("没有值"); // 处理None情况 | Handle None case
      }
      
      // 等价的match表达式 | Equivalent match expression
      match opt {
          Some(value) => println!("找到值: {}", value),
          None => println!("没有值"),
      }
  }

  // if let与自定义枚举 | if let with custom enums
  enum Command {
      Save(String),
      Load(String),
      Quit,
  }

  fn handle_command(cmd: Command) {
      // 只关心Save命令 | Only care about Save command
      if let Command::Save(filename) = cmd {
          println!("保存文件: {}", filename);
          // 执行保存逻辑 | Execute save logic
      } else {
          println!("其他命令，暂不处理"); // 忽略其他所有命令 | Ignore all other commands
      }
  }

  // 链式if let | Chained if let
  fn process_nested_option(nested: Option<Option<i32>>) {
      if let Some(inner) = nested {
          if let Some(value) = inner {
              println!("嵌套值: {}", value);
          } else {
              println!("内层为None");
          }
      } else {
          println!("外层为None");
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - `if let Some(x) = Some(5)` 会绑定x为什么值？| What value will x be bound to in `if let Some(x) = Some(5)`?
    **答案 | Answer:** 5 - x会被绑定为Some中包含的值 | x will be bound to the value contained in Some
  - if let比match的优势是什么？| What are the advantages of if let over match?
    **答案 | Answer:** 语法更简洁，减少样板代码，适合只关心一种情况的场景 | More concise syntax, less boilerplate code, suitable for scenarios caring about only one case

### 4. 高级模式匹配 | Advanced Pattern Matching (45分钟 | 45 minutes)

- **模式守卫和绑定 | Pattern Guards and Bindings**
  
  **概念定义 | Concept Definition:**
  高级模式匹配包括使用模式守卫（match guards）添加额外条件，以及使用@操作符进行模式绑定。模式守卫允许在模式匹配的基础上添加运行时条件检查，而@绑定允许在匹配模式的同时保存整个值的引用。| Advanced pattern matching includes using pattern guards to add additional conditions and using the @ operator for pattern binding. Pattern guards allow adding runtime condition checks on top of pattern matching, while @ binding allows saving a reference to the entire value while matching the pattern.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 模式守卫可以访问外部变量吗？| Can pattern guards access external variables?
     **答案 | Answer:** 是 | Yes - 守卫可以使用作用域内的任何变量 | Guards can use any variables in scope
  2. @绑定操作符的作用是什么？| What is the purpose of the @ binding operator?  
     **答案 | Answer:** 同时进行模式匹配和值绑定 | Simultaneously perform pattern matching and value binding
  3. 可以在一个模式中使用多个@绑定吗？| Can multiple @ bindings be used in one pattern?
     **答案 | Answer:** 是 | Yes - 可以在复杂模式中使用多个@绑定 | Multiple @ bindings can be used in complex patterns
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 模式守卫示例 | Pattern guard examples
  fn categorize_number(x: i32) -> &'static str {
      match x {
          n if n < 0 => "负数", // 守卫条件 | Guard condition
          0 => "零",
          n if n % 2 == 0 => "正偶数", // 条件检查 | Condition check
          _ => "正奇数",
      }
  }

  // @绑定示例 | @ binding examples
  enum Shape {
      Rectangle { width: u32, height: u32 },
      Circle { radius: u32 },
  }

  fn analyze_shape(shape: Shape) -> String {
      match shape {
          // @绑定保存整个Rectangle值，同时解构其字段 | @ binding saves entire Rectangle value while destructuring fields
          rect @ Shape::Rectangle { width, height } if width == height => {
              format!("正方形: {:?}", rect)
          }
          Shape::Rectangle { width, height } => {
              format!("长方形: {}x{}", width, height)
          }
          // @绑定保存Circle值，同时匹配radius | @ binding saves Circle value while matching radius  
          circle @ Shape::Circle { radius } if radius > 10 => {
              format!("大圆形: {:?}", circle)
          }
          Shape::Circle { radius } => {
              format!("小圆形: 半径{}", radius)
          }
      }
  }
  ```

### 5. 模式匹配最佳实践 | Pattern Matching Best Practices (30分钟 | 30 minutes)

- **性能与可读性优化 | Performance and Readability Optimization**
  
  **关键原则 | Key Principles:**
  - 按匹配频率排序模式，最常见的放在前面 | Order patterns by matching frequency, most common first
  - 使用具体模式而非过度依赖通配符 | Use specific patterns rather than over-relying on wildcards
  - 合理使用if let简化单一模式匹配 | Properly use if let to simplify single pattern matching
  
  **实践验证问题 | Practice Verification Questions:**
  1. 什么时候应该使用match而不是if let？| When should match be used instead of if let?
     **答案 | Answer:** 需要处理多个模式或确保穷尽性时 | When multiple patterns need to be handled or exhaustiveness is required
  2. 如何提高模式匹配的性能？| How to improve pattern matching performance?
     **答案 | Answer:** 将最常匹配的模式放在前面，避免复杂的守卫条件 | Place most commonly matched patterns first, avoid complex guard conditions
  3. 过度使用通配符_有什么问题？| What problems arise from overusing wildcards _?
     **答案 | Answer:** 可能隐藏bug，降低代码的表达力和维护性 | May hide bugs, reduce code expressiveness and maintainability

### 6. 知识巩固与检查 | Knowledge Consolidation and Review (20分钟 | 20 minutes)

- **综合概念检查 | Comprehensive Concept Check**
  
  1. match表达式和其他语言的switch语句最大的区别是什么？| What is the biggest difference between match expressions and switch statements in other languages?
     **答案 | Answer:** match必须穷尽所有可能情况，确保类型安全和内存安全 | match must exhaustively cover all possible cases, ensuring type and memory safety
  2. 模式匹配如何增强代码的安全性？| How does pattern matching enhance code safety?
     **答案 | Answer:** 编译时检查确保所有情况都被处理，避免运行时错误 | Compile-time checking ensures all cases are handled, avoiding runtime errors
  3. if let相比match的适用场景是什么？| What are the applicable scenarios for if let compared to match?
     **答案 | Answer:** 只关心一个特定模式时，简化代码并提高可读性 | When only one specific pattern is of concern, simplifying code and improving readability
  4. 枚举匹配在错误处理中的作用是什么？| What role does enum matching play in error handling?
     **答案 | Answer:** 强制处理所有错误情况，通过类型系统确保错误不被忽略 | Forces handling of all error cases, ensuring errors are not ignored through the type system
  5. 模式匹配如何支持函数式编程风格？| How does pattern matching support functional programming style?
     **答案 | Answer:** 通过表达式而非语句的方式处理数据，支持不可变性和代数数据类型 | Processes data through expressions rather than statements, supporting immutability and algebraic data types

## 实践项目：简单状态机实现 | Practical Project: Simple State Machine Implementation

### 目标 | Objective
实现一个简单的状态机来模拟自动售货机的工作流程，综合应用枚举、模式匹配和状态转换等概念 | Implement a simple state machine to simulate vending machine workflow, comprehensively applying concepts like enums, pattern matching, and state transitions

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 能够定义包含数据的复杂枚举吗？| Can you define complex enums that contain data?
   **答案 | Answer:** 是，可以定义元组风格和结构体风格的枚举变体 | Yes, tuple-style and struct-style enum variants can be defined
2. match表达式必须处理所有可能的状态吗？| Must match expressions handle all possible states?
   **答案 | Answer:** 是，必须穷尽所有枚举变体，确保状态转换的完整性 | Yes, all enum variants must be exhausted to ensure state transition completeness
3. 可以在状态转换中携带数据吗？| Can data be carried during state transitions?
   **答案 | Answer:** 是，通过枚举变体可以在状态间传递相关数据 | Yes, related data can be passed between states through enum variants

### 步骤 | Steps
1. 定义售货机状态枚举和事件类型 | Define vending machine state enum and event types
2. 实现状态转换逻辑使用match表达式 | Implement state transition logic using match expressions
3. 添加错误处理和边界情况检查 | Add error handling and edge case checking
4. 创建用户交互接口展示状态变化 | Create user interaction interface showing state changes
5. 测试各种状态转换路径和异常情况 | Test various state transition paths and exceptional cases

### 示例代码 | Example Code
```rust
"""
自动售货机状态机 | Vending Machine State Machine
演示模式匹配在状态管理中的应用 | Demonstrates pattern matching application in state management

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 枚举定义和变体数据 | Enum definition and variant data
- match表达式穷尽性检查 | match expression exhaustiveness checking  
- 状态转换和数据流控制 | State transition and data flow control
"""

#[derive(Debug, Clone)]
// 售货机状态定义 | Vending machine state definition
enum VendingState {
    Idle,                           // 空闲状态 | Idle state
    CoinInserted { amount: u32 },   // 已投币状态，记录金额 | Coin inserted state with amount
    ItemSelected { 
        amount: u32, 
        item: String, 
        price: u32 
    },  // 已选择商品状态 | Item selected state
    Dispensing { item: String },    // 出货状态 | Dispensing state
    ReturningChange { amount: u32 }, // 找零状态 | Returning change state
    Error(String),                  // 错误状态，记录错误信息 | Error state with error message
}

#[derive(Debug)]
// 售货机事件类型 | Vending machine event types
enum VendingEvent {
    InsertCoin(u32),           // 投币事件 | Insert coin event
    SelectItem(String, u32),   // 选择商品事件（商品名，价格）| Select item event (name, price)
    DispenseItem,              // 出货事件 | Dispense item event
    ReturnChange,              // 找零事件 | Return change event
    Reset,                     // 重置事件 | Reset event
}

struct VendingMachine {
    state: VendingState,
    inventory: std::collections::HashMap<String, u32>, // 库存管理 | Inventory management
}

impl VendingMachine {
    fn new() -> Self {
        let mut inventory = std::collections::HashMap::new();
        inventory.insert("可乐".to_string(), 3); // 初始化库存 | Initialize inventory
        inventory.insert("薯片".to_string(), 5);
        inventory.insert("巧克力".to_string(), 2);
        
        VendingMachine {
            state: VendingState::Idle,
            inventory,
        }
    }
    
    // 核心状态转换逻辑 - 演示match表达式的穷尽性 | Core state transition logic - demonstrates match exhaustiveness
    fn handle_event(&mut self, event: VendingEvent) -> Result<String, String> {
        use VendingState::*;
        use VendingEvent::*;
        
        let (new_state, message) = match (&self.state, event) {
            // 空闲状态的处理 | Idle state handling
            (Idle, InsertCoin(amount)) => {
                if amount > 0 {
                    (CoinInserted { amount }, format!("已投币 {} 元", amount))
                } else {
                    (Error("无效金额".to_string()), "投币金额必须大于0".to_string())
                }
            }
            (Idle, _) => (Error("请先投币".to_string()), "请先投币再操作".to_string()),
            
            // 已投币状态的处理 | Coin inserted state handling  
            (CoinInserted { amount }, InsertCoin(additional)) => {
                let total = amount + additional;
                (CoinInserted { amount: total }, format!("总投币金额: {} 元", total))
            }
            (CoinInserted { amount }, SelectItem(item, price)) => {
                // 检查库存 | Check inventory
                if let Some(&stock) = self.inventory.get(&item) {
                    if stock > 0 {
                        if *amount >= price {
                            (ItemSelected { 
                                amount: *amount, 
                                item: item.clone(), 
                                price 
                            }, format!("已选择 {}，价格 {} 元", item, price))
                        } else {
                            (Error(format!("金额不足，需要 {} 元", price)), 
                             format!("金额不足，还需 {} 元", price - amount))
                        }
                    } else {
                        (Error("商品缺货".to_string()), format!("{} 已售完", item))
                    }
                } else {
                    (Error("商品不存在".to_string()), format!("{} 不存在", item))
                }
            }
            
            // 商品选择状态的处理 | Item selected state handling
            (ItemSelected { amount, item, price }, DispenseItem) => {
                // 更新库存 | Update inventory
                if let Some(stock) = self.inventory.get_mut(item) {
                    *stock -= 1;
                }
                
                if *amount > *price {
                    (ReturningChange { amount: amount - price }, 
                     format!("出货: {}，准备找零 {} 元", item, amount - price))
                } else {
                    (Idle, format!("出货: {}，交易完成", item))
                }
            }
            
            // 找零状态的处理 | Returning change state handling
            (ReturningChange { amount }, ReturnChange) => {
                (Idle, format!("找零 {} 元，交易完成", amount))
            }
            
            // 错误状态的处理 | Error state handling  
            (Error(_), Reset) => {
                (Idle, "系统已重置".to_string())
            }
            
            // 任何状态都可以重置 | Any state can be reset
            (_, Reset) => {
                (Idle, "系统已重置".to_string())
            }
            
            // 处理无效的状态-事件组合 | Handle invalid state-event combinations
            (state, event) => {
                (Error("无效操作".to_string()), 
                 format!("当前状态 {:?} 不能处理事件 {:?}", state, event))
            }
        };
        
        self.state = new_state;
        Ok(message)
    }
    
    // 获取当前状态信息 | Get current state information
    fn get_state_info(&self) -> String {
        match &self.state {
            VendingState::Idle => "空闲状态 - 请投币".to_string(),
            VendingState::CoinInserted { amount } => format!("已投币 {} 元 - 请选择商品", amount),
            VendingState::ItemSelected { amount, item, price } => {
                format!("已选择 {} (价格: {} 元，已投币: {} 元) - 请确认出货", item, price, amount)
            }
            VendingState::Dispensing { item } => format!("正在出货: {}", item),
            VendingState::ReturningChange { amount } => format!("正在找零: {} 元", amount),
            VendingState::Error(msg) => format!("错误状态: {}", msg),
        }
    }
    
    // 显示可用商品 | Display available items
    fn show_inventory(&self) -> String {
        let mut items = Vec::new();
        for (item, &stock) in &self.inventory {
            if stock > 0 {
                items.push(format!("{} (库存: {})", item, stock));
            }
        }
        format!("可用商品: {}", items.join(", "))
    }
}

fn main() {
    let mut machine = VendingMachine::new();
    println!("=== 自动售货机状态机演示 ===");
    println!("{}", machine.get_state_info());
    println!("{}", machine.show_inventory());
    
    // 模拟一次完整的购买流程 | Simulate a complete purchase flow
    let events = vec![
        VendingEvent::InsertCoin(5),
        VendingEvent::SelectItem("可乐".to_string(), 3),
        VendingEvent::DispenseItem,
        VendingEvent::ReturnChange,
    ];
    
    for event in events {
        println!("\n事件: {:?}", event);
        match machine.handle_event(event) {
            Ok(message) => {
                println!("结果: {}", message);
                println!("状态: {}", machine.get_state_info());
            }
            Err(error) => {
                println!("错误: {}", error);
            }
        }
    }
    
    // 演示错误处理 | Demonstrate error handling
    println!("\n=== 错误处理演示 ===");
    machine.handle_event(VendingEvent::SelectItem("不存在的商品".to_string(), 1)).ok();
    println!("状态: {}", machine.get_state_info());
    machine.handle_event(VendingEvent::Reset).ok();
    println!("重置后状态: {}", machine.get_state_info());
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确使用了枚举来表示不同状态？| Does the project correctly use enums to represent different states?
2. match表达式是否处理了所有可能的状态转换？| Do the match expressions handle all possible state transitions?
3. 代码是否体现了模式匹配的穷尽性检查？| Does the code reflect the exhaustiveness checking of pattern matching?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **复杂模式匹配强化练习 | Complex Pattern Matching Reinforcement Exercise**
   - **练习描述 | Exercise Description:** 创建一个配置文件解析器，使用嵌套枚举和复杂模式匹配来解析不同类型的配置项
   - **概念检查 | Concept Check:** 能够设计和匹配嵌套的枚举结构吗？
   - **学习目标 | Learning Objective:** 深入理解复杂模式的设计和匹配技巧

2. **枚举错误处理应用练习 | Enum Error Handling Application Exercise**
   - **练习描述 | Exercise Description:** 设计一个文件操作库，使用自定义错误枚举和Result类型进行全面的错误处理
   - **概念检查 | Concept Check:** 理解如何使用枚举设计错误类型层次？
   - **学习目标 | Learning Objective:** 掌握枚举在错误处理中的最佳实践

3. **状态机设计整合练习 | State Machine Design Integration Exercise**
   - **练习描述 | Exercise Description:** 扩展售货机项目，添加维护模式、统计功能和网络通信状态
   - **概念检查 | Concept Check:** 能够设计复杂的状态转换图并实现？
   - **学习目标 | Learning Objective:** 发展复杂系统的状态建模思维

4. **性能优化模式匹配练习 | Performance Optimized Pattern Matching Exercise**
   - **练习描述 | Exercise Description:** 优化一个数据处理管道，使用最佳的模式匹配策略提高性能
   - **概念检查 | Concept Check:** 理解模式匹配的性能考虑因素？
   - **学习目标 | Learning Objective:** 培养性能敏感的模式匹配编程能力

5. **函数式风格模式匹配练习 | Functional Style Pattern Matching Exercise**
   - **练习描述 | Exercise Description:** 实现一个表达式求值器，使用纯函数式风格和递归模式匹配
   - **概念检查 | Concept Check:** 能够将模式匹配与函数式编程原则结合？
   - **学习目标 | Learning Objective:** 发展函数式编程思维和表达式处理能力

6. **模式匹配教学练习 | Pattern Matching Teaching Exercise**
   - **练习描述 | Exercise Description:** 创建一个交互式教学程序，帮助他人学习模式匹配的概念和用法
   - **概念检查 | Concept Check:** 能够清晰解释模式匹配的各种概念？
   - **学习目标 | Learning Objective:** 通过教学深化对概念的理解

7. **实际应用场景扩展练习 | Real-world Application Scenario Extension Exercise**
   - **练习描述 | Exercise Description:** 选择一个实际问题（如日志解析、协议处理等），使用模式匹配设计解决方案
   - **概念检查 | Concept Check:** 能够识别适合使用模式匹配的实际场景？
   - **学习目标 | Learning Objective:** 提高将理论知识应用到实际问题的能力

## 学习资源 | Learning Resources
- [Rust官方文档 - 模式匹配](https://doc.rust-lang.org/book/ch06-00-enums.html) | [Rust Official Documentation - Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust模式匹配深入理解](https://doc.rust-lang.org/book/ch18-00-patterns.html) | [Deep Understanding of Rust Patterns](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [枚举和模式匹配最佳实践](https://rust-unofficial.github.io/patterns/) | [Enum and Pattern Matching Best Practices](https://rust-unofficial.github.io/patterns/)
- [状态机设计模式指南](https://hoverbear.org/blog/rust-state-machine-pattern/) | [State Machine Design Pattern Guide](https://hoverbear.org/blog/rust-state-machine-pattern/)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] match表达式语法和穷尽性理解 | match expression syntax and exhaustiveness understanding
- [ ] 枚举匹配和数据提取掌握 | Enum matching and data extraction mastery
- [ ] if let语法糖的适用场景认识 | if let syntax sugar usage scenarios recognition
- [ ] 高级模式匹配技巧应用 | Advanced pattern matching techniques application
- [ ] 售货机状态机项目完成 | Vending machine state machine project completion
- [ ] 所有CCQs正确回答 | All CCQs answered correctly
- [ ] 模式匹配最佳实践掌握 | Pattern matching best practices mastered
- [ ] 错误处理和状态管理理解 | Error handling and state management understanding
- [ ] 性能和可读性优化意识 | Performance and readability optimization awareness
- [ ] 扩展练习至少完成3个 | At least 3 extension exercises completed

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够设计和实现一个简单的状态机系统，清晰地向他人解释模式匹配的核心概念和应用场景。
Before marking as complete, ensure you can correctly answer all CCQs from today and design and implement a simple state machine system, clearly explaining core concepts and application scenarios of pattern matching to others.