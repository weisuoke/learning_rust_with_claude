# Rust入门 - 第8天：方法与关联函数 | Rust Introduction - Day 8: Methods and Associated Functions

## 学习目标 | Learning Objectives
- 理解方法和关联函数的区别与用途 | Understand the difference and purposes of methods and associated functions
- 掌握impl块的使用方法 | Master the usage of impl blocks
- 学会为结构体定义方法和关联函数 | Learn to define methods and associated functions for structs
- 理解self、&self和&mut self的区别 | Understand the differences between self, &self, and &mut self
- 能够为自定义类型添加行为和功能 | Be able to add behaviors and functionality to custom types
- 掌握方法调用的语法糖机制 | Master the syntactic sugar mechanism of method calls

## 详细内容 | Detailed Content

### 1. 方法基础概念 | Method Fundamentals (1小时 | 1 hour)

- **方法定义与特征 | Method Definition and Characteristics**
  
  **概念定义 | Concept Definition:**
  方法是与特定类型关联的函数，它们在impl块中定义，第一个参数总是self的某种形式。| Methods are functions associated with a specific type, defined in impl blocks, with the first parameter always being some form of self.
  
  **核心特征 | Key Characteristics:**
  - 方法必须在impl块内定义，与特定类型绑定 | Methods must be defined inside impl blocks, bound to a specific type
  - 第一个参数总是self、&self或&mut self | The first parameter is always self, &self, or &mut self
  - 可以通过点记号语法调用，具有语法糖特性 | Can be called with dot notation syntax, featuring syntactic sugar
  - 可以访问和修改实例的数据 | Can access and modify instance data
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 方法必须在impl块中定义吗？| Must methods be defined in impl blocks?
     **答案 | Answer:** 是 | Yes - 方法只能在impl块中定义，这样才能与类型关联 | Methods can only be defined in impl blocks to be associated with types
  2. 方法的第一个参数可以是任意名称吗？| Can the first parameter of a method have any name?
     **答案 | Answer:** 否 | No - 第一个参数必须是self的某种形式（self、&self或&mut self）| The first parameter must be some form of self (self, &self, or &mut self)
  3. 方法调用obj.method()和Type::method(&obj)是等价的吗？| Are obj.method() and Type::method(&obj) equivalent?
     **答案 | Answer:** 是 | Yes - 点记号是语法糖，编译器会自动转换 | Dot notation is syntactic sugar, the compiler automatically converts it
  4. 方法可以修改实例的数据吗？| Can methods modify instance data?
     **答案 | Answer:** 取决于self类型 | Depends on self type - &mut self可以修改，&self不能修改 | &mut self can modify, &self cannot modify
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 基础结构体定义 | Basic struct definition
  struct Rectangle {
      width: u32,
      height: u32,
  }
  
  // impl块中定义方法 | Define methods in impl block
  impl Rectangle {
      // 不可变借用self的方法 | Method with immutable borrow of self
      fn area(&self) -> u32 {
          self.width * self.height
      }
      
      // 可变借用self的方法 | Method with mutable borrow of self
      fn double_size(&mut self) {
          self.width *= 2;
          self.height *= 2;
      }
  }
  
  // 使用方法 | Using methods
  let mut rect = Rectangle { width: 30, height: 50 };
  println!("面积: {}", rect.area()); // 调用不可变方法 | Call immutable method
  rect.double_size(); // 调用可变方法 | Call mutable method
  println!("新面积: {}", rect.area());
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码会输出什么？| What will this code output?
    **答案 | Answer:** 面积: 1500, 新面积: 6000 | Area: 1500, New area: 6000
  - 如果去掉rect前的mut关键字会发生什么？| What happens if we remove the mut keyword before rect?
    **答案 | Answer:** 编译错误，因为double_size需要可变引用 | Compilation error because double_size requires a mutable reference
  
  **常见误区检查 | Common Misconception Checks:**
  - 方法只能访问公共字段吗？| Can methods only access public fields?
    **答案 | Answer:** 否，方法可以访问所有字段，包括私有字段 | No, methods can access all fields including private ones
  - 方法必须返回值吗？| Must methods return values?
    **答案 | Answer:** 否，方法可以不返回值或返回() | No, methods can return nothing or return ()

- **self参数的三种形式 | Three Forms of self Parameter**
  
  **概念定义 | Concept Definition:**
  self参数有三种形式：self（获取所有权）、&self（不可变借用）、&mut self（可变借用），决定了方法如何访问实例数据。| The self parameter has three forms: self (takes ownership), &self (immutable borrow), &mut self (mutable borrow), determining how methods access instance data.
  
  **核心特征 | Key Characteristics:**
  - self：转移所有权，方法调用后实例不可再用 | self: transfers ownership, instance unusable after method call
  - &self：不可变借用，只能读取数据，不消耗实例 | &self: immutable borrow, can only read data, doesn't consume instance
  - &mut self：可变借用，可以修改数据，不消耗实例 | &mut self: mutable borrow, can modify data, doesn't consume instance
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 使用self参数的方法调用后，原实例还能使用吗？| After calling a method with self parameter, can the original instance still be used?
     **答案 | Answer:** 否 | No - self转移了所有权，原实例被消耗 | self transfers ownership, the original instance is consumed
  2. &self和&mut self的主要区别是什么？| What's the main difference between &self and &mut self?
     **答案 | Answer:** 可变性 | Mutability - &self只读，&mut self可写 | &self is read-only, &mut self is writable
  3. 一个结构体可以同时有使用不同self形式的方法吗？| Can a struct have methods using different forms of self simultaneously?
     **答案 | Answer:** 是 | Yes - 可以根据需要定义不同形式的方法 | Can define methods with different forms as needed
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  struct Book {
      title: String,
      pages: u32,
  }
  
  impl Book {
      // 使用&self - 只读访问 | Using &self - read-only access
      fn get_info(&self) -> String {
          format!("《{}》共{}页", self.title, self.pages)
      }
      
      // 使用&mut self - 可修改 | Using &mut self - mutable
      fn add_pages(&mut self, additional: u32) {
          self.pages += additional;
      }
      
      // 使用self - 转移所有权 | Using self - transfer ownership
      fn into_summary(self) -> String {
          format!("《{}》已读完，共{}页", self.title, self.pages)
      }
  }
  
  let mut book = Book {
      title: "Rust编程指南".to_string(),
      pages: 300,
  };
  
  println!("{}", book.get_info()); // book仍然可用 | book is still usable
  book.add_pages(50); // book仍然可用 | book is still usable
  println!("{}", book.into_summary()); // book被消耗 | book is consumed
  // println!("{}", book.get_info()); // 编译错误！| Compilation error!
  ```

### 2. 关联函数详解 | Associated Functions Explained (1小时 | 1 hour)

- **关联函数概念与特征 | Associated Function Concept and Characteristics**
  
  **概念定义 | Concept Definition:**
  关联函数是与类型相关联但不需要实例的函数，定义在impl块中，没有self参数，通常用作构造函数。| Associated functions are functions associated with a type but don't require an instance, defined in impl blocks without self parameter, often used as constructors.
  
  **核心特征 | Key Characteristics:**
  - 没有self参数，不需要实例即可调用 | No self parameter, can be called without an instance
  - 使用::语法调用，如Type::function() | Called using :: syntax, like Type::function()
  - 常用于创建实例（构造函数模式）| Often used for creating instances (constructor pattern)
  - 可以有多个关联函数提供不同的创建方式 | Can have multiple associated functions providing different creation methods
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 关联函数需要实例才能调用吗？| Do associated functions need an instance to be called?
     **答案 | Answer:** 否 | No - 关联函数通过类型名直接调用 | Associated functions are called directly through type name
  2. 关联函数和方法的主要区别是什么？| What's the main difference between associated functions and methods?
     **答案 | Answer:** self参数 | self parameter - 关联函数没有self参数 | Associated functions don't have self parameter
  3. 关联函数可以访问实例的字段吗？| Can associated functions access instance fields?
     **答案 | Answer:** 否 | No - 因为没有实例，无法访问实例字段 | No instance exists, so cannot access instance fields
  4. new是Rust的关键字吗？| Is new a Rust keyword?
     **答案 | Answer:** 否 | No - new只是惯例命名，不是关键字 | new is just a naming convention, not a keyword
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  struct Circle {
      radius: f64,
  }
  
  impl Circle {
      // 关联函数 - 标准构造函数 | Associated function - standard constructor
      fn new(radius: f64) -> Circle {
          Circle { radius }
      }
      
      // 关联函数 - 从直径创建 | Associated function - create from diameter
      fn from_diameter(diameter: f64) -> Circle {
          Circle {
              radius: diameter / 2.0,
          }
      }
      
      // 关联函数 - 单位圆 | Associated function - unit circle
      fn unit_circle() -> Circle {
          Circle { radius: 1.0 }
      }
      
      // 方法 - 计算面积 | Method - calculate area
      fn area(&self) -> f64 {
          std::f64::consts::PI * self.radius * self.radius
      }
  }
  
  // 使用关联函数创建实例 | Using associated functions to create instances
  let circle1 = Circle::new(5.0);
  let circle2 = Circle::from_diameter(10.0);
  let circle3 = Circle::unit_circle();
  
  println!("Circle1 area: {:.2}", circle1.area());
  println!("Circle2 area: {:.2}", circle2.area());
  println!("Circle3 area: {:.2}", circle3.area());
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - circle1和circle2的半径相同吗？| Do circle1 and circle2 have the same radius?
    **答案 | Answer:** 是 | Yes - 都是5.0 | Both are 5.0
  - 如何调用关联函数？| How to call associated functions?
    **答案 | Answer:** 使用Type::function_name()语法 | Use Type::function_name() syntax

### 3. 多impl块与方法组织 | Multiple impl Blocks and Method Organization (45分钟 | 45 minutes)

- **多impl块的使用 | Using Multiple impl Blocks**
  
  **概念定义 | Concept Definition:**
  一个类型可以有多个impl块，用于组织不同类别的方法，提高代码的可读性和维护性。| A type can have multiple impl blocks to organize different categories of methods, improving code readability and maintainability.
  
  **核心特征 | Key Characteristics:**
  - 同一类型可以定义多个impl块 | The same type can have multiple impl blocks
  - 不同impl块中的方法会合并到同一类型上 | Methods from different impl blocks are merged onto the same type
  - 可以按功能分组组织方法 | Methods can be organized by functional grouping
  - 有助于代码模块化和维护 | Helps with code modularization and maintenance
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 一个结构体可以有多个impl块吗？| Can a struct have multiple impl blocks?
     **答案 | Answer:** 是 | Yes - 可以定义任意数量的impl块 | Can define any number of impl blocks
  2. 不同impl块中的方法名可以重复吗？| Can method names be duplicated in different impl blocks?
     **答案 | Answer:** 否 | No - 方法名在同一类型中必须唯一 | Method names must be unique within the same type
  3. impl块的顺序重要吗？| Does the order of impl blocks matter?
     **答案 | Answer:** 否 | No - 编译器会合并所有impl块 | The compiler merges all impl blocks
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  struct Student {
      name: String,
      age: u8,
      grades: Vec<f64>,
  }
  
  // 基础功能impl块 | Basic functionality impl block
  impl Student {
      fn new(name: String, age: u8) -> Student {
          Student {
              name,
              age,
              grades: Vec::new(),
          }
      }
      
      fn get_info(&self) -> String {
          format!("学生: {}, 年龄: {}", self.name, self.age)
      }
  }
  
  // 成绩管理impl块 | Grade management impl block
  impl Student {
      fn add_grade(&mut self, grade: f64) {
          if grade >= 0.0 && grade <= 100.0 {
              self.grades.push(grade);
          }
      }
      
      fn average_grade(&self) -> Option<f64> {
          if self.grades.is_empty() {
              None
          } else {
              let sum: f64 = self.grades.iter().sum();
              Some(sum / self.grades.len() as f64)
          }
      }
      
      fn grade_count(&self) -> usize {
          self.grades.len()
      }
  }
  
  // 使用示例 | Usage example
  let mut student = Student::new("张三".to_string(), 18);
  println!("{}", student.get_info());
  
  student.add_grade(85.5);
  student.add_grade(92.0);
  student.add_grade(78.5);
  
  if let Some(avg) = student.average_grade() {
      println!("平均成绩: {:.1}", avg);
  }
  println!("成绩数量: {}", student.grade_count());
  ```

### 4. 方法链调用 | Method Chaining (45分钟 | 45 minutes)

- **方法链的概念与实现 | Method Chaining Concept and Implementation**
  
  **概念定义 | Concept Definition:**
  方法链是一种编程模式，通过让方法返回self的引用或所有权，可以连续调用多个方法。| Method chaining is a programming pattern where methods return a reference to self or ownership of self, allowing multiple method calls to be chained together.
  
  **核心特征 | Key Characteristics:**
  - 方法返回Self或&mut Self以支持链式调用 | Methods return Self or &mut Self to support chaining
  - 提供流畅的API接口体验 | Provides a fluent API interface experience
  - 减少中间变量的使用 | Reduces the use of intermediate variables
  - 常用于构建者模式和配置对象 | Commonly used in builder pattern and configuration objects
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 方法链调用需要方法返回什么？| What do methods need to return for method chaining?
     **答案 | Answer:** self的某种形式 | Some form of self - 通常是Self或&mut Self | Usually Self or &mut Self
  2. 方法链可以混合可变和不可变方法吗？| Can method chains mix mutable and immutable methods?
     **答案 | Answer:** 有限制 | With restrictions - 需要考虑借用规则 | Need to consider borrowing rules
  3. 所有方法都适合用于链式调用吗？| Are all methods suitable for chaining?
     **答案 | Answer:** 否 | No - 只有返回self形式的方法才适合 | Only methods that return some form of self are suitable
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  struct StringBuilder {
      content: String,
  }
  
  impl StringBuilder {
      fn new() -> StringBuilder {
          StringBuilder {
              content: String::new(),
          }
      }
      
      // 支持链式调用的方法 | Methods supporting chaining
      fn append(&mut self, text: &str) -> &mut Self {
          self.content.push_str(text);
          self
      }
      
      fn append_line(&mut self, text: &str) -> &mut Self {
          self.content.push_str(text);
          self.content.push('\n');
          self
      }
      
      fn repeat(&mut self, text: &str, count: usize) -> &mut Self {
          for _ in 0..count {
              self.content.push_str(text);
          }
          self
      }
      
      // 最终方法，不返回self | Final method, doesn't return self
      fn build(self) -> String {
          self.content
      }
      
      fn len(&self) -> usize {
          self.content.len()
      }
  }
  
  // 方法链使用示例 | Method chaining usage example
  let result = StringBuilder::new()
      .append("Hello, ")
      .append("World!")
      .append_line("")
      .repeat("Rust ", 3)
      .append_line("")
      .append("Programming")
      .build();
  
  println!("{}", result);
  ```

### 5. 实际应用场景 | Real-world Application Scenarios (30分钟 | 30 minutes)

- **配置构建者模式 | Configuration Builder Pattern**
  
  **概念定义 | Concept Definition:**
  构建者模式使用方法链来逐步配置复杂对象，提供清晰的API和灵活的配置方式。| The builder pattern uses method chaining to progressively configure complex objects, providing a clear API and flexible configuration approach.
  
  **解决的问题 | Problems It Solves:**
  - 复杂对象的创建过程难以管理 | Complex object creation processes are hard to manage
  - 构造函数参数过多导致不易使用 | Too many constructor parameters make it hard to use
  - 需要可选配置和默认值 | Need optional configurations and default values
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 构建者模式主要解决什么问题？| What problem does the builder pattern mainly solve?
     **答案 | Answer:** 复杂对象创建 | Complex object creation - 简化配置和构造过程 | Simplifies configuration and construction process
  2. 构建者模式通常如何结束链式调用？| How does the builder pattern typically end the method chain?
     **答案 | Answer:** build()方法 | build() method - 返回最终构建的对象 | Returns the finally constructed object
  3. 构建者模式中的方法应该返回什么？| What should methods in builder pattern return?
     **答案 | Answer:** 构建者自身 | The builder itself - 支持继续链式调用 | To support continued chaining
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  struct DatabaseConnection {
      host: String,
      port: u16,
      database: String,
      username: String,
      password: String,
      timeout: u32,
      ssl: bool,
  }
  
  struct DatabaseConnectionBuilder {
      host: Option<String>,
      port: Option<u16>,
      database: Option<String>,
      username: Option<String>,
      password: Option<String>,
      timeout: Option<u32>,
      ssl: Option<bool>,
  }
  
  impl DatabaseConnectionBuilder {
      fn new() -> Self {
          DatabaseConnectionBuilder {
              host: None,
              port: None,
              database: None,
              username: None,
              password: None,
              timeout: None,
              ssl: None,
          }
      }
      
      fn host(&mut self, host: &str) -> &mut Self {
          self.host = Some(host.to_string());
          self
      }
      
      fn port(&mut self, port: u16) -> &mut Self {
          self.port = Some(port);
          self
      }
      
      fn database(&mut self, database: &str) -> &mut Self {
          self.database = Some(database.to_string());
          self
      }
      
      fn credentials(&mut self, username: &str, password: &str) -> &mut Self {
          self.username = Some(username.to_string());
          self.password = Some(password.to_string());
          self
      }
      
      fn timeout(&mut self, seconds: u32) -> &mut Self {
          self.timeout = Some(seconds);
          self
      }
      
      fn enable_ssl(&mut self) -> &mut Self {
          self.ssl = Some(true);
          self
      }
      
      fn build(&self) -> Result<DatabaseConnection, &'static str> {
          Ok(DatabaseConnection {
              host: self.host.clone().unwrap_or_else(|| "localhost".to_string()),
              port: self.port.unwrap_or(5432),
              database: self.database.clone().ok_or("Database name is required")?,
              username: self.username.clone().ok_or("Username is required")?,
              password: self.password.clone().ok_or("Password is required")?,
              timeout: self.timeout.unwrap_or(30),
              ssl: self.ssl.unwrap_or(false),
          })
      }
  }
  
  // 使用构建者模式 | Using builder pattern
  let connection = DatabaseConnectionBuilder::new()
      .host("192.168.1.100")
      .port(5432)
      .database("myapp")
      .credentials("user", "password")
      .timeout(60)
      .enable_ssl()
      .build();
  
  match connection {
      Ok(conn) => println!("连接配置: {}:{}", conn.host, conn.port),
      Err(e) => println!("配置错误: {}", e),
  }
  ```

### 6. 最佳实践与性能考虑 | Best Practices and Performance Considerations (30分钟 | 30 minutes)

- **方法设计原则 | Method Design Principles**
  
  **概念定义 | Concept Definition:**
  好的方法设计应该遵循单一职责、明确的所有权语义、合理的返回类型等原则，以提供清晰、高效的API。| Good method design should follow principles like single responsibility, clear ownership semantics, and reasonable return types to provide clear and efficient APIs.
  
  **关键原则 | Key Principles:**
  - 方法应该有清晰的职责和目的 | Methods should have clear responsibilities and purposes
  - 选择合适的self参数形式 | Choose appropriate self parameter forms
  - 考虑方法的性能影响 | Consider performance impact of methods
  - 提供一致的API风格 | Provide consistent API style
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 什么时候应该使用&self而不是&mut self？| When should you use &self instead of &mut self?
     **答案 | Answer:** 不需要修改数据时 | When no data modification is needed - 提供更好的并发性和安全性 | Provides better concurrency and safety
  2. 方法应该返回&String还是&str？| Should methods return &String or &str?
     **答案 | Answer:** 通常返回&str | Usually return &str - 更灵活，可以接受各种字符串类型 | More flexible, can accept various string types
  3. 什么时候使用self而不是&self？| When to use self instead of &self?
     **答案 | Answer:** 转换或消耗对象时 | When transforming or consuming the object - 如into_*方法或构建者模式的build | Like into_* methods or builder pattern's build
  
  **综合应用检查 | Comprehensive Application Check:**
  - 如何选择合适的self参数类型？| How to choose appropriate self parameter types?
  - 方法链调用的性能考虑有哪些？| What are the performance considerations for method chaining?
  - 如何设计既安全又高效的API？| How to design APIs that are both safe and efficient?

## 实践项目：任务管理器 | Practical Project: Task Manager

### 目标 | Objective
创建一个任务管理系统，综合运用方法、关联函数、方法链等概念，实现任务的创建、修改、查询和状态管理功能。| Create a task management system that comprehensively applies methods, associated functions, and method chaining to implement task creation, modification, querying, and status management features.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 方法和关联函数有什么区别？| What's the difference between methods and associated functions?
   **答案 | Answer:** 方法有self参数，关联函数没有；方法需要实例调用，关联函数通过类型调用 | Methods have self parameter, associated functions don't; methods need instance to call, associated functions are called through type
2. &mut self参数允许方法做什么？| What does &mut self parameter allow methods to do?
   **答案 | Answer:** 修改实例的数据，同时保持实例的所有权 | Modify instance data while maintaining instance ownership
3. 方法链调用的关键要求是什么？| What's the key requirement for method chaining?
   **答案 | Answer:** 方法必须返回self的某种形式（Self或&mut Self）| Methods must return some form of self (Self or &mut Self)

### 步骤 | Steps
1. **定义任务结构体和状态枚举** | **Define Task struct and Status enum**
2. **实现任务的关联函数（构造函数）** | **Implement associated functions for Task (constructors)**
3. **添加任务操作方法（状态变更、信息获取）** | **Add task operation methods (status changes, information retrieval)**
4. **实现任务管理器结构体** | **Implement TaskManager struct**
5. **为管理器添加方法链支持** | **Add method chaining support to manager**
6. **实现查询和过滤功能** | **Implement query and filtering functionality**
7. **添加统计和报告方法** | **Add statistics and reporting methods**

### 示例代码 | Example Code
```rust
"""
任务管理器 | Task Manager
一个展示方法、关联函数和方法链应用的项目 | A project demonstrating methods, associated functions, and method chaining

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 方法的三种self形式 | Three forms of self in methods
- 关联函数作为构造函数 | Associated functions as constructors
- 方法链调用模式 | Method chaining patterns
- 多impl块的组织方式 | Multiple impl block organization
"""

use std::fmt;

// 任务状态枚举 | Task status enum
#[derive(Debug, Clone, PartialEq)]
enum TaskStatus {
    Todo,       // 待办 | To-do
    InProgress, // 进行中 | In progress
    Completed,  // 已完成 | Completed
    Cancelled,  // 已取消 | Cancelled
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "待办"),
            TaskStatus::InProgress => write!(f, "进行中"),
            TaskStatus::Completed => write!(f, "已完成"),
            TaskStatus::Cancelled => write!(f, "已取消"),
        }
    }
}

// 任务结构体 | Task struct
#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
    status: TaskStatus,
    priority: u8, // 1-5，5最高 | 1-5, 5 is highest
}

// 任务基础功能impl块 | Task basic functionality impl block
impl Task {
    // 关联函数 - 创建新任务 | Associated function - create new task
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            description: String::new(),
            status: TaskStatus::Todo,
            priority: 3, // 默认中等优先级 | Default medium priority
        }
    }
    
    // 关联函数 - 创建带描述的任务 | Associated function - create task with description
    fn with_description(id: u32, title: String, description: String) -> Task {
        Task {
            id,
            title,
            description,
            status: TaskStatus::Todo,
            priority: 3,
        }
    }
    
    // 关联函数 - 创建高优先级任务 | Associated function - create high priority task
    fn urgent(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            description: String::new(),
            status: TaskStatus::Todo,
            priority: 5,
        }
    }
    
    // 方法 - 获取任务信息 | Method - get task info
    fn get_info(&self) -> String {
        format!(
            "任务 #{}: {} [{}] (优先级: {})",
            self.id, self.title, self.status, self.priority
        )
    }
    
    // 方法 - 获取详细信息 | Method - get detailed info
    fn get_details(&self) -> String {
        format!(
            "任务 #{}: {}\n描述: {}\n状态: {}\n优先级: {}",
            self.id, self.title, 
            if self.description.is_empty() { "无" } else { &self.description },
            self.status, self.priority
        )
    }
}

// 任务状态管理impl块 | Task status management impl block
impl Task {
    // 方法 - 开始任务 | Method - start task
    fn start(&mut self) -> &mut Self {
        if self.status == TaskStatus::Todo {
            self.status = TaskStatus::InProgress;
        }
        self
    }
    
    // 方法 - 完成任务 | Method - complete task
    fn complete(&mut self) -> &mut Self {
        if self.status == TaskStatus::InProgress {
            self.status = TaskStatus::Completed;
        }
        self
    }
    
    // 方法 - 取消任务 | Method - cancel task
    fn cancel(&mut self) -> &mut Self {
        if self.status != TaskStatus::Completed {
            self.status = TaskStatus::Cancelled;
        }
        self
    }
    
    // 方法 - 重置任务 | Method - reset task
    fn reset(&mut self) -> &mut Self {
        self.status = TaskStatus::Todo;
        self
    }
    
    // 方法 - 设置优先级 | Method - set priority
    fn set_priority(&mut self, priority: u8) -> &mut Self {
        self.priority = priority.clamp(1, 5);
        self
    }
    
    // 方法 - 更新描述 | Method - update description
    fn update_description(&mut self, description: String) -> &mut Self {
        self.description = description;
        self
    }
}

// 任务查询impl块 | Task query impl block
impl Task {
    // 方法 - 检查是否完成 | Method - check if completed
    fn is_completed(&self) -> bool {
        self.status == TaskStatus::Completed
    }
    
    // 方法 - 检查是否进行中 | Method - check if in progress
    fn is_in_progress(&self) -> bool {
        self.status == TaskStatus::InProgress
    }
    
    // 方法 - 检查优先级 | Method - check priority
    fn is_high_priority(&self) -> bool {
        self.priority >= 4
    }
    
    // 方法 - 匹配关键词 | Method - match keywords
    fn matches_keyword(&self, keyword: &str) -> bool {
        self.title.to_lowercase().contains(&keyword.to_lowercase()) ||
        self.description.to_lowercase().contains(&keyword.to_lowercase())
    }
}

// 任务管理器结构体 | Task manager struct
struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    // 关联函数 - 创建新的任务管理器 | Associated function - create new task manager
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    // 方法 - 添加任务并支持链式调用 | Method - add task with chaining support
    fn add_task(&mut self, title: String) -> &mut Self {
        let task = Task::new(self.next_id, title);
        self.tasks.push(task);
        self.next_id += 1;
        self
    }
    
    // 方法 - 添加带描述的任务 | Method - add task with description
    fn add_task_with_desc(&mut self, title: String, description: String) -> &mut Self {
        let task = Task::with_description(self.next_id, title, description);
        self.tasks.push(task);
        self.next_id += 1;
        self
    }
    
    // 方法 - 添加紧急任务 | Method - add urgent task
    fn add_urgent_task(&mut self, title: String) -> &mut Self {
        let task = Task::urgent(self.next_id, title);
        self.tasks.push(task);
        self.next_id += 1;
        self
    }
    
    // 方法 - 按ID查找任务 | Method - find task by ID
    fn find_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }
    
    // 方法 - 开始指定任务 | Method - start specific task
    fn start_task(&mut self, id: u32) -> &mut Self {
        if let Some(task) = self.find_task_mut(id) {
            task.start();
        }
        self
    }
    
    // 方法 - 完成指定任务 | Method - complete specific task
    fn complete_task(&mut self, id: u32) -> &mut Self {
        if let Some(task) = self.find_task_mut(id) {
            task.complete();
        }
        self
    }
    
    // 方法 - 显示所有任务 | Method - show all tasks
    fn show_all_tasks(&self) {
        println!("\n=== 所有任务 ===");
        for task in &self.tasks {
            println!("{}", task.get_info());
        }
    }
    
    // 方法 - 显示任务统计 | Method - show task statistics
    fn show_statistics(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.is_completed()).count();
        let in_progress = self.tasks.iter().filter(|t| t.is_in_progress()).count();
        let high_priority = self.tasks.iter().filter(|t| t.is_high_priority()).count();
        
        println!("\n=== 任务统计 ===");
        println!("总任务数: {}", total);
        println!("已完成: {}", completed);
        println!("进行中: {}", in_progress);
        println!("高优先级: {}", high_priority);
        
        if total > 0 {
            println!("完成率: {:.1}%", (completed as f64 / total as f64) * 100.0);
        }
    }
}

fn main() {
    println!("=== 任务管理器演示 ===\n");
    
    // 创建任务管理器并使用方法链添加多个任务 | Create task manager and add multiple tasks using method chaining
    let mut manager = TaskManager::new()
        .add_task("学习Rust基础语法".to_string())
        .add_task_with_desc(
            "完成项目文档".to_string(),
            "编写详细的API文档和使用说明".to_string()
        )
        .add_urgent_task("修复生产环境bug".to_string())
        .clone(); // 注意：这里需要clone因为链式调用返回&mut Self
    
    // 使用方法链操作任务状态 | Use method chaining to operate task status
    manager
        .start_task(1)
        .complete_task(1)
        .start_task(3);
    
    // 显示任务信息 | Show task information
    manager.show_all_tasks();
    manager.show_statistics();
    
    // 演示单个任务的方法链调用 | Demonstrate method chaining on individual tasks
    println!("\n=== 任务状态变更演示 ===");
    if let Some(task) = manager.find_task_mut(2) {
        task.start()
            .set_priority(4)
            .update_description("更新后的描述：包含详细的实现步骤".to_string());
        
        println!("{}", task.get_details());
    }
    
    println!("\n项目完成！成功演示了方法、关联函数和方法链的应用。");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_creation() {
        let task = Task::new(1, "测试任务".to_string());
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "测试任务");
        assert_eq!(task.status, TaskStatus::Todo);
        assert_eq!(task.priority, 3);
    }
    
    #[test]
    fn test_task_status_changes() {
        let mut task = Task::new(1, "测试".to_string());
        
        task.start();
        assert_eq!(task.status, TaskStatus::InProgress);
        
        task.complete();
        assert_eq!(task.status, TaskStatus::Completed);
    }
    
    #[test]
    fn test_method_chaining() {
        let mut task = Task::new(1, "测试".to_string());
        
        task.start()
            .set_priority(5)
            .update_description("链式调用测试".to_string());
        
        assert_eq!(task.status, TaskStatus::InProgress);
        assert_eq!(task.priority, 5);
        assert_eq!(task.description, "链式调用测试");
    }
    
    #[test]
    fn test_task_manager() {
        let mut manager = TaskManager::new();
        
        manager
            .add_task("任务1".to_string())
            .add_urgent_task("紧急任务".to_string());
        
        assert_eq!(manager.tasks.len(), 2);
        assert_eq!(manager.tasks[1].priority, 5); // 紧急任务高优先级
    }
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确使用了三种形式的self参数？| Does the project correctly use three forms of self parameter?
2. 关联函数是否用作了构造函数？| Are associated functions used as constructors?
3. 方法链是否实现并能正常工作？| Is method chaining implemented and working properly?
4. 代码是否按功能组织在多个impl块中？| Is code organized into multiple impl blocks by functionality?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **方法参数理解练习 | Method Parameter Understanding Exercise**
   - **练习描述 | Exercise Description:** 创建一个图书管理系统，为Book结构体实现borrowing、returning、和renewing方法，分别使用&self、&mut self参数
   - **概念检查 | Concept Check:** 什么情况下使用&self，什么情况下使用&mut self？
   - **学习目标 | Learning Objective:** 深入理解不同self参数形式的应用场景

2. **关联函数应用练习 | Associated Function Application Exercise**
   - **练习描述 | Exercise Description:** 为Point结构体创建多个关联函数：origin()、from_coords()、random()等，提供不同的创建方式
   - **概念检查 | Concept Check:** 关联函数和方法的调用语法有何不同？
   - **学习目标 | Learning Objective:** 掌握关联函数作为构造函数的设计模式

3. **方法链设计练习 | Method Chaining Design Exercise**
   - **练习描述 | Exercise Description:** 设计一个HTTP客户端构建器，支持链式设置头部、超时、认证等配置
   - **概念检查 | Concept Check:** 方法链中的每个方法应该返回什么类型？
   - **学习目标 | Learning Objective:** 学会设计流畅的链式API

4. **多impl块组织练习 | Multiple impl Block Organization Exercise**
   - **练习描述 | Exercise Description:** 重构一个大型结构体，将其方法按功能分组到不同的impl块中
   - **概念检查 | Concept Check:** 如何合理地组织和分类结构体的方法？
   - **学习目标 | Learning Objective:** 提高代码组织和可维护性能力

5. **所有权转移练习 | Ownership Transfer Exercise**
   - **练习描述 | Exercise Description:** 实现一个状态机，某些状态转换需要消费对象（使用self参数）
   - **概念检查 | Concept Check:** 什么时候应该使用self而不是&self或&mut self？
   - **学习目标 | Learning Objective:** 理解所有权转移在API设计中的作用

6. **方法返回类型练习 | Method Return Type Exercise**
   - **练习描述 | Exercise Description:** 设计一个数据验证器，方法可以返回Result、Option或自定义类型
   - **概念检查 | Concept Check:** 如何选择合适的返回类型来表达方法的语义？
   - **学习目标 | Learning Objective:** 掌握方法返回类型设计的最佳实践

7. **综合应用练习 | Comprehensive Application Exercise**
   - **练习描述 | Exercise Description:** 创建一个完整的订单处理系统，综合运用所有学到的概念
   - **概念检查 | Concept Check:** 如何在实际项目中合理使用方法、关联函数和方法链？
   - **学习目标 | Learning Objective:** 培养在真实项目中应用这些概念的能力

## 学习资源 | Learning Resources
- [Rust官方文档 - 方法语法](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Rust官方文档 - 定义和实例化结构体](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Rust By Example - 方法](https://doc.rust-lang.org/rust-by-example/fn/methods.html)
- [Rust设计模式 - 构建者模式](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解方法和关联函数的概念差异 | Understand conceptual differences between methods and associated functions
- [ ] 掌握self、&self、&mut self三种参数形式 | Master the three forms of self parameters
- [ ] 能够为结构体定义各种类型的方法 | Can define various types of methods for structs
- [ ] 理解impl块的作用和组织方式 | Understand the role and organization of impl blocks
- [ ] 掌握方法链调用的实现技巧 | Master implementation techniques for method chaining
- [ ] 完成任务管理器项目并正确应用所有概念 | Complete task manager project applying all concepts correctly
- [ ] 能够设计清晰、高效的方法API | Can design clear and efficient method APIs
- [ ] 理解构建者模式在Rust中的实现 | Understand builder pattern implementation in Rust
- [ ] 掌握多impl块的代码组织最佳实践 | Master best practices for multi-impl block code organization
- [ ] 至少完成3个扩展练习 | Complete at least 3 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释方法、关联函数、方法链等核心概念的区别和应用场景。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain the differences and application scenarios of methods, associated functions, method chaining, and other core concepts to others.