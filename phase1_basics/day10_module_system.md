# Rust入门 - 第10天：模块系统基础 | Rust Introduction - Day 10: Module System Basics

## 学习目标 | Learning Objectives
- 理解Rust模块系统的层次结构和组织原则 | Understand the hierarchical structure and organization principles of Rust's module system
- 掌握mod关键字创建和使用模块的方法 | Master the methods of creating and using modules with the mod keyword
- 学会使用pub关键字控制模块和项的可见性 | Learn to control module and item visibility using the pub keyword
- 熟练使用use语句导入和简化路径访问 | Become proficient in using use statements to import and simplify path access
- 掌握文件模块的组织和项目结构设计 | Master file module organization and project structure design
- 能够重构代码为模块化结构提高可维护性 | Be able to refactor code into modular structures to improve maintainability

## 详细内容 | Detailed Content

### 1. 模块系统概述 | Module System Overview (1小时 | 1 hour)

- **模块系统的作用 | Role of Module System**
  
  **概念定义 | Concept Definition:**
  模块系统是Rust用来组织代码、控制可见性和管理命名空间的机制，它允许我们将相关功能组织在一起，避免命名冲突，并控制哪些代码可以被外部访问。| The module system is Rust's mechanism for organizing code, controlling visibility, and managing namespaces. It allows us to group related functionality, avoid naming conflicts, and control which code can be accessed externally.
  
  **核心特征 | Key Characteristics:**
  - 模块形成树状层次结构，从crate根开始 | Modules form a tree-like hierarchical structure starting from the crate root
  - 默认情况下模块内容是私有的，需要pub关键字公开 | Module contents are private by default and require the pub keyword to be made public
  - 模块可以嵌套，支持多层级组织 | Modules can be nested, supporting multi-level organization
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Rust模块中的项默认是公有的还是私有的？| Are items in Rust modules public or private by default?
     **答案 | Answer:** 私有的 | Private - 这确保了良好的封装性和安全性
  2. 模块可以嵌套在其他模块内部吗？| Can modules be nested inside other modules?  
     **答案 | Answer:** 是的 | Yes - 这允许创建复杂的层次结构来组织代码
  3. 一个crate可以包含多个模块吗？| Can a crate contain multiple modules?
     **答案 | Answer:** 是的 | Yes - 这是模块系统的基本功能，用于组织大型项目
  4. 模块系统的主要目的是什么？| What is the main purpose of the module system?
     **答案 | Answer:** 组织代码、控制可见性、管理命名空间 | Organize code, control visibility, manage namespaces
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 基本模块定义 | Basic module definition
  mod garden {
      // 模块内的项默认是私有的 | Items in modules are private by default
      fn plant_seed() {
          println!("种植种子 | Planting seed");
      }
      
      // 使用pub使项变为公有 | Use pub to make items public
      pub fn water_plant() {
          println!("浇水 | Watering plant");
          plant_seed(); // 同模块内可以访问私有函数 | Can access private functions within the same module
      }
  }
  
  fn main() {
      garden::water_plant(); // 可以访问公有函数 | Can access public functions
      // garden::plant_seed(); // 错误：私有函数不能访问 | Error: private function cannot be accessed
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码中哪个函数可以在main中调用？| Which function can be called in main in this code?
    **答案 | Answer:** 只有water_plant()，因为它是公有的 | Only water_plant() because it's public
  - 如果移除pub关键字会发生什么？| What happens if we remove the pub keyword?
    **答案 | Answer:** water_plant()也会变成私有的，main中无法访问 | water_plant() becomes private too and cannot be accessed from main
  
  **常见误区检查 | Common Misconception Checks:**
  - 模块内的所有项都自动对外可见吗？| Are all items in a module automatically visible externally?
    **答案 | Answer:** 不是，需要显式使用pub关键字 | No, they need explicit pub keyword
  - 同一模块内的项可以互相访问私有成员吗？| Can items within the same module access each other's private members?
    **答案 | Answer:** 是的，同模块内项可以访问彼此的私有成员 | Yes, items within the same module can access each other's private members

### 2. 模块定义与嵌套 | Module Definition and Nesting (45分钟 | 45 minutes)

- **嵌套模块结构 | Nested Module Structure**
  
  **概念定义 | Concept Definition:**
  嵌套模块是指在一个模块内部定义另一个模块，形成树状的层次结构。这种结构允许我们创建逻辑清晰的代码组织方式，将相关功能分组到不同层级的模块中。| Nested modules refer to defining one module inside another, forming a tree-like hierarchical structure. This structure allows us to create logically clear code organization, grouping related functionality into modules at different levels.
  
  **核心特征 | Key Characteristics:**
  - 子模块可以访问父模块的所有项（包括私有项） | Child modules can access all items in parent modules (including private ones)
  - 父模块不能直接访问子模块的私有项 | Parent modules cannot directly access private items in child modules
  - 模块路径使用::分隔符表示层次关系 | Module paths use :: separator to indicate hierarchical relationships
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 子模块能够访问父模块的私有项吗？| Can child modules access private items in parent modules?
     **答案 | Answer:** 是的 | Yes - 这是Rust模块系统的特殊设计
  2. 如何表示嵌套模块的路径？| How do we represent paths to nested modules?
     **答案 | Answer:** 使用::分隔符，如parent::child | Use :: separator, like parent::child
  3. 模块嵌套有层数限制吗？| Is there a limit to module nesting levels?
     **答案 | Answer:** 没有硬性限制，但应保持合理的深度 | No hard limit, but should maintain reasonable depth
  4. 父模块可以直接访问孙子模块的项吗？| Can parent modules directly access items in grandchild modules?
     **答案 | Answer:** 需要通过完整路径，且遵循可见性规则 | Need full path and must follow visibility rules
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  mod restaurant {
      // 父模块的私有项 | Private item in parent module
      fn take_order() {
          println!("接受订单 | Taking order");
      }
      
      pub mod kitchen {
          // 子模块可以访问父模块的所有项 | Child modules can access all parent items
          pub fn cook_meal() {
              super::take_order(); // 使用super访问父模块 | Use super to access parent module
              println!("烹饪菜品 | Cooking meal");
          }
          
          mod prep {
              pub fn wash_vegetables() {
                  println!("清洗蔬菜 | Washing vegetables");
                  // 可以访问祖父模块 | Can access grandparent module
                  super::super::take_order();
              }
          }
          
          pub fn prepare() {
              prep::wash_vegetables();
          }
      }
      
      pub fn serve_customer() {
          kitchen::cook_meal();
          kitchen::prepare();
      }
  }
  
  fn main() {
      restaurant::serve_customer();
      // restaurant::take_order(); // 错误：私有函数 | Error: private function
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - prep模块如何访问restaurant模块的函数？| How does the prep module access functions in the restaurant module?
    **答案 | Answer:** 使用super::super::访问祖父模块 | Use super::super:: to access grandparent module
  - main函数能直接调用kitchen::cook_meal()吗？| Can main function directly call kitchen::cook_meal()?
    **答案 | Answer:** 不能，需要完整路径restaurant::kitchen::cook_meal() | No, need full path restaurant::kitchen::cook_meal()

### 3. 可见性控制与pub关键字 | Visibility Control and pub Keyword (45分钟 | 45 minutes)

- **pub关键字的不同形式 | Different Forms of pub Keyword**
  
  **概念定义 | Concept Definition:**
  pub关键字是Rust中控制可见性的机制，它有多种形式来精确控制项的访问范围。除了基本的pub，还有pub(crate)、pub(super)、pub(in path)等限定符，允许我们精确指定谁可以访问这些项。| The pub keyword is Rust's mechanism for controlling visibility, with multiple forms to precisely control item access scope. Besides basic pub, there are qualifiers like pub(crate), pub(super), pub(in path) that allow us to precisely specify who can access these items.
  
  **核心特征 | Key Characteristics:**
  - pub使项在模块外部可见 | pub makes items visible outside the module
  - pub(crate)限制在当前crate内可见 | pub(crate) restricts visibility to within current crate
  - pub(super)限制在父模块内可见 | pub(super) restricts visibility to within parent module
  - pub(in path)限制在指定路径内可见 | pub(in path) restricts visibility to within specified path
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. pub和pub(crate)有什么区别？| What's the difference between pub and pub(crate)?
     **答案 | Answer:** pub对外部crate也可见，pub(crate)只在当前crate内可见 | pub is visible to external crates, pub(crate) only visible within current crate
  2. pub(super)适用于什么场景？| What scenarios is pub(super) suitable for?
     **答案 | Answer:** 当只想让父模块访问，不想对更外层暴露时 | When you only want parent modules to access, not expose to outer layers
  3. 不加任何修饰符的项是什么可见性？| What visibility do items have without any modifiers?
     **答案 | Answer:** 私有的，仅在当前模块内可见 | Private, only visible within current module
  4. 可以组合使用不同的pub修饰符吗？| Can different pub modifiers be combined?
     **答案 | Answer:** 不可以，一个项只能有一种可见性级别 | No, an item can only have one visibility level
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  mod outer {
      pub fn public_function() {
          println!("完全公开的函数 | Fully public function");
      }
      
      pub(crate) fn crate_public_function() {
          println!("crate内公开的函数 | Crate-public function");
      }
      
      pub(super) fn super_public_function() {
          println!("父模块公开的函数 | Super-public function");
      }
      
      pub mod inner {
          pub(in crate::outer) fn limited_function() {
              println!("限定路径公开的函数 | Path-limited public function");
          }
          
          pub fn call_all() {
              // 子模块可以调用父模块的所有函数 | Child modules can call all parent functions
              super::public_function();
              super::crate_public_function();
              super::super_public_function();
              limited_function();
          }
      }
  }
  
  fn main() {
      outer::public_function(); // ✓ 可以访问 | Can access
      outer::crate_public_function(); // ✓ 同crate内可以访问 | Can access within same crate
      // outer::super_public_function(); // ✗ 不能访问，我们不是outer的父模块 | Cannot access, we're not outer's parent
      
      outer::inner::call_all();
      // outer::inner::limited_function(); // ✗ 不能访问，路径限制 | Cannot access, path restricted
  }
  ```
  
  **常见误区检查 | Common Misconception Checks:**
  - pub(crate)和pub在同一个crate内的行为是否相同？| Do pub(crate) and pub behave the same within the same crate?
    **答案 | Answer:** 在单crate项目中相同，但在多crate项目中pub会暴露给外部 | Same in single-crate projects, but pub exposes to external crates in multi-crate projects

### 4. use语句与路径导入 | use Statements and Path Imports (45分钟 | 45 minutes)

- **use语句的作用与语法 | Purpose and Syntax of use Statements**
  
  **概念定义 | Concept Definition:**
  use语句用于将模块路径引入当前作用域，简化对深层嵌套模块中项的访问。它不会移动或复制项，而是创建一个快捷方式，让我们可以用更短的名称访问项。| use statements bring module paths into the current scope, simplifying access to items in deeply nested modules. They don't move or copy items, but create shortcuts that allow us to access items with shorter names.
  
  **核心特征 | Key Characteristics:**
  - use创建路径的别名，不影响原始项的位置 | use creates path aliases without affecting original item locations
  - 可以使用as关键字重命名导入的项 | Can use as keyword to rename imported items
  - 支持glob导入（*）和嵌套导入 | Supports glob imports (*) and nested imports
  - use语句遵循同样的可见性规则 | use statements follow the same visibility rules
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. use语句会移动原始项的位置吗？| Do use statements move original items from their location?
     **答案 | Answer:** 不会，只是创建一个引用路径 | No, they only create a reference path
  2. 什么时候应该使用as重命名？| When should we use as for renaming?
     **答案 | Answer:** 当导入的名称冲突或名称太长时 | When imported names conflict or are too long
  3. 可以在模块内部使用use导入外部项吗？| Can we use use to import external items inside modules?
     **答案 | Answer:** 可以，use可以在任何作用域中使用 | Yes, use can be used in any scope
  4. use std::*这样的导入方式好吗？| Is importing with use std::* a good practice?
     **答案 | Answer:** 通常不推荐，会导致命名空间污染 | Usually not recommended, causes namespace pollution
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  mod math {
      pub mod geometry {
          pub fn calculate_area(radius: f64) -> f64 {
              std::f64::consts::PI * radius * radius
          }
          
          pub fn calculate_perimeter(radius: f64) -> f64 {
              2.0 * std::f64::consts::PI * radius
          }
      }
      
      pub mod algebra {
          pub fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
              let discriminant = b * b - 4.0 * a * c;
              let sqrt_d = discriminant.sqrt();
              ((-b + sqrt_d) / (2.0 * a), (-b - sqrt_d) / (2.0 * a))
          }
      }
  }
  
  // 不同的use导入方式 | Different use import methods
  use math::geometry::calculate_area; // 导入具体函数 | Import specific function
  use math::algebra as alg; // 重命名模块 | Rename module
  use math::geometry::{calculate_area as area, calculate_perimeter}; // 嵌套导入和重命名 | Nested import and rename
  
  fn main() {
      // 使用导入的函数 | Use imported functions
      println!("面积: {}", calculate_area(5.0));
      println!("面积: {}", area(5.0)); // 使用重命名的函数 | Use renamed function
      println!("周长: {}", calculate_perimeter(5.0));
      
      // 使用重命名的模块 | Use renamed module
      let roots = alg::solve_quadratic(1.0, -5.0, 6.0);
      println!("方程根: {:?}", roots);
      
      // 仍然可以使用完整路径 | Can still use full paths
      println!("面积: {}", math::geometry::calculate_area(3.0));
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 在这个例子中，calculate_area被导入了几次？| How many times is calculate_area imported in this example?
    **答案 | Answer:** 两次，一次直接导入，一次重命名为area | Twice, once directly and once renamed as area
  - 如果删除use语句，代码还能工作吗？| Would the code still work if we remove the use statements?
    **答案 | Answer:** 可以，但需要使用完整路径访问函数 | Yes, but would need to use full paths to access functions

### 5. 文件模块组织 | File Module Organization (45分钟 | 45 minutes)

- **文件系统与模块对应关系 | File System and Module Correspondence**
  
  **概念定义 | Concept Definition:**
  Rust允许将模块定义在单独的文件中，文件名对应模块名。这种组织方式使得大型项目的代码结构更加清晰，每个文件专注于特定的功能模块，便于维护和协作开发。| Rust allows defining modules in separate files, where file names correspond to module names. This organization makes code structure clearer in large projects, with each file focusing on specific functionality modules, facilitating maintenance and collaborative development.
  
  **核心特征 | Key Characteristics:**
  - 文件名必须与模块名匹配 | File names must match module names
  - mod.rs文件可以将目录作为模块使用 | mod.rs files allow directories to be used as modules
  - lib.rs是库crate的根模块 | lib.rs is the root module of library crates
  - main.rs是二进制crate的入口点 | main.rs is the entry point of binary crates
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 如果有一个名为utils的模块，应该创建什么文件？| If there's a module named utils, what file should be created?
     **答案 | Answer:** utils.rs文件或utils/mod.rs | utils.rs file or utils/mod.rs
  2. lib.rs和main.rs有什么区别？| What's the difference between lib.rs and main.rs?
     **答案 | Answer:** lib.rs是库的根，main.rs是可执行程序的入口 | lib.rs is the library root, main.rs is executable entry point
  3. 可以在一个项目中同时有lib.rs和main.rs吗？| Can a project have both lib.rs and main.rs?
     **答案 | Answer:** 可以，这样既可以作为库又可以作为可执行程序 | Yes, this makes it both a library and executable
  4. mod.rs文件的作用是什么？| What is the purpose of mod.rs files?
     **答案 | Answer:** 将目录识别为模块，并定义该目录模块的内容 | Identify directories as modules and define the module's contents
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // src/main.rs - 主入口文件 | Main entry file
  mod utils; // 声明utils模块，对应utils.rs文件 | Declare utils module, corresponds to utils.rs file
  mod calculator; // 声明calculator模块 | Declare calculator module
  
  use utils::formatter;
  use calculator::operations;
  
  fn main() {
      let result = operations::add(10, 5);
      println!("{}", formatter::format_result("加法结果", result));
  }
  
  // src/utils.rs - 工具模块文件 | Utility module file
  pub mod formatter {
      pub fn format_result(operation: &str, result: i32) -> String {
          format!("{}: {}", operation, result)
      }
      
      pub fn format_error(message: &str) -> String {
          format!("错误: {}", message)
      }
  }
  
  pub mod validator {
      pub fn is_valid_number(input: &str) -> bool {
          input.parse::<f64>().is_ok()
      }
  }
  
  // src/calculator/mod.rs - 计算器模块目录的入口 | Calculator module directory entry
  pub mod operations; // 声明operations子模块 | Declare operations submodule
  pub mod history; // 声明history子模块 | Declare history submodule
  
  // src/calculator/operations.rs - 运算操作子模块 | Operations submodule
  pub fn add(a: i32, b: i32) -> i32 {
      a + b
  }
  
  pub fn subtract(a: i32, b: i32) -> i32 {
      a - b
  }
  
  pub fn multiply(a: i32, b: i32) -> i32 {
      a * b
  }
  
  pub fn divide(a: i32, b: i32) -> Result<f64, String> {
      if b == 0 {
          Err("除零错误".to_string())
      } else {
          Ok(a as f64 / b as f64)
      }
  }
  
  // src/calculator/history.rs - 历史记录子模块 | History submodule
  use std::collections::VecDeque;
  
  pub struct History {
      records: VecDeque<String>,
      max_size: usize,
  }
  
  impl History {
      pub fn new(max_size: usize) -> Self {
          Self {
              records: VecDeque::new(),
              max_size,
          }
      }
      
      pub fn add_record(&mut self, record: String) {
          if self.records.len() >= self.max_size {
              self.records.pop_front();
          }
          self.records.push_back(record);
      }
      
      pub fn get_history(&self) -> &VecDeque<String> {
          &self.records
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这个项目结构中，如何访问validator模块的函数？| How do we access functions in the validator module in this project structure?
    **答案 | Answer:** utils::validator::is_valid_number() | utils::validator::is_valid_number()
  - 为什么calculator使用mod.rs而utils使用单个文件？| Why does calculator use mod.rs while utils uses a single file?
    **答案 | Answer:** calculator有多个子模块，使用目录组织更清晰 | calculator has multiple submodules, directory organization is clearer

### 6. 模块化重构实践 | Modular Refactoring Practice (30分钟 | 30 minutes)

- **重构策略与最佳实践 | Refactoring Strategies and Best Practices**
  
  **概念定义 | Concept Definition:**
  模块化重构是指将单一文件中的大量代码按照功能逻辑分解到多个模块中，提高代码的可维护性、可重用性和可测试性。这个过程需要识别相关功能、确定适当的抽象级别并设计合理的模块边界。| Modular refactoring refers to decomposing large amounts of code in a single file into multiple modules based on functional logic, improving code maintainability, reusability, and testability. This process requires identifying related functionality, determining appropriate abstraction levels, and designing reasonable module boundaries.
  
  **重构原则 | Refactoring Principles:**
  - 单一职责：每个模块专注于一个特定功能 | Single responsibility: each module focuses on one specific function
  - 高内聚低耦合：相关功能组织在一起，模块间依赖最小 | High cohesion low coupling: related functions organized together, minimal inter-module dependencies
  - 渐进式重构：逐步分离功能，避免一次性大幅改动 | Progressive refactoring: gradually separate functions, avoid massive changes at once
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 什么时候应该考虑将代码重构为模块？| When should we consider refactoring code into modules?
     **答案 | Answer:** 当单个文件变得过长或包含多个不相关功能时 | When a single file becomes too long or contains multiple unrelated functions
  2. 模块化重构的主要好处是什么？| What are the main benefits of modular refactoring?
     **答案 | Answer:** 提高可维护性、可重用性、可测试性和代码组织性 | Improve maintainability, reusability, testability and code organization
  3. 如何确定模块的边界？| How do we determine module boundaries?
     **答案 | Answer:** 根据功能相关性和职责分离原则 | Based on functional relevance and separation of concerns principle
  4. 重构时应该优先考虑什么？| What should be prioritized during refactoring?
     **答案 | Answer:** 保持功能不变，确保测试通过 | Maintain functionality unchanged, ensure tests pass
  
  **重构示例 | Refactoring Example:**
  ```rust
  // 重构前：所有代码在一个文件中 | Before refactoring: all code in one file
  // src/main.rs (original monolithic version)
  /*
  use std::io;
  use std::collections::HashMap;
  
  fn main() {
      let mut calculator = Calculator::new();
      calculator.run();
  }
  
  struct Calculator {
      history: Vec<String>,
      variables: HashMap<String, f64>,
  }
  
  impl Calculator {
      fn new() -> Self { /* ... */ }
      fn run(&mut self) { /* ... */ }
      fn add(&self, a: f64, b: f64) -> f64 { a + b }
      fn subtract(&self, a: f64, b: f64) -> f64 { a - b }
      fn multiply(&self, a: f64, b: f64) -> f64 { a * b }
      fn divide(&self, a: f64, b: f64) -> Result<f64, String> { /* ... */ }
      fn parse_expression(&self, expr: &str) -> Result<f64, String> { /* ... */ }
      fn get_input(&self) -> String { /* ... */ }
      fn display_history(&self) { /* ... */ }
      fn save_to_variable(&mut self, name: String, value: f64) { /* ... */ }
  }
  */
  
  // 重构后：模块化结构 | After refactoring: modular structure
  // src/main.rs
  mod calculator;
  mod operations;
  mod history;
  mod variables;
  mod parser;
  mod ui;
  
  use calculator::Calculator;
  
  fn main() {
      let mut calc = Calculator::new();
      calc.run();
  }
  
  // src/calculator.rs - 主要的计算器结构 | Main calculator structure
  use crate::{operations, history, variables, parser, ui};
  
  pub struct Calculator {
      history: history::History,
      variables: variables::VariableStore,
  }
  
  impl Calculator {
      pub fn new() -> Self {
          Self {
              history: history::History::new(),
              variables: variables::VariableStore::new(),
          }
      }
      
      pub fn run(&mut self) {
          loop {
              let input = ui::get_input();
              match self.process_input(&input) {
                  Ok(result) => {
                      println!("结果: {}", result);
                      self.history.add_calculation(&input, result);
                  },
                  Err(e) => println!("错误: {}", e),
              }
          }
      }
      
      fn process_input(&mut self, input: &str) -> Result<f64, String> {
          if input.starts_with("let ") {
              self.handle_variable_assignment(input)
          } else {
              parser::parse_and_evaluate(input, &self.variables)
          }
      }
      
      fn handle_variable_assignment(&mut self, input: &str) -> Result<f64, String> {
          // 处理变量赋值逻辑 | Handle variable assignment logic
          let parts: Vec<&str> = input[4..].split('=').collect();
          if parts.len() != 2 {
              return Err("变量赋值格式错误".to_string());
          }
          
          let var_name = parts[0].trim().to_string();
          let value = parser::parse_and_evaluate(parts[1].trim(), &self.variables)?;
          self.variables.set(var_name, value);
          Ok(value)
      }
  }
  ```
  
  **重构验证 | Refactoring Verification:**
  - 重构后的代码是否保持了原有功能？| Does the refactored code maintain original functionality?
  - 各个模块的职责是否清晰？| Are the responsibilities of each module clear?
  - 模块间的依赖关系是否合理？| Are the dependencies between modules reasonable?

## 实践项目：模块化计算器重构 | Practical Project: Modular Calculator Refactoring

### 目标 | Objective
将之前创建的计算器代码重构为模块化结构，演示模块系统的组织原理和可见性控制，提高代码的可维护性和扩展性。| Refactor previously created calculator code into a modular structure, demonstrating module system organization principles and visibility control, improving code maintainability and extensibility.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 如何在文件间组织模块结构？| How do we organize module structure across files?
   **答案 | Answer:** 使用mod声明模块，创建对应的.rs文件或目录/mod.rs
2. 什么时候使用pub(crate)而不是pub？| When do we use pub(crate) instead of pub?
   **答案 | Answer:** 当只想在当前crate内暴露，不想对外部crate暴露时
3. use语句应该放在文件的什么位置？| Where should use statements be placed in a file?
   **答案 | Answer:** 通常放在文件顶部，模块声明之后

### 步骤 | Steps
1. **项目结构设计 | Project Structure Design**：分析功能需求，设计合理的模块层次
2. **创建核心模块 | Create Core Modules**：将计算逻辑分离到独立模块
3. **实现可见性控制 | Implement Visibility Control**：使用适当的pub修饰符
4. **添加模块导入 | Add Module Imports**：使用use语句简化访问
5. **测试模块化结构 | Test Modular Structure**：验证重构后的功能完整性

### 示例代码 | Example Code
```rust
// src/main.rs - 程序入口 | Program entry point
mod calculator;
mod operations;
mod parser;
mod history;
mod errors;

use calculator::Calculator;
use std::io::{self, Write};

fn main() {
    println!("=== 模块化计算器 v2.0 ===");
    println!("Modular Calculator v2.0");
    println!("输入 'help' 查看帮助，'quit' 退出");
    println!("Enter 'help' for help, 'quit' to exit\n");
    
    let mut calc = Calculator::new();
    
    loop {
        print!("calc> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                if input == "quit" || input == "exit" {
                    println!("再见！| Goodbye!");
                    break;
                }
                
                if input == "help" {
                    print_help();
                    continue;
                }
                
                if input == "history" {
                    calc.show_history();
                    continue;
                }
                
                if input == "clear" {
                    calc.clear_history();
                    println!("历史记录已清空 | History cleared");
                    continue;
                }
                
                if input.is_empty() {
                    continue;
                }
                
                match calc.evaluate(input) {
                    Ok(result) => println!("结果 | Result: {}", result),
                    Err(e) => println!("错误 | Error: {}", e),
                }
            }
            Err(e) => {
                eprintln!("读取输入失败 | Failed to read input: {}", e);
                break;
            }
        }
    }
}

fn print_help() {
    println!("=== 帮助信息 | Help ===");
    println!("支持的操作 | Supported operations:");
    println!("  + 加法 | addition");
    println!("  - 减法 | subtraction");
    println!("  * 乘法 | multiplication");
    println!("  / 除法 | division");
    println!("  ^ 幂运算 | exponentiation");
    println!("  sqrt(x) 平方根 | square root");
    println!("  () 括号 | parentheses");
    println!("");
    println!("特殊命令 | Special commands:");
    println!("  help - 显示帮助 | show help");
    println!("  history - 显示历史 | show history");
    println!("  clear - 清空历史 | clear history");
    println!("  quit/exit - 退出程序 | quit program");
    println!("");
}

// src/calculator.rs - 计算器主结构 | Main calculator structure
use crate::parser::Parser;
use crate::history::History;
use crate::errors::CalculatorError;

pub struct Calculator {
    parser: Parser,
    history: History,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            history: History::new(50), // 保存最近50条记录 | Keep last 50 records
        }
    }
    
    pub fn evaluate(&mut self, expression: &str) -> Result<f64, CalculatorError> {
        let result = self.parser.parse_and_evaluate(expression)?;
        self.history.add_entry(expression.to_string(), result);
        Ok(result)
    }
    
    pub fn show_history(&self) {
        println!("=== 计算历史 | Calculation History ===");
        let entries = self.history.get_recent(10);
        if entries.is_empty() {
            println!("暂无历史记录 | No history available");
        } else {
            for (i, entry) in entries.iter().enumerate() {
                println!("{}. {} = {}", i + 1, entry.expression, entry.result);
            }
        }
        println!();
    }
    
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

// src/operations.rs - 数学运算模块 | Mathematical operations module
use crate::errors::CalculatorError;

pub struct Operations;

impl Operations {
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }
    
    pub fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }
    
    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }
    
    pub fn divide(a: f64, b: f64) -> Result<f64, CalculatorError> {
        if b.abs() < f64::EPSILON {
            Err(CalculatorError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
    
    pub fn power(base: f64, exponent: f64) -> Result<f64, CalculatorError> {
        if base == 0.0 && exponent < 0.0 {
            Err(CalculatorError::InvalidOperation("0的负次方未定义 | Negative power of 0 is undefined".to_string()))
        } else {
            let result = base.powf(exponent);
            if result.is_finite() {
                Ok(result)
            } else {
                Err(CalculatorError::Overflow)
            }
        }
    }
    
    pub fn sqrt(x: f64) -> Result<f64, CalculatorError> {
        if x < 0.0 {
            Err(CalculatorError::InvalidOperation("负数的平方根未定义 | Square root of negative number is undefined".to_string()))
        } else {
            Ok(x.sqrt())
        }
    }
    
    // 私有辅助函数 | Private helper functions
    pub(crate) fn is_close(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }
}

// src/parser.rs - 表达式解析模块 | Expression parsing module
use crate::operations::Operations;
use crate::errors::CalculatorError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LeftParen,
    RightParen,
    Sqrt,
    End,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
        }
    }
    
    pub fn parse_and_evaluate(&mut self, expression: &str) -> Result<f64, CalculatorError> {
        self.tokenize(expression)?;
        self.current = 0;
        self.parse_expression()
    }
    
    fn tokenize(&mut self, expression: &str) -> Result<(), CalculatorError> {
        self.tokens.clear();
        let chars: Vec<char> = expression.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            match chars[i] {
                ' ' | '\t' => {
                    i += 1;
                    continue;
                }
                '+' => self.tokens.push(Token::Plus),
                '-' => self.tokens.push(Token::Minus),
                '*' => self.tokens.push(Token::Multiply),
                '/' => self.tokens.push(Token::Divide),
                '^' => self.tokens.push(Token::Power),
                '(' => self.tokens.push(Token::LeftParen),
                ')' => self.tokens.push(Token::RightParen),
                c if c.is_ascii_digit() || c == '.' => {
                    let start = i;
                    while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                        i += 1;
                    }
                    let number_str: String = chars[start..i].iter().collect();
                    match number_str.parse::<f64>() {
                        Ok(num) => self.tokens.push(Token::Number(num)),
                        Err(_) => return Err(CalculatorError::InvalidNumber(number_str)),
                    }
                    continue;
                }
                c if c.is_alphabetic() => {
                    let start = i;
                    while i < chars.len() && chars[i].is_alphabetic() {
                        i += 1;
                    }
                    let word: String = chars[start..i].iter().collect();
                    match word.as_str() {
                        "sqrt" => self.tokens.push(Token::Sqrt),
                        _ => return Err(CalculatorError::UnknownFunction(word)),
                    }
                    continue;
                }
                c => return Err(CalculatorError::InvalidCharacter(c)),
            }
            i += 1;
        }
        
        self.tokens.push(Token::End);
        Ok(())
    }
    
    fn parse_expression(&mut self) -> Result<f64, CalculatorError> {
        self.parse_term()
    }
    
    fn parse_term(&mut self) -> Result<f64, CalculatorError> {
        let mut left = self.parse_factor()?;
        
        while self.current < self.tokens.len() {
            match &self.tokens[self.current] {
                Token::Plus => {
                    self.current += 1;
                    let right = self.parse_factor()?;
                    left = Operations::add(left, right);
                }
                Token::Minus => {
                    self.current += 1;
                    let right = self.parse_factor()?;
                    left = Operations::subtract(left, right);
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_factor(&mut self) -> Result<f64, CalculatorError> {
        let mut left = self.parse_power()?;
        
        while self.current < self.tokens.len() {
            match &self.tokens[self.current] {
                Token::Multiply => {
                    self.current += 1;
                    let right = self.parse_power()?;
                    left = Operations::multiply(left, right);
                }
                Token::Divide => {
                    self.current += 1;
                    let right = self.parse_power()?;
                    left = Operations::divide(left, right)?;
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_power(&mut self) -> Result<f64, CalculatorError> {
        let mut left = self.parse_primary()?;
        
        if self.current < self.tokens.len() {
            if let Token::Power = &self.tokens[self.current] {
                self.current += 1;
                let right = self.parse_power()?; // 右结合 | Right associative
                left = Operations::power(left, right)?;
            }
        }
        
        Ok(left)
    }
    
    fn parse_primary(&mut self) -> Result<f64, CalculatorError> {
        if self.current >= self.tokens.len() {
            return Err(CalculatorError::UnexpectedEndOfExpression);
        }
        
        match &self.tokens[self.current] {
            Token::Number(n) => {
                let result = *n;
                self.current += 1;
                Ok(result)
            }
            Token::Minus => {
                self.current += 1;
                let value = self.parse_primary()?;
                Ok(-value)
            }
            Token::Plus => {
                self.current += 1;
                self.parse_primary()
            }
            Token::LeftParen => {
                self.current += 1;
                let result = self.parse_expression()?;
                if self.current < self.tokens.len() && self.tokens[self.current] == Token::RightParen {
                    self.current += 1;
                    Ok(result)
                } else {
                    Err(CalculatorError::MismatchedParentheses)
                }
            }
            Token::Sqrt => {
                self.current += 1;
                if self.current < self.tokens.len() && self.tokens[self.current] == Token::LeftParen {
                    self.current += 1;
                    let value = self.parse_expression()?;
                    if self.current < self.tokens.len() && self.tokens[self.current] == Token::RightParen {
                        self.current += 1;
                        Operations::sqrt(value)
                    } else {
                        Err(CalculatorError::MismatchedParentheses)
                    }
                } else {
                    Err(CalculatorError::InvalidSyntax("sqrt需要括号 | sqrt requires parentheses".to_string()))
                }
            }
            _ => Err(CalculatorError::InvalidSyntax("意外的令牌 | Unexpected token".to_string())),
        }
    }
}

// src/history.rs - 历史记录模块 | History module
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub expression: String,
    pub result: f64,
    pub timestamp: std::time::SystemTime,
}

pub struct History {
    entries: VecDeque<HistoryEntry>,
    max_size: usize,
}

impl History {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_size),
            max_size,
        }
    }
    
    pub fn add_entry(&mut self, expression: String, result: f64) {
        let entry = HistoryEntry {
            expression,
            result,
            timestamp: std::time::SystemTime::now(),
        };
        
        if self.entries.len() >= self.max_size {
            self.entries.pop_front();
        }
        
        self.entries.push_back(entry);
    }
    
    pub fn get_recent(&self, count: usize) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .rev()
            .take(count)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }
    
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

// src/errors.rs - 错误类型模块 | Error types module
use std::fmt;

#[derive(Debug, Clone)]
pub enum CalculatorError {
    InvalidNumber(String),
    InvalidCharacter(char),
    UnknownFunction(String),
    DivisionByZero,
    InvalidOperation(String),
    Overflow,
    UnexpectedEndOfExpression,
    MismatchedParentheses,
    InvalidSyntax(String),
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::InvalidNumber(num) => write!(f, "无效数字: {} | Invalid number: {}", num, num),
            CalculatorError::InvalidCharacter(c) => write!(f, "无效字符: {} | Invalid character: {}", c, c),
            CalculatorError::UnknownFunction(func) => write!(f, "未知函数: {} | Unknown function: {}", func, func),
            CalculatorError::DivisionByZero => write!(f, "除零错误 | Division by zero"),
            CalculatorError::InvalidOperation(msg) => write!(f, "无效操作: {} | Invalid operation: {}", msg, msg),
            CalculatorError::Overflow => write!(f, "数值溢出 | Numeric overflow"),
            CalculatorError::UnexpectedEndOfExpression => write!(f, "表达式意外结束 | Unexpected end of expression"),
            CalculatorError::MismatchedParentheses => write!(f, "括号不匹配 | Mismatched parentheses"),
            CalculatorError::InvalidSyntax(msg) => write!(f, "语法错误: {} | Syntax error: {}", msg, msg),
        }
    }
}

impl std::error::Error for CalculatorError {}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确应用了模块系统组织代码？| Does the project correctly apply module system to organize code?
2. 可见性控制是否合理使用？| Is visibility control used appropriately?
3. 模块间的依赖关系是否清晰？| Are inter-module dependencies clear?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **模块可见性实验 | Module Visibility Experiment**
   - **练习描述 | Exercise Description:** 创建多层嵌套模块，测试不同pub修饰符的作用范围
   - **概念检查 | Concept Check:** 能否正确区分pub、pub(crate)、pub(super)的可见性范围？
   - **学习目标 | Learning Objective:** 深入理解可见性控制机制

2. **模块重构挑战 | Module Refactoring Challenge**
   - **练习描述 | Exercise Description:** 将一个包含多种功能的大文件重构为模块化结构
   - **概念检查 | Concept Check:** 如何识别应该分离的功能并确定模块边界？
   - **学习目标 | Learning Objective:** 提高代码组织和模块设计能力

3. **use语句优化练习 | use Statement Optimization Exercise**
   - **练习描述 | Exercise Description:** 优化项目中的import语句，使用嵌套导入和重命名
   - **概念检查 | Concept Check:** 什么情况下应该使用glob导入，什么时候应该避免？
   - **学习目标 | Learning Objective:** 学会高效管理模块导入

4. **库项目结构设计 | Library Project Structure Design**
   - **练习描述 | Exercise Description:** 设计一个数学计算库的模块结构，包含多个子领域
   - **概念检查 | Concept Check:** 如何设计既易于使用又便于维护的公共API？
   - **学习目标 | Learning Objective:** 培养大型项目的架构设计能力

5. **模块测试组织 | Module Testing Organization**
   - **练习描述 | Exercise Description:** 为模块化项目组织单元测试和集成测试
   - **概念检查 | Concept Check:** 测试模块应该如何组织以测试私有函数？
   - **学习目标 | Learning Objective:** 学会在模块化项目中组织测试

6. **条件编译与模块 | Conditional Compilation and Modules**
   - **练习描述 | Exercise Description:** 使用cfg属性在不同平台或功能下启用不同的模块
   - **概念检查 | Concept Check:** 如何使用feature gates控制模块的可用性？
   - **学习目标 | Learning Objective:** 掌握高级模块配置技巧

7. **模块文档生成 | Module Documentation Generation**
   - **练习描述 | Exercise Description:** 为模块化项目添加完整的文档注释并生成文档
   - **概念检查 | Concept Check:** 如何组织模块文档使其既全面又易于浏览？
   - **学习目标 | Learning Objective:** 提高项目文档编写和组织能力

## 学习资源 | Learning Resources
- [Rust官方文档 - 模块系统](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) | [Rust Official Documentation - Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust模块系统最佳实践指南](https://rust-lang.github.io/api-guidelines/organization.html) | [Rust Module System Best Practices Guide](https://rust-lang.github.io/api-guidelines/organization.html)
- [Rust项目结构组织建议](https://doc.rust-lang.org/cargo/guide/project-layout.html) | [Rust Project Structure Organization Recommendations](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- [可见性与隐私规则详解](https://doc.rust-lang.org/reference/visibility-and-privacy.html) | [Visibility and Privacy Rules Detailed Explanation](https://doc.rust-lang.org/reference/visibility-and-privacy.html)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解模块系统的层次结构和组织原则 | Understand hierarchical structure and organization principles of module system
- [ ] 掌握mod关键字创建和使用模块 | Master creating and using modules with mod keyword
- [ ] 熟练使用pub关键字控制可见性 | Proficiently use pub keyword to control visibility
- [ ] 掌握use语句简化路径访问 | Master use statements to simplify path access
- [ ] 理解文件模块组织方式 | Understand file module organization methods
- [ ] 完成模块化计算器重构项目 | Complete modular calculator refactoring project
- [ ] 正确回答所有CCQs问题 | Correctly answer all CCQ questions
- [ ] 理解模块间依赖关系设计原则 | Understand inter-module dependency design principles
- [ ] 掌握不同pub修饰符的使用场景 | Master usage scenarios of different pub modifiers
- [ ] 完成至少3个扩展练习 | Complete at least 3 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释模块系统的核心概念和最佳实践。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain core concepts and best practices of the module system to others.