# Rust入门 - 第4天：函数基础 | Rust Introduction - Day 4: Function Basics

## 学习目标 | Learning Objectives
- 理解Rust函数的定义与调用语法 | Understand Rust function definition and calling syntax
- 掌握函数参数传递的机制和规则 | Master function parameter passing mechanisms and rules
- 学会函数返回值的不同方式和类型 | Learn different ways and types of function return values
- 区分表达式和语句在函数中的作用 | Distinguish the roles of expressions and statements in functions
- 理解Rust函数的所有权转移规则 | Understand ownership transfer rules in Rust functions
- 能够编写解决实际问题的函数 | Be able to write functions that solve real-world problems

## 详细内容 | Detailed Content

### 1. 函数定义与基本语法 | Function Definition and Basic Syntax (1小时 | 1 hour)

- **函数定义语法 | Function Definition Syntax**
  
  **概念定义 | Concept Definition:**
  Rust函数使用`fn`关键字定义，具有明确的参数类型和返回类型声明。函数是Rust程序的基本构建块，用于封装可重用的代码逻辑。 | Rust functions are defined using the `fn` keyword, with explicit parameter types and return type declarations. Functions are the basic building blocks of Rust programs, used to encapsulate reusable code logic.
  
  **核心特征 | Key Characteristics:**
  - 使用`fn`关键字声明函数 | Functions are declared using the `fn` keyword
  - 参数必须明确指定类型 | Parameters must explicitly specify types
  - 返回类型用`->`箭头语法声明 | Return types are declared using `->` arrow syntax
  - 函数名使用snake_case命名约定 | Function names follow snake_case naming convention
  - 函数体用花括号`{}`包围 | Function bodies are enclosed in curly braces `{}`
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Rust函数定义必须使用什么关键字？| What keyword must be used to define Rust functions?
     **答案 | Answer:** fn - 这是Rust中定义函数的唯一关键字 | This is the only keyword for defining functions in Rust
  2. 函数参数的类型声明是可选的吗？| Is type declaration for function parameters optional?
     **答案 | Answer:** 否 | No - Rust要求所有函数参数都必须明确指定类型 | Rust requires all function parameters to explicitly specify types
  3. 函数名`calculateSum`符合Rust命名约定吗？| Does the function name `calculateSum` follow Rust naming conventions?
     **答案 | Answer:** 否 | No - 应该使用snake_case，即`calculate_sum` | Should use snake_case, i.e., `calculate_sum`
  4. 如果函数有返回值，返回类型声明是必需的吗？| If a function has a return value, is return type declaration required?
     **答案 | Answer:** 是 | Yes - 除非返回单元类型`()` | Unless returning the unit type `()`
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 最简单的函数定义 | Simplest function definition
  fn say_hello() {
      println!("Hello, World!"); // 无返回值的函数 | Function with no return value
  }
  
  // 带参数的函数 | Function with parameters
  fn greet(name: &str) {
      println!("Hello, {}!", name); // 参数类型必须明确指定 | Parameter type must be explicitly specified
  }
  
  // 带返回值的函数 | Function with return value
  fn add(x: i32, y: i32) -> i32 {
      x + y // 最后一行表达式作为返回值 | Last line expression as return value
  }
  
  // 多个参数和返回值 | Multiple parameters and return value
  fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
      width * height // 计算矩形面积 | Calculate rectangle area
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码会输出什么？| What will this code output?
    **答案 | Answer:** Hello, World! 和 Hello, Rust! | Hello, World! and Hello, Rust!
  - 如果改变`add`函数的返回类型为`f32`会发生什么？| What happens if we change the return type of `add` function to `f32`?
    **答案 | Answer:** 编译错误，因为返回的是i32类型 | Compilation error because it returns i32 type
  
  **常见误区检查 | Common Misconception Checks:**
  - 函数可以没有返回类型声明吗？| Can functions have no return type declaration?
    **答案 | Answer:** 可以，但默认返回单元类型`()` | Yes, but they default to returning unit type `()`
  - 函数名可以使用驼峰命名法吗？| Can function names use camelCase?
    **答案 | Answer:** 可以编译但不符合Rust约定，应使用snake_case | Can compile but doesn't follow Rust conventions, should use snake_case

- **函数调用机制 | Function Calling Mechanism**
  
  **概念定义 | Concept Definition:**
  函数调用是通过函数名加圆括号的方式执行函数代码，参数按位置顺序传递。调用时会创建新的栈帧。 | Function calling executes function code by using function name with parentheses, parameters are passed in positional order. A new stack frame is created during calls.
  
  **核心特征 | Key Characteristics:**
  - 函数调用创建新的作用域 | Function calls create new scope
  - 参数按值传递或引用传递 | Parameters passed by value or by reference
  - 调用栈管理函数执行顺序 | Call stack manages function execution order
  - 局部变量在函数结束时销毁 | Local variables are destroyed when function ends
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 函数调用时参数的顺序重要吗？| Does the order of parameters matter when calling functions?
     **答案 | Answer:** 是 | Yes - 参数必须按照函数定义的顺序传递 | Parameters must be passed in the order defined in the function
  2. 函数内部定义的变量在函数外部可以访问吗？| Can variables defined inside a function be accessed outside the function?
     **答案 | Answer:** 否 | No - 局部变量只在函数作用域内有效 | Local variables are only valid within function scope
  3. 一个函数可以调用自己吗？| Can a function call itself?
     **答案 | Answer:** 可以 | Yes - 这叫做递归调用 | This is called recursive calling
  4. 函数调用必须提供所有参数吗？| Must function calls provide all parameters?
     **答案 | Answer:** 是 | Yes - Rust不支持默认参数 | Rust doesn't support default parameters

### 2. 参数传递机制 | Parameter Passing Mechanisms (1小时 | 1 hour)

- **值传递与引用传递 | Pass by Value and Pass by Reference**
  
  **概念定义 | Concept Definition:**
  Rust函数参数可以通过值传递（获取所有权）或引用传递（借用）的方式传递。这直接影响到所有权的转移和数据的可用性。 | Rust function parameters can be passed by value (taking ownership) or by reference (borrowing). This directly affects ownership transfer and data availability.
  
  **核心特征 | Key Characteristics:**
  - 值传递会转移所有权 | Pass by value transfers ownership
  - 引用传递只是借用，不转移所有权 | Pass by reference only borrows, doesn't transfer ownership
  - 可变引用允许修改数据 | Mutable references allow data modification
  - 不可变引用只允许读取 | Immutable references only allow reading
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 当一个值被传递给函数时，原变量还能使用吗？| When a value is passed to a function, can the original variable still be used?
     **答案 | Answer:** 取决于传递方式 | Depends on the passing method - 值传递后不能，引用传递后可以 | Cannot after pass by value, can after pass by reference
  2. `&T`和`&mut T`参数有什么区别？| What's the difference between `&T` and `&mut T` parameters?
     **答案 | Answer:** `&T`是不可变引用，`&mut T`是可变引用 | `&T` is immutable reference, `&mut T` is mutable reference
  3. 一个函数可以同时接受值和引用参数吗？| Can a function accept both value and reference parameters simultaneously?
     **答案 | Answer:** 可以 | Yes - 不同参数可以使用不同的传递方式 | Different parameters can use different passing methods
  4. 传递基本类型（如i32）会转移所有权吗？| Does passing basic types (like i32) transfer ownership?
     **答案 | Answer:** 是 | Yes - 但基本类型实现了Copy trait，所以原变量仍可用 | But basic types implement Copy trait, so original variables remain usable
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 值传递示例 | Pass by value example
  fn take_ownership(s: String) {
      println!("接收到字符串: {}", s); // 函数获得s的所有权 | Function takes ownership of s
  } // s在此处被drop | s is dropped here
  
  // 引用传递示例 | Pass by reference example
  fn borrow_string(s: &String) {
      println!("借用字符串: {}", s); // 只是借用，不获取所有权 | Just borrowing, not taking ownership
  } // 无需drop，因为没有所有权 | No need to drop, as there's no ownership
  
  // 可变引用传递 | Mutable reference passing
  fn modify_string(s: &mut String) {
      s.push_str(" - 已修改"); // 可以修改借用的数据 | Can modify borrowed data
  }
  
  // 基本类型传递 | Basic type passing
  fn double_number(x: i32) -> i32 {
      x * 2 // i32实现了Copy，原变量仍可用 | i32 implements Copy, original variable remains usable
  }
  
  fn main() {
      let mut text = String::from("Hello");
      
      // 值传递后text不再可用 | After pass by value, text is no longer usable
      // take_ownership(text); 
      // println!("{}", text); // 这会编译错误 | This would cause compilation error
      
      // 引用传递，text仍然可用 | Pass by reference, text remains usable
      borrow_string(&text);
      println!("原始text: {}", text);
      
      // 可变引用传递 | Mutable reference passing
      modify_string(&mut text);
      println!("修改后的text: {}", text);
      
      let number = 42;
      let doubled = double_number(number);
      println!("原数字: {}, 双倍: {}", number, doubled); // number仍可用 | number still usable
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码会输出什么？| What will this code output?
    **答案 | Answer:** 依次输出借用、原始、修改后的字符串和数字 | Outputs borrowed, original, modified strings and numbers in sequence
  - 如果去掉`modify_string`中的`mut`关键字会发生什么？| What happens if we remove the `mut` keyword from `modify_string`?
    **答案 | Answer:** 编译错误，不能修改不可变引用的数据 | Compilation error, cannot modify data through immutable reference

### 3. 返回值机制 | Return Value Mechanisms (1小时 | 1 hour)

- **表达式返回与return语句 | Expression Return and return Statement**
  
  **概念定义 | Concept Definition:**
  Rust函数可以通过两种方式返回值：表达式返回（函数体最后一个表达式的值）和显式return语句。表达式返回是Rust的惯用方式。 | Rust functions can return values in two ways: expression return (value of the last expression in function body) and explicit return statement. Expression return is the idiomatic Rust way.
  
  **核心特征 | Key Characteristics:**
  - 最后一个表达式自动作为返回值 | Last expression automatically becomes return value
  - 表达式后面不能有分号 | Expressions cannot be followed by semicolons
  - return语句可以在任何位置提前返回 | return statement can early return at any position
  - 所有返回路径必须返回相同类型 | All return paths must return the same type
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 函数体最后一行加分号会影响返回值吗？| Does adding semicolon to the last line of function body affect return value?
     **答案 | Answer:** 是 | Yes - 分号会将表达式变成语句，返回`()` | Semicolon turns expression into statement, returning `()`
  2. 函数可以有多个return语句吗？| Can a function have multiple return statements?
     **答案 | Answer:** 可以 | Yes - 但所有return必须返回相同类型 | But all returns must return the same type
  3. 没有返回值的函数实际上返回什么？| What do functions without return values actually return?
     **答案 | Answer:** 单元类型`()` | Unit type `()` - 表示无意义的值 | Represents a meaningless value
  4. 可以在函数中间使用return提前返回吗？| Can return be used in the middle of a function for early return?
     **答案 | Answer:** 可以 | Yes - 这用于条件返回和错误处理 | This is used for conditional returns and error handling
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 表达式返回 | Expression return
  fn add_numbers(x: i32, y: i32) -> i32 {
      x + y // 无分号，作为返回值 | No semicolon, acts as return value
  }
  
  // return语句返回 | return statement return
  fn max_value(x: i32, y: i32) -> i32 {
      if x > y {
          return x; // 提前返回 | Early return
      }
      y // 最终返回 | Final return
  }
  
  // 多个返回路径 | Multiple return paths
  fn categorize_number(n: i32) -> &'static str {
      if n < 0 {
          return "负数"; // 负数情况 | Negative case
      }
      if n == 0 {
          return "零"; // 零的情况 | Zero case
      }
      "正数" // 正数情况 | Positive case
  }
  
  // 返回复杂类型 | Return complex types
  fn create_point(x: f64, y: f64) -> (f64, f64) {
      (x, y) // 返回元组 | Return tuple
  }
  
  // 条件返回 | Conditional return
  fn safe_divide(dividend: f64, divisor: f64) -> Option<f64> {
      if divisor == 0.0 {
          return None; // 除零情况 | Division by zero case
      }
      Some(dividend / divisor) // 正常情况 | Normal case
  }
  
  fn main() {
      println!("3 + 5 = {}", add_numbers(3, 5));
      println!("max(10, 7) = {}", max_value(10, 7));
      println!("分类数字-5: {}", categorize_number(-5));
      
      let point = create_point(3.0, 4.0);
      println!("点坐标: ({}, {})", point.0, point.1);
      
      match safe_divide(10.0, 3.0) {
          Some(result) => println!("除法结果: {}", result),
          None => println!("不能除以零"),
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 如果在`add_numbers`的最后加上分号会怎样？| What happens if we add semicolon at the end of `add_numbers`?
    **答案 | Answer:** 编译错误，因为返回类型不匹配 | Compilation error due to return type mismatch
  - `safe_divide`函数返回什么类型？| What type does `safe_divide` function return?
    **答案 | Answer:** Option<f64> - 可能有值也可能没有值 | Maybe has value or maybe doesn't

### 4. 表达式与语句的区别 | Difference Between Expressions and Statements (45分钟 | 45 minutes)

- **表达式 vs 语句概念 | Expression vs Statement Concepts**
  
  **概念定义 | Concept Definition:**
  表达式会计算并产生一个值，而语句执行某些操作但不返回值。这个区别在Rust中很重要，因为它影响函数返回值和变量绑定。 | Expressions evaluate and produce a value, while statements perform some action but don't return a value. This distinction is important in Rust as it affects function return values and variable binding.
  
  **核心特征 | Key Characteristics:**
  - 表达式有返回值，可以赋给变量 | Expressions have return values and can be assigned to variables
  - 语句以分号结尾，返回单元类型() | Statements end with semicolons and return unit type ()
  - 代码块{}也是表达式 | Code blocks {} are also expressions
  - 函数调用是表达式，函数定义是语句 | Function calls are expressions, function definitions are statements
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. `let x = 5`是表达式还是语句？| Is `let x = 5` an expression or statement?
     **答案 | Answer:** 语句 | Statement - let绑定是语句，不返回值 | let binding is a statement, doesn't return value
  2. `x + y`是表达式还是语句？| Is `x + y` an expression or statement?
     **答案 | Answer:** 表达式 | Expression - 计算并返回加法结果 | Calculates and returns addition result
  3. 代码块`{x + y}`会返回值吗？| Does code block `{x + y}` return a value?
     **答案 | Answer:** 是 | Yes - 代码块是表达式，返回最后一个表达式的值 | Code blocks are expressions, return the value of last expression
  4. 为什么`let x = (let y = 6)`会编译错误？| Why does `let x = (let y = 6)` cause compilation error?
     **答案 | Answer:** 因为let是语句不是表达式，不能赋值 | Because let is a statement not expression, cannot be assigned
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn demonstrate_expressions_statements() {
      // 语句示例 | Statement examples
      let x = 5; // let语句 | let statement
      let y = 6; // let语句 | let statement
      
      // 表达式示例 | Expression examples
      let sum = x + y; // x + y是表达式 | x + y is expression
      let product = x * y; // x * y是表达式 | x * y is expression
      
      // 代码块表达式 | Code block expression
      let result = {
          let temp = x * 2; // 语句 | statement
          temp + y // 表达式，作为代码块的返回值 | expression, as code block return value
      }; // 整个代码块是表达式 | entire code block is expression
      
      println!("结果: {}", result);
      
      // 函数调用是表达式 | Function call is expression
      let doubled = double_value(10);
      
      // if也是表达式 | if is also expression
      let message = if doubled > 15 {
          "大数"
      } else {
          "小数"
      };
      
      println!("消息: {}", message);
  }
  
  fn double_value(n: i32) -> i32 {
      n * 2 // 表达式返回 | expression return
  }
  
  // 演示错误的语句使用 | Demonstrate incorrect statement usage
  fn wrong_usage_examples() {
      let x = 5;
      
      // 这会编译错误 | This will cause compilation error
      // let y = (let z = 6); // let是语句，不能赋值 | let is statement, cannot assign
      
      // 这也会有问题 | This will also be problematic
      fn inner_function() -> i32 {
          let result = x + 1; // 语句 | statement
          result; // 加了分号变成语句，返回() | Adding semicolon makes it statement, returns ()
      }
      
      // 正确的方式 | Correct way
      fn correct_inner_function() -> i32 {
          let result = x + 1;
          result // 无分号，表达式返回 | No semicolon, expression return
      }
  }
  ```

### 5. 函数作用域与生命周期 | Function Scope and Lifetime (45分钟 | 45 minutes)

- **局部作用域管理 | Local Scope Management**
  
  **概念定义 | Concept Definition:**
  函数创建独立的作用域，局部变量只在函数执行期间存在。理解作用域对于管理内存和避免编译错误至关重要。 | Functions create independent scopes where local variables only exist during function execution. Understanding scope is crucial for memory management and avoiding compilation errors.
  
  **核心特征 | Key Characteristics:**
  - 每个函数调用创建新的栈帧 | Each function call creates a new stack frame
  - 局部变量在函数结束时自动销毁 | Local variables are automatically destroyed when function ends
  - 变量作用域遵循词法作用域规则 | Variable scope follows lexical scope rules
  - 内部作用域可以屏蔽外部同名变量 | Inner scope can shadow outer variables with same name
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 函数内定义的变量可以在函数外访问吗？| Can variables defined inside a function be accessed outside?
     **答案 | Answer:** 否 | No - 局部变量只在定义的作用域内可见 | Local variables are only visible within their defining scope
  2. 函数参数的生命周期是什么？| What is the lifetime of function parameters?
     **答案 | Answer:** 与函数执行期间相同 | Same as function execution duration
  3. 变量屏蔽(shadowing)在函数中如何工作？| How does variable shadowing work in functions?
     **答案 | Answer:** 内部作用域的同名变量会隐藏外部变量 | Same-named variables in inner scope hide outer variables
  4. 函数结束时会发生什么？| What happens when a function ends?
     **答案 | Answer:** 所有局部变量被销毁，栈帧被清理 | All local variables are destroyed, stack frame is cleaned up
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn demonstrate_scope() {
      let outer_var = "外部变量"; // 外部作用域 | outer scope
      
      println!("函数开始，外部变量: {}", outer_var);
      
      { // 内部作用域开始 | inner scope begins
          let inner_var = "内部变量"; // 只在此作用域内有效 | only valid in this scope
          println!("内部作用域，内部变量: {}", inner_var);
          println!("内部作用域，外部变量: {}", outer_var); // 可以访问外部变量 | can access outer variable
          
          { // 更深层的作用域 | deeper scope
              let deep_var = "深层变量"; // 最内层变量 | innermost variable
              let outer_var = "屏蔽的外部变量"; // 屏蔽外部同名变量 | shadows outer variable with same name
              println!("深层作用域，深层变量: {}", deep_var);
              println!("深层作用域，屏蔽变量: {}", outer_var); // 显示屏蔽的值 | shows shadowed value
          } // deep_var在此处销毁 | deep_var destroyed here
          
          println!("回到内部作用域，外部变量: {}", outer_var); // 恢复原值 | original value restored
          // println!("{}", deep_var); // 编译错误，deep_var已销毁 | compilation error, deep_var destroyed
      } // inner_var在此处销毁 | inner_var destroyed here
      
      println!("函数结束，外部变量: {}", outer_var);
      // println!("{}", inner_var); // 编译错误，inner_var已销毁 | compilation error, inner_var destroyed
  }
  
  fn parameter_scope_example(param: i32) {
      println!("参数值: {}", param); // param只在函数内有效 | param only valid inside function
      let local = param * 2; // 基于参数创建局部变量 | create local variable based on parameter
      println!("局部值: {}", local);
  } // param和local都在此处销毁 | both param and local destroyed here
  
  fn main() {
      demonstrate_scope();
      parameter_scope_example(42);
  }
  ```

### 6. 函数最佳实践与常见模式 | Function Best Practices and Common Patterns (30分钟 | 30 minutes)

- **函数设计原则 | Function Design Principles**
  
  **概念定义 | Concept Definition:**
  良好的函数设计应该遵循单一职责、简洁明了、易于测试的原则。函数应该具有清晰的接口和可预测的行为。 | Good function design should follow principles of single responsibility, clarity, and testability. Functions should have clear interfaces and predictable behavior.
  
  **函数命名与文档 | Function Naming and Documentation:**
  - 使用描述性的函数名 | Use descriptive function names
  - 遵循snake_case命名约定 | Follow snake_case naming convention
  - 添加有意义的注释和文档 | Add meaningful comments and documentation
  - 保持函数简短和专注 | Keep functions short and focused
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 一个函数应该只做一件事吗？| Should a function do only one thing?
     **答案 | Answer:** 是 | Yes - 单一职责原则提高代码可维护性 | Single responsibility principle improves code maintainability
  2. 函数名应该说明函数做什么还是怎么做？| Should function names describe what the function does or how it does it?
     **答案 | Answer:** 做什么 | What it does - 关注接口而不是实现 | Focus on interface rather than implementation
  3. 长函数有什么问题？| What problems do long functions have?
     **答案 | Answer:** 难以理解、测试和维护 | Difficult to understand, test, and maintain
  4. 什么时候应该将代码提取为单独的函数？| When should code be extracted into separate functions?
     **答案 | Answer:** 当代码重复或逻辑可以独立命名时 | When code is repeated or logic can be independently named
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 良好的函数设计示例 | Good function design examples
  
  /// 计算圆的面积 | Calculate area of a circle
  /// 参数radius必须为非负数 | Parameter radius must be non-negative
  fn calculate_circle_area(radius: f64) -> f64 {
      const PI: f64 = 3.14159265359; // 常量定义 | constant definition
      PI * radius * radius // 简洁的实现 | concise implementation
  }
  
  /// 验证电子邮件格式 | Validate email format
  /// 返回true表示格式有效 | Returns true if format is valid
  fn is_valid_email(email: &str) -> bool {
      email.contains('@') && email.contains('.') // 简化的验证逻辑 | simplified validation logic
  }
  
  /// 格式化用户显示名称 | Format user display name
  fn format_display_name(first_name: &str, last_name: &str) -> String {
      format!("{} {}", first_name.trim(), last_name.trim()) // 处理空白字符 | handle whitespace
  }
  
  /// 安全除法运算 | Safe division operation
  fn safe_division(dividend: f64, divisor: f64) -> Result<f64, String> {
      if divisor == 0.0 {
          Err("除数不能为零".to_string()) // 错误处理 | error handling
      } else {
          Ok(dividend / divisor) // 成功情况 | success case
      }
  }
  
  // 重构前的长函数示例 | Example of long function before refactoring
  fn process_user_data_bad(name: &str, email: &str, age: i32) -> String {
      // 验证姓名 | validate name
      let trimmed_name = name.trim();
      if trimmed_name.is_empty() {
          return "姓名不能为空".to_string();
      }
      
      // 验证邮箱 | validate email  
      if !email.contains('@') || !email.contains('.') {
          return "邮箱格式无效".to_string();
      }
      
      // 验证年龄 | validate age
      if age < 0 || age > 150 {
          return "年龄超出合理范围".to_string();
      }
      
      // 格式化输出 | format output
      format!("用户: {}, 邮箱: {}, 年龄: {}", trimmed_name, email, age)
  }
  
  // 重构后的模块化函数 | Refactored modular functions
  fn validate_name(name: &str) -> Result<String, String> {
      let trimmed = name.trim();
      if trimmed.is_empty() {
          Err("姓名不能为空".to_string())
      } else {
          Ok(trimmed.to_string())
      }
  }
  
  fn validate_age(age: i32) -> Result<i32, String> {
      if age < 0 || age > 150 {
          Err("年龄超出合理范围".to_string())
      } else {
          Ok(age)
      }
  }
  
  fn process_user_data_good(name: &str, email: &str, age: i32) -> Result<String, String> {
      let valid_name = validate_name(name)?; // 使用?操作符 | use ? operator
      if !is_valid_email(email) {
          return Err("邮箱格式无效".to_string());
      }
      let valid_age = validate_age(age)?;
      
      Ok(format!("用户: {}, 邮箱: {}, 年龄: {}", valid_name, email, valid_age))
  }
  
  fn main() {
      // 测试各种函数 | test various functions
      println!("圆面积: {:.2}", calculate_circle_area(5.0));
      println!("邮箱有效: {}", is_valid_email("user@example.com"));
      println!("显示名: {}", format_display_name("张", "三"));
      
      match safe_division(10.0, 3.0) {
          Ok(result) => println!("除法结果: {:.2}", result),
          Err(error) => println!("错误: {}", error),
      }
      
      match process_user_data_good("张三", "zhang@example.com", 25) {
          Ok(info) => println!("处理结果: {}", info),
          Err(error) => println!("处理错误: {}", error),
      }
  }
  ```

## 实践项目：数学运算计算器 | Practical Project: Mathematical Calculator

### 目标 | Objective
创建一个综合运算计算器，演示函数定义、参数传递、返回值处理和错误管理等核心概念的实际应用。 | Create a comprehensive mathematical calculator that demonstrates practical application of function definition, parameter passing, return value handling, and error management.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 函数如何接收参数并返回计算结果？| How do functions receive parameters and return calculation results?
   **答案 | Answer:** 通过参数列表接收输入，通过返回类型声明和表达式/return语句返回结果 | Through parameter lists to receive input, through return type declaration and expressions/return statements to return results
2. 如何处理可能失败的运算（如除零）？| How to handle operations that might fail (like division by zero)?
   **答案 | Answer:** 使用Result类型或Option类型来表示可能的错误情况 | Use Result type or Option type to represent possible error conditions
3. 函数中的变量作用域如何影响数据访问？| How does variable scope in functions affect data access?
   **答案 | Answer:** 局部变量只在函数内可见，参数在函数执行期间有效 | Local variables are only visible within functions, parameters are valid during function execution

### 步骤 | Steps
1. 设计基本算术运算函数 | Design basic arithmetic operation functions
2. 实现错误处理机制 | Implement error handling mechanisms
3. 创建用户交互界面 | Create user interaction interface
4. 添加高级运算功能 | Add advanced calculation features
5. 测试和验证所有功能 | Test and verify all functionality

### 示例代码 | Example Code
```rust
"""
数学运算计算器 | Mathematical Calculator
综合演示Rust函数的各种特性和最佳实践 | Comprehensive demonstration of various Rust function features and best practices

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 函数定义与调用 | Function definition and calling
- 参数传递机制 | Parameter passing mechanisms  
- 返回值处理 | Return value handling
- 错误处理与Result类型 | Error handling and Result type
- 函数作用域管理 | Function scope management
"""

use std::io;

/// 基本算术运算结果类型 | Basic arithmetic operation result type
type CalcResult = Result<f64, String>;

/// 加法运算 | Addition operation
fn add(a: f64, b: f64) -> CalcResult {
    Ok(a + b) // 加法不会失败 | Addition cannot fail
}

/// 减法运算 | Subtraction operation  
fn subtract(a: f64, b: f64) -> CalcResult {
    Ok(a - b) // 减法不会失败 | Subtraction cannot fail
}

/// 乘法运算 | Multiplication operation
fn multiply(a: f64, b: f64) -> CalcResult {
    Ok(a * b) // 乘法不会失败 | Multiplication cannot fail
}

/// 安全除法运算 | Safe division operation
/// 处理除零错误 | Handle division by zero error
fn divide(dividend: f64, divisor: f64) -> CalcResult {
    if divisor == 0.0 {
        Err("错误：不能除以零".to_string()) // 除零错误 | Division by zero error
    } else {
        Ok(dividend / divisor) // 正常除法 | Normal division
    }
}

/// 幂运算 | Power operation
fn power(base: f64, exponent: f64) -> CalcResult {
    if base == 0.0 && exponent < 0.0 {
        Err("错误：0的负次幂未定义".to_string()) // 0的负次幂错误 | 0 to negative power error
    } else {
        Ok(base.powf(exponent)) // 计算幂 | Calculate power
    }
}

/// 平方根运算 | Square root operation
fn sqrt(number: f64) -> CalcResult {
    if number < 0.0 {
        Err("错误：负数没有实数平方根".to_string()) // 负数平方根错误 | Negative square root error
    } else {
        Ok(number.sqrt()) // 计算平方根 | Calculate square root
    }
}

/// 解析用户输入的数字 | Parse user input number
fn parse_number(input: &str) -> Result<f64, String> {
    input.trim().parse::<f64>()
        .map_err(|_| format!("无效的数字格式: '{}'", input)) // 解析错误处理 | Parse error handling
}

/// 获取用户输入的数字 | Get number input from user
fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        // 读取用户输入 | Read user input
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // 尝试解析数字 | Try to parse number
                match parse_number(&input) {
                    Ok(number) => return number, // 解析成功 | Parse successful
                    Err(error) => {
                        println!("{}", error); // 显示错误信息 | Display error message
                        continue; // 继续循环 | Continue loop
                    }
                }
            }
            Err(_) => {
                println!("输入读取失败，请重试"); // 输入读取错误 | Input read error
                continue;
            }
        }
    }
}

/// 显示计算结果 | Display calculation result
fn display_result(operation: &str, result: CalcResult) {
    match result {
        Ok(value) => println!("{}运算结果: {:.4}", operation, value), // 显示成功结果 | Display successful result
        Err(error) => println!("{}", error), // 显示错误信息 | Display error message
    }
}

/// 显示菜单选项 | Display menu options
fn display_menu() {
    println!("\n=== 数学运算计算器 | Mathematical Calculator ===");
    println!("1. 加法 | Addition (+)");
    println!("2. 减法 | Subtraction (-)");  
    println!("3. 乘法 | Multiplication (×)");
    println!("4. 除法 | Division (÷)");
    println!("5. 幂运算 | Power (^)");
    println!("6. 平方根 | Square Root (√)");
    println!("0. 退出 | Exit");
    println!("请选择操作 | Please select operation: ");
}

/// 获取用户选择 | Get user choice
fn get_user_choice() -> Result<u32, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|_| "输入读取失败".to_string())?; // 读取错误处理 | Read error handling
    
    input.trim().parse::<u32>()
        .map_err(|_| "无效的选择".to_string()) // 解析错误处理 | Parse error handling
}

/// 执行双参数运算 | Execute binary operation
fn execute_binary_operation<F>(operation_name: &str, operation: F) 
where 
    F: Fn(f64, f64) -> CalcResult, // 函数参数约束 | Function parameter constraint
{
    let a = get_number("请输入第一个数字 | Enter first number:");
    let b = get_number("请输入第二个数字 | Enter second number:");
    
    let result = operation(a, b); // 调用传入的运算函数 | Call the passed operation function
    display_result(operation_name, result);
}

/// 执行单参数运算 | Execute unary operation
fn execute_unary_operation<F>(operation_name: &str, operation: F)
where
    F: Fn(f64) -> CalcResult, // 单参数函数约束 | Single parameter function constraint
{
    let number = get_number("请输入数字 | Enter number:");
    
    let result = operation(number); // 调用传入的运算函数 | Call the passed operation function
    display_result(operation_name, result);
}

/// 主程序循环 | Main program loop
fn run_calculator() {
    println!("欢迎使用数学计算器! | Welcome to Mathematical Calculator!");
    
    loop {
        display_menu(); // 显示菜单 | Display menu
        
        match get_user_choice() {
            Ok(choice) => {
                match choice {
                    1 => execute_binary_operation("加法", add), // 加法运算 | Addition operation
                    2 => execute_binary_operation("减法", subtract), // 减法运算 | Subtraction operation
                    3 => execute_binary_operation("乘法", multiply), // 乘法运算 | Multiplication operation
                    4 => execute_binary_operation("除法", divide), // 除法运算 | Division operation
                    5 => execute_binary_operation("幂运算", power), // 幂运算 | Power operation
                    6 => execute_unary_operation("平方根", sqrt), // 平方根运算 | Square root operation
                    0 => {
                        println!("感谢使用计算器！| Thank you for using the calculator!");
                        break; // 退出循环 | Exit loop
                    }
                    _ => println!("无效选择，请重新输入 | Invalid choice, please try again"),
                }
            }
            Err(error) => println!("选择错误: {} | Choice error: {}", error, error),
        }
        
        println!(); // 添加空行分隔 | Add blank line separator
    }
}

fn main() {
    run_calculator(); // 启动计算器 | Start calculator
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0).unwrap(), 5.0);
        assert_eq!(add(-1.0, 1.0).unwrap(), 0.0);
    }
    
    #[test] 
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
        assert!(divide(5.0, 0.0).is_err()); // 测试除零错误 | Test division by zero error
    }
    
    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(9.0).unwrap(), 3.0);
        assert!(sqrt(-1.0).is_err()); // 测试负数平方根错误 | Test negative square root error
    }
    
    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("3.14").unwrap(), 3.14);
        assert!(parse_number("abc").is_err()); // 测试无效输入 | Test invalid input
    }
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确应用了函数定义语法？| Does the project correctly apply function definition syntax?
2. 参数传递和返回值处理是否符合最佳实践？| Do parameter passing and return value handling follow best practices?  
3. 错误处理是否完善且用户友好？| Is error handling comprehensive and user-friendly?
4. 代码是否体现了函数间的正确协作关系？| Does the code reflect correct collaborative relationships between functions?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **函数签名理解练习 | Function Signature Understanding Exercise**
   - **练习描述 | Exercise Description:** 设计不同签名的函数来处理各种数据转换任务 | Design functions with different signatures to handle various data conversion tasks
   - **概念检查 | Concept Check:** 你能解释为什么函数签名对类型安全很重要吗？| Can you explain why function signatures are important for type safety?
   - **学习目标 | Learning Objective:** 深入理解函数签名设计的原则和影响 | Deeply understand principles and impacts of function signature design

2. **参数传递机制练习 | Parameter Passing Mechanism Exercise**
   - **练习描述 | Exercise Description:** 创建函数演示值传递、引用传递和可变引用传递的区别 | Create functions to demonstrate differences between pass by value, pass by reference, and pass by mutable reference
   - **概念检查 | Concept Check:** 什么时候应该使用引用传递而不是值传递？| When should you use pass by reference instead of pass by value?
   - **学习目标 | Learning Objective:** 掌握不同传递方式的适用场景和性能影响 | Master applicable scenarios and performance impacts of different passing methods

3. **返回值模式练习 | Return Value Pattern Exercise**
   - **练习描述 | Exercise Description:** 实现函数使用不同的返回值模式：Option、Result、元组等 | Implement functions using different return value patterns: Option, Result, tuples, etc.
   - **概念检查 | Concept Check:** 如何选择合适的返回值类型来表达函数的语义？| How to choose appropriate return types to express function semantics?
   - **学习目标 | Learning Objective:** 提高选择合适返回类型的判断能力 | Improve judgment ability in choosing appropriate return types

4. **函数组合应用练习 | Function Composition Application Exercise**
   - **练习描述 | Exercise Description:** 创建一个文本处理系统，用多个小函数组合完成复杂任务 | Create a text processing system that uses multiple small functions to complete complex tasks
   - **概念检查 | Concept Check:** 如何将复杂问题分解为多个简单函数？| How to decompose complex problems into multiple simple functions?
   - **学习目标 | Learning Objective:** 培养模块化编程思维和函数设计能力 | Develop modular programming thinking and function design skills

5. **错误处理策略练习 | Error Handling Strategy Exercise**
   - **练习描述 | Exercise Description:** 设计一个文件处理库，展示不同的错误处理和传播策略 | Design a file processing library that demonstrates different error handling and propagation strategies
   - **概念检查 | Concept Check:** 什么时候使用panic!，什么时候使用Result？| When to use panic!, when to use Result?
   - **学习目标 | Learning Objective:** 掌握Rust中的错误处理最佳实践 | Master error handling best practices in Rust

6. **函数性能优化练习 | Function Performance Optimization Exercise**
   - **练习描述 | Exercise Description:** 比较不同函数实现方式的性能差异，如递归vs迭代 | Compare performance differences of different function implementation approaches, such as recursion vs iteration
   - **概念检查 | Concept Check:** 如何评估和改进函数的性能？| How to evaluate and improve function performance?
   - **学习目标 | Learning Objective:** 理解函数设计对程序性能的影响 | Understand the impact of function design on program performance

7. **高阶函数入门练习 | Higher-Order Function Introduction Exercise**
   - **练习描述 | Exercise Description:** 创建接受其他函数作为参数的函数，实现通用的数据处理模式 | Create functions that accept other functions as parameters, implementing generic data processing patterns
   - **概念检查 | Concept Check:** 如何将函数作为参数传递给其他函数？| How to pass functions as parameters to other functions?
   - **学习目标 | Learning Objective:** 为学习闭包和迭代器打下基础 | Lay foundation for learning closures and iterators

## 学习资源 | Learning Resources
- [Rust官方文档 - 函数](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) | [Rust Official Documentation - Functions]
- [Rust By Example - 函数](https://doc.rust-lang.org/rust-by-example/fn.html) | [Rust By Example - Functions]
- [函数设计最佳实践指南](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) | [Function Design Best Practices Guide]
- [错误处理模式详解](https://doc.rust-lang.org/book/ch09-00-error-handling.html) | [Error Handling Patterns Explained]

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解函数定义的基本语法和命名约定 | Understand basic syntax and naming conventions of function definitions
- [ ] 掌握参数传递的不同方式和适用场景 | Master different parameter passing methods and applicable scenarios  
- [ ] 能够正确处理函数返回值和错误情况 | Correctly handle function return values and error conditions
- [ ] 区分表达式和语句在函数中的使用 | Distinguish usage of expressions and statements in functions
- [ ] 理解函数作用域和变量生命周期 | Understand function scope and variable lifetime
- [ ] 能够设计符合最佳实践的函数接口 | Design function interfaces following best practices
- [ ] 完成数学计算器实践项目 | Complete mathematical calculator practical project
- [ ] 所有CCQs都能正确回答 | All CCQs answered correctly
- [ ] 代码示例理解并能独立运行 | Code examples understood and can run independently
- [ ] 至少完成3个扩展练习 | At least 3 extension exercises completed

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释函数定义、参数传递、返回值处理等核心概念。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain core concepts like function definition, parameter passing, and return value handling to others.