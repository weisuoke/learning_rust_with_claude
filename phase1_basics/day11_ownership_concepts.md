# Rust入门 - 第11天：所有权概念 | Rust Introduction - Day 11: Ownership Concepts

## 学习目标 | Learning Objectives
- 理解Rust所有权系统的基本原理和设计理念 | Understand the fundamental principles and design philosophy of Rust's ownership system
- 掌握所有权的三个核心规则及其在实际编程中的应用 | Master the three core ownership rules and their practical applications in programming
- 深入理解变量作用域与内存管理的关系 | Gain deep understanding of the relationship between variable scope and memory management
- 掌握移动语义(move semantics)的工作机制和使用场景 | Master the working mechanisms and usage scenarios of move semantics
- 学会避免常见的所有权相关编译错误 | Learn to avoid common ownership-related compilation errors
- 培养Rust式的内存安全编程思维 | Develop Rust-style memory-safe programming mindset

## 详细内容 | Detailed Content

### 1. 所有权系统概述 | Ownership System Overview (1小时 | 1 hour)

- **所有权系统设计理念 | Ownership System Design Philosophy**
  
  **概念定义 | Concept Definition:**
  所有权是Rust的核心特性，它通过编译时检查来确保内存安全，无需垃圾回收器。所有权系统基于"资源获取即初始化(RAII)"原则，每个值都有一个明确的所有者，当所有者离开作用域时，资源被自动释放。| Ownership is Rust's core feature that ensures memory safety through compile-time checks without needing a garbage collector. The ownership system is based on the "Resource Acquisition Is Initialization (RAII)" principle, where each value has a clear owner, and resources are automatically freed when the owner goes out of scope.
  
  **核心特征 | Key Characteristics:**
  - 编译时内存安全保证，零运行时开销 | Compile-time memory safety guarantees with zero runtime overhead
  - 自动内存管理，无需手动malloc/free | Automatic memory management without manual malloc/free
  - 防止数据竞争、悬垂指针和内存泄漏 | Prevents data races, dangling pointers, and memory leaks
  - 每个值都有唯一的所有者 | Each value has a unique owner
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Rust的所有权系统是在编译时还是运行时检查内存安全？| Does Rust's ownership system check memory safety at compile time or runtime?
     **答案 | Answer:** 编译时 | Compile time - 所有权规则在编译时强制执行，确保零运行时开销 | Ownership rules are enforced at compile time, ensuring zero runtime overhead
  2. 在Rust中，一个值可以同时有多个所有者吗？| In Rust, can a value have multiple owners simultaneously?
     **答案 | Answer:** 不可以 | No - 每个值在任何时候都只能有一个所有者 | Each value can only have one owner at any given time
  3. 当变量离开作用域时会自动发生什么？| What automatically happens when a variable goes out of scope?
     **答案 | Answer:** 资源被释放 | Resources are freed - Rust自动调用drop函数清理资源 | Rust automatically calls the drop function to clean up resources
  4. 所有权系统能防止哪些内存安全问题？| What memory safety issues can the ownership system prevent?
     **答案 | Answer:** 悬垂指针、双重释放、内存泄漏、数据竞争 | Dangling pointers, double free, memory leaks, data races
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 所有权基本概念演示 | Basic ownership concept demonstration
  fn main() {
      // s1拥有字符串的所有权 | s1 owns the string
      let s1 = String::from("hello");
      
      // 当s1离开作用域时，内存被自动释放 | Memory is automatically freed when s1 goes out of scope
      println!("s1: {}", s1);
  } // s1在这里离开作用域并被释放 | s1 goes out of scope and is freed here
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码中谁拥有字符串"hello"的所有权？| Who owns the string "hello" in this code?
    **答案 | Answer:** 变量s1拥有所有权 | Variable s1 owns it
  - s1什么时候被释放？| When is s1 freed?
    **答案 | Answer:** 在main函数结束时，s1离开作用域被自动释放 | At the end of main function when s1 goes out of scope
  
  **常见误区检查 | Common Misconception Checks:**
  - 所有权系统会影响程序运行性能吗？| Does the ownership system affect program runtime performance?
    **答案 | Answer:** 不会，所有检查都在编译时完成 | No, all checks are done at compile time
  - 所有权只适用于堆上的数据吗？| Does ownership only apply to heap data?
    **答案 | Answer:** 不是，栈上数据也有所有权概念，但处理方式不同 | No, stack data also has ownership concepts, but is handled differently

### 2. 所有权三大规则 | Three Rules of Ownership (1小时 | 1 hour)

- **所有权核心规则 | Core Ownership Rules**
  
  **概念定义 | Concept Definition:**
  Rust的所有权系统基于三个基本规则：(1)每个值都有一个所有者；(2)同一时间只能有一个所有者；(3)当所有者离开作用域时，值被丢弃。这三个规则共同确保了内存安全和资源管理的正确性。| Rust's ownership system is based on three fundamental rules: (1) Each value has an owner; (2) There can only be one owner at a time; (3) When the owner goes out of scope, the value is dropped. These three rules together ensure memory safety and correct resource management.
  
  **规则详解 | Rule Details:**
  - 规则1：每个值都有一个变量作为其所有者 | Rule 1: Each value has a variable as its owner
  - 规则2：同一时间只能有一个所有者，防止数据竞争 | Rule 2: Only one owner at a time, preventing data races
  - 规则3：所有者离开作用域时，值被自动释放(drop) | Rule 3: Value is automatically dropped when owner goes out of scope
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 根据所有权规则，一个String值可以同时被两个变量拥有吗？| According to ownership rules, can a String value be owned by two variables simultaneously?
     **答案 | Answer:** 不可以 | No - 违反规则2，同一时间只能有一个所有者 | Violates rule 2, only one owner at a time
  2. 如果我们有 `let x = 5; let y = x;`，这里发生了所有权转移吗？| If we have `let x = 5; let y = x;`, does ownership transfer occur here?
     **答案 | Answer:** 不是 | No - 整数实现了Copy trait，这是复制而不是移动 | Integers implement Copy trait, this is copying not moving
  3. 什么时候会触发drop函数的调用？| When is the drop function called?
     **答案 | Answer:** 当所有者离开作用域时 | When the owner goes out of scope - 根据规则3自动执行 | Automatically executed according to rule 3
  4. 所有权转移后，原来的变量还能使用吗？| Can the original variable still be used after ownership transfer?
     **答案 | Answer:** 不能 | No - 原变量已失效，使用会导致编译错误 | Original variable is invalidated, using it causes compile error
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 所有权三大规则演示 | Demonstration of three ownership rules
  fn main() {
      // 规则1：s1是字符串的所有者 | Rule 1: s1 is the owner of the string
      let s1 = String::from("hello");
      
      // 规则2：所有权转移给s2，s1不再有效 | Rule 2: ownership transfers to s2, s1 is no longer valid
      let s2 = s1;
      
      // 这行代码会编译错误！| This line would cause compile error!
      // println!("s1: {}", s1); // error: borrow of moved value
      
      println!("s2: {}", s2); // 正常使用 | Normal usage
      
      // 规则3：s2在这里离开作用域并被drop | Rule 3: s2 goes out of scope and is dropped here
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么注释的println!会导致编译错误？| Why would the commented println! cause a compile error?
    **答案 | Answer:** 因为s1的所有权已经转移给了s2，s1不再有效 | Because s1's ownership has been transferred to s2, s1 is no longer valid
  - 在什么情况下可以同时使用s1和s2？| Under what circumstances can both s1 and s2 be used?
    **答案 | Answer:** 如果使用引用(&)或克隆(clone) | If using references (&) or cloning (clone)

### 3. 变量作用域与内存管理 | Variable Scope and Memory Management (1小时 | 1 hour)

- **作用域生命周期管理 | Scope Lifetime Management**
  
  **概念定义 | Concept Definition:**
  作用域(scope)是程序中变量有效的区域范围。在Rust中，变量的生命周期与其作用域紧密相关，当变量离开作用域时，其拥有的资源会被自动清理。这种确定性的资源管理是RAII原则的体现。| Scope is the region in a program where a variable is valid. In Rust, a variable's lifetime is closely tied to its scope, and when a variable goes out of scope, the resources it owns are automatically cleaned up. This deterministic resource management is an embodiment of the RAII principle.
  
  **作用域特征 | Scope Characteristics:**
  - 用花括号{}定义作用域边界 | Curly braces {} define scope boundaries
  - 变量在声明点开始有效 | Variables become valid from their declaration point
  - 变量在离开作用域时自动失效 | Variables automatically become invalid when leaving scope
  - 嵌套作用域遵循栈式管理 | Nested scopes follow stack-like management
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 在Rust中，变量的生命周期是由什么决定的？| In Rust, what determines a variable's lifetime?
     **答案 | Answer:** 作用域 | Scope - 变量从声明到离开作用域的整个期间 | From declaration until leaving the scope
  2. 当变量离开作用域时，Rust会自动做什么？| What does Rust automatically do when a variable goes out of scope?
     **答案 | Answer:** 调用drop函数释放资源 | Calls drop function to free resources
  3. 内层作用域的变量可以访问外层作用域的变量吗？| Can variables in inner scope access variables from outer scope?
     **答案 | Answer:** 可以，但需要符合借用规则 | Yes, but must follow borrowing rules
  4. 两个同级作用域中的同名变量会冲突吗？| Do variables with the same name in sibling scopes conflict?
     **答案 | Answer:** 不会 | No - 它们在不同的作用域中独立存在 | They exist independently in different scopes
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 作用域与内存管理演示 | Scope and memory management demonstration
  fn main() {
      let outer = String::from("outer");
      
      { // 内层作用域开始 | Inner scope begins
          let inner = String::from("inner");
          println!("Inner scope: {}, {}", outer, inner);
          
          // inner的作用域即将结束 | inner's scope is about to end
      } // inner在这里被drop | inner is dropped here
      
      println!("Outer scope: {}", outer);
      // println!("{}", inner); // 编译错误：inner已不在作用域中 | Compile error: inner not in scope
      
  } // outer在这里被drop | outer is dropped here
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - inner变量在什么时候被释放？| When is the inner variable freed?
    **答案 | Answer:** 在内层作用域结束时，即第一个}处 | At the end of inner scope, at the first closing brace
  - 为什么在外层作用域中无法使用inner变量？| Why can't the inner variable be used in the outer scope?
    **答案 | Answer:** 因为inner已经离开了其作用域并被释放 | Because inner has left its scope and been freed

### 4. 移动语义深入理解 | Deep Understanding of Move Semantics (1.5小时 | 1.5 hours)

- **移动语义核心概念 | Core Move Semantics Concepts**
  
  **概念定义 | Concept Definition:**
  移动语义是Rust所有权系统的核心机制，当一个值的所有权从一个变量转移到另一个变量时，原变量失效，新变量获得完全的所有权。这避免了深拷贝的开销，同时确保了内存安全。| Move semantics is the core mechanism of Rust's ownership system. When ownership of a value transfers from one variable to another, the original variable becomes invalid and the new variable gains complete ownership. This avoids the overhead of deep copying while ensuring memory safety.
  
  **移动发生的场景 | Scenarios Where Moves Occur:**
  - 赋值操作：`let y = x` | Assignment operations: `let y = x`
  - 函数参数传递：`function(x)` | Function parameter passing: `function(x)`
  - 函数返回值：`return x` | Function return values: `return x`
  - 集合插入：`vec.push(x)` | Collection insertion: `vec.push(x)`
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 移动操作是复制数据还是转移所有权？| Does a move operation copy data or transfer ownership?
     **答案 | Answer:** 转移所有权 | Transfer ownership - 不进行数据复制，只是所有权的转移 | No data copying, just ownership transfer
  2. 移动后原变量还能继续使用吗？| Can the original variable still be used after a move?
     **答案 | Answer:** 不能 | No - 原变量已失效，编译器会阻止使用 | Original variable is invalidated, compiler prevents usage
  3. 什么类型的数据会发生移动而不是复制？| What types of data undergo moves rather than copies?
     **答案 | Answer:** 没有实现Copy trait的类型 | Types that don't implement the Copy trait - 如String、Vec等 | Like String, Vec, etc.
  4. 如何避免移动语义带来的所有权转移？| How to avoid ownership transfer caused by move semantics?
     **答案 | Answer:** 使用引用(&)或克隆(clone) | Use references (&) or cloning (clone)
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 移动语义详细演示 | Detailed move semantics demonstration
  fn main() {
      // 字符串移动示例 | String move example
      let s1 = String::from("hello");
      let s2 = s1; // s1移动到s2 | s1 moves to s2
      
      // println!("{}", s1); // 错误：s1已被移动 | Error: s1 has been moved
      println!("{}", s2); // 正常：s2拥有所有权 | OK: s2 owns the value
      
      // 函数调用中的移动 | Move in function calls
      let s3 = String::from("world");
      take_ownership(s3); // s3移动到函数中 | s3 moves into function
      // println!("{}", s3); // 错误：s3已被移动 | Error: s3 has been moved
      
      // Copy类型不会移动 | Copy types don't move
      let x = 5;
      let y = x; // 复制而不是移动 | Copy, not move
      println!("x: {}, y: {}", x, y); // 两个都可以使用 | Both can be used
  }
  
  fn take_ownership(some_string: String) {
      println!("{}", some_string);
  } // some_string离开作用域并被drop | some_string goes out of scope and is dropped
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么x和y都可以使用，而s1被移动后不能使用？| Why can both x and y be used, but s1 cannot be used after being moved?
    **答案 | Answer:** 因为整数类型实现了Copy trait，是复制而不是移动 | Because integer types implement Copy trait, it's copying not moving
  - take_ownership函数执行后，传入的字符串发生了什么？| What happens to the string passed to take_ownership function after execution?
    **答案 | Answer:** 在函数结束时被drop释放 | It's dropped and freed at the end of the function

### 5. Copy trait与Clone trait对比 | Copy trait vs Clone trait Comparison (45分钟 | 45 minutes)

- **Copy和Clone的区别 | Difference Between Copy and Clone**
  
  **概念定义 | Concept Definition:**
  Copy trait标记可以通过简单位拷贝来复制的类型，这种复制是隐式的、廉价的。Clone trait定义了显式复制的方法，通常涉及更复杂的资源分配。实现Copy的类型在赋值时不会发生移动，而是进行复制。| Copy trait marks types that can be duplicated through simple bit copying, this duplication is implicit and cheap. Clone trait defines explicit duplication methods, usually involving more complex resource allocation. Types that implement Copy don't move during assignment, but copy instead.
  
  **trait特征对比 | Trait Characteristics Comparison:**
  - Copy：隐式复制，零成本抽象，只能用于简单类型 | Copy: Implicit copying, zero-cost abstraction, only for simple types
  - Clone：显式复制，可能有性能开销，适用于复杂类型 | Clone: Explicit copying, may have performance overhead, suitable for complex types
  - Copy类型必须同时实现Clone | Copy types must also implement Clone
  - 并非所有Clone类型都能实现Copy | Not all Clone types can implement Copy
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 实现了Copy trait的类型在赋值时会发生移动吗？| Do types that implement Copy trait move during assignment?
     **答案 | Answer:** 不会 | No - 它们会被复制而不是移动 | They are copied, not moved
  2. String类型实现了Copy trait吗？为什么？| Does String type implement Copy trait? Why?
     **答案 | Answer:** 没有 | No - 因为String包含堆上的数据，复制成本高 | Because String contains heap data, copying is expensive
  3. 如何显式复制一个没有实现Copy的类型？| How to explicitly copy a type that doesn't implement Copy?
     **答案 | Answer:** 调用clone()方法 | Call the clone() method
  4. 哪些基本类型实现了Copy trait？| Which basic types implement Copy trait?
     **答案 | Answer:** 整数、浮点数、布尔值、字符类型等 | Integers, floats, booleans, character types, etc.
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // Copy和Clone对比演示 | Copy and Clone comparison demonstration
  fn main() {
      // Copy类型示例 - 整数 | Copy type example - integers
      let x = 5;
      let y = x; // 隐式复制 | Implicit copy
      println!("x: {}, y: {}", x, y); // 两个都有效 | Both are valid
      
      // 非Copy类型 - String | Non-Copy type - String
      let s1 = String::from("hello");
      // let s2 = s1; // 这会移动s1 | This would move s1
      let s2 = s1.clone(); // 显式克隆 | Explicit clone
      println!("s1: {}, s2: {}", s1, s2); // 两个都有效 | Both are valid
      
      // 自定义结构体 | Custom struct
      #[derive(Copy, Clone)]
      struct Point {
          x: i32,
          y: i32,
      }
      
      let p1 = Point { x: 1, y: 2 };
      let p2 = p1; // Copy，p1仍然有效 | Copy, p1 still valid
      println!("p1: ({}, {}), p2: ({}, {})", p1.x, p1.y, p2.x, p2.y);
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么Point结构体可以实现Copy trait？| Why can the Point struct implement Copy trait?
    **答案 | Answer:** 因为它的所有字段都实现了Copy trait | Because all its fields implement Copy trait
  - 如果Point包含一个String字段，还能实现Copy吗？| If Point contained a String field, could it still implement Copy?
    **答案 | Answer:** 不能，因为String没有实现Copy | No, because String doesn't implement Copy

### 6. 所有权最佳实践与常见陷阱 | Ownership Best Practices and Common Pitfalls (30分钟 | 30 minutes)

- **所有权编程最佳实践 | Ownership Programming Best Practices**
  
  **实践原则 | Practice Principles:**
  - 优先使用引用而非移动来避免不必要的所有权转移 | Prefer references over moves to avoid unnecessary ownership transfers
  - 在函数设计时考虑所有权的获取和返还 | Consider ownership acquisition and return in function design
  - 使用Clone只在必要时，注意性能影响 | Use Clone only when necessary, mind performance impact
  - 理解move closure的使用场景 | Understand usage scenarios for move closures
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 什么时候应该让函数获取参数的所有权？| When should a function take ownership of its parameters?
     **答案 | Answer:** 当函数需要消费或转移值时 | When the function needs to consume or transfer the value
  2. 如何设计既不获取所有权也不修改数据的函数？| How to design functions that neither take ownership nor modify data?
     **答案 | Answer:** 使用不可变引用(&T)作为参数 | Use immutable references (&T) as parameters
  3. 什么情况下clone()是合理的选择？| When is clone() a reasonable choice?
     **答案 | Answer:** 当需要独立的副本且性能可接受时 | When independent copies are needed and performance is acceptable
  4. 如何避免不必要的所有权转移？| How to avoid unnecessary ownership transfers?
     **答案 | Answer:** 使用借用(引用)而不是直接传递值 | Use borrowing (references) instead of passing values directly

## 实践项目：内存使用跟踪器 | Practical Project: Memory Usage Tracker

### 目标 | Objective
创建一个简单的内存使用跟踪器，演示所有权概念在实际项目中的应用，包括资源的获取、转移和释放。| Create a simple memory usage tracker that demonstrates ownership concepts in practical applications, including resource acquisition, transfer, and release.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 当变量获得一个资源的所有权时，该变量负责什么？| When a variable gains ownership of a resource, what is the variable responsible for?
   **答案 | Answer:** 负责在适当时机释放该资源 | Responsible for releasing the resource at the appropriate time
2. 移动语义如何避免数据的不必要复制？| How does move semantics avoid unnecessary data copying?
   **答案 | Answer:** 通过转移所有权而不是复制数据本身 | By transferring ownership rather than copying the data itself
3. 为什么Rust不允许在移动后使用原变量？| Why doesn't Rust allow using the original variable after a move?
   **答案 | Answer:** 为了防止访问已释放的内存，确保内存安全 | To prevent accessing freed memory and ensure memory safety

### 步骤 | Steps
1. 设计资源结构体，体现所有权概念 | Design resource struct to embody ownership concepts
2. 实现资源获取和释放机制 | Implement resource acquisition and release mechanisms
3. 创建跟踪器管理多个资源的所有权 | Create tracker to manage ownership of multiple resources
4. 演示所有权转移和生命周期管理 | Demonstrate ownership transfer and lifetime management
5. 添加内存使用统计和报告功能 | Add memory usage statistics and reporting features

### 示例代码 | Example Code
```rust
"""
内存使用跟踪器 | Memory Usage Tracker
演示Rust所有权系统在资源管理中的应用 | Demonstrates Rust ownership system in resource management

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 所有权转移与管理 | Ownership transfer and management  
- 资源自动释放 | Automatic resource cleanup
- 作用域生命周期控制 | Scope lifetime control
"""

use std::collections::HashMap;

// 模拟内存资源 | Simulate memory resource
struct MemoryBlock {
    id: u32,
    size: usize,
    data: String,
}

impl MemoryBlock {
    // 创建新的内存块 | Create new memory block
    fn new(id: u32, size: usize, content: &str) -> Self {
        println!("📦 分配内存块 {} ({}字节) | Allocating memory block {} ({} bytes)", id, size);
        MemoryBlock {
            id,
            size,
            data: content.to_string(),
        }
    }
    
    // 获取内存块信息 | Get memory block info
    fn info(&self) -> String {
        format!("Block[{}]: {} bytes", self.id, self.size)
    }
}

// 实现Drop trait来观察资源释放 | Implement Drop trait to observe resource cleanup
impl Drop for MemoryBlock {
    fn drop(&mut self) {
        println!("🗑️  释放内存块 {} | Freeing memory block {}", self.id);
    }
}

// 内存跟踪器 | Memory tracker
struct MemoryTracker {
    blocks: HashMap<u32, MemoryBlock>,
    total_allocated: usize,
}

impl MemoryTracker {
    // 创建新跟踪器 | Create new tracker
    fn new() -> Self {
        println!("🎯 创建内存跟踪器 | Creating memory tracker");
        MemoryTracker {
            blocks: HashMap::new(),
            total_allocated: 0,
        }
    }
    
    // 分配内存块（获取所有权）| Allocate memory block (take ownership)
    fn allocate(&mut self, id: u32, size: usize, content: &str) -> Result<(), String> {
        if self.blocks.contains_key(&id) {
            return Err(format!("内存块 {} 已存在 | Memory block {} already exists", id));
        }
        
        // 创建内存块并转移所有权给跟踪器 | Create memory block and transfer ownership to tracker
        let block = MemoryBlock::new(id, size, content);
        self.total_allocated += size;
        self.blocks.insert(id, block); // block的所有权移动到HashMap中 | block ownership moves to HashMap
        
        Ok(())
    }
    
    // 释放内存块（转移所有权）| Deallocate memory block (transfer ownership)
    fn deallocate(&mut self, id: u32) -> Result<MemoryBlock, String> {
        match self.blocks.remove(&id) {
            Some(block) => {
                self.total_allocated -= block.size;
                println!("📤 内存块 {} 从跟踪器中移除 | Memory block {} removed from tracker", id);
                Ok(block) // 将所有权转移给调用者 | Transfer ownership to caller
            }
            None => Err(format!("内存块 {} 不存在 | Memory block {} doesn't exist", id)),
        }
    }
    
    // 查看内存块信息（借用，不转移所有权）| View memory block info (borrow, no ownership transfer)
    fn get_block_info(&self, id: u32) -> Option<String> {
        self.blocks.get(&id).map(|block| block.info())
    }
    
    // 生成内存使用报告 | Generate memory usage report
    fn report(&self) {
        println!("\n📊 内存使用报告 | Memory Usage Report");
        println!("总分配内存: {} bytes | Total allocated: {} bytes", self.total_allocated);
        println!("活跃内存块数: {} | Active blocks: {}", self.blocks.len());
        
        for (id, block) in &self.blocks {
            println!("  - {}", block.info());
        }
    }
}

impl Drop for MemoryTracker {
    fn drop(&mut self) {
        println!("🔚 内存跟踪器销毁，自动清理所有资源 | Memory tracker destroyed, auto-cleaning all resources");
    }
}

// 演示所有权转移的函数 | Function demonstrating ownership transfer  
fn transfer_ownership_demo() {
    println!("\n🔄 所有权转移演示 | Ownership Transfer Demo");
    
    // 创建内存块 | Create memory block
    let block = MemoryBlock::new(999, 1024, "temporary data");
    println!("创建了临时内存块: {}", block.info());
    
    // 模拟将所有权转移给其他系统 | Simulate transferring ownership to other system
    let moved_block = block; // 所有权转移 | Ownership transfer
    // println!("{}", block.info()); // 这行会编译错误！| This would cause compile error!
    
    println!("所有权已转移: {}", moved_block.info());
    
    // moved_block在函数结束时被自动释放 | moved_block automatically freed at function end
}

fn main() {
    println!("🚀 内存使用跟踪器演示 | Memory Usage Tracker Demo\n");
    
    // 创建跟踪器 | Create tracker
    let mut tracker = MemoryTracker::new();
    
    // 分配一些内存块 | Allocate some memory blocks
    tracker.allocate(1, 512, "用户数据 | User data").unwrap();
    tracker.allocate(2, 256, "缓存数据 | Cache data").unwrap();
    tracker.allocate(3, 1024, "临时数据 | Temp data").unwrap();
    
    // 查看当前状态 | View current state
    tracker.report();
    
    // 查看特定内存块信息（借用）| View specific block info (borrowing)
    if let Some(info) = tracker.get_block_info(2) {
        println!("\n🔍 查询内存块信息: {} | Querying block info: {}", info);
    }
    
    // 释放一个内存块 | Deallocate a memory block
    {
        println!("\n📤 释放内存块演示 | Deallocate block demo");
        let released_block = tracker.deallocate(2).unwrap();
        println!("获得释放的内存块: {}", released_block.info());
        // released_block在这个作用域结束时被drop | released_block dropped at end of this scope
    }
    
    // 查看释放后的状态 | View state after deallocation
    tracker.report();
    
    // 所有权转移演示 | Ownership transfer demo
    transfer_ownership_demo();
    
    println!("\n✅ 程序结束，观察资源自动清理 | Program ending, observe automatic resource cleanup");
    // tracker在main结束时被drop，自动清理所有剩余资源 | tracker dropped at main end, auto-cleaning remaining resources
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确演示了所有权的获取和转移？| Does the project correctly demonstrate ownership acquisition and transfer?
2. 资源是否在适当的时机被自动释放？| Are resources automatically freed at appropriate times?
3. 代码是否体现了所有权规则的三个要点？| Does the code reflect the three key points of ownership rules?
4. Drop trait的实现是否帮助理解资源清理过程？| Does the Drop trait implementation help understand the resource cleanup process?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **所有权规则强化练习 | Ownership Rules Reinforcement Exercise**
   - **练习描述 | Exercise Description:** 编写代码示例违反每一条所有权规则，观察编译器错误信息
   - **概念检查 | Concept Check:** 你能准确说出所有权的三条规则吗？| Can you accurately state the three ownership rules?
   - **学习目标 | Learning Objective:** 通过错误加深对规则的理解

2. **作用域分析练习 | Scope Analysis Exercise**
   - **练习描述 | Exercise Description:** 分析复杂嵌套作用域中变量的生命周期
   - **概念检查 | Concept Check:** 变量的生命周期如何与作用域关联？| How is a variable's lifetime associated with its scope?
   - **学习目标 | Learning Objective:** 掌握复杂场景下的内存管理

3. **移动语义综合练习 | Move Semantics Integration Exercise**
   - **练习描述 | Exercise Description:** 设计一个文件管理系统，体现移动语义的各种应用场景
   - **概念检查 | Concept Check:** 什么情况下会发生移动？| When do moves occur?
   - **学习目标 | Learning Objective:** 在实际项目中熟练运用移动语义

4. **Copy vs Clone辨析练习 | Copy vs Clone Analysis Exercise**
   - **练习描述 | Exercise Description:** 为不同类型的数据结构选择合适的复制策略
   - **概念检查 | Concept Check:** Copy和Clone的根本区别是什么？| What's the fundamental difference between Copy and Clone?
   - **学习目标 | Learning Objective:** 优化程序性能和内存使用

5. **所有权传递模式练习 | Ownership Transfer Patterns Exercise**
   - **练习描述 | Exercise Description:** 实现Builder模式，展示所有权的优雅传递
   - **概念检查 | Concept Check:** 如何设计API来清晰表达所有权意图？| How to design APIs that clearly express ownership intentions?
   - **学习目标 | Learning Objective:** 学会设计符合Rust习惯的API

6. **资源管理教学练习 | Resource Management Teaching Exercise**
   - **练习描述 | Exercise Description:** 向其他人解释RAII原则和Rust的实现方式
   - **概念检查 | Concept Check:** RAII如何解决内存管理问题？| How does RAII solve memory management problems?
   - **学习目标 | Learning Objective:** 通过教学深化理解

7. **所有权调试练习 | Ownership Debugging Exercise**
   - **练习描述 | Exercise Description:** 修复一系列所有权相关的编译错误
   - **概念检查 | Concept Check:** 常见的所有权错误有哪些？| What are common ownership errors?
   - **学习目标 | Learning Objective:** 提高调试和解决所有权问题的能力

## 学习资源 | Learning Resources
- [Rust官方文档 - 所有权](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) | [Rust Official Documentation - Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [所有权可视化工具](https://github.com/rustviz/rustviz) | [Ownership Visualization Tool](https://github.com/rustviz/rustviz)
- [Rust内存管理深入解析](https://doc.rust-lang.org/nomicon/) | [Deep Dive into Rust Memory Management](https://doc.rust-lang.org/nomicon/)
- [RAII原则详解](https://en.cppreference.com/w/cpp/language/raii) | [RAII Principle Explained](https://en.cppreference.com/w/cpp/language/raii)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解所有权系统的设计理念和三大规则 | Understand ownership system design philosophy and three rules
- [ ] 掌握变量作用域与内存管理的关系 | Master relationship between variable scope and memory management  
- [ ] 熟练运用移动语义避免不必要的复制 | Proficiently use move semantics to avoid unnecessary copying
- [ ] 区分Copy和Clone trait的使用场景 | Distinguish usage scenarios for Copy and Clone traits
- [ ] 完成内存跟踪器项目并理解其工作原理 | Complete memory tracker project and understand its working principles
- [ ] 能够识别和修复常见的所有权错误 | Able to identify and fix common ownership errors
- [ ] 掌握RAII原则在Rust中的具体实现 | Master specific implementation of RAII principle in Rust
- [ ] 理解所有权系统如何确保内存安全 | Understand how ownership system ensures memory safety
- [ ] 学会设计遵循所有权原则的API | Learn to design APIs following ownership principles
- [ ] 完成至少5个扩展练习加深理解 | Complete at least 5 extension exercises to deepen understanding

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释所有权的核心概念和工作机制。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain the core concepts and working mechanisms of ownership to others.