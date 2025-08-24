# Rust入门 - 第12天：借用基础 | Rust Introduction - Day 12: Borrowing Basics

## 学习目标 | Learning Objectives
- 掌握不可变借用的概念和使用方法 | Master the concept and usage of immutable borrowing
- 理解可变借用的规则和应用场景 | Understand the rules and application scenarios of mutable borrowing
- 学会借用检查器的工作原理和常见错误处理 | Learn how the borrow checker works and handle common errors
- 掌握借用规则以避免数据竞争和内存安全问题 | Master borrowing rules to avoid data races and memory safety issues
- 理解引用的生命周期基础概念 | Understand basic concepts of reference lifetimes
- 学会在实际编程中正确使用借用机制 | Learn to correctly use borrowing mechanisms in practical programming

## 详细内容 | Detailed Content

### 1. 借用概念基础 | Borrowing Concept Fundamentals (1小时 | 1 hour)

- **借用的核心概念 | Core Concept of Borrowing**
  
  **概念定义 | Concept Definition:**
  借用是Rust中一种允许你使用值而不取得其所有权的机制。通过创建引用，你可以访问数据而不需要移动它，这是Rust内存安全的核心特性之一。 | Borrowing is a mechanism in Rust that allows you to use values without taking ownership of them. By creating references, you can access data without needing to move it, which is one of the core features of Rust's memory safety.
  
  **核心特征 | Key Characteristics:**
  - 借用创建引用而不转移所有权 | Borrowing creates references without transferring ownership
  - 引用在作用域结束时自动清理，不影响原数据 | References are automatically cleaned up when scope ends, without affecting original data
  - 借用检查器在编译时确保内存安全 | The borrow checker ensures memory safety at compile time
  - 借用遵循严格的规则以防止数据竞争 | Borrowing follows strict rules to prevent data races
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 借用会转移数据的所有权吗？| Does borrowing transfer ownership of data?
     **答案 | Answer:** 否 | No - 借用创建引用，原始数据保持其所有权 | Borrowing creates references, the original data keeps its ownership
  2. 引用的符号是什么？| What is the symbol for references?
     **答案 | Answer:** & - & 符号用于创建引用 | The & symbol is used to create references
  3. 借用检查器在什么时候进行检查？| When does the borrow checker perform checks?
     **答案 | Answer:** 编译时 | Compile time - 这确保了运行时的内存安全 | This ensures memory safety at runtime
  4. 多个不可变借用可以同时存在吗？| Can multiple immutable borrows exist simultaneously?
     **答案 | Answer:** 是 | Yes - 多个不可变借用是安全的，因为它们不会修改数据 | Multiple immutable borrows are safe because they don't modify data
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 借用概念演示 | Borrowing concept demonstration
  fn main() {
      let s1 = String::from("hello"); // s1 拥有字符串的所有权 | s1 owns the string
      let len = calculate_length(&s1); // 借用 s1，不转移所有权 | Borrow s1, don't transfer ownership
      
      println!("'{}'的长度是 {} | The length of '{}' is {}", s1, len, s1, len);
      // s1 仍然有效，因为我们只是借用了它 | s1 is still valid because we only borrowed it
  }

  // 接受字符串引用作为参数 | Accept string reference as parameter
  fn calculate_length(s: &String) -> usize {
      s.len() // 通过引用访问数据 | Access data through reference
  } // s 超出作用域，但因为它是借用，所以不会释放数据 | s goes out of scope, but since it's borrowed, data isn't freed
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码会输出什么？| What will this code output?
    **答案 | Answer:** 'hello'的长度是 5 | The length of 'hello' is 5
  - 如果我们在函数中尝试修改借用的字符串会发生什么？| What happens if we try to modify the borrowed string in the function?
    **答案 | Answer:** 编译错误，因为默认的借用是不可变的 | Compilation error, because default borrowing is immutable
  
  **常见误区检查 | Common Misconception Checks:**
  - 借用会复制数据吗？| Does borrowing copy data?
    **答案 | Answer:** 否，借用只创建指向原数据的引用 | No, borrowing only creates a reference pointing to the original data
  - 函数结束时借用的数据会被释放吗？| Is borrowed data freed when the function ends?
    **答案 | Answer:** 否，只有拥有所有权的变量才会在作用域结束时释放数据 | No, only variables that own the data free it when scope ends

- **引用的类型系统 | Reference Type System**
  
  **概念定义 | Concept Definition:**
  Rust中的引用是一种指向内存中某个位置的指针，但它们受到类型系统和借用检查器的严格管理，确保内存安全和类型安全。 | References in Rust are pointers to memory locations, but they are strictly managed by the type system and borrow checker to ensure memory and type safety.
  
  **核心特征 | Key Characteristics:**
  - 引用必须总是有效的（不能悬垂）| References must always be valid (cannot be dangling)
  - 引用的类型必须与被引用的数据类型匹配 | Reference types must match the type of the referenced data
  - 编译器会推断引用的生命周期 | The compiler infers the lifetime of references
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 引用可以指向已经被释放的内存吗？| Can references point to freed memory?
     **答案 | Answer:** 否 | No - Rust的借用检查器会阻止这种悬垂引用 | Rust's borrow checker prevents such dangling references
  2. &i32 和 i32 是相同的类型吗？| Are &i32 and i32 the same type?
     **答案 | Answer:** 否 | No - &i32是i32的引用类型，它们是不同的类型 | &i32 is a reference type to i32, they are different types
  3. 引用需要手动释放内存吗？| Do references need manual memory deallocation?
     **答案 | Answer:** 否 | No - 引用在超出作用域时自动失效，不涉及内存释放 | References automatically become invalid when out of scope, no memory deallocation involved
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      let x = 42; // 拥有一个整数值 | Own an integer value
      let r = &x; // 创建对x的不可变引用 | Create an immutable reference to x
      
      println!("x = {}, r = {}", x, r); // 都可以访问值 | Both can access the value
      println!("r指向的地址: {:p}", r); // 打印引用的地址 | Print the address the reference points to
      
      // 类型注解示例 | Type annotation example
      let y: i32 = 100;
      let ref_y: &i32 = &y; // 明确指定引用类型 | Explicitly specify reference type
      
      assert_eq!(*ref_y, 100); // 解引用操作 | Dereferencing operation
  }
  ```

### 2. 不可变借用详解 | Immutable Borrowing Deep Dive (1小时 | 1 hour)

- **不可变借用规则 | Immutable Borrowing Rules**
  
  **概念定义 | Concept Definition:**
  不可变借用允许你读取数据但不能修改它。可以同时拥有任意数量的不可变引用，因为只读操作不会产生数据竞争。 | Immutable borrowing allows you to read data but not modify it. You can have any number of immutable references simultaneously because read-only operations don't create data races.
  
  **核心特征 | Key Characteristics:**
  - 使用&语法创建不可变引用 | Use & syntax to create immutable references
  - 可以同时存在多个不可变引用 | Multiple immutable references can exist simultaneously
  - 不可变引用不能用于修改数据 | Immutable references cannot be used to modify data
  - 在不可变引用存在期间，原始数据也不能被修改 | Original data cannot be modified while immutable references exist
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 可以同时创建多个不可变引用吗？| Can multiple immutable references be created simultaneously?
     **答案 | Answer:** 是 | Yes - 多个只读引用是安全的 | Multiple read-only references are safe
  2. 通过不可变引用可以修改数据吗？| Can data be modified through immutable references?
     **答案 | Answer:** 否 | No - 不可变引用只允许读取 | Immutable references only allow reading
  3. 当存在不可变引用时，可以修改原始数据吗？| Can original data be modified when immutable references exist?
     **答案 | Answer:** 否 | No - 这会违反借用规则 | This would violate borrowing rules
  4. &str 是什么类型的引用？| What type of reference is &str?
     **答案 | Answer:** 不可变引用 | Immutable reference - 字符串字面量的不可变引用 | Immutable reference to string literals
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      let data = vec![1, 2, 3, 4, 5]; // 拥有一个向量 | Own a vector
      
      // 创建多个不可变引用 | Create multiple immutable references
      let r1 = &data;
      let r2 = &data;
      let r3 = &data;
      
      // 所有引用都可以读取数据 | All references can read data
      println!("r1: {:?}", r1);
      println!("r2: {:?}", r2);
      println!("r3: {:?}", r3);
      
      // 可以传递给需要不可变引用的函数 | Can be passed to functions requiring immutable references
      let sum = calculate_sum(&data);
      println!("总和: {} | Sum: {}", sum, sum);
      
      // 原始数据仍然可以读取 | Original data can still be read
      println!("原始数据: {:?} | Original data: {:?}", data, data);
  }

  fn calculate_sum(numbers: &Vec<i32>) -> i32 {
      numbers.iter().sum() // 通过不可变引用计算总和 | Calculate sum through immutable reference
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码能编译通过吗？| Will this code compile?
    **答案 | Answer:** 是 | Yes - 多个不可变引用是允许的 | Multiple immutable references are allowed
  - 如果在存在引用的情况下尝试修改data会怎样？| What if we try to modify data while references exist?
    **答案 | Answer:** 编译错误 | Compilation error
  
  **常见误区检查 | Common Misconception Checks:**
  - 不可变引用是否意味着数据永远不能改变？| Do immutable references mean data can never change?
    **答案 | Answer:** 否，只是在引用存在期间不能改变 | No, just can't change while references exist
  - &和*是相对的操作吗？| Are & and * opposite operations?
    **答案 | Answer:** 是，&创建引用，*解引用 | Yes, & creates references, * dereferences

### 3. 可变借用规则 | Mutable Borrowing Rules (1小时 | 1 hour)

- **可变借用的独占性 | Exclusivity of Mutable Borrowing**
  
  **概念定义 | Concept Definition:**
  可变借用允许通过引用修改数据，但遵循严格的独占规则：在任何时间点，对于特定数据只能有一个可变引用，并且不能与任何不可变引用共存。 | Mutable borrowing allows modifying data through references, but follows strict exclusivity rules: at any time, there can only be one mutable reference to specific data, and it cannot coexist with any immutable references.
  
  **核心特征 | Key Characteristics:**
  - 使用&mut语法创建可变引用 | Use &mut syntax to create mutable references
  - 在同一时间只能有一个可变引用 | Only one mutable reference can exist at a time
  - 可变引用与不可变引用不能共存 | Mutable references cannot coexist with immutable references
  - 提供读写访问权限 | Provides read-write access permissions
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 可以同时创建两个可变引用吗？| Can two mutable references be created simultaneously?
     **答案 | Answer:** 否 | No - 这会导致数据竞争 | This would cause data races
  2. 可变引用和不可变引用可以同时存在吗？| Can mutable and immutable references exist simultaneously?
     **答案 | Answer:** 否 | No - 这违反了借用规则 | This violates borrowing rules
  3. 通过可变引用可以读取数据吗？| Can data be read through mutable references?
     **答案 | Answer:** 是 | Yes - 可变引用提供读写权限 | Mutable references provide read-write permissions
  4. &mut T 和 &T 是相同类型吗？| Are &mut T and &T the same type?
     **答案 | Answer:** 否 | No - 它们是不同的引用类型 | They are different reference types
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      let mut data = vec![1, 2, 3]; // 必须声明为可变 | Must be declared as mutable
      
      // 创建可变引用 | Create mutable reference
      {
          let r1 = &mut data;
          r1.push(4); // 通过可变引用修改数据 | Modify data through mutable reference
          println!("通过r1修改后: {:?} | After modification via r1: {:?}", r1, r1);
      } // r1 超出作用域 | r1 goes out of scope
      
      // 现在可以创建新的引用 | Now we can create new references
      let r2 = &data; // 不可变引用 | Immutable reference
      println!("数据现在是: {:?} | Data is now: {:?}", r2, r2);
      
      // 示例：修改函数 | Example: modifying function
      modify_vector(&mut data);
      println!("最终数据: {:?} | Final data: {:?}", data, data);
  }

  fn modify_vector(v: &mut Vec<i32>) {
      v.push(5); // 通过可变引用添加元素 | Add element through mutable reference
      v[0] = 10; // 修改现有元素 | Modify existing element
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么data必须声明为mut？| Why must data be declared as mut?
    **答案 | Answer:** 要创建可变引用，原始数据必须是可变的 | To create mutable references, original data must be mutable
  - 如果同时创建两个&mut引用会怎样？| What happens if we create two &mut references simultaneously?
    **答案 | Answer:** 编译错误 | Compilation error

- **借用作用域和NLL | Borrowing Scope and NLL**
  
  **概念定义 | Concept Definition:**
  非词法生命周期（Non-Lexical Lifetimes, NLL）是Rust编译器的改进，它基于实际使用而不是词法作用域来确定引用的生命周期，使借用更加灵活。 | Non-Lexical Lifetimes (NLL) is an improvement in the Rust compiler that determines reference lifetimes based on actual usage rather than lexical scope, making borrowing more flexible.
  
  **核心特征 | Key Characteristics:**
  - 引用的生命周期基于最后一次使用 | Reference lifetimes are based on last usage
  - 允许更灵活的借用模式 | Allows more flexible borrowing patterns
  - 减少了不必要的借用冲突 | Reduces unnecessary borrowing conflicts
  - 编译器更智能地分析代码流 | Compiler analyzes code flow more intelligently
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. NLL基于什么来确定引用的生命周期？| What does NLL base reference lifetimes on?
     **答案 | Answer:** 实际使用 | Actual usage - 而不是词法作用域 | Rather than lexical scope
  2. 引用在最后一次使用后还会阻止其他借用吗？| Do references prevent other borrows after their last use?
     **答案 | Answer:** 否 | No - NLL允许更早地"释放"引用 | NLL allows references to be "released" earlier
  3. 这种改进让借用检查器更严格还是更宽松？| Does this improvement make the borrow checker stricter or more permissive?
     **答案 | Answer:** 更宽松 | More permissive - 在保持安全的前提下 | While maintaining safety
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      let mut data = vec![1, 2, 3];
      
      // NLL允许这种模式 | NLL allows this pattern
      let r1 = &data; // 不可变引用 | Immutable reference
      println!("不可变引用: {:?} | Immutable reference: {:?}", r1, r1);
      // r1最后一次使用 ↑ | r1 last used ↑
      
      let r2 = &mut data; // 现在可以创建可变引用 | Now we can create mutable reference
      r2.push(4);
      println!("可变引用: {:?} | Mutable reference: {:?}", r2, r2);
      // r2最后一次使用 ↑ | r2 last used ↑
      
      // 现在又可以创建不可变引用 | Now we can create immutable reference again
      let r3 = &data;
      println!("又一个不可变引用: {:?} | Another immutable reference: {:?}", r3, r3);
  }
  ```

### 4. 借用检查器工作原理 | How the Borrow Checker Works (1小时 | 1 hour)

- **借用检查器的安全保证 | Safety Guarantees of Borrow Checker**
  
  **概念定义 | Concept Definition:**
  借用检查器是Rust编译器的一个组件，它通过静态分析确保内存安全，防止数据竞争、使用已释放内存、缓冲区溢出等常见内存安全问题。 | The borrow checker is a component of the Rust compiler that ensures memory safety through static analysis, preventing common memory safety issues like data races, use-after-free, and buffer overflows.
  
  **解决的问题 | Problems It Solves:**
  - 数据竞争：防止多个可变访问 | Data races: prevent multiple mutable accesses
  - 悬垂指针：防止使用已释放的内存 | Dangling pointers: prevent use of freed memory
  - 迭代器失效：防止在迭代时修改集合 | Iterator invalidation: prevent modifying collections during iteration
  - 内存泄露：通过所有权系统管理资源 | Memory leaks: manage resources through ownership system
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 借用检查器在什么时候进行检查？| When does the borrow checker perform checks?
     **答案 | Answer:** 编译时 | Compile time - 这是零成本抽象的一部分 | This is part of zero-cost abstractions
  2. 借用检查器的主要目标是什么？| What is the main goal of the borrow checker?
     **答案 | Answer:** 内存安全 | Memory safety - 防止内存相关的错误 | Prevent memory-related errors
  3. 借用规则会影响运行时性能吗？| Do borrowing rules affect runtime performance?
     **答案 | Answer:** 否 | No - 所有检查都在编译时完成 | All checks are done at compile time
  4. 违反借用规则的代码能编译通过吗？| Can code that violates borrowing rules compile?
     **答案 | Answer:** 否 | No - 编译器会报错并拒绝编译 | Compiler will error and refuse to compile
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      let mut data = vec![1, 2, 3, 4, 5];
      
      // 借用检查器保护：防止数据竞争 | Borrow checker protection: prevent data races
      let r1 = &data;
      // let r2 = &mut data; // 这行会导致编译错误 | This line would cause compilation error
      println!("安全的读取: {:?} | Safe read: {:?}", r1, r1);
      
      // 借用检查器保护：防止迭代器失效 | Borrow checker protection: prevent iterator invalidation
      for item in &data { // 创建不可变引用进行迭代 | Create immutable reference for iteration
          println!("项目: {} | Item: {}", item, item);
          // data.push(6); // 这行会导致编译错误 | This line would cause compilation error
      }
      
      // 安全的修改模式 | Safe modification pattern
      {
          let r_mut = &mut data;
          r_mut.push(6); // 在独占可变借用下安全修改 | Safe modification under exclusive mutable borrow
      }
      
      println!("修改后的数据: {:?} | Modified data: {:?}", data, data);
  }
  
  // 借用检查器防止悬垂引用 | Borrow checker prevents dangling references
  fn no_dangling_references() -> &String { // 这个函数无法编译 | This function cannot compile
      // let s = String::from("hello");
      // &s // 返回对局部变量的引用会导致编译错误 | Returning reference to local variable causes error
      todo!("This function demonstrates compiler protection")
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么在迭代时不能修改向量？| Why can't we modify a vector while iterating?
    **答案 | Answer:** 会使迭代器失效，可能导致未定义行为 | Would invalidate the iterator, potentially causing undefined behavior
  - 借用检查器如何知道引用是否安全？| How does the borrow checker know if references are safe?
    **答案 | Answer:** 通过分析代码的控制流和生命周期 | By analyzing code control flow and lifetimes

### 5. 常见借用错误和解决方案 | Common Borrowing Errors and Solutions (30分钟 | 30 minutes)

- **典型错误模式识别 | Identifying Typical Error Patterns**
  
  **关键原则 | Key Principles:**
  - 理解错误信息的含义 | Understand error message meanings
  - 识别常见的借用冲突模式 | Identify common borrowing conflict patterns  
  - 学会重构代码以满足借用规则 | Learn to refactor code to satisfy borrowing rules
  
  **实践验证问题 | Practice Verification Questions:**
  1. "cannot borrow as mutable"错误通常意味着什么？| What does "cannot borrow as mutable" error usually mean?
  2. 如何解决"cannot borrow ... as mutable more than once"错误？| How to solve "cannot borrow ... as mutable more than once" error?
  3. 作用域分离是什么技术？| What is scope separation technique?

### 6. 最佳实践总结 | Best Practices Summary (30分钟 | 30 minutes)

- **借用优化策略 | Borrowing Optimization Strategies**
  
  **关键原则 | Key Principles:**
  - 尽可能使用不可变借用 | Use immutable borrowing whenever possible
  - 保持借用作用域最小化 | Keep borrowing scopes minimal
  - 理解何时需要所有权转移 | Understand when ownership transfer is needed
  
  **实践验证问题 | Practice Verification Questions:**
  1. 什么时候应该选择借用而不是移动？| When should you choose borrowing over moving?
  2. 如何设计API来最大化借用的灵活性？| How to design APIs to maximize borrowing flexibility?
  3. 克隆数据什么时候是合理的选择？| When is cloning data a reasonable choice?

## 实践项目：图书馆管理系统 | Practical Project: Library Management System

### 目标 | Objective
创建一个图书馆管理系统，演示各种借用场景，包括不可变借用查询、可变借用更新和复杂的数据操作，帮助理解借用在实际项目中的应用。 | Create a library management system that demonstrates various borrowing scenarios, including immutable borrowing for queries, mutable borrowing for updates, and complex data operations, helping understand borrowing applications in real projects.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 什么时候应该使用不可变借用？| When should immutable borrowing be used?
   **答案 | Answer:** 当只需要读取数据，不修改数据时 | When only reading data without modification
2. 为什么可变借用具有独占性？| Why are mutable borrows exclusive?
   **答案 | Answer:** 防止数据竞争，确保内存安全 | To prevent data races and ensure memory safety
3. 如何解决借用检查器错误？| How to resolve borrow checker errors?
   **答案 | Answer:** 调整作用域、使用克隆或重构代码结构 | Adjust scopes, use cloning, or refactor code structure

### 步骤 | Steps
1. 设计数据结构：书籍和图书馆结构体 | Design data structures: Book and Library structs
2. 实现查询功能：使用不可变借用进行搜索 | Implement query functions: use immutable borrowing for search
3. 实现更新功能：使用可变借用修改数据 | Implement update functions: use mutable borrowing to modify data
4. 处理复杂操作：多步骤的借用协调 | Handle complex operations: multi-step borrowing coordination
5. 添加错误处理和用户界面 | Add error handling and user interface

### 示例代码 | Example Code
```rust
"""
图书馆管理系统 | Library Management System
演示Rust中借用机制的实际应用 | Demonstrates practical application of borrowing mechanisms in Rust

本项目演示以下借用概念的综合应用：| This project demonstrates comprehensive application of:
- 不可变借用进行数据查询 | Immutable borrowing for data queries
- 可变借用进行数据修改 | Mutable borrowing for data modification
- 借用规则在复杂操作中的应用 | Application of borrowing rules in complex operations
"""

#[derive(Debug, Clone)]
struct Book {
    id: u32,
    title: String,
    author: String,
    available: bool,
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
    name: String,
}

impl Library {
    // 使用不可变借用创建新图书馆 | Create new library using immutable borrowing
    fn new(name: &str) -> Self {
        Library {
            name: name.to_string(), // 借用字符串并转换为拥有的String | Borrow string and convert to owned String
            books: Vec::new(),
        }
    }
    
    // 添加书籍 - 需要可变借用self | Add book - requires mutable borrow of self
    fn add_book(&mut self, book: Book) {
        println!("添加书籍到 {} 图书馆 | Adding book to {} library", self.name, self.name);
        self.books.push(book); // 可变借用允许修改 | Mutable borrow allows modification
    }
    
    // 搜索书籍 - 只需要不可变借用 | Search books - only needs immutable borrow
    fn search_by_title(&self, title: &str) -> Option<&Book> {
        // 不可变借用允许多个同时查询 | Immutable borrowing allows multiple simultaneous queries
        self.books.iter().find(|book| book.title == title)
    }
    
    // 搜索可用书籍 - 返回借用的引用 | Search available books - return borrowed references
    fn get_available_books(&self) -> Vec<&Book> {
        self.books.iter().filter(|book| book.available).collect()
    }
    
    // 借出书籍 - 需要可变借用进行状态修改 | Lend book - requires mutable borrow for state modification
    fn lend_book(&mut self, title: &str) -> Result<(), String> {
        // 先进行不可变借用搜索，然后进行可变修改 | First immutable search, then mutable modification
        match self.books.iter_mut().find(|book| book.title == title) {
            Some(book) => {
                if book.available {
                    book.available = false; // 可变借用允许修改 | Mutable borrow allows modification
                    println!("成功借出: {} | Successfully lent: {}", book.title, book.title);
                    Ok(())
                } else {
                    Err(format!("书籍 '{}' 已被借出 | Book '{}' is already lent", title, title))
                }
            }
            None => Err(format!("未找到书籍 '{}' | Book '{}' not found", title, title)),
        }
    }
    
    // 归还书籍 - 可变借用修改状态 | Return book - mutable borrow to modify state
    fn return_book(&mut self, title: &str) -> Result<(), String> {
        match self.books.iter_mut().find(|book| book.title == title) {
            Some(book) => {
                if !book.available {
                    book.available = true;
                    println!("成功归还: {} | Successfully returned: {}", book.title, book.title);
                    Ok(())
                } else {
                    Err(format!("书籍 '{}' 并未被借出 | Book '{}' was not lent", title, title))
                }
            }
            None => Err(format!("未找到书籍 '{}' | Book '{}' not found", title, title)),
        }
    }
    
    // 获取统计信息 - 不可变借用进行计算 | Get statistics - immutable borrow for calculations
    fn get_statistics(&self) -> (usize, usize, usize) {
        let total = self.books.len();
        let available = self.books.iter().filter(|book| book.available).count();
        let lent = total - available;
        (total, available, lent)
    }
    
    // 演示复杂借用场景：批量操作 | Demonstrate complex borrowing: batch operations
    fn batch_return(&mut self, titles: &[&str]) -> Vec<Result<(), String>> {
        // 对每个标题进行可变借用操作 | Perform mutable borrow operation for each title
        titles.iter().map(|title| self.return_book(title)).collect()
    }
}

// 辅助函数：打印图书馆状态 - 使用不可变借用 | Helper function: print library status - using immutable borrow
fn print_library_status(library: &Library) {
    println!("\n=== {} 图书馆状态 | {} Library Status ===", library.name, library.name);
    let (total, available, lent) = library.get_statistics();
    println!("总书籍: {} | 可用: {} | 已借出: {} | Total books: {} | Available: {} | Lent: {}", 
             total, available, lent, total, available, lent);
    
    // 展示不可变借用的灵活性：同时创建多个引用 | Show flexibility of immutable borrowing: create multiple references simultaneously
    let available_books = library.get_available_books();
    if !available_books.is_empty() {
        println!("可用书籍 | Available books:");
        for book in available_books {
            println!("  - {} by {}", book.title, book.author);
        }
    }
}

// 演示借用规则的函数 | Function demonstrating borrowing rules
fn demonstrate_borrowing_rules() {
    println!("\n=== 借用规则演示 | Borrowing Rules Demonstration ===");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // 1. 多个不可变借用可以共存 | Multiple immutable borrows can coexist
    {
        let r1 = &numbers;
        let r2 = &numbers;
        let r3 = &numbers;
        println!("多个不可变引用 | Multiple immutable references: {:?}, {:?}, {:?}", r1, r2, r3);
    } // 所有不可变引用超出作用域 | All immutable references go out of scope
    
    // 2. 可变借用是独占的 | Mutable borrow is exclusive
    {
        let r_mut = &mut numbers;
        r_mut.push(6);
        println!("通过可变引用修改 | Modified through mutable reference: {:?}", r_mut);
        // 在这个作用域内不能创建其他引用 | Cannot create other references in this scope
    }
    
    // 3. NLL允许更灵活的借用 | NLL allows more flexible borrowing
    let r_read = &numbers;
    println!("读取数据 | Read data: {:?}", r_read);
    // r_read最后一次使用 | r_read last use
    
    let r_write = &mut numbers;
    r_write[0] = 10;
    println!("修改后数据 | Modified data: {:?}", r_write);
}

fn main() {
    // 创建图书馆实例 | Create library instance
    let mut library = Library::new("中央图书馆 | Central Library");
    
    // 添加书籍 - 需要可变借用 | Add books - requires mutable borrow
    library.add_book(Book {
        id: 1,
        title: "Rust编程语言 | The Rust Programming Language".to_string(),
        author: "Steve Klabnik".to_string(),
        available: true,
    });
    
    library.add_book(Book {
        id: 2,
        title: "Rust实战 | Rust in Action".to_string(),
        author: "Tim McNamara".to_string(),
        available: true,
    });
    
    library.add_book(Book {
        id: 3,
        title: "Programming Rust".to_string(),
        author: "Jim Blandy".to_string(),
        available: true,
    });
    
    // 展示不可变借用：查询操作 | Demonstrate immutable borrowing: query operations
    print_library_status(&library); // 不可变借用传递给函数 | Immutable borrow passed to function
    
    // 搜索特定书籍 | Search for specific book
    if let Some(book) = library.search_by_title("Rust实战 | Rust in Action") {
        println!("\n找到书籍 | Found book: {} by {}", book.title, book.author);
    }
    
    // 展示可变借用：修改操作 | Demonstrate mutable borrowing: modification operations
    println!("\n=== 借书操作 | Book Lending Operations ===");
    
    // 借出书籍 | Lend books
    match library.lend_book("Rust实战 | Rust in Action") {
        Ok(()) => println!("借书成功 | Lending successful"),
        Err(e) => println!("借书失败: {} | Lending failed: {}", e, e),
    }
    
    match library.lend_book("Programming Rust") {
        Ok(()) => println!("借书成功 | Lending successful"),
        Err(e) => println!("借书失败: {} | Lending failed: {}", e, e),
    }
    
    // 显示更新后的状态 | Show updated status
    print_library_status(&library);
    
    // 尝试借出已借出的书 | Try to lend already lent book
    match library.lend_book("Rust实战 | Rust in Action") {
        Ok(()) => println!("借书成功 | Lending successful"),
        Err(e) => println!("预期错误 | Expected error: {}", e),
    }
    
    // 批量归还书籍 | Batch return books
    println!("\n=== 批量归还 | Batch Return ===");
    let titles_to_return = &["Rust实战 | Rust in Action", "Programming Rust", "不存在的书 | Non-existent Book"];
    let results = library.batch_return(titles_to_return);
    
    for (title, result) in titles_to_return.iter().zip(results.iter()) {
        match result {
            Ok(()) => println!("成功归还: {} | Successfully returned: {}", title, title),
            Err(e) => println!("归还失败: {} - {} | Return failed: {} - {}", title, e, title, e),
        }
    }
    
    // 最终状态 | Final status
    print_library_status(&library);
    
    // 演示借用规则 | Demonstrate borrowing rules
    demonstrate_borrowing_rules();
    
    println!("\n=== 项目完成 | Project Complete ===");
    println!("成功演示了Rust借用机制的各个方面！| Successfully demonstrated all aspects of Rust borrowing mechanisms!");
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确使用了不可变借用进行查询操作？| Does the project correctly use immutable borrowing for query operations?
2. 可变借用的使用是否遵循了独占性规则？| Does the usage of mutable borrowing follow exclusivity rules?
3. 代码是否演示了借用检查器如何防止数据竞争？| Does the code demonstrate how the borrow checker prevents data races?
4. 是否展示了NLL带来的借用灵活性？| Does it show the borrowing flexibility brought by NLL?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **借用规则理解练习 | Borrowing Rules Understanding Exercise**
   - **练习描述 | Exercise Description:** 创建不同的借用场景，观察编译器行为
   - **概念检查 | Concept Check:** 能否解释为什么某些借用组合会导致编译错误？
   - **学习目标 | Learning Objective:** 深入理解借用规则的本质和原因

2. **引用生命周期练习 | Reference Lifetime Exercise**
   - **练习描述 | Exercise Description:** 实验不同作用域下引用的有效性
   - **概念检查 | Concept Check:** 理解NLL如何影响引用的生命周期？
   - **学习目标 | Learning Objective:** 掌握引用生命周期的实际应用

3. **借用与所有权转换练习 | Borrowing and Ownership Transfer Exercise**
   - **练习描述 | Exercise Description:** 设计需要在借用和移动之间选择的场景
   - **概念检查 | Concept Check:** 何时选择借用，何时选择所有权转移？
   - **学习目标 | Learning Objective:** 学会在不同场景下做出正确选择

4. **复杂数据结构借用练习 | Complex Data Structure Borrowing Exercise**
   - **练习描述 | Exercise Description:** 在嵌套数据结构中应用借用规则
   - **概念检查 | Concept Check:** 如何处理复杂结构中的借用冲突？
   - **学习目标 | Learning Objective:** 提高处理复杂借用场景的能力

5. **API设计练习 | API Design Exercise**
   - **练习描述 | Exercise Description:** 设计对借用友好的API接口
   - **概念检查 | Concept Check:** 如何设计API以最大化借用的灵活性？
   - **学习目标 | Learning Objective:** 学会设计符合Rust习惯的API

6. **错误诊断练习 | Error Diagnosis Exercise**
   - **练习描述 | Exercise Description:** 诊断和修复各种借用检查器错误
   - **概念检查 | Concept Check:** 能否快速识别和解决借用错误？
   - **学习目标 | Learning Objective:** 提高调试和问题解决能力

7. **性能对比练习 | Performance Comparison Exercise**
   - **练习描述 | Exercise Description:** 比较借用、克隆和移动的性能影响
   - **概念检查 | Concept Check:** 理解不同选择对性能的影响？
   - **学习目标 | Learning Objective:** 学会在性能和便利性之间做出平衡

## 学习资源 | Learning Resources
- [Rust官方文档 - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust Reference - Borrowing](https://doc.rust-lang.org/reference/expressions/operator-expr.html#borrow-operators)
- [The Rustonomicon - Ownership and Lifetimes](https://doc.rust-lang.org/nomicon/ownership.html)
- [Rust by Example - Borrowing](https://doc.rust-lang.org/rust-by-example/scope/borrow.html)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解借用的基本概念和语法 | Understand basic concepts and syntax of borrowing
- [ ] 掌握不可变借用的规则和应用 | Master rules and applications of immutable borrowing
- [ ] 理解可变借用的独占性要求 | Understand exclusivity requirements of mutable borrowing
- [ ] 掌握借用检查器的工作原理 | Master how the borrow checker works
- [ ] 能够诊断和解决常见借用错误 | Can diagnose and resolve common borrowing errors
- [ ] 理解NLL对借用灵活性的改进 | Understand NLL improvements to borrowing flexibility
- [ ] 完成图书馆管理系统项目 | Complete the library management system project
- [ ] 能够在实际项目中正确使用借用 | Can correctly use borrowing in real projects
- [ ] 理解借用与所有权的关系 | Understand relationship between borrowing and ownership
- [ ] 至少完成3个扩展练习 | Complete at least 3 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释每个核心概念。特别重要的是理解为什么Rust的借用规则能够在编译时防止内存安全问题，以及如何在实际编程中应用这些规则。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain each core concept to others. It's particularly important to understand why Rust's borrowing rules can prevent memory safety issues at compile time, and how to apply these rules in practical programming.