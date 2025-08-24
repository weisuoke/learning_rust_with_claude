# Rust入门 - 第2天：变量与数据类型 | Rust Introduction - Day 2: Variables and Data Types

## 学习目标 | Learning Objectives
- 掌握Rust中变量的声明和使用方式 | Master variable declaration and usage in Rust
- 理解可变性(mutability)的概念和重要性 | Understand the concept and importance of mutability
- 熟练使用Rust的基本数据类型 | Proficiently use Rust's basic data types
- 掌握常量和静态变量的定义和用法 | Master the definition and usage of constants and static variables
- 理解类型推断与显式类型注解的使用场景 | Understand when to use type inference vs explicit type annotations
- 能够选择合适的数据类型解决实际问题 | Be able to choose appropriate data types for real-world problems

## 详细内容 | Detailed Content

### 1. 变量声明基础 | Variable Declaration Basics (1小时 | 1 hour)

- **变量声明与绑定 | Variable Declaration and Binding**
  
  **概念定义 | Concept Definition:**
  在Rust中，变量是通过`let`关键字声明的，它将一个值绑定到一个标识符上。默认情况下，所有变量都是不可变的(immutable)。 | In Rust, variables are declared using the `let` keyword, which binds a value to an identifier. By default, all variables are immutable.
  
  **核心特征 | Key Characteristics:**
  - 变量默认不可变，需要`mut`关键字才能修改 | Variables are immutable by default, requiring the `mut` keyword to modify
  - 变量名使用snake_case命名约定 | Variable names use snake_case naming convention
  - 变量可以被重新声明(shadowing) | Variables can be redeclared (shadowing)
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Rust中的变量默认是可变的吗？| Are variables in Rust mutable by default?
     **答案 | Answer:** 否 | No - Rust变量默认是不可变的，这是Rust安全性设计的核心特征
  2. 如果我想修改一个变量的值，需要什么关键字？| What keyword is needed to modify a variable's value?  
     **答案 | Answer:** mut - 在声明时加上mut关键字使变量可变
  3. 可以用相同的名字重新声明变量吗？| Can you redeclare a variable with the same name?
     **答案 | Answer:** 可以 | Yes - 这叫做变量遮蔽(shadowing)，是Rust的合法特性
  4. 变量命名应该使用什么风格？| What naming style should be used for variables?
     **答案 | Answer:** snake_case - Rust推荐使用下划线分隔的小写命名风格
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      // 不可变变量声明 | Immutable variable declaration
      let x = 5;
      println!("x的值是: {}", x); // x的值是: 5 | The value of x is: 5
      
      // x = 6; // 这会导致编译错误！| This would cause a compile error!
      
      // 可变变量声明 | Mutable variable declaration
      let mut y = 10;
      println!("y的初始值: {}", y); // y的初始值: 10 | Initial value of y: 10
      y = 20; // 可以修改 | Can be modified
      println!("y的新值: {}", y); // y的新值: 20 | New value of y: 20
      
      // 变量遮蔽 | Variable shadowing
      let z = 5;
      let z = z + 1; // 创建新变量，遮蔽了之前的z | Creates new variable, shadows previous z
      let z = z * 2; // 再次遮蔽 | Shadow again
      println!("z的最终值: {}", z); // z的最终值: 12 | Final value of z: 12
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码会输出什么？| What will this code output?
    **答案 | Answer:** x的值是: 5, y的初始值: 10, y的新值: 20, z的最终值: 12
  - 如果去掉y声明中的mut会发生什么？| What happens if we remove mut from y's declaration?
    **答案 | Answer:** 编译错误，因为试图修改不可变变量 | Compile error, because trying to modify an immutable variable
  
  **常见误区检查 | Common Misconception Checks:**
  - 变量遮蔽会修改原来的变量吗？| Does variable shadowing modify the original variable?
    **答案 | Answer:** 不会，遮蔽是创建新的变量，原变量在新作用域中不可访问 | No, shadowing creates a new variable, the original is inaccessible in the new scope
  - mut关键字是否可以后续添加？| Can the mut keyword be added later?
    **答案 | Answer:** 不能，mut必须在变量声明时指定 | No, mut must be specified at variable declaration time

### 2. 基本数据类型 | Basic Data Types (1.5小时 | 1.5 hours)

- **整数类型 | Integer Types**
  
  **概念定义 | Concept Definition:**
  Rust提供了多种整数类型，包括有符号(i8, i16, i32, i64, i128, isize)和无符号(u8, u16, u32, u64, u128, usize)整数，数字表示位数。 | Rust provides multiple integer types, including signed (i8, i16, i32, i64, i128, isize) and unsigned (u8, u16, u32, u64, u128, usize) integers, where the number indicates the bit size.
  
  **核心特征 | Key Characteristics:**
  - 有符号整数可以存储负数，无符号整数只能存储非负数 | Signed integers can store negative numbers, unsigned integers only store non-negative numbers
  - 默认整数类型是i32 | Default integer type is i32
  - isize和usize的大小取决于目标架构(32位或64位) | isize and usize size depends on target architecture (32-bit or 64-bit)
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. u8类型可以存储负数吗？| Can u8 type store negative numbers?
     **答案 | Answer:** 不能 | No - u8是无符号8位整数，只能存储0-255
  2. 如果不指定整数类型，Rust会使用什么默认类型？| What default type does Rust use for integers if not specified?
     **答案 | Answer:** i32 - 这是Rust的默认整数类型
  3. i8能存储的最大值是多少？| What's the maximum value that i8 can store?
     **答案 | Answer:** 127 - i8是有符号8位整数，范围是-128到127
  4. usize类型的大小是固定的吗？| Is the size of usize type fixed?
     **答案 | Answer:** 不是 | No - usize大小取决于目标平台的指针大小
  
- **浮点数类型 | Floating-Point Types**
  
  **概念定义 | Concept Definition:**
  Rust有两种浮点数类型：f32(单精度)和f64(双精度)。默认使用f64，因为现代处理器上f64和f32速度相当但精度更高。 | Rust has two floating-point types: f32 (single precision) and f64 (double precision). f64 is the default because on modern processors f64 and f32 have similar speed but f64 provides better precision.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Rust的默认浮点数类型是什么？| What is Rust's default floating-point type?
     **答案 | Answer:** f64 - 双精度浮点数
  2. f32和f64哪个精度更高？| Which has higher precision, f32 or f64?
     **答案 | Answer:** f64 - 双精度比单精度精度更高
  
- **布尔类型 | Boolean Type**
  
  **概念定义 | Concept Definition:**
  布尔类型(bool)只有两个值：true和false，占用1个字节的存储空间。 | The boolean type (bool) has only two values: true and false, occupying 1 byte of storage.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Rust中数字0等于false吗？| Does the number 0 equal false in Rust?
     **答案 | Answer:** 不是 | No - Rust中只有显式的true和false，不会自动转换
  2. bool类型占用多少字节？| How many bytes does bool type occupy?
     **答案 | Answer:** 1字节 | 1 byte
  
- **字符类型 | Character Type**
  
  **概念定义 | Concept Definition:**
  字符类型(char)表示单个Unicode标量值，使用单引号声明，占用4个字节。 | The character type (char) represents a single Unicode scalar value, declared with single quotes, occupying 4 bytes.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. char类型使用什么符号声明？| What symbol is used to declare char type?
     **答案 | Answer:** 单引号 | Single quotes (')
  2. char可以存储中文字符吗？| Can char store Chinese characters?
     **答案 | Answer:** 可以 | Yes - char支持Unicode，可以存储任何Unicode字符
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      // 整数类型示例 | Integer type examples
      let decimal = 98_222;      // 十进制 | Decimal
      let hex = 0xff;            // 十六进制 | Hexadecimal  
      let octal = 0o77;          // 八进制 | Octal
      let binary = 0b1111_0000;  // 二进制 | Binary
      let byte = b'A';           // 字节(u8) | Byte (u8)
      
      println!("不同进制的数字 | Numbers in different bases:");
      println!("decimal: {}, hex: {}, octal: {}, binary: {}, byte: {}", 
               decimal, hex, octal, binary, byte);
      
      // 显式类型注解 | Explicit type annotation
      let x: i8 = 127;
      let y: u64 = 18_446_744_073_709_551_615;
      println!("i8 最大值: {}, u64 最大值: {}", x, y);
      
      // 浮点数 | Floating-point numbers
      let float_default = 2.0;      // f64 by default
      let float_32: f32 = 3.0;      // f32 explicitly
      println!("浮点数: f64={}, f32={}", float_default, float_32);
      
      // 布尔值 | Boolean values
      let t = true;
      let f: bool = false;
      println!("布尔值: t={}, f={}", t, f);
      
      // 字符 | Characters
      let c = 'z';
      let z = 'ℤ';
      let heart_eyed_cat = '😻';
      let chinese = '中';
      println!("字符: ASCII={}, Math={}, Emoji={}, Chinese={}", 
               c, z, heart_eyed_cat, chinese);
  }
  ```

### 3. 常量与静态变量 | Constants and Static Variables (45分钟 | 45 minutes)

- **常量定义 | Constant Definition**
  
  **概念定义 | Concept Definition:**
  常量使用const关键字声明，必须注解类型，在编译时确定值，在整个程序生命周期内有效。常量名使用SCREAMING_SNAKE_CASE。 | Constants are declared using the const keyword, must have type annotations, values are determined at compile time, and are valid for the entire program lifetime. Constant names use SCREAMING_SNAKE_CASE.
  
  **核心特征 | Key Characteristics:**
  - 必须在声明时初始化且不能更改 | Must be initialized at declaration and cannot be changed
  - 值必须是编译时常量表达式 | Value must be a compile-time constant expression
  - 可以在任何作用域中声明 | Can be declared in any scope
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 常量可以在运行时计算值吗？| Can constants have values calculated at runtime?
     **答案 | Answer:** 不能 | No - 常量值必须在编译时确定
  2. 常量需要类型注解吗？| Do constants require type annotations?
     **答案 | Answer:** 需要 | Yes - 常量必须显式指定类型
  3. 常量命名应该用什么风格？| What naming style should be used for constants?
     **答案 | Answer:** SCREAMING_SNAKE_CASE - 全大写字母加下划线
  
- **静态变量 | Static Variables**
  
  **概念定义 | Concept Definition:**
  静态变量使用static关键字声明，具有'static生命周期，在程序整个运行期间都存在。可以是可变的，但访问可变静态变量是unsafe的。 | Static variables are declared using the static keyword, have a 'static lifetime, and exist for the entire duration of the program. They can be mutable, but accessing mutable static variables is unsafe.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 静态变量和常量的主要区别是什么？| What's the main difference between static variables and constants?
     **答案 | Answer:** 静态变量有固定内存地址，常量可能被内联 | Static variables have a fixed memory address, constants may be inlined
  2. 访问可变静态变量需要什么？| What is required to access mutable static variables?
     **答案 | Answer:** unsafe块 | unsafe block
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 全局常量 | Global constants
  const MAX_POINTS: u32 = 100_000;
  const PI: f64 = 3.14159265359;
  
  // 静态变量 | Static variables
  static HELLO_WORLD: &str = "Hello, world!";
  static mut COUNTER: usize = 0;
  
  fn main() {
      // 使用常量 | Using constants
      println!("最大分数: {}", MAX_POINTS);
      println!("圆周率: {}", PI);
      
      // 局部常量 | Local constants
      const SECONDS_IN_HOUR: u32 = 60 * 60;
      println!("一小时有 {} 秒", SECONDS_IN_HOUR);
      
      // 使用静态变量 | Using static variables
      println!("静态字符串: {}", HELLO_WORLD);
      
      // 访问可变静态变量(unsafe) | Accessing mutable static variable (unsafe)
      unsafe {
          COUNTER += 1;
          println!("计数器: {}", COUNTER);
      }
  }
  ```

### 4. 类型推断与类型注解 | Type Inference and Type Annotations (30分钟 | 30 minutes)

- **类型推断机制 | Type Inference Mechanism**
  
  **概念定义 | Concept Definition:**
  Rust编译器可以根据值和使用上下文自动推断变量的类型，减少了显式类型注解的需要。但在某些情况下仍需要显式指定类型。 | The Rust compiler can automatically infer variable types based on values and usage context, reducing the need for explicit type annotations. However, explicit type specification is still needed in some cases.
  
  **核心特征 | Key Characteristics:**
  - 编译器会分析变量的使用方式来推断类型 | Compiler analyzes variable usage patterns to infer types
  - 当推断不明确时需要显式类型注解 | Explicit type annotations are needed when inference is ambiguous
  - 类型一旦确定就不能改变 | Once determined, types cannot be changed
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Rust总是需要显式类型注解吗？| Does Rust always require explicit type annotations?
     **答案 | Answer:** 不是 | No - Rust有强大的类型推断系统
  2. 什么时候必须提供类型注解？| When must type annotations be provided?
     **答案 | Answer:** 当编译器无法推断类型或推断不明确时 | When the compiler cannot infer the type or inference is ambiguous
  3. 变量的类型可以在运行时改变吗？| Can a variable's type change at runtime?
     **答案 | Answer:** 不能 | No - Rust是静态类型语言
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      // 类型推断 | Type inference
      let guess = "42".parse().expect("Not a number!"); // 编译错误！类型不明确
      // let guess: i32 = "42".parse().expect("Not a number!"); // 正确
      // let guess = "42".parse::<i32>().expect("Not a number!"); // 也正确
      
      // 推断示例 | Inference examples
      let x = 5; // 推断为 i32 | Inferred as i32
      let y = 3.0; // 推断为 f64 | Inferred as f64
      let z = x + 1; // z 也是 i32 | z is also i32
      
      println!("x: {}, y: {}, z: {}", x, y, z);
      
      // 显式类型注解 | Explicit type annotations
      let a: u8 = 255;
      let b: f32 = 2.5;
      let c: char = 'A';
      let d: bool = true;
      
      println!("显式类型: a={}, b={}, c={}, d={}", a, b, c, d);
      
      // 集合类型需要注解 | Collection types need annotations
      let numbers: Vec<i32> = Vec::new();
      // let numbers = Vec::new(); // 编译错误！类型不明确
      
      println!("空向量长度: {}", numbers.len());
  }
  ```

### 5. 类型转换基础 | Basic Type Conversion (30分钟 | 30 minutes)

- **显式类型转换 | Explicit Type Conversion**
  
  **概念定义 | Concept Definition:**
  Rust不允许隐式类型转换，所有类型转换必须显式进行。可以使用as关键字进行基本类型转换，或使用From/Into trait进行更复杂的转换。 | Rust doesn't allow implicit type conversions; all type conversions must be explicit. You can use the as keyword for basic type conversions, or the From/Into traits for more complex conversions.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Rust允许隐式类型转换吗？| Does Rust allow implicit type conversions?
     **答案 | Answer:** 不允许 | No - 所有转换必须显式进行
  2. 使用什么关键字进行基本类型转换？| What keyword is used for basic type conversions?
     **答案 | Answer:** as - 用于基本数值类型之间的转换
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      // 数值类型转换 | Numeric type conversions
      let x: i32 = 42;
      let y: f64 = x as f64; // i32 转 f64
      let z: u8 = y as u8;   // f64 转 u8 (可能丢失精度！)
      
      println!("转换: {} -> {} -> {}", x, y, z);
      
      // 字符和数字转换 | Character and number conversions
      let c = 'A';
      let ascii_value = c as u8;
      let back_to_char = ascii_value as char;
      
      println!("字符转换: '{}' -> {} -> '{}'", c, ascii_value, back_to_char);
      
      // 注意：转换可能丢失数据 | Note: conversions may lose data
      let large_number: i32 = 1000;
      let small_number: u8 = large_number as u8; // 1000 % 256 = 232
      println!("数据丢失示例: {} -> {}", large_number, small_number);
  }
  ```

### 6. 实践应用与最佳实践 | Practical Applications and Best Practices (15分钟 | 15 minutes)

- **变量和类型选择指南 | Variable and Type Selection Guidelines**
  
  **概念定义 | Concept Definition:**
  选择合适的变量可变性和数据类型对程序的安全性、性能和可读性都很重要。应该优先选择不可变变量和最小够用的数据类型。 | Choosing appropriate variable mutability and data types is important for program safety, performance, and readability. Prioritize immutable variables and the smallest sufficient data types.
  
  **最佳实践原则 | Best Practice Principles:**
  - 默认使用不可变变量，只在必要时使用mut | Use immutable variables by default, only use mut when necessary
  - 选择合适大小的整数类型，避免过度分配 | Choose appropriately sized integer types, avoid over-allocation
  - 在需要精确计算时考虑使用整数而非浮点数 | Consider using integers instead of floating-point for precise calculations
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 应该优先选择可变还是不可变变量？| Should you prioritize mutable or immutable variables?
     **答案 | Answer:** 不可变 | Immutable - 默认不可变提高代码安全性
  2. 存储0-100范围的数字应该选择什么类型？| What type should be chosen for storing numbers in the range 0-100?
     **答案 | Answer:** u8 - 无符号8位整数足够且内存效率高
  3. 什么情况下应该使用显式类型注解？| When should explicit type annotations be used?
     **答案 | Answer:** 当推断不明确或需要特定类型时 | When inference is ambiguous or a specific type is needed

## 实践项目：个人信息管理器 | Practical Project: Personal Information Manager

### 目标 | Objective
创建一个简单的个人信息管理器，综合应用变量声明、数据类型选择、常量定义等概念，演示不同数据类型在实际项目中的使用。 | Create a simple personal information manager that comprehensively applies variable declaration, data type selection, constant definition, and demonstrates the use of different data types in a real project.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 什么时候变量需要使用mut关键字？| When do variables need the mut keyword?
   **答案 | Answer:** 当变量的值需要在声明后修改时 | When the variable's value needs to be modified after declaration
2. 存储年龄信息应该选择什么数据类型？| What data type should be chosen for storing age information?
   **答案 | Answer:** u8 - 年龄范围通常在0-255内，u8足够且节省内存 | u8 - age range is typically within 0-255, u8 is sufficient and memory-efficient
3. 程序中的固定配置值应该如何定义？| How should fixed configuration values be defined in a program?
   **答案 | Answer:** 使用const关键字定义常量 | Use const keyword to define constants

### 步骤 | Steps
1. 定义程序常量和配置 | Define program constants and configuration
2. 创建用户信息变量 | Create user information variables
3. 实现信息输入和验证 | Implement information input and validation
4. 演示类型转换和计算 | Demonstrate type conversion and calculations
5. 输出格式化的用户信息 | Output formatted user information

### 示例代码 | Example Code
```rust
"""
个人信息管理器 | Personal Information Manager
演示Rust变量、数据类型和常量的综合应用 | Demonstrates comprehensive application of Rust variables, data types, and constants

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 变量声明和可变性 | Variable declaration and mutability
- 基本数据类型选择 | Basic data type selection
- 常量定义和使用 | Constant definition and usage
- 类型推断和显式注解 | Type inference and explicit annotations
"""

use std::io;

// 程序常量定义 | Program constant definitions
const MAX_NAME_LENGTH: usize = 50;
const MIN_AGE: u8 = 0;
const MAX_AGE: u8 = 150;
const COMPANY_NAME: &str = "个人信息管理系统 | Personal Info Manager";

fn main() {
    println!("=== {} ===", COMPANY_NAME);
    println!("欢迎使用个人信息管理器！| Welcome to Personal Information Manager!");
    
    // 用户信息变量 | User information variables
    let mut name = String::new();
    let mut age: u8 = 0;
    let mut height: f32 = 0.0;
    let mut is_student: bool = false;
    let mut grade_average: f64 = 0.0;
    
    // 统计信息 | Statistics
    let mut total_inputs: u8 = 0;
    
    // 收集姓名信息 | Collect name information
    println!("\n请输入您的姓名 (最多{}个字符): | Please enter your name (max {} characters):", 
             MAX_NAME_LENGTH, MAX_NAME_LENGTH);
    io::stdin().read_line(&mut name).expect("读取输入失败 | Failed to read input");
    name = name.trim().to_string();
    total_inputs += 1;
    
    // 验证姓名长度 | Validate name length
    if name.len() > MAX_NAME_LENGTH {
        println!("警告：姓名过长，将截取前{}个字符 | Warning: Name too long, truncating to {} characters", 
                 MAX_NAME_LENGTH, MAX_NAME_LENGTH);
        name.truncate(MAX_NAME_LENGTH);
    }
    
    // 收集年龄信息 | Collect age information
    loop {
        println!("请输入您的年龄 ({}-{}): | Please enter your age ({}-{}):", MIN_AGE, MAX_AGE, MIN_AGE, MAX_AGE);
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("读取输入失败 | Failed to read input");
        
        // 类型转换和验证 | Type conversion and validation
        match age_input.trim().parse::<u8>() {
            Ok(parsed_age) => {
                if parsed_age >= MIN_AGE && parsed_age <= MAX_AGE {
                    age = parsed_age;
                    total_inputs += 1;
                    break;
                } else {
                    println!("年龄必须在{}-{}范围内！| Age must be between {}-{}!", 
                             MIN_AGE, MAX_AGE, MIN_AGE, MAX_AGE);
                }
            }
            Err(_) => {
                println!("请输入有效的数字！| Please enter a valid number!");
            }
        }
    }
    
    // 收集身高信息 | Collect height information
    println!("请输入您的身高(米): | Please enter your height (meters):");
    let mut height_input = String::new();
    io::stdin().read_line(&mut height_input).expect("读取输入失败 | Failed to read input");
    
    match height_input.trim().parse::<f32>() {
        Ok(parsed_height) => {
            height = parsed_height;
            total_inputs += 1;
        }
        Err(_) => {
            println!("身高输入无效，使用默认值0.0 | Invalid height input, using default 0.0");
            height = 0.0;
        }
    }
    
    // 收集学生状态 | Collect student status
    println!("您是学生吗？(y/n): | Are you a student? (y/n):");
    let mut student_input = String::new();
    io::stdin().read_line(&mut student_input).expect("读取输入失败 | Failed to read input");
    
    is_student = match student_input.trim().to_lowercase().as_str() {
        "y" | "yes" | "是" => {
            total_inputs += 1;
            true
        }
        _ => false,
    };
    
    // 如果是学生，收集平均分 | If student, collect grade average
    if is_student {
        println!("请输入您的平均分 (0.0-100.0): | Please enter your grade average (0.0-100.0):");
        let mut grade_input = String::new();
        io::stdin().read_line(&mut grade_input).expect("读取输入失败 | Failed to read input");
        
        match grade_input.trim().parse::<f64>() {
            Ok(parsed_grade) => {
                if parsed_grade >= 0.0 && parsed_grade <= 100.0 {
                    grade_average = parsed_grade;
                    total_inputs += 1;
                } else {
                    println!("平均分必须在0.0-100.0范围内 | Grade average must be between 0.0-100.0");
                }
            }
            Err(_) => {
                println!("平均分输入无效 | Invalid grade average input");
            }
        }
    }
    
    // 计算和派生信息 | Calculate and derive information
    let height_cm: u16 = (height * 100.0) as u16; // 类型转换：f32 -> u16
    let birth_year: u16 = 2024 - (age as u16); // 类型转换：u8 -> u16
    let name_length: usize = name.len();
    let is_adult: bool = age >= 18;
    
    // 输出格式化信息 | Output formatted information
    println!("\n=== 个人信息汇总 | Personal Information Summary ===");
    println!("姓名 | Name: {}", name);
    println!("年龄 | Age: {} 岁 | years old", age);
    println!("出生年份 | Birth Year: {}", birth_year);
    println!("身高 | Height: {:.2} 米 | meters ({} 厘米 | cm)", height, height_cm);
    println!("是否成年 | Adult: {}", if is_adult { "是 | Yes" } else { "否 | No" });
    println!("学生状态 | Student Status: {}", if is_student { "是 | Yes" } else { "否 | No" });
    
    if is_student && grade_average > 0.0 {
        println!("平均分 | Grade Average: {:.2}", grade_average);
        let grade_level = match grade_average {
            90.0..=100.0 => "优秀 | Excellent",
            80.0..=89.99 => "良好 | Good", 
            70.0..=79.99 => "中等 | Average",
            60.0..=69.99 => "及格 | Pass",
            _ => "不及格 | Fail",
        };
        println!("成绩等级 | Grade Level: {}", grade_level);
    }
    
    // 统计信息 | Statistics
    println!("\n=== 数据统计 | Data Statistics ===");
    println!("姓名长度 | Name Length: {} 字符 | characters", name_length);
    println!("有效输入次数 | Valid Inputs: {}", total_inputs);
    println!("数据完整性 | Data Completeness: {:.1}%", 
             (total_inputs as f32 / 5.0) * 100.0);
    
    // 类型信息展示 | Type information display
    println!("\n=== 类型信息 | Type Information ===");
    println!("姓名类型 | Name type: String");
    println!("年龄类型 | Age type: u8 (范围 | range: 0-255)");
    println!("身高类型 | Height type: f32 (单精度浮点 | single precision float)");
    println!("学生状态类型 | Student status type: bool");
    println!("平均分类型 | Grade average type: f64 (双精度浮点 | double precision float)");
    
    println!("\n谢谢使用！| Thank you for using!");
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确应用了不同的数据类型？| Does the project correctly apply different data types?
2. 常量定义是否遵循了命名规范？| Do constant definitions follow naming conventions?
3. 变量的可变性设置是否合理？| Is the mutability setting of variables reasonable?
4. 类型转换是否安全和正确？| Are type conversions safe and correct?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **变量可变性理解强化练习 | Variable Mutability Understanding Reinforcement Exercise**
   - **练习描述 | Exercise Description:** 创建一个程序，演示可变和不可变变量在不同场景下的使用，包括变量遮蔽的应用 | Create a program demonstrating the use of mutable and immutable variables in different scenarios, including variable shadowing applications
   - **概念检查 | Concept Check:** 能否解释为什么Rust默认变量不可变？变量遮蔽和重新赋值有什么区别？| Can you explain why Rust variables are immutable by default? What's the difference between variable shadowing and reassignment?
   - **学习目标 | Learning Objective:** 深入理解Rust的安全性设计理念和变量管理机制 | Deeply understand Rust's safety design philosophy and variable management mechanisms

2. **数据类型选择练习 | Data Type Selection Exercise**
   - **练习描述 | Exercise Description:** 为不同的应用场景选择最合适的数据类型，如存储像素RGB值、计算器程序、计数器等 | Choose the most appropriate data types for different application scenarios, such as storing pixel RGB values, calculator programs, counters, etc.
   - **概念检查 | Concept Check:** 为什么选择特定的数据类型？如何平衡内存使用和数据范围需求？| Why choose specific data types? How to balance memory usage and data range requirements?
   - **学习目标 | Learning Objective:** 培养根据实际需求选择合适数据类型的能力 | Develop the ability to choose appropriate data types based on actual requirements

3. **类型转换安全练习 | Type Conversion Safety Exercise**
   - **练习描述 | Exercise Description:** 实现一个数据转换工具，处理不同数据类型之间的安全转换，包括错误处理 | Implement a data conversion tool that handles safe conversions between different data types, including error handling
   - **概念检查 | Concept Check:** 什么情况下类型转换会丢失数据？如何检测和处理转换错误？| Under what circumstances do type conversions lose data? How to detect and handle conversion errors?
   - **学习目标 | Learning Objective:** 掌握安全的类型转换技术和错误处理策略 | Master safe type conversion techniques and error handling strategies

4. **常量和配置管理练习 | Constants and Configuration Management Exercise**
   - **练习描述 | Exercise Description:** 设计一个程序的配置系统，使用常量定义各种配置参数，演示配置的组织和使用 | Design a configuration system for a program, using constants to define various configuration parameters, demonstrating configuration organization and usage
   - **概念检查 | Concept Check:** 何时使用常量vs静态变量？如何组织程序的配置信息？| When to use constants vs static variables? How to organize program configuration information?
   - **学习目标 | Learning Objective:** 学会设计清晰的配置管理系统 | Learn to design clear configuration management systems

5. **类型推断边界探索练习 | Type Inference Boundary Exploration Exercise**
   - **练习描述 | Exercise Description:** 创建各种场景来测试Rust类型推断的能力和限制，了解何时需要显式类型注解 | Create various scenarios to test Rust's type inference capabilities and limitations, understanding when explicit type annotations are needed
   - **概念检查 | Concept Check:** 类型推断在哪些情况下会失败？如何帮助编译器推断类型？| In what situations does type inference fail? How to help the compiler infer types?
   - **学习目标 | Learning Objective:** 深入理解Rust的类型系统和推断机制 | Deeply understand Rust's type system and inference mechanisms

6. **内存使用优化练习 | Memory Usage Optimization Exercise**
   - **练习描述 | Exercise Description:** 分析不同数据类型的内存占用，设计内存高效的数据结构 | Analyze memory usage of different data types and design memory-efficient data structures
   - **概念检查 | Concept Check:** 不同数据类型占用多少内存？如何减少程序的内存占用？| How much memory do different data types consume? How to reduce program memory usage?
   - **学习目标 | Learning Objective:** 培养内存效率意识和优化技能 | Develop memory efficiency awareness and optimization skills

7. **综合应用项目扩展练习 | Comprehensive Application Project Extension Exercise**
   - **练习描述 | Exercise Description:** 扩展个人信息管理器项目，添加数据持久化、多用户支持、数据验证等功能 | Extend the personal information manager project by adding data persistence, multi-user support, data validation, and other features
   - **概念检查 | Concept Check:** 如何在复杂项目中应用今日学习的概念？如何保持代码的类型安全？| How to apply today's learned concepts in complex projects? How to maintain type safety in code?
   - **学习目标 | Learning Objective:** 提高在实际项目中综合运用基础概念的能力 | Improve the ability to comprehensively apply basic concepts in real projects

## 学习资源 | Learning Resources
- [Rust官方文档 - 数据类型 | Rust Official Documentation - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust官方文档 - 变量和可变性 | Rust Official Documentation - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust Reference - 类型推断 | Rust Reference - Type Inference](https://doc.rust-lang.org/reference/type-inference.html)
- [Rust By Example - 基本类型 | Rust By Example - Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解变量的不可变性默认特性和mut关键字的作用 | Understand the default immutability of variables and the role of the mut keyword
- [ ] 掌握Rust的基本数据类型及其使用场景 | Master Rust's basic data types and their usage scenarios
- [ ] 能够正确选择适合的数据类型解决实际问题 | Able to correctly choose appropriate data types to solve real problems
- [ ] 理解常量和静态变量的区别和使用方法 | Understand the difference and usage of constants and static variables
- [ ] 掌握类型推断机制和显式类型注解的使用时机 | Master type inference mechanisms and when to use explicit type annotations
- [ ] 理解类型转换的安全性考虑 | Understand safety considerations for type conversion
- [ ] 能够回答所有CCQs并解释核心概念 | Able to answer all CCQs and explain core concepts
- [ ] 完成个人信息管理器项目并理解其设计思路 | Complete the personal information manager project and understand its design approach
- [ ] 能够向他人解释变量可变性和数据类型的重要性 | Able to explain the importance of variable mutability and data types to others
- [ ] 完成至少3个扩展练习以巩固理解 | Complete at least 3 extension exercises to consolidate understanding

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释每个核心概念的特点、使用场景和重要性。特别要能解释为什么Rust选择默认不可变性，以及如何根据实际需求选择合适的数据类型。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain the characteristics, usage scenarios, and importance of each core concept to others. Particularly, be able to explain why Rust chooses default immutability and how to select appropriate data types based on actual requirements.