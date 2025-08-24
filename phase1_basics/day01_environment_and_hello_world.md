# Rust入门 - 第1天：环境搭建与Hello World | Rust Introduction - Day 1: Environment Setup and Hello World

## 学习目标 | Learning Objectives
- 成功安装和配置Rust开发环境 | Successfully install and configure Rust development environment
- 理解Rust工具链的基本组件和用途 | Understand basic components and purposes of Rust toolchain
- 掌握Cargo项目管理工具的基本使用 | Master basic usage of Cargo project management tool
- 能够创建、编译和运行第一个Rust程序 | Be able to create, compile, and run the first Rust program
- 理解Rust项目的基本结构和文件组织 | Understand basic structure and file organization of Rust projects
- 配置和使用IDE进行Rust开发 | Configure and use IDE for Rust development

## 详细内容 | Detailed Content

### 1. Rust开发环境安装 | Rust Development Environment Installation (1小时 | 1 hour)

- **Rustup工具链管理器 | Rustup Toolchain Manager**
  
  **概念定义 | Concept Definition:**
  Rustup是Rust官方的工具链安装和管理工具，用于安装、更新和管理不同版本的Rust编译器和相关工具 | Rustup is Rust's official toolchain installer and manager, used to install, update, and manage different versions of the Rust compiler and related tools
  
  **核心特征 | Key Characteristics:**
  - 跨平台支持：支持Windows、macOS和Linux系统 | Cross-platform support: supports Windows, macOS, and Linux systems
  - 版本管理：可以管理多个Rust版本（stable、beta、nightly） | Version management: can manage multiple Rust versions (stable, beta, nightly)
  - 组件管理：自动安装和更新相关工具（rustc、cargo、rustfmt等） | Component management: automatically installs and updates related tools (rustc, cargo, rustfmt, etc.)
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Rustup是用来编译Rust代码的工具吗？| Is Rustup used to compile Rust code?
     **答案 | Answer:** 否 | No - Rustup是工具链管理器，rustc才是编译器 | Rustup is a toolchain manager, rustc is the compiler
  2. 一台电脑上可以同时安装多个版本的Rust吗？| Can multiple versions of Rust be installed on one computer simultaneously?
     **答案 | Answer:** 是 | Yes - Rustup支持多版本管理和切换 | Rustup supports multi-version management and switching
  3. 安装Rustup时会自动安装哪些核心组件？| Which core components are automatically installed with Rustup?
     **答案 | Answer:** rustc（编译器）、cargo（包管理器）、rustup（工具链管理器）和标准库 | rustc (compiler), cargo (package manager), rustup (toolchain manager), and standard library
  4. Stable、Beta和Nightly版本有什么区别？| What's the difference between Stable, Beta, and Nightly versions?
     **答案 | Answer:** Stable是稳定版，Beta是候选版，Nightly是每日构建的实验版本 | Stable is the stable release, Beta is release candidate, Nightly is experimental daily builds
  
  **代码示例与验证 | Code Examples and Verification:**
  ```bash
  # 下载并安装Rustup | Download and install Rustup
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  
  # Windows用户可以下载安装程序 | Windows users can download the installer
  # https://rustup.rs/
  
  # 验证安装 | Verify installation
  rustup --version
  rustc --version
  cargo --version
  
  # 更新工具链 | Update toolchain
  rustup update
  
  # 查看已安装的工具链 | View installed toolchains
  rustup show
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 安装成功后，运行 `rustc --version` 应该显示什么？| What should `rustc --version` display after successful installation?
    **答案 | Answer:** 显示Rust编译器版本信息，如"rustc 1.75.0" | Shows Rust compiler version info, like "rustc 1.75.0"
  - 如何检查当前激活的工具链？| How to check the currently active toolchain?
    **答案 | Answer:** 使用 `rustup show` 命令 | Use the `rustup show` command
  
  **常见误区检查 | Common Misconception Checks:**
  - Rustup和rustc是同一个工具吗？| Are Rustup and rustc the same tool?
    **答案 | Answer:** 不是，Rustup是管理器，rustc是编译器 | No, Rustup is the manager, rustc is the compiler
  - 必须手动安装cargo吗？| Must cargo be installed manually?
    **答案 | Answer:** 不需要，Rustup会自动安装cargo | No, Rustup automatically installs cargo

- **IDE配置 | IDE Configuration**
  
  **概念定义 | Concept Definition:**
  IDE（集成开发环境）为Rust开发提供语法高亮、代码补全、错误检测和调试功能 | IDE (Integrated Development Environment) provides syntax highlighting, code completion, error detection, and debugging features for Rust development
  
  **核心特征 | Key Characteristics:**
  - 语言服务器协议支持：使用rust-analyzer提供智能代码辅助 | Language Server Protocol support: uses rust-analyzer for intelligent code assistance
  - 实时错误检测：在编写代码时即时显示错误和警告 | Real-time error detection: shows errors and warnings while writing code
  - 集成终端：可以直接在IDE中运行Rust命令 | Integrated terminal: can run Rust commands directly in the IDE
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. rust-analyzer是Rust的官方语言服务器吗？| Is rust-analyzer the official Rust language server?
     **答案 | Answer:** 是 | Yes - 它是Rust官方支持的语言服务器实现 | It's the officially supported language server implementation for Rust
  2. 可以在没有IDE的情况下开发Rust程序吗？| Can Rust programs be developed without an IDE?
     **答案 | Answer:** 可以 | Yes - 可以使用文本编辑器和命令行工具 | Can use text editor and command-line tools
  3. VS Code需要安装什么扩展来支持Rust开发？| What extension needs to be installed in VS Code for Rust development?
     **答案 | Answer:** rust-analyzer扩展 | rust-analyzer extension
  4. IDE能够自动格式化Rust代码吗？| Can IDE automatically format Rust code?
     **答案 | Answer:** 可以 | Yes - 通过rustfmt工具实现自动格式化 | Through rustfmt tool for automatic formatting
  
  **代码示例与验证 | Code Examples and Verification:**
  ```bash
  # 安装VS Code后，安装rust-analyzer扩展
  # After installing VS Code, install rust-analyzer extension
  
  # 验证rustfmt是否可用 | Verify rustfmt availability
  rustfmt --version
  
  # 验证rust-analyzer是否可用 | Verify rust-analyzer availability  
  rust-analyzer --version
  
  # 创建测试文件来验证IDE功能 | Create test file to verify IDE functionality
  echo 'fn main(){println!("Hello, World!");}' > test.rs
  
  # 使用rustfmt格式化代码 | Format code using rustfmt
  rustfmt test.rs
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - VS Code中如何快速格式化Rust代码？| How to quickly format Rust code in VS Code?
    **答案 | Answer:** 使用 Shift+Alt+F (Windows/Linux) 或 Shift+Option+F (macOS) | Use Shift+Alt+F (Windows/Linux) or Shift+Option+F (macOS)
  - IDE显示红色波浪线表示什么？| What do red squiggly lines in IDE indicate?
    **答案 | Answer:** 语法错误或编译错误 | Syntax errors or compilation errors
  
  **常见误区检查 | Common Misconception Checks:**
  - IDE可以运行没有main函数的Rust文件吗？| Can IDE run Rust files without main function?
    **答案 | Answer:** 不可以，Rust程序需要main函数作为入口点 | No, Rust programs need main function as entry point
  - 必须使用特定的IDE才能开发Rust吗？| Must use a specific IDE to develop Rust?
    **答案 | Answer:** 不是，可以使用任何支持Rust的编辑器或IDE | No, can use any editor or IDE that supports Rust

### 2. Cargo项目管理系统 | Cargo Project Management System (1小时 | 1 hour)

- **Cargo基础概念 | Cargo Basic Concepts**
  
  **概念定义 | Concept Definition:**
  Cargo是Rust的官方构建工具和包管理器，负责项目创建、依赖管理、编译构建和测试运行 | Cargo is Rust's official build tool and package manager, responsible for project creation, dependency management, compilation building, and test execution
  
  **核心特征 | Key Characteristics:**
  - 项目模板：自动创建标准的Rust项目结构 | Project templates: automatically creates standard Rust project structure
  - 依赖管理：通过Cargo.toml文件管理第三方库依赖 | Dependency management: manages third-party library dependencies through Cargo.toml
  - 构建系统：智能增量编译，只编译修改的部分 | Build system: intelligent incremental compilation, only compiles modified parts
  - 包发布：支持向crates.io发布和分享包 | Package publishing: supports publishing and sharing packages to crates.io
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Cargo可以管理Rust项目的依赖关系吗？| Can Cargo manage dependencies for Rust projects?
     **答案 | Answer:** 可以 | Yes - 这是Cargo的核心功能之一 | This is one of Cargo's core functions
  2. Cargo.toml文件存储什么信息？| What information does Cargo.toml file store?
     **答案 | Answer:** 项目元数据、依赖关系、构建配置等 | Project metadata, dependencies, build configuration, etc.
  3. `cargo new` 命令会创建哪些文件和目录？| What files and directories does `cargo new` command create?
     **答案 | Answer:** Cargo.toml、src目录、main.rs文件和.gitignore | Cargo.toml, src directory, main.rs file, and .gitignore
  4. Cargo是否只能用于可执行程序项目？| Can Cargo only be used for executable program projects?
     **答案 | Answer:** 不是 | No - 还可以创建库项目（--lib参数） | Can also create library projects (--lib parameter)
  
  **代码示例与验证 | Code Examples and Verification:**
  ```bash
  # 创建新的二进制项目 | Create new binary project
  cargo new hello_world
  cd hello_world
  
  # 创建新的库项目 | Create new library project  
  cargo new my_library --lib
  
  # 查看项目结构 | View project structure
  tree hello_world
  # 或使用 ls -la 在Windows上 | or use ls -la on Windows
  
  # 编译项目 | Compile project
  cargo build
  
  # 编译并运行项目 | Compile and run project
  cargo run
  
  # 运行测试 | Run tests
  cargo test
  
  # 检查代码但不生成可执行文件 | Check code without generating executable
  cargo check
  
  # 清理构建产物 | Clean build artifacts
  cargo clean
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - `cargo build` 和 `cargo run` 有什么区别？| What's the difference between `cargo build` and `cargo run`?
    **答案 | Answer:** build只编译，run编译后还会执行程序 | build only compiles, run compiles and then executes the program
  - 编译后的可执行文件在哪个目录？| In which directory is the compiled executable file?
    **答案 | Answer:** target/debug/ 目录下 | In the target/debug/ directory
  
  **常见误区检查 | Common Misconception Checks:**
  - 必须在项目根目录才能运行cargo命令吗？| Must cargo commands be run from project root directory?
    **答案 | Answer:** 是的 | Yes - Cargo需要找到Cargo.toml文件 | Cargo needs to find the Cargo.toml file
  - `cargo new` 会覆盖已存在的目录吗？| Will `cargo new` overwrite an existing directory?
    **答案 | Answer:** 不会 | No - 如果目录已存在会报错 | Will error if directory already exists

### 3. 第一个Rust程序 | First Rust Program (1小时 | 1 hour)

- **Hello World程序结构 | Hello World Program Structure**
  
  **概念定义 | Concept Definition:**
  Hello World程序是学习编程语言的传统起点，展示了Rust程序的基本结构和语法要素 | Hello World program is the traditional starting point for learning programming languages, demonstrating basic structure and syntax elements of Rust programs
  
  **核心特征 | Key Characteristics:**
  - main函数：程序的入口点，每个可执行的Rust程序都必须有 | main function: program entry point, every executable Rust program must have one
  - println!宏：用于输出文本到控制台的宏，注意感叹号 | println! macro: macro for outputting text to console, note the exclamation mark
  - 花括号作用域：使用{}定义代码块和作用域 | Curly brace scope: uses {} to define code blocks and scope
  - 语句与表达式：语句以分号结尾，表达式求值返回结果 | Statements vs expressions: statements end with semicolon, expressions evaluate to return results
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 每个Rust可执行程序都必须有main函数吗？| Must every Rust executable program have a main function?
     **答案 | Answer:** 是 | Yes - main函数是程序执行的入口点 | main function is the program's entry point
  2. println!后面的感叹号有什么意义？| What's the significance of the exclamation mark after println!?
     **答案 | Answer:** 表示这是一个宏，不是普通函数 | Indicates this is a macro, not a regular function
  3. Rust代码必须以分号结尾吗？| Must Rust code end with semicolons?
     **答案 | Answer:** 语句必须，表达式可选 | Statements must, expressions optional
  4. 可以在一行写多个语句吗？| Can multiple statements be written on one line?
     **答案 | Answer:** 可以 | Yes - 用分号分隔 | Separated by semicolons
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 最基本的Hello World程序 | Most basic Hello World program
  fn main() {
      println!("Hello, world!");
  }
  
  // 带变量的版本 | Version with variables
  fn main() {
      let greeting = "Hello, Rust!";
      let name = "Developer";
      println!("{} Welcome, {}!", greeting, name);
  }
  
  // 多行输出示例 | Multi-line output example
  fn main() {
      println!("欢迎学习Rust编程语言！");
      println!("Welcome to Rust programming language!");
      println!("今天是学习的第一天。");
      println!("Today is the first day of learning.");
  }
  
  // 格式化输出示例 | Formatted output example
  fn main() {
      let language = "Rust";
      let year = 2024;
      println!("我正在学习 {} 编程语言，现在是 {} 年。", language, year);
      println!("I am learning {} programming language, it's year {}.", language, year);
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 删除main函数的花括号会发生什么？| What happens if you remove the curly braces from main function?
    **答案 | Answer:** 编译错误，函数体必须用花括号包围 | Compilation error, function body must be surrounded by curly braces
  - println!中的{}是什么作用？| What's the purpose of {} in println!?
    **答案 | Answer:** 占位符，用于插入变量值 | Placeholder for inserting variable values
  
  **常见误区检查 | Common Misconception Checks:**
  - 可以有多个main函数吗？| Can there be multiple main functions?
    **答案 | Answer:** 不可以 | No - 一个可执行程序只能有一个main函数 | One executable program can only have one main function
  - 忘记写分号会怎样？| What happens if you forget to write semicolons?
    **答案 | Answer:** 可能编译错误或改变语义 | May cause compilation error or change semantics

### 4. 项目结构理解 | Project Structure Understanding (30分钟 | 30 minutes)

- **Rust项目目录结构 | Rust Project Directory Structure**
  
  **概念定义 | Concept Definition:**
  标准的Rust项目有固定的目录结构，这种结构有助于代码组织和工具链的正确工作 | Standard Rust projects have a fixed directory structure that helps with code organization and proper toolchain operation
  
  **核心特征 | Key Characteristics:**
  - src目录：存放所有源代码文件 | src directory: stores all source code files
  - Cargo.toml：项目配置文件，描述项目元数据和依赖 | Cargo.toml: project configuration file, describes project metadata and dependencies
  - target目录：存放编译产生的文件 | target directory: stores files generated by compilation
  - Cargo.lock：锁定具体的依赖版本 | Cargo.lock: locks specific dependency versions
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. src目录是必须的吗？| Is the src directory required?
     **答案 | Answer:** 是 | Yes - Cargo默认在src目录查找源代码 | Cargo looks for source code in src directory by default
  2. 可以手动修改Cargo.lock文件吗？| Can Cargo.lock file be manually modified?
     **答案 | Answer:** 不建议 | Not recommended - 应该让Cargo自动管理 | Should let Cargo manage it automatically
  3. target目录需要提交到版本控制吗？| Should target directory be committed to version control?
     **答案 | Answer:** 不需要 | No - 这是构建产物，应该忽略 | These are build artifacts, should be ignored
  4. 库项目和二进制项目的入口文件分别是什么？| What are the entry files for library and binary projects respectively?
     **答案 | Answer:** lib.rs（库）和main.rs（二进制） | lib.rs (library) and main.rs (binary)
  
  **代码示例与验证 | Code Examples and Verification:**
  ```bash
  # 典型的Rust项目结构 | Typical Rust project structure
  my_project/
  ├── Cargo.toml          # 项目配置文件 | Project configuration
  ├── Cargo.lock          # 依赖版本锁定文件 | Dependency version lock
  ├── src/                # 源代码目录 | Source code directory
  │   ├── main.rs         # 主程序入口 | Main program entry
  │   └── lib.rs          # 库入口（如果是库项目）| Library entry (if library project)
  ├── tests/              # 集成测试目录 | Integration tests directory
  ├── examples/           # 示例代码目录 | Example code directory
  ├── benches/            # 性能测试目录 | Benchmark directory
  └── target/             # 构建产物目录 | Build artifacts directory
      ├── debug/          # 调试版本 | Debug version
      └── release/        # 发布版本 | Release version
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 如何创建一个库项目而不是可执行项目？| How to create a library project instead of executable project?
    **答案 | Answer:** 使用 `cargo new project_name --lib` | Use `cargo new project_name --lib`
  - 编译后在哪里找到可执行文件？| Where to find executable file after compilation?
    **答案 | Answer:** target/debug/ 或 target/release/ 目录 | target/debug/ or target/release/ directory

### 5. 基本Cargo命令 | Basic Cargo Commands (30分钟 | 30 minutes)

- **常用Cargo命令 | Common Cargo Commands**
  
  **概念定义 | Concept Definition:**
  Cargo提供了一系列命令来管理Rust项目的生命周期，从创建到构建、测试和发布 | Cargo provides a series of commands to manage the lifecycle of Rust projects, from creation to building, testing, and publishing
  
  **核心特征 | Key Characteristics:**
  - 增量编译：只重新编译修改过的代码部分 | Incremental compilation: only recompiles modified code parts
  - 并行构建：利用多核处理器加速编译过程 | Parallel building: utilizes multi-core processors to speed up compilation
  - 优化选项：提供debug和release两种构建模式 | Optimization options: provides debug and release build modes
  - 集成工具：集成格式化、测试、文档生成等工具 | Integrated tools: integrates formatting, testing, documentation generation tools
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. `cargo check` 和 `cargo build` 的区别是什么？| What's the difference between `cargo check` and `cargo build`?
     **答案 | Answer:** check只检查语法不生成可执行文件，build会生成可执行文件 | check only checks syntax without generating executable, build generates executable
  2. 如何构建优化版本？| How to build optimized version?
     **答案 | Answer:** 使用 `cargo build --release` | Use `cargo build --release`
  3. `cargo clean` 命令的作用是什么？| What does `cargo clean` command do?
     **答案 | Answer:** 清理target目录中的构建产物 | Cleans build artifacts in target directory
  4. 如何只运行测试而不运行主程序？| How to run only tests without running main program?
     **答案 | Answer:** 使用 `cargo test` 命令 | Use `cargo test` command
  
  **代码示例与验证 | Code Examples and Verification:**
  ```bash
  # 创建新项目 | Create new project
  cargo new hello_world
  cd hello_world
  
  # 快速语法检查（最快） | Quick syntax check (fastest)
  cargo check
  
  # 编译debug版本 | Compile debug version
  cargo build
  
  # 编译并运行 | Compile and run
  cargo run
  
  # 编译优化版本（更慢但性能更好）| Compile optimized version (slower but better performance)
  cargo build --release
  cargo run --release
  
  # 运行测试 | Run tests
  cargo test
  
  # 生成文档 | Generate documentation
  cargo doc --open
  
  # 格式化代码 | Format code
  cargo fmt
  
  # 代码检查（lint）| Code linting
  cargo clippy
  
  # 清理构建产物 | Clean build artifacts
  cargo clean
  
  # 显示依赖树 | Show dependency tree
  cargo tree
  ```

### 6. 开发环境验证 | Development Environment Verification (30分钟 | 30 minutes)

- **环境完整性检查 | Environment Integrity Check**
  
  **概念定义 | Concept Definition:**
  验证开发环境是否正确安装和配置，确保所有必要的工具都能正常工作 | Verify that the development environment is correctly installed and configured, ensuring all necessary tools work properly
  
  **综合概念检查 | Comprehensive Concept Check:**
  1. 完整的Rust开发环境包含哪些核心组件？| What core components does a complete Rust development environment include?
     **答案 | Answer:** rustc（编译器）、cargo（包管理器）、rustup（工具链管理器）、IDE/编辑器 | rustc (compiler), cargo (package manager), rustup (toolchain manager), IDE/editor
  2. 如何验证Rust环境安装成功？| How to verify successful Rust environment installation?
     **答案 | Answer:** 运行版本检查命令并创建测试项目 | Run version check commands and create test project
  3. 开发环境配置完成后的第一步应该做什么？| What should be the first step after development environment configuration?
     **答案 | Answer:** 创建Hello World项目验证环境 | Create Hello World project to verify environment
  4. 如何保持开发环境的更新？| How to keep development environment updated?
     **答案 | Answer:** 定期运行 `rustup update` 更新工具链 | Regularly run `rustup update` to update toolchain
  5. IDE配置正确的标志是什么？| What are the signs of correct IDE configuration?
     **答案 | Answer:** 语法高亮、代码补全、错误提示正常工作 | Syntax highlighting, code completion, error hints work properly

## 实践项目：Rust开发环境完整设置 | Practical Project: Complete Rust Development Environment Setup

### 目标 | Objective
通过创建一个简单但功能完整的Rust项目，验证开发环境的正确安装和配置，同时练习基本的Cargo操作和Rust语法 | Verify correct installation and configuration of development environment by creating a simple but functionally complete Rust project, while practicing basic Cargo operations and Rust syntax

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. Cargo如何管理Rust项目？| How does Cargo manage Rust projects?
   **答案 | Answer:** 通过Cargo.toml文件管理元数据和依赖，提供构建、运行、测试等命令
2. main函数在Rust程序中的作用？| What's the role of main function in Rust programs?
   **答案 | Answer:** 作为程序执行的入口点，每个可执行程序必须有且只能有一个main函数
3. println!宏与普通函数的区别？| What's the difference between println! macro and regular functions?
   **答案 | Answer:** 宏在编译时展开，感叹号是宏的标识符，功能更强大但编译时处理

### 步骤 | Steps
1. **环境安装验证** | Environment Installation Verification
   - 安装Rustup并验证版本
   - 配置IDE和rust-analyzer插件
   - 测试基本命令功能

2. **创建项目** | Create Project
   - 使用Cargo创建新项目
   - 理解项目结构
   - 编写多个版本的Hello World程序

3. **功能扩展** | Feature Extension
   - 添加用户交互
   - 实现简单的计算功能
   - 练习格式化输出

4. **工具使用** | Tool Usage
   - 使用不同的Cargo命令
   - 练习代码格式化
   - 生成项目文档

5. **环境优化** | Environment Optimization
   - 配置IDE快捷键
   - 设置代码片段
   - 优化开发工作流

### 示例代码 | Example Code
```rust
"""
互动式欢迎程序 | Interactive Welcome Program
演示Rust环境设置和基本语法的综合应用 | Demonstrates comprehensive application of Rust environment setup and basic syntax

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- Cargo项目管理 | Cargo project management
- 基本Rust语法 | Basic Rust syntax
- 用户交互 | User interaction
- 格式化输出 | Formatted output
"""

use std::io;

fn main() {
    // 显示欢迎信息 | Display welcome message
    println!("🦀 欢迎来到Rust编程世界！| Welcome to Rust Programming World!");
    println!("📚 这是您的第一个交互式Rust程序 | This is your first interactive Rust program");
    
    // 获取用户姓名 | Get user name
    println!("\n请输入您的姓名 | Please enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("读取输入失败 | Failed to read input");
    
    // 去除换行符 | Remove newline
    let name = name.trim();
    
    // 个性化问候 | Personalized greeting
    println!("\n你好，{}！| Hello, {}!", name, name);
    
    // 显示环境信息 | Display environment info
    println!("🔧 您的Rust开发环境已成功配置！| Your Rust development environment is successfully configured!");
    println!("📈 编程学习的第一步已经完成 | The first step of programming learning is completed");
    
    // 简单的交互计算 | Simple interactive calculation
    println!("\n让我们做一个简单的计算 | Let's do a simple calculation");
    println!("请输入第一个数字 | Please enter the first number:");
    
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("读取输入失败 | Failed to read input");
    
    println!("请输入第二个数字 | Please enter the second number:");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("读取输入失败 | Failed to read input");
    
    // 解析数字 | Parse numbers
    let num1: f64 = input1.trim().parse().expect("请输入有效数字 | Please enter valid number");
    let num2: f64 = input2.trim().parse().expect("请输入有效数字 | Please enter valid number");
    
    // 执行计算 | Perform calculations
    println!("\n🧮 计算结果 | Calculation Results:");
    println!("{} + {} = {}", num1, num2, num1 + num2);
    println!("{} - {} = {}", num1, num2, num1 - num2);
    println!("{} × {} = {}", num1, num2, num1 * num2);
    if num2 != 0.0 {
        println!("{} ÷ {} = {:.2}", num1, num2, num1 / num2);
    } else {
        println!("除数不能为零 | Division by zero is not allowed");
    }
    
    // 结束信息 | Closing message
    println!("\n🎉 恭喜！您已成功运行第一个Rust程序！| Congratulations! You have successfully run your first Rust program!");
    println!("🚀 准备好开始Rust编程之旅了吗？| Ready to start your Rust programming journey?");
}
```

### 项目完成检查 | Project Completion Check
1. 项目能否成功编译和运行？| Can the project compile and run successfully?
2. 所有用户交互功能是否正常工作？| Do all user interaction features work properly?
3. IDE是否提供正确的语法高亮和错误提示？| Does IDE provide correct syntax highlighting and error hints?
4. 代码格式化功能是否正常？| Does code formatting function work properly?
5. 能否使用不同的Cargo命令操作项目？| Can different Cargo commands operate the project?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **环境管理练习 | Environment Management Exercise**
   - **练习描述 | Exercise Description:** 练习安装不同版本的Rust工具链，学会在stable、beta、nightly之间切换
   - **概念检查 | Concept Check:** 多工具链管理的优势是什么？什么时候需要使用nightly版本？
   - **学习目标 | Learning Objective:** 掌握Rust版本管理和工具链切换

2. **项目结构探索练习 | Project Structure Exploration Exercise**
   - **练习描述 | Exercise Description:** 创建不同类型的Rust项目（库、二进制、工作空间），理解各自的特点和用途
   - **概念检查 | Concept Check:** 库项目和二进制项目的主要区别是什么？工作空间解决什么问题？
   - **学习目标 | Learning Objective:** 深入理解Rust项目组织结构

3. **Cargo功能深化练习 | Cargo Functionality Deepening Exercise**
   - **练习描述 | Exercise Description:** 探索Cargo的高级功能，如自定义构建脚本、特性标志、发布配置等
   - **概念检查 | Concept Check:** 什么是特性标志？如何使用构建脚本？
   - **学习目标 | Learning Objective:** 掌握Cargo的高级项目管理功能

4. **开发工具集成练习 | Development Tools Integration Exercise**
   - **练习描述 | Exercise Description:** 配置和使用rust-analyzer、rustfmt、clippy等工具，优化开发体验
   - **概念检查 | Concept Check:** 这些工具分别解决什么问题？如何配置它们？
   - **学习目标 | Learning Objective:** 建立高效的Rust开发环境

5. **跨平台开发练习 | Cross-Platform Development Exercise**
   - **练习描述 | Exercise Description:** 学习如何为不同平台编译Rust程序，理解目标三元组概念
   - **概念检查 | Concept Check:** 什么是目标三元组？如何进行交叉编译？
   - **学习目标 | Learning Objective:** 理解Rust的跨平台编译能力

6. **错误诊断练习 | Error Diagnosis Exercise**
   - **练习描述 | Exercise Description:** 故意引入各种常见错误，学会阅读和理解Rust编译器错误信息
   - **概念检查 | Concept Check:** Rust编译器错误信息的结构是什么？如何快速定位问题？
   - **学习目标 | Learning Objective:** 提高调试和问题解决能力

7. **环境文档化练习 | Environment Documentation Exercise**
   - **练习描述 | Exercise Description:** 创建开发环境设置的文档和脚本，帮助团队成员快速搭建环境
   - **概念检查 | Concept Check:** 良好的环境文档应该包含哪些信息？如何自动化环境设置？
   - **学习目标 | Learning Objective:** 学会分享和复现开发环境

## 学习资源 | Learning Resources
- [Rust官方文档 - 安装指南 | Rust Official Documentation - Installation Guide](https://www.rust-lang.org/tools/install)
- [Cargo指南 | The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust编程语言入门 | The Rust Programming Language](https://doc.rust-lang.org/book/)
- [rust-analyzer用户手册 | rust-analyzer User Manual](https://rust-analyzer.github.io/)
- [Rustup文档 | Rustup Documentation](https://rust-lang.github.io/rustup/)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 成功安装Rustup工具链 | Successfully installed Rustup toolchain
- [ ] 验证rustc和cargo版本信息 | Verified rustc and cargo version info
- [ ] 配置IDE和rust-analyzer扩展 | Configured IDE and rust-analyzer extension
- [ ] 创建并运行Hello World项目 | Created and ran Hello World project
- [ ] 理解Rust项目目录结构 | Understood Rust project directory structure
- [ ] 掌握基本Cargo命令使用 | Mastered basic Cargo command usage
- [ ] 能够编写简单的交互式程序 | Can write simple interactive programs
- [ ] 完成实践项目并验证功能 | Completed practical project and verified functionality
- [ ] 环境配置文档化和备份 | Documented and backed up environment configuration
- [ ] 至少完成3个扩展练习 | Completed at least 3 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释Rust开发环境的组成部分、Cargo项目管理的核心概念，以及Rust程序的基本结构。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain the components of Rust development environment, core concepts of Cargo project management, and basic structure of Rust programs to others.