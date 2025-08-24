# Rust入门 - 第17天：自定义错误类型 | Rust Introduction - Day 17: Custom Error Types

## 学习目标 | Learning Objectives
- 理解为什么需要自定义错误类型 | Understand why custom error types are needed
- 学会定义自定义错误枚举 | Learn to define custom error enums
- 掌握错误类型转换和传播 | Master error type conversion and propagation
- 实现Error trait和Display trait | Implement Error trait and Display trait
- 理解错误链和错误上下文 | Understand error chains and error context
- 构建完整的错误处理系统 | Build complete error handling systems

## 详细内容 | Detailed Content

### 1. 自定义错误类型基础 | Custom Error Type Basics (1小时 | 1 hour)

- **自定义错误枚举 | Custom Error Enums**
  
  **概念定义 | Concept Definition:**
  自定义错误类型是用户定义的错误枚举，用于表示程序中可能出现的特定错误情况，提供比标准错误类型更精确和有意义的错误信息。| Custom error types are user-defined error enums that represent specific error conditions in programs, providing more precise and meaningful error information than standard error types.
  
  **核心特征 | Key Characteristics:**
  - 类型安全：编译时检查所有可能的错误情况 | Type safety: compile-time checking of all possible error conditions
  - 语义清晰：每种错误都有明确的含义和上下文 | Semantic clarity: each error has clear meaning and context
  - 可组合：可以包含其他错误类型作为变体 | Composable: can contain other error types as variants
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 自定义错误类型必须实现哪个trait才能与Result<T,E>配合使用？| Which trait must custom error types implement to work with Result<T,E>?
     **答案 | Answer:** 不必须实现特定trait，但通常实现Error trait | No specific trait is required, but Error trait is typically implemented
  2. 错误枚举的每个变体可以携带额外数据吗？| Can each variant of an error enum carry additional data?  
     **答案 | Answer:** 可以 | Yes - 可以携带字符串、其他错误类型或结构化数据 | can carry strings, other error types, or structured data
  3. 自定义错误类型相比字符串错误有什么优势？| What advantages do custom error types have over string errors?
     **答案 | Answer:** 类型安全、编译时检查、可模式匹配 | Type safety, compile-time checking, pattern matching capability
  4. 错误枚举变体的命名应该遵循什么原则？| What principles should error enum variant naming follow?
     **答案 | Answer:** 描述性、一致性、表达错误原因而非症状 | Descriptive, consistent, express cause rather than symptoms
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 定义自定义错误类型 | Define custom error type
  #[derive(Debug, Clone)]
  enum CalculatorError {
      DivisionByZero,           // 除零错误 | Division by zero error
      InvalidOperation(String), // 无效操作 | Invalid operation
      ParseError(String),      // 解析错误 | Parse error
      Overflow,                // 数值溢出 | Numeric overflow
  }
  
  // 使用自定义错误类型的函数 | Function using custom error type
  fn divide(a: f64, b: f64) -> Result<f64, CalculatorError> {
      if b == 0.0 {
          Err(CalculatorError::DivisionByZero)
      } else {
          Ok(a / b)
      }
  }
  
  fn parse_number(s: &str) -> Result<f64, CalculatorError> {
      s.parse()
          .map_err(|_| CalculatorError::ParseError(format!("Cannot parse '{}'", s)))
  }
  
  fn main() {
      // 测试除零错误 | Test division by zero error
      match divide(10.0, 0.0) {
          Ok(result) => println!("结果: {}", result),
          Err(CalculatorError::DivisionByZero) => println!("错误: 不能除零"),
          Err(e) => println!("其他错误: {:?}", e),
      }
      
      // 测试解析错误 | Test parse error
      match parse_number("abc") {
          Ok(num) => println!("解析成功: {}", num),
          Err(CalculatorError::ParseError(msg)) => println!("解析错误: {}", msg),
          Err(e) => println!("其他错误: {:?}", e),
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码会输出什么？| What will this code output?
    **答案 | Answer:** "错误: 不能除零" 和 "解析错误: Cannot parse 'abc'"
  - 如果改变divide函数返回Ok(f64::INFINITY)会发生什么？| What happens if we change the divide function to return Ok(f64::INFINITY)?
    **答案 | Answer:** 不会触发错误，但会返回无穷大值 | Won't trigger error but will return infinity value
  
  **常见误区检查 | Common Misconception Checks:**
  - 自定义错误类型必须实现std::error::Error trait吗？| Must custom error types implement std::error::Error trait?
    **答案 | Answer:** 不必须，但建议实现以获得更好的互操作性 | Not required, but recommended for better interoperability
  - 错误枚举变体可以包含相同类型的数据吗？| Can error enum variants contain the same type of data?
    **答案 | Answer:** 可以，但应该有不同的语义含义 | Yes, but they should have different semantic meanings

### 2. 实现Error和Display trait | Implementing Error and Display Traits (1小时 | 1 hour)

- **标准错误trait实现 | Standard Error Trait Implementation**
  
  **概念定义 | Concept Definition:**
  Error trait是Rust标准库定义的错误处理接口，Display trait用于格式化错误信息，实现这些trait使自定义错误类型与标准错误处理机制完全兼容。| Error trait is the error handling interface defined in Rust's standard library, Display trait is used for formatting error messages, implementing these traits makes custom error types fully compatible with standard error handling mechanisms.
  
  **核心特征 | Key Characteristics:**
  - Display trait：定义错误的用户友好显示格式 | Display trait: defines user-friendly display format for errors
  - Error trait：提供错误源追踪和描述 | Error trait: provides error source tracking and description
  - 自动转换：实现后可自动转换为Box<dyn Error> | Automatic conversion: enables automatic conversion to Box<dyn Error>
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Display trait的fmt方法返回什么类型？| What type does the fmt method of Display trait return?
     **答案 | Answer:** Result<(), fmt::Error> - 格式化结果 | formatting result
  2. Error trait需要实现哪些必需方法？| What required methods must Error trait implement?
     **答案 | Answer:** 在新版本中无必需方法，都有默认实现 | No required methods in newer versions, all have default implementations
  3. source()方法的作用是什么？| What is the purpose of the source() method?
     **答案 | Answer:** 返回导致当前错误的底层错误 | Returns the underlying error that caused the current error
  4. 实现Error trait有什么好处？| What are the benefits of implementing Error trait?
     **答案 | Answer:** 与标准库集成、错误链支持、自动类型转换 | Standard library integration, error chain support, automatic type conversion
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::fmt;
  use std::error::Error;
  
  // 自定义错误类型 | Custom error type
  #[derive(Debug, Clone)]
  enum FileProcessError {
      IoError(String),
      ParseError { line: usize, message: String },
      ValidationError(String),
      ConfigError { key: String, expected: String },
  }
  
  // 实现Display trait | Implement Display trait
  impl fmt::Display for FileProcessError {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          match self {
              FileProcessError::IoError(msg) => 
                  write!(f, "IO错误: {}", msg),
              FileProcessError::ParseError { line, message } => 
                  write!(f, "解析错误 (第{}行): {}", line, message),
              FileProcessError::ValidationError(msg) => 
                  write!(f, "验证错误: {}", msg),
              FileProcessError::ConfigError { key, expected } => 
                  write!(f, "配置错误: 键'{}', 期望类型'{}'", key, expected),
          }
      }
  }
  
  // 实现Error trait | Implement Error trait
  impl Error for FileProcessError {
      fn description(&self) -> &str {
          match self {
              FileProcessError::IoError(_) => "文件IO操作失败",
              FileProcessError::ParseError { .. } => "文件解析失败",
              FileProcessError::ValidationError(_) => "数据验证失败",
              FileProcessError::ConfigError { .. } => "配置参数错误",
          }
      }
  }
  
  // 使用自定义错误的函数 | Function using custom error
  fn process_config_file(content: &str) -> Result<(), FileProcessError> {
      if content.is_empty() {
          return Err(FileProcessError::IoError("文件为空".to_string()));
      }
      
      for (line_num, line) in content.lines().enumerate() {
          if line.trim().is_empty() {
              continue;
          }
          
          if !line.contains('=') {
              return Err(FileProcessError::ParseError {
                  line: line_num + 1,
                  message: "缺少等号分隔符".to_string(),
              });
          }
          
          let parts: Vec<&str> = line.split('=').collect();
          if parts.len() != 2 {
              return Err(FileProcessError::ConfigError {
                  key: parts[0].to_string(),
                  expected: "key=value格式".to_string(),
              });
          }
      }
      
      Ok(())
  }
  
  fn main() {
      let test_configs = vec![
          "",
          "key1=value1\nkey2\nkey3=value3",
          "key1=value1=extra",
          "key1=value1\nkey2=value2",
      ];
      
      for (i, config) in test_configs.iter().enumerate() {
          println!("测试配置 {}: ", i + 1);
          match process_config_file(config) {
              Ok(()) => println!("✓ 配置有效"),
              Err(e) => {
                  println!("✗ {}", e);  // 使用Display trait
                  println!("  描述: {}", e.description()); // 使用Error trait
              }
          }
          println!();
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - Display和Error trait实现后的主要优势是什么？| What are the main advantages after implementing Display and Error traits?
    **答案 | Answer:** 标准化错误显示、与生态系统兼容、支持错误链 | Standardized error display, ecosystem compatibility, error chain support
  - description()方法与Display trait有什么区别？| What's the difference between description() method and Display trait?
    **答案 | Answer:** description()提供简短描述，Display提供格式化显示 | description() provides brief description, Display provides formatted display

### 3. 错误类型转换 | Error Type Conversion (1小时 | 1 hour)

- **From trait实现与错误转换 | From Trait Implementation and Error Conversion**
  
  **概念定义 | Concept Definition:**
  错误类型转换允许将一种错误类型转换为另一种错误类型，通过实现From trait可以实现自动错误转换，简化错误处理和传播。| Error type conversion allows converting one error type to another, implementing From trait enables automatic error conversion, simplifying error handling and propagation.
  
  **核心特征 | Key Characteristics:**
  - 自动转换：?操作符自动调用From::from进行转换 | Automatic conversion: ? operator automatically calls From::from for conversion
  - 类型安全：编译时保证转换的正确性 | Type safety: compile-time guarantee of conversion correctness
  - 组合性：可以构建复杂的错误类型层次结构 | Composability: can build complex error type hierarchies
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 实现From<A> for B trait后，是否自动获得Into<B> for A？| After implementing From<A> for B trait, do you automatically get Into<B> for A?
     **答案 | Answer:** 是的 | Yes - 标准库提供了blanket implementation
  2. ?操作符在遇到不匹配的错误类型时会做什么？| What does the ? operator do when it encounters mismatched error types?
     **答案 | Answer:** 尝试使用From trait进行类型转换 | Attempts to use From trait for type conversion
  3. 可以为同一个目标类型实现多个From trait吗？| Can you implement multiple From traits for the same target type?
     **答案 | Answer:** 可以，只要源类型不同 | Yes, as long as the source types are different
  4. map_err和From trait转换有什么区别？| What's the difference between map_err and From trait conversion?
     **答案 | Answer:** map_err手动转换，From trait自动转换 | map_err is manual conversion, From trait is automatic conversion
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::fmt;
  use std::error::Error;
  use std::num::ParseIntError;
  use std::io;
  
  // 应用层错误类型 | Application-level error type
  #[derive(Debug)]
  enum AppError {
      Io(io::Error),
      Parse(ParseIntError),
      Custom(String),
      Network { code: u32, message: String },
  }
  
  impl fmt::Display for AppError {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          match self {
              AppError::Io(err) => write!(f, "IO错误: {}", err),
              AppError::Parse(err) => write!(f, "解析错误: {}", err),
              AppError::Custom(msg) => write!(f, "应用错误: {}", msg),
              AppError::Network { code, message } => 
                  write!(f, "网络错误 [{}]: {}", code, message),
          }
      }
  }
  
  impl Error for AppError {
      fn source(&self) -> Option<&(dyn Error + 'static)> {
          match self {
              AppError::Io(err) => Some(err),
              AppError::Parse(err) => Some(err),
              AppError::Custom(_) => None,
              AppError::Network { .. } => None,
          }
      }
  }
  
  // 实现From trait进行自动转换 | Implement From trait for automatic conversion
  impl From<io::Error> for AppError {
      fn from(err: io::Error) -> Self {
          AppError::Io(err)
      }
  }
  
  impl From<ParseIntError> for AppError {
      fn from(err: ParseIntError) -> Self {
          AppError::Parse(err)
      }
  }
  
  impl From<&str> for AppError {
      fn from(msg: &str) -> Self {
          AppError::Custom(msg.to_string())
      }
  }
  
  // 演示自动错误转换的函数 | Functions demonstrating automatic error conversion
  fn read_number_from_string(s: &str) -> Result<i32, AppError> {
      let number: i32 = s.parse()?; // ParseIntError自动转换为AppError
      Ok(number)
  }
  
  fn read_file_content(filename: &str) -> Result<String, AppError> {
      std::fs::read_to_string(filename).map_err(AppError::from) // 显式转换
  }
  
  fn process_data(data: &str) -> Result<i32, AppError> {
      if data.is_empty() {
          return Err("数据不能为空".into()); // &str自动转换为AppError
      }
      
      let number = read_number_from_string(data)?; // 错误自动传播和转换
      
      if number < 0 {
          Err(AppError::Custom("数字不能为负数".to_string()))
      } else {
          Ok(number * 2)
      }
  }
  
  // 网络操作示例 | Network operation example  
  fn simulate_network_request(url: &str) -> Result<String, AppError> {
      if url.starts_with("https://") {
          Ok("模拟响应数据".to_string())
      } else {
          Err(AppError::Network {
              code: 400,
              message: "不安全的URL".to_string(),
          })
      }
  }
  
  fn main() {
      // 测试不同类型的错误转换 | Test different types of error conversion
      let test_cases = vec![
          ("123", true),
          ("abc", false),
          ("", false),
          ("-456", false),
      ];
      
      for (input, should_succeed) in test_cases {
          println!("处理输入: '{}'", input);
          match process_data(input) {
              Ok(result) => {
                  if should_succeed {
                      println!("✓ 成功: {}", result);
                  } else {
                      println!("? 意外成功: {}", result);
                  }
              }
              Err(e) => {
                  if !should_succeed {
                      println!("✓ 预期错误: {}", e);
                      if let Some(source) = e.source() {
                          println!("  -> 错误源: {}", source);
                      }
                  } else {
                      println!("? 意外错误: {}", e);
                  }
              }
          }
          println!();
      }
      
      // 测试网络错误 | Test network error
      match simulate_network_request("http://example.com") {
          Ok(data) => println!("网络响应: {}", data),
          Err(e) => println!("网络错误: {}", e),
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - ?操作符如何知道要转换错误类型？| How does the ? operator know to convert error types?
    **答案 | Answer:** 通过From trait的实现进行类型推断和转换 | Through From trait implementation for type inference and conversion
  - source()方法在错误链中的作用是什么？| What is the role of the source() method in error chains?
    **答案 | Answer:** 提供错误的根本原因，支持错误追踪 | Provides root cause of errors, supports error tracking

### 4. 错误上下文和链式错误 | Error Context and Chained Errors (1.5小时 | 1.5 hours)

- **构建错误上下文 | Building Error Context**
  
  **概念定义 | Concept Definition:**
  错误上下文为错误提供额外的背景信息，错误链将多个相关错误连接起来，形成完整的错误传播路径，帮助开发者快速定位和解决问题。| Error context provides additional background information for errors, error chains connect multiple related errors to form complete error propagation paths, helping developers quickly locate and resolve issues.
  
  **核心特征 | Key Characteristics:**
  - 上下文信息：为错误添加发生时的环境信息 | Context information: adds environmental information when errors occur
  - 错误链：保持错误的完整传播历史 | Error chain: maintains complete error propagation history
  - 调试友好：提供丰富的错误调试信息 | Debug-friendly: provides rich error debugging information
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 错误上下文应该包含什么类型的信息？| What type of information should error context contain?
     **答案 | Answer:** 操作描述、输入参数、时间戳、调用路径 | Operation description, input parameters, timestamps, call paths
  2. 如何在不丢失原始错误的情况下添加上下文？| How to add context without losing the original error?
     **答案 | Answer:** 将原始错误包装在新错误类型中，实现source()方法 | Wrap original error in new error type, implement source() method
  3. 错误链的最大深度有限制吗？| Is there a limit to the maximum depth of error chains?
     **答案 | Answer:** 没有硬性限制，但过深的链可能影响性能 | No hard limit, but overly deep chains may impact performance
  4. 什么时候应该添加错误上下文？| When should error context be added?
     **答案 | Answer:** 跨越抽象边界或需要额外信息理解错误时 | When crossing abstraction boundaries or additional information is needed to understand the error
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::fmt;
  use std::error::Error;
  use std::fs;
  use std::path::Path;
  
  // 带上下文的错误类型 | Error type with context
  #[derive(Debug)]
  struct ContextualError {
      message: String,
      operation: String,
      source: Option<Box<dyn Error + Send + Sync>>,
  }
  
  impl fmt::Display for ContextualError {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "{}: {}", self.operation, self.message)
      }
  }
  
  impl Error for ContextualError {
      fn source(&self) -> Option<&(dyn Error + 'static)> {
          self.source.as_ref().map(|e| e.as_ref())
      }
  }
  
  impl ContextualError {
      fn new(operation: impl Into<String>, message: impl Into<String>) -> Self {
          ContextualError {
              operation: operation.into(),
              message: message.into(),
              source: None,
          }
      }
      
      fn with_source<E>(mut self, source: E) -> Self 
      where
          E: Error + Send + Sync + 'static,
      {
          self.source = Some(Box::new(source));
          self
      }
  }
  
  // 配置管理错误类型 | Configuration management error type
  #[derive(Debug)]
  enum ConfigError {
      FileNotFound(String),
      InvalidFormat { file: String, line: usize },
      MissingKey(String),
      TypeError { key: String, expected: String, actual: String },
  }
  
  impl fmt::Display for ConfigError {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          match self {
              ConfigError::FileNotFound(file) => write!(f, "配置文件未找到: {}", file),
              ConfigError::InvalidFormat { file, line } => 
                  write!(f, "配置文件格式错误: {} (行 {})", file, line),
              ConfigError::MissingKey(key) => write!(f, "缺少必需的配置项: {}", key),
              ConfigError::TypeError { key, expected, actual } => 
                  write!(f, "配置项类型错误: {} (期望: {}, 实际: {})", key, expected, actual),
          }
      }
  }
  
  impl Error for ConfigError {}
  
  // 配置管理器 | Configuration manager
  struct ConfigManager {
      config_path: String,
  }
  
  impl ConfigManager {
      fn new(path: impl Into<String>) -> Self {
          ConfigManager {
              config_path: path.into(),
          }
      }
      
      fn load_config(&self) -> Result<Vec<(String, String)>, ContextualError> {
          // 尝试读取文件 | Try to read file
          let content = fs::read_to_string(&self.config_path)
              .map_err(|e| {
                  ContextualError::new(
                      format!("读取配置文件 '{}'", self.config_path),
                      "无法访问配置文件"
                  ).with_source(e)
              })?;
          
          // 解析配置内容 | Parse configuration content
          self.parse_config(&content)
              .map_err(|e| {
                  ContextualError::new(
                      format!("解析配置文件 '{}'", self.config_path),
                      "配置文件格式不正确"
                  ).with_source(e)
              })
      }
      
      fn parse_config(&self, content: &str) -> Result<Vec<(String, String)>, ConfigError> {
          let mut config = Vec::new();
          
          for (line_num, line) in content.lines().enumerate() {
              let line = line.trim();
              if line.is_empty() || line.starts_with('#') {
                  continue;
              }
              
              if !line.contains('=') {
                  return Err(ConfigError::InvalidFormat {
                      file: self.config_path.clone(),
                      line: line_num + 1,
                  });
              }
              
              let parts: Vec<&str> = line.splitn(2, '=').collect();
              if parts.len() != 2 {
                  return Err(ConfigError::InvalidFormat {
                      file: self.config_path.clone(),
                      line: line_num + 1,
                  });
              }
              
              let key = parts[0].trim().to_string();
              let value = parts[1].trim().to_string();
              
              if key.is_empty() {
                  return Err(ConfigError::InvalidFormat {
                      file: self.config_path.clone(),
                      line: line_num + 1,
                  });
              }
              
              config.push((key, value));
          }
          
          Ok(config)
      }
      
      fn validate_required_keys(&self, config: &[(String, String)]) -> Result<(), ContextualError> {
          let required_keys = vec!["database_url", "port", "log_level"];
          
          for required_key in required_keys {
              if !config.iter().any(|(key, _)| key == required_key) {
                  return Err(
                      ContextualError::new(
                          format!("验证配置文件 '{}'", self.config_path),
                          format!("缺少必需的配置项: {}", required_key)
                      ).with_source(ConfigError::MissingKey(required_key.to_string()))
                  );
              }
          }
          
          Ok(())
      }
  }
  
  // 错误链遍历函数 | Error chain traversal function
  fn print_error_chain(e: &dyn Error) {
      println!("错误链: ");
      let mut current = Some(e);
      let mut level = 0;
      
      while let Some(err) = current {
          println!("{:indent$}└─ {}", "", err, indent = level * 2);
          current = err.source();
          level += 1;
      }
  }
  
  fn main() {
      // 创建测试配置文件内容 | Create test configuration file content
      let test_configs = vec![
          ("valid_config.txt", "database_url=sqlite://db.sqlite\nport=8080\nlog_level=info\n"),
          ("invalid_format.txt", "database_url=sqlite://db.sqlite\nport\nlog_level=info\n"),
          ("missing_key.txt", "database_url=sqlite://db.sqlite\nlog_level=info\n"),
          ("nonexistent.txt", ""), // 文件不存在 | File doesn't exist
      ];
      
      // 创建测试文件 | Create test files
      for (filename, content) in &test_configs[..3] { // 跳过不存在的文件
          if let Err(e) = fs::write(filename, content) {
              println!("创建测试文件失败: {}", e);
              return;
          }
      }
      
      // 测试不同的错误场景 | Test different error scenarios
      for (filename, _) in test_configs {
          println!("=== 测试配置文件: {} ===", filename);
          
          let config_manager = ConfigManager::new(filename);
          
          match config_manager.load_config() {
              Ok(config) => {
                  println!("✓ 配置加载成功:");
                  for (key, value) in &config {
                      println!("  {}={}", key, value);
                  }
                  
                  // 验证必需的键 | Validate required keys
                  if let Err(e) = config_manager.validate_required_keys(&config) {
                      println!("✗ 配置验证失败:");
                      print_error_chain(&*e);
                  } else {
                      println!("✓ 配置验证通过");
                  }
              }
              Err(e) => {
                  println!("✗ 配置加载失败:");
                  print_error_chain(&*e);
              }
          }
          println!();
      }
      
      // 清理测试文件 | Clean up test files
      for (filename, _) in &test_configs[..3] {
          let _ = fs::remove_file(filename);
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 错误链的深度过深时应该如何处理？| How should overly deep error chains be handled?
    **答案 | Answer:** 在适当层次截断并总结，避免信息过载 | Truncate at appropriate levels and summarize to avoid information overload
  - 上下文信息应该包含敏感数据吗？| Should context information include sensitive data?
    **答案 | Answer:** 不应该，应过滤敏感信息如密码、密钥 | No, sensitive information like passwords and keys should be filtered

### 5. 错误处理最佳实践 | Error Handling Best Practices (30分钟 | 30 minutes)

- **错误设计原则 | Error Design Principles**
  
  **关键原则 | Key Principles:**
  - 描述性命名：错误类型和变体应清晰表达错误含义 | Descriptive naming: error types and variants should clearly express error meaning
  - 适当粒度：既不过于宽泛也不过于细分 | Appropriate granularity: neither too broad nor too fine-grained
  - 可操作性：错误信息应帮助用户采取纠正措施 | Actionability: error messages should help users take corrective action
  
  **实践验证问题 | Practice Verification Questions:**
  1. 如何判断错误分类的粒度是否合适？| How to judge whether error classification granularity is appropriate?
  2. 什么时候应该使用Result而不是panic!？| When should Result be used instead of panic!?
  3. 如何平衡错误信息的详细程度和安全性？| How to balance the detail level and security of error messages?

### 6. 知识巩固与检查 | Knowledge Consolidation and Review (30分钟 | 30 minutes)

- **综合概念检查 | Comprehensive Concept Check**
  
  **综合概念检查 | Comprehensive Concept Check:**
  1. 自定义错误类型相比字符串错误的主要优势是什么？| What are the main advantages of custom error types compared to string errors?
  2. From trait在错误处理中扮演什么角色？| What role does the From trait play in error handling?
  3. 错误链如何帮助调试复杂的错误情况？| How do error chains help debug complex error situations?
  4. 何时应该为错误添加上下文信息？| When should context information be added to errors?
  5. 如何设计可维护的错误处理系统？| How to design maintainable error handling systems?

## 实践项目：配置文件处理器 | Practical Project: Configuration File Processor

### 目标 | Objective
构建一个完整的配置文件处理器，展示自定义错误类型、错误转换、错误链和上下文处理的综合应用 | Build a complete configuration file processor demonstrating comprehensive application of custom error types, error conversion, error chains, and context handling

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 自定义错误枚举是否可以包含其他错误类型作为字段？| Can custom error enums contain other error types as fields?
   **答案 | Answer:** 可以，这是构建错误层次结构的常用方法 | Yes, this is a common way to build error hierarchies
2. From trait实现后是否可以使用?操作符进行自动转换？| After implementing From trait, can the ? operator be used for automatic conversion?
   **答案 | Answer:** 可以，?操作符会自动调用From::from | Yes, the ? operator will automatically call From::from
3. Error trait的source()方法的返回值应该是什么？| What should the return value of Error trait's source() method be?
   **答案 | Answer:** Option<&(dyn Error + 'static)>，指向导致当前错误的源错误 | Option<&(dyn Error + 'static)>, pointing to the source error that caused the current error

### 步骤 | Steps
1. 设计错误类型层次结构 | Design error type hierarchy
2. 实现基本的配置解析功能 | Implement basic configuration parsing functionality
3. 添加错误转换和上下文 | Add error conversion and context
4. 实现错误链追踪 | Implement error chain tracking
5. 完善用户友好的错误报告 | Complete user-friendly error reporting

### 示例代码 | Example Code
```rust
"""
配置文件处理器 | Configuration File Processor
一个展示Rust错误处理最佳实践的完整应用 | A complete application demonstrating Rust error handling best practices

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 自定义错误类型设计 | Custom error type design
- 错误类型转换和传播 | Error type conversion and propagation  
- 错误链和上下文管理 | Error chain and context management
- 用户友好的错误报告 | User-friendly error reporting
"""

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;

// 主应用错误类型 | Main application error type
#[derive(Debug)]
pub enum ConfigProcessorError {
    Io {
        operation: String,
        source: std::io::Error,
    },
    Parse {
        file: String,
        line: usize,
        source: ParseError,
    },
    Validation {
        context: String,
        source: ValidationError,
    },
    Configuration {
        message: String,
    },
}

// 解析错误子类型 | Parse error subtype
#[derive(Debug)]
pub enum ParseError {
    InvalidSyntax(String),
    InvalidValue { key: String, value: String, expected: String },
    MissingDelimiter(String),
}

// 验证错误子类型 | Validation error subtype
#[derive(Debug)]
pub enum ValidationError {
    MissingRequiredKey(String),
    InvalidRange { key: String, value: i32, min: i32, max: i32 },
    InvalidFormat { key: String, pattern: String },
}

// 为所有错误类型实现Display trait | Implement Display trait for all error types
impl fmt::Display for ConfigProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigProcessorError::Io { operation, source } => 
                write!(f, "IO错误在操作'{}': {}", operation, source),
            ConfigProcessorError::Parse { file, line, source } => 
                write!(f, "解析错误在文件'{}'第{}行: {}", file, line, source),
            ConfigProcessorError::Validation { context, source } => 
                write!(f, "验证错误在{}: {}", context, source),
            ConfigProcessorError::Configuration { message } => 
                write!(f, "配置错误: {}", message),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidSyntax(msg) => write!(f, "语法错误: {}", msg),
            ParseError::InvalidValue { key, value, expected } => 
                write!(f, "无效值 '{}={}', 期望: {}", key, value, expected),
            ParseError::MissingDelimiter(line) => 
                write!(f, "缺少分隔符: {}", line),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::MissingRequiredKey(key) => 
                write!(f, "缺少必需的配置项: {}", key),
            ValidationError::InvalidRange { key, value, min, max } => 
                write!(f, "配置项'{}'的值{}超出范围[{}, {}]", key, value, min, max),
            ValidationError::InvalidFormat { key, pattern } => 
                write!(f, "配置项'{}'格式不符合: {}", key, pattern),
        }
    }
}

// 实现Error trait | Implement Error trait
impl Error for ConfigProcessorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigProcessorError::Io { source, .. } => Some(source),
            ConfigProcessorError::Parse { source, .. } => Some(source),
            ConfigProcessorError::Validation { source, .. } => Some(source),
            ConfigProcessorError::Configuration { .. } => None,
        }
    }
}

impl Error for ParseError {}
impl Error for ValidationError {}

// From trait实现，支持错误自动转换 | From trait implementation for automatic error conversion
impl From<std::io::Error> for ConfigProcessorError {
    fn from(error: std::io::Error) -> Self {
        ConfigProcessorError::Io {
            operation: "文件操作".to_string(),
            source: error,
        }
    }
}

// 配置处理器主结构 | Main configuration processor structure
pub struct ConfigProcessor {
    file_path: String,
    config: HashMap<String, String>,
}

impl ConfigProcessor {
    pub fn new(file_path: impl Into<String>) -> Self {
        ConfigProcessor {
            file_path: file_path.into(),
            config: HashMap::new(),
        }
    }
    
    pub fn load(&mut self) -> Result<(), ConfigProcessorError> {
        // 读取文件内容 | Read file content
        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| ConfigProcessorError::Io {
                operation: format!("读取配置文件 '{}'", self.file_path),
                source: e,
            })?;
        
        // 解析配置 | Parse configuration
        self.config = self.parse_content(&content)?;
        
        // 验证配置 | Validate configuration
        self.validate_config()?;
        
        Ok(())
    }
    
    fn parse_content(&self, content: &str) -> Result<HashMap<String, String>, ConfigProcessorError> {
        let mut config = HashMap::new();
        
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            
            // 跳过空行和注释 | Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // 检查是否包含等号分隔符 | Check for equals delimiter
            if !line.contains('=') {
                return Err(ConfigProcessorError::Parse {
                    file: self.file_path.clone(),
                    line: line_num + 1,
                    source: ParseError::MissingDelimiter(line.to_string()),
                });
            }
            
            // 分割键值对 | Split key-value pair
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(ConfigProcessorError::Parse {
                    file: self.file_path.clone(),
                    line: line_num + 1,
                    source: ParseError::InvalidSyntax("无效的键值对格式".to_string()),
                });
            }
            
            let key = parts[0].trim();
            let value = parts[1].trim();
            
            // 验证键不为空 | Validate key is not empty
            if key.is_empty() {
                return Err(ConfigProcessorError::Parse {
                    file: self.file_path.clone(),
                    line: line_num + 1,
                    source: ParseError::InvalidSyntax("键不能为空".to_string()),
                });
            }
            
            config.insert(key.to_string(), value.to_string());
        }
        
        Ok(config)
    }
    
    fn validate_config(&self) -> Result<(), ConfigProcessorError> {
        let required_keys = vec!["server_host", "server_port", "database_url"];
        
        // 检查必需的键 | Check required keys
        for key in required_keys {
            if !self.config.contains_key(key) {
                return Err(ConfigProcessorError::Validation {
                    context: format!("配置文件 '{}'", self.file_path),
                    source: ValidationError::MissingRequiredKey(key.to_string()),
                });
            }
        }
        
        // 验证端口号范围 | Validate port number range
        if let Some(port_str) = self.config.get("server_port") {
            match port_str.parse::<i32>() {
                Ok(port) => {
                    if port < 1024 || port > 65535 {
                        return Err(ConfigProcessorError::Validation {
                            context: format!("配置文件 '{}'", self.file_path),
                            source: ValidationError::InvalidRange {
                                key: "server_port".to_string(),
                                value: port,
                                min: 1024,
                                max: 65535,
                            },
                        });
                    }
                }
                Err(_) => {
                    return Err(ConfigProcessorError::Parse {
                        file: self.file_path.clone(),
                        line: 0, // 实际应用中应该记录行号
                        source: ParseError::InvalidValue {
                            key: "server_port".to_string(),
                            value: port_str.clone(),
                            expected: "整数".to_string(),
                        },
                    });
                }
            }
        }
        
        // 验证主机格式 | Validate host format
        if let Some(host) = self.config.get("server_host") {
            if host.is_empty() {
                return Err(ConfigProcessorError::Validation {
                    context: format!("配置文件 '{}'", self.file_path),
                    source: ValidationError::InvalidFormat {
                        key: "server_host".to_string(),
                        pattern: "非空主机名".to_string(),
                    },
                });
            }
        }
        
        Ok(())
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
    
    pub fn print_config(&self) {
        println!("当前配置:");
        for (key, value) in &self.config {
            println!("  {} = {}", key, value);
        }
    }
}

// 错误链打印工具函数 | Error chain printing utility function
pub fn print_error_chain(err: &dyn Error) {
    eprintln!("错误详情:");
    
    let mut current = Some(err);
    let mut level = 0;
    
    while let Some(error) = current {
        if level == 0 {
            eprintln!("  ✗ {}", error);
        } else {
            eprintln!("{:indent$}└─ 原因: {}", "", error, indent = (level - 1) * 4 + 2);
        }
        
        current = error.source();
        level += 1;
    }
}

fn main() {
    // 创建测试配置文件 | Create test configuration files
    let test_files = vec![
        ("good_config.conf", 
         "# 服务器配置\nserver_host=localhost\nserver_port=8080\ndatabase_url=sqlite://./db.sqlite"),
        ("bad_syntax.conf", 
         "server_host=localhost\nserver_port\ndatabase_url=sqlite://./db.sqlite"),
        ("missing_key.conf", 
         "server_host=localhost\nserver_port=8080"),
        ("invalid_port.conf", 
         "server_host=localhost\nserver_port=99999\ndatabase_url=sqlite://./db.sqlite"),
    ];
    
    // 创建测试文件 | Create test files
    for (filename, content) in &test_files {
        if let Err(e) = fs::write(filename, content) {
            eprintln!("创建测试文件{}失败: {}", filename, e);
            return;
        }
    }
    
    // 测试不同的配置文件 | Test different configuration files
    for (filename, _) in &test_files {
        println!("\n{'='*50}");
        println!("测试配置文件: {}", filename);
        println!("{'='*50}");
        
        let mut processor = ConfigProcessor::new(*filename);
        
        match processor.load() {
            Ok(()) => {
                println!("✓ 配置加载成功!");
                processor.print_config();
            }
            Err(e) => {
                println!("✗ 配置加载失败!");
                print_error_chain(&e);
            }
        }
    }
    
    // 测试不存在的文件 | Test non-existent file
    println!("\n{'='*50}");
    println!("测试不存在的文件: nonexistent.conf");
    println!("{'='*50}");
    
    let mut processor = ConfigProcessor::new("nonexistent.conf");
    match processor.load() {
        Ok(()) => println!("✓ 意外成功"),
        Err(e) => {
            println!("✗ 预期的文件不存在错误:");
            print_error_chain(&e);
        }
    }
    
    // 清理测试文件 | Clean up test files
    for (filename, _) in &test_files {
        let _ = fs::remove_file(filename);
    }
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确实现了错误类型层次结构？| Does the project correctly implement error type hierarchy?
2. From trait转换是否工作正常？| Does the From trait conversion work properly?
3. 错误链是否能够完整追踪错误传播路径？| Can error chains completely track error propagation paths?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **错误类型设计练习 | Error Type Design Exercise**
   - **练习描述 | Exercise Description:** 为一个HTTP客户端库设计完整的错误类型系统
   - **概念检查 | Concept Check:** 如何区分网络错误、解析错误和业务逻辑错误？
   - **学习目标 | Learning Objective:** 掌握复杂系统的错误分类设计

2. **错误转换练习 | Error Conversion Exercise**
   - **练习描述 | Exercise Description:** 实现一个日志解析器，处理多种格式的错误转换
   - **概念检查 | Concept Check:** 何时使用From trait vs map_err？
   - **学习目标 | Learning Objective:** 理解错误转换的时机和方法

3. **错误上下文练习 | Error Context Exercise**
   - **练习描述 | Exercise Description:** 为文件批处理工具添加详细的错误上下文
   - **概念检查 | Concept Check:** 什么信息应该包含在错误上下文中？
   - **学习目标 | Learning Objective:** 学会设计有用的错误上下文

4. **错误恢复练习 | Error Recovery Exercise**
   - **练习描述 | Exercise Description:** 实现一个支持错误恢复的数据处理管道
   - **概念检查 | Concept Check:** 哪些错误应该尝试恢复，哪些应该立即失败？
   - **学习目标 | Learning Objective:** 理解错误处理策略的选择

5. **性能优化练习 | Performance Optimization Exercise**
   - **练习描述 | Exercise Description:** 优化错误处理的性能影响，减少分配
   - **概念检查 | Concept Check:** 错误处理如何影响程序性能？
   - **学习目标 | Learning Objective:** 平衡错误信息丰富度和性能

6. **测试驱动练习 | Test-Driven Exercise**
   - **练习描述 | Exercise Description:** 为错误处理编写全面的单元测试
   - **概念检查 | Concept Check:** 如何测试错误路径和边界情况？
   - **学习目标 | Learning Objective:** 确保错误处理的正确性

7. **集成练习 | Integration Exercise**
   - **练习描述 | Exercise Description:** 将今天学习的错误处理技术集成到之前的项目中
   - **概念检查 | Concept Check:** 如何重构现有代码以改进错误处理？
   - **学习目标 | Learning Objective:** 实际应用错误处理最佳实践

## 学习资源 | Learning Resources
- [Rust官方文档 - 错误处理](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [std::error::Error trait文档](https://doc.rust-lang.org/std/error/trait.Error.html)
- [错误处理最佳实践指南](https://blog.burntsushi.net/rust-error-handling/)
- [anyhow和thiserror库使用指南](https://github.com/dtolnay/anyhow)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解自定义错误类型的设计原则 | Understand design principles of custom error types
- [ ] 掌握Error和Display trait的实现 | Master implementation of Error and Display traits  
- [ ] 学会使用From trait进行错误转换 | Learn to use From trait for error conversion
- [ ] 理解错误链和上下文的构建 | Understand construction of error chains and context
- [ ] 完成配置文件处理器项目 | Complete configuration file processor project
- [ ] 所有CCQs都能正确回答 | All CCQs can be answered correctly
- [ ] 代码示例理解和运行成功 | Code examples understood and executed successfully
- [ ] 错误处理最佳实践掌握 | Error handling best practices mastered
- [ ] 能够设计适合特定场景的错误类型 | Able to design error types suitable for specific scenarios
- [ ] 至少完成3个扩展练习 | At least 3 extension exercises completed

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释自定义错误类型的设计原则、实现方法和最佳实践。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain the design principles, implementation methods, and best practices of custom error types to others.