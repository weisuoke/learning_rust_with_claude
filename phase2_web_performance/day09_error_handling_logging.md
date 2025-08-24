# Rust入门 - 第9天：错误处理与日志 | Rust Introduction - Day 9: Error Handling and Logging

## 学习目标 | Learning Objectives
- 掌握Rust网络编程中的错误处理最佳实践 | Master error handling best practices in Rust network programming
- 深入理解不同类型的网络错误和处理策略 | Deeply understand different types of network errors and handling strategies
- 熟练使用log和tracing生态系统进行日志记录 | Proficiently use log and tracing ecosystem for logging
- 能够设计和实现结构化日志系统 | Be able to design and implement structured logging systems
- 理解日志在生产环境中的重要性和最佳实践 | Understand the importance and best practices of logging in production environments
- 构建健壮的网络服务错误处理和监控体系 | Build robust network service error handling and monitoring systems

## 详细内容 | Detailed Content

### 1. Rust错误处理基础回顾 | Rust Error Handling Fundamentals Review (45分钟 | 45 minutes)

- **Rust错误处理核心概念 | Core Concepts of Rust Error Handling**
  
  **概念定义 | Concept Definition:**
  Rust采用显式错误处理模式，使用Result<T, E>类型来表示可能失败的操作，通过类型系统强制开发者处理错误，避免了传统语言中的异常抛出和隐式错误传播。 | Rust uses explicit error handling with Result<T, E> type to represent potentially failing operations, forcing developers to handle errors through the type system, avoiding exception throwing and implicit error propagation found in traditional languages.
  
  **核心特征 | Key Characteristics:**
  - 显式错误处理：所有可能的错误都在类型签名中明确表示 | Explicit error handling: all possible errors are clearly represented in type signatures
  - 零成本抽象：错误处理在编译时优化，运行时开销最小 | Zero-cost abstraction: error handling is optimized at compile time with minimal runtime overhead
  - 组合性强：错误类型可以轻松组合和转换 | Highly composable: error types can be easily combined and converted
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Result<T, E>中的T和E分别代表什么？| What do T and E represent in Result<T, E>?
     **答案 | Answer:** T代表成功值类型，E代表错误类型 | T represents success value type, E represents error type
  2. ?操作符的作用是什么？| What is the purpose of the ? operator?
     **答案 | Answer:** 提前返回错误，简化错误传播 | Early return on error, simplifying error propagation
  3. panic!和返回错误有什么区别？| What's the difference between panic! and returning an error?
     **答案 | Answer:** panic!会终止程序，返回错误允许调用者处理 | panic! terminates the program, returning errors allows callers to handle them
  4. 什么时候应该使用expect()而不是unwrap()？| When should you use expect() instead of unwrap()?
     **答案 | Answer:** expect()提供更好的错误信息，用于调试和故障排除 | expect() provides better error messages, useful for debugging and troubleshooting
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::fs::File;
  use std::io::{self, Read};
  use std::num::ParseIntError;
  
  // 错误处理基础回顾 | Error handling fundamentals review
  
  // 1. 基础Result使用 | Basic Result usage
  fn read_file_content(filename: &str) -> Result<String, io::Error> {
      let mut file = File::open(filename)?; // ?操作符传播错误 | ? operator propagates error
      let mut content = String::new();
      file.read_to_string(&mut content)?;
      Ok(content)
  }
  
  // 2. 自定义错误类型 | Custom error type
  #[derive(Debug)]
  enum ConfigError {
      IoError(io::Error),
      ParseError(ParseIntError),
      ValidationError(String),
  }
  
  // 实现From trait用于错误转换 | Implement From trait for error conversion
  impl From<io::Error> for ConfigError {
      fn from(error: io::Error) -> Self {
          ConfigError::IoError(error)
      }
  }
  
  impl From<ParseIntError> for ConfigError {
      fn from(error: ParseIntError) -> Self {
          ConfigError::ParseError(error)
      }
  }
  
  // 使用自定义错误类型 | Using custom error type
  fn parse_config(filename: &str) -> Result<u32, ConfigError> {
      let content = read_file_content(filename)?; // io::Error自动转换 | io::Error automatically converted
      let number = content.trim().parse::<u32>()?; // ParseIntError自动转换 | ParseIntError automatically converted
      
      if number > 100 {
          return Err(ConfigError::ValidationError("数值不能超过100 | Value cannot exceed 100".to_string()));
      }
      
      Ok(number)
  }
  
  // 3. 错误处理组合子 | Error handling combinators
  fn process_data_with_combinators() -> Result<String, Box<dyn std::error::Error>> {
      let result = read_file_content("data.txt")
          .and_then(|content| {
              // 成功时执行的操作 | Operation to execute on success
              content.lines()
                  .next()
                  .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "文件为空 | File is empty"))
                  .map(|line| line.to_string())
          })
          .or_else(|e| {
              // 错误时的恢复操作 | Recovery operation on error
              println!("读取失败，使用默认值: {:?} | Read failed, using default value: {:?}", e);
              Ok("默认内容 | Default content".to_string())
          })?;
      
      Ok(result)
  }
  
  // 4. 多种错误处理策略演示 | Multiple error handling strategies demonstration
  fn demonstrate_error_strategies() {
      println!("=== 错误处理策略演示 | Error Handling Strategies Demo ===");
      
      // 策略1: 传播错误 | Strategy 1: Propagate error
      match parse_config("config.txt") {
          Ok(value) => println!("配置值: {} | Config value: {}", value, value),
          Err(e) => println!("配置解析错误: {:?} | Config parsing error: {:?}", e, e),
      }
      
      // 策略2: 提供默认值 | Strategy 2: Provide default value
      let config_value = parse_config("config.txt").unwrap_or(42);
      println!("使用的配置值: {} | Used config value: {}", config_value, config_value);
      
      // 策略3: 重试机制 | Strategy 3: Retry mechanism
      for attempt in 1..=3 {
          match parse_config("config.txt") {
              Ok(value) => {
                  println!("第{}次尝试成功: {} | Attempt {} succeeded: {}", attempt, value, attempt, value);
                  break;
              }
              Err(e) if attempt < 3 => {
                  println!("第{}次尝试失败，重试中: {:?} | Attempt {} failed, retrying: {:?}", attempt, e, attempt, e);
                  std::thread::sleep(std::time::Duration::from_millis(100));
              }
              Err(e) => println!("所有尝试都失败了: {:?} | All attempts failed: {:?}", e, e),
          }
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - ConfigError枚举实现了哪些错误转换？| What error conversions does ConfigError enum implement?
    **答案 | Answer:** io::Error和ParseIntError的From实现 | From implementations for io::Error and ParseIntError
  - and_then和or_else的区别是什么？| What's the difference between and_then and or_else?
    **答案 | Answer:** and_then处理成功情况，or_else处理错误情况 | and_then handles success cases, or_else handles error cases
  
  **常见误区检查 | Common Misconception Checks:**
  - 是否应该总是使用unwrap()？| Should you always use unwrap()?
    **答案 | Answer:** 否，只在确定不会失败或原型阶段使用 | No, only use when certain of no failure or in prototyping
  - ?操作符是否只能用于Result类型？| Can the ? operator only be used with Result types?
    **答案 | Answer:** 否，也可以用于Option和实现Try trait的类型 | No, also works with Option and types implementing Try trait

### 2. 网络错误类型分析 | Network Error Types Analysis (1小时 | 1 hour)

- **网络编程常见错误分类 | Common Network Programming Error Classification**
  
  **概念定义 | Concept Definition:**
  网络编程中的错误可以分为连接错误、传输错误、协议错误和应用错误四大类，每种错误需要不同的处理策略和恢复机制。 | Network programming errors can be classified into four major categories: connection errors, transmission errors, protocol errors, and application errors, each requiring different handling strategies and recovery mechanisms.
  
  **核心特征 | Key Characteristics:**
  - 瞬时性错误：可能通过重试解决的临时性问题 | Transient errors: temporary issues that might be resolved through retry
  - 永久性错误：需要人工干预或配置更改的问题 | Permanent errors: issues requiring manual intervention or configuration changes
  - 系统性错误：由基础设施或环境因素导致的错误 | Systemic errors: errors caused by infrastructure or environmental factors
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 连接超时错误是否应该立即重试？| Should connection timeout errors be retried immediately?
     **答案 | Answer:** 否，应该使用退避策略避免加重负载 | No, should use backoff strategy to avoid increasing load
  2. DNS解析错误属于哪种错误类型？| What type of error is DNS resolution error?
     **答案 | Answer:** 通常是瞬时性错误，可以重试 | Usually transient error, can be retried
  3. 连接被拒绝和连接超时有什么区别？| What's the difference between connection refused and connection timeout?
     **答案 | Answer:** 拒绝是服务器主动拒绝，超时是网络或服务器无响应 | Refused is server actively rejecting, timeout is network/server not responding
  4. 如何判断一个网络错误是否值得重试？| How to determine if a network error is worth retrying?
     **答案 | Answer:** 考虑错误类型、重试次数、时间间隔和业务影响 | Consider error type, retry count, time interval, and business impact
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::io;
  use std::net::{TcpStream, ToSocketAddrs};
  use std::time::{Duration, Instant};
  use thiserror::Error;
  
  // 网络错误分类和处理 | Network error classification and handling
  
  // 自定义网络错误类型 | Custom network error types
  #[derive(Error, Debug)]
  pub enum NetworkError {
      #[error("连接错误: {0} | Connection error: {0}")]
      Connection(#[from] io::Error),
      
      #[error("超时错误: 操作超时 {timeout:?} | Timeout error: operation timed out after {timeout:?}")]
      Timeout { timeout: Duration },
      
      #[error("DNS解析错误: 无法解析 {host} | DNS resolution error: cannot resolve {host}")]
      DnsResolution { host: String },
      
      #[error("协议错误: {message} | Protocol error: {message}")]
      Protocol { message: String },
      
      #[error("认证错误: 凭据无效 | Authentication error: invalid credentials")]
      Authentication,
      
      #[error("限流错误: 请求过于频繁 | Rate limit error: too many requests")]
      RateLimit,
      
      #[error("服务不可用: {reason} | Service unavailable: {reason}")]
      ServiceUnavailable { reason: String },
  }
  
  // 错误分类器 | Error classifier
  impl NetworkError {
      // 判断是否为瞬时性错误 | Determine if error is transient
      pub fn is_transient(&self) -> bool {
          match self {
              NetworkError::Timeout { .. } => true,
              NetworkError::DnsResolution { .. } => true,
              NetworkError::ServiceUnavailable { .. } => true,
              NetworkError::RateLimit => true,
              NetworkError::Connection(io_err) => {
                  match io_err.kind() {
                      io::ErrorKind::TimedOut => true,
                      io::ErrorKind::ConnectionRefused => false, // 服务未启动 | Service not started
                      io::ErrorKind::ConnectionReset => true,   // 可能是临时网络问题 | Might be temporary network issue
                      io::ErrorKind::ConnectionAborted => true,
                      io::ErrorKind::NotConnected => true,
                      io::ErrorKind::AddrInUse => false,        // 端口被占用 | Port already in use
                      io::ErrorKind::AddrNotAvailable => false,
                      _ => false,
                  }
              }
              NetworkError::Protocol { .. } => false,
              NetworkError::Authentication => false,
          }
      }
      
      // 获取建议的重试延迟 | Get suggested retry delay
      pub fn suggested_retry_delay(&self) -> Option<Duration> {
          if self.is_transient() {
              match self {
                  NetworkError::RateLimit => Some(Duration::from_secs(60)), // 限流需要更长等待 | Rate limit needs longer wait
                  NetworkError::Timeout { timeout } => Some(*timeout / 2), // 超时用一半时间重试 | Retry with half timeout
                  _ => Some(Duration::from_millis(1000)), // 默认1秒 | Default 1 second
              }
          } else {
              None
          }
      }
      
      // 获取最大重试次数建议 | Get max retry count suggestion
      pub fn max_retry_count(&self) -> u32 {
          match self {
              NetworkError::RateLimit => 2,
              NetworkError::DnsResolution { .. } => 3,
              NetworkError::Timeout { .. } => 3,
              NetworkError::ServiceUnavailable { .. } => 5,
              NetworkError::Connection(io_err) => {
                  match io_err.kind() {
                      io::ErrorKind::ConnectionReset => 3,
                      io::ErrorKind::ConnectionAborted => 3,
                      _ => 1,
                  }
              }
              _ => 0, // 不建议重试 | Not recommended to retry
          }
      }
  }
  
  // 网络操作重试器 | Network operation retrier
  pub struct NetworkRetrier {
      max_attempts: u32,
      base_delay: Duration,
      max_delay: Duration,
  }
  
  impl NetworkRetrier {
      pub fn new() -> Self {
          Self {
              max_attempts: 3,
              base_delay: Duration::from_millis(100),
              max_delay: Duration::from_secs(30),
          }
      }
      
      pub fn with_max_attempts(mut self, attempts: u32) -> Self {
          self.max_attempts = attempts;
          self
      }
      
      // 执行带重试的网络操作 | Execute network operation with retry
      pub async fn retry<F, T, E>(&self, mut operation: F) -> Result<T, E>
      where
          F: FnMut() -> Result<T, E>,
          E: std::fmt::Debug,
      {
          let mut last_error = None;
          
          for attempt in 1..=self.max_attempts {
              match operation() {
                  Ok(result) => return Ok(result),
                  Err(error) => {
                      println!("尝试 {}/{} 失败: {:?} | Attempt {}/{} failed: {:?}", 
                              attempt, self.max_attempts, error, attempt, self.max_attempts);
                      
                      last_error = Some(error);
                      
                      if attempt < self.max_attempts {
                          // 指数退避 | Exponential backoff
                          let delay = std::cmp::min(
                              self.base_delay * 2_u32.pow(attempt - 1),
                              self.max_delay
                          );
                          println!("等待 {:?} 后重试 | Waiting {:?} before retry", delay, delay);
                          tokio::time::sleep(delay).await;
                      }
                  }
              }
          }
          
          Err(last_error.unwrap())
      }
  }
  
  // 网络连接管理器 | Network connection manager
  pub struct NetworkClient {
      retrier: NetworkRetrier,
  }
  
  impl NetworkClient {
      pub fn new() -> Self {
          Self {
              retrier: NetworkRetrier::new().with_max_attempts(3),
          }
      }
      
      // 带重试的TCP连接 | TCP connection with retry
      pub async fn connect_with_retry<A: ToSocketAddrs + Clone>(&self, addr: A) -> Result<TcpStream, NetworkError> {
          let start_time = Instant::now();
          let timeout = Duration::from_secs(10);
          
          self.retrier.retry(|| {
              match TcpStream::connect(addr.clone()) {
                  Ok(stream) => {
                      println!("连接成功 | Connection successful");
                      Ok(stream)
                  }
                  Err(e) => {
                      if start_time.elapsed() > timeout {
                          Err(NetworkError::Timeout { timeout })
                      } else {
                          Err(NetworkError::Connection(e))
                      }
                  }
              }
          }).await
      }
      
      // 模拟HTTP请求 | Simulate HTTP request
      pub async fn http_request(&self, url: &str) -> Result<String, NetworkError> {
          // 模拟各种网络错误 | Simulate various network errors
          use rand::Rng;
          let mut rng = rand::thread_rng();
          
          self.retrier.retry(|| {
              match rng.gen_range(0..10) {
                  0..=2 => {
                      // 30% 成功 | 30% success
                      Ok(format!("HTTP响应来自 {} | HTTP response from {}", url, url))
                  }
                  3..=4 => {
                      // 20% 超时 | 20% timeout
                      Err(NetworkError::Timeout { timeout: Duration::from_secs(5) })
                  }
                  5 => {
                      // 10% DNS错误 | 10% DNS error
                      Err(NetworkError::DnsResolution { host: url.to_string() })
                  }
                  6 => {
                      // 10% 限流 | 10% rate limit
                      Err(NetworkError::RateLimit)
                  }
                  7 => {
                      // 10% 服务不可用 | 10% service unavailable
                      Err(NetworkError::ServiceUnavailable { 
                          reason: "服务器维护中 | Server under maintenance".to_string() 
                      })
                  }
                  8 => {
                      // 10% 连接被拒绝 | 10% connection refused
                      Err(NetworkError::Connection(
                          io::Error::new(io::ErrorKind::ConnectionRefused, "连接被拒绝 | Connection refused")
                      ))
                  }
                  _ => {
                      // 10% 连接重置 | 10% connection reset
                      Err(NetworkError::Connection(
                          io::Error::new(io::ErrorKind::ConnectionReset, "连接被重置 | Connection reset")
                      ))
                  }
              }
          }).await
      }
  }
  
  // 网络错误处理演示 | Network error handling demonstration
  pub async fn demonstrate_network_error_handling() {
      println!("=== 网络错误处理演示 | Network Error Handling Demo ===");
      
      let client = NetworkClient::new();
      
      // 测试多个URL请求 | Test multiple URL requests
      let urls = vec![
          "https://api.example1.com",
          "https://api.example2.com", 
          "https://api.example3.com",
      ];
      
      for url in urls {
          println!("\n--- 请求 {} | Requesting {} ---", url, url);
          let start = Instant::now();
          
          match client.http_request(url).await {
              Ok(response) => {
                  println!("✅ 成功: {} (耗时: {:?}) | Success: {} (duration: {:?})", 
                          response, start.elapsed(), response, start.elapsed());
              }
              Err(error) => {
                  println!("❌ 失败: {} (耗时: {:?}) | Failed: {} (duration: {:?})", 
                          error, start.elapsed(), error, start.elapsed());
                  
                  // 显示错误特征 | Show error characteristics
                  println!("   瞬时性: {} | Transient: {}", error.is_transient(), error.is_transient());
                  if let Some(delay) = error.suggested_retry_delay() {
                      println!("   建议重试延迟: {:?} | Suggested retry delay: {:?}", delay, delay);
                  }
                  println!("   最大重试次数: {} | Max retry count: {}", error.max_retry_count(), error.max_retry_count());
              }
          }
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 什么样的网络错误应该使用指数退避？| What types of network errors should use exponential backoff?
    **答案 | Answer:** 瞬时性错误，特别是可能由负载引起的错误 | Transient errors, especially those potentially caused by load
  - 连接被拒绝错误是否应该重试？| Should connection refused errors be retried?
    **答案 | Answer:** 通常不应该，因为表示服务未启动或不可达 | Usually no, as it indicates service is down or unreachable
  
  **常见误区检查 | Common Misconception Checks:**
  - 是否所有网络错误都应该重试？| Should all network errors be retried?
    **答案 | Answer:** 否，永久性错误（如认证失败）不应重试 | No, permanent errors (like authentication failure) should not be retried
  - 重试是否总是有益的？| Is retrying always beneficial?
    **答案 | Answer:** 否，过度重试可能加重系统负载 | No, excessive retrying may increase system load

### 3. Log生态系统详解 | Log Ecosystem Deep Dive (1小时 | 1 hour)

- **Rust日志库架构理解 | Understanding Rust Logging Library Architecture**
  
  **概念定义 | Concept Definition:**
  Rust的log生态系统采用facade模式，log crate提供统一的日志接口，具体的日志实现由env_logger、fern等后端提供，实现了日志接口与实现的解耦。 | Rust's log ecosystem uses the facade pattern, where the log crate provides a unified logging interface, and specific implementations are provided by backends like env_logger and fern, achieving decoupling between logging interface and implementation.
  
  **解决的问题 | Problems It Solves:**
  - 日志库解耦：应用代码不依赖特定日志实现 | Logging library decoupling: application code doesn't depend on specific logging implementations
  - 灵活配置：运行时可以更改日志级别和输出格式 | Flexible configuration: log levels and output formats can be changed at runtime
  - 性能优化：编译时可以完全移除日志代码 | Performance optimization: logging code can be completely removed at compile time
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. log crate本身是否包含日志输出实现？| Does the log crate itself contain logging output implementation?
     **答案 | Answer:** 否，只提供接口，需要后端实现 | No, only provides interface, needs backend implementation
  2. 日志级别的优先级顺序是什么？| What is the priority order of log levels?
     **答案 | Answer:** ERROR > WARN > INFO > DEBUG > TRACE | ERROR > WARN > INFO > DEBUG > TRACE
  3. RUST_LOG环境变量的作用是什么？| What is the purpose of the RUST_LOG environment variable?
     **答案 | Answer:** 控制日志输出级别和过滤规则 | Controls log output levels and filtering rules
  4. 生产环境应该使用什么日志级别？| What log level should be used in production?
     **答案 | Answer:** 通常是INFO或WARN，避免DEBUG/TRACE | Usually INFO or WARN, avoid DEBUG/TRACE
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use log::{debug, error, info, trace, warn};
  use std::io::Write;
  
  // 自定义日志格式化器 | Custom log formatter
  pub struct CustomFormatter;
  
  impl env_logger::fmt::Formatter for CustomFormatter {
      fn format(
          &self,
          buf: &mut env_logger::fmt::Formatter,
          record: &log::Record,
      ) -> Result<(), std::io::Error> {
          let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC");
          let level = record.level();
          let target = record.target();
          let message = record.args();
          
          writeln!(
              buf,
              "[{}] {} [{}:{}] {} - {}",
              timestamp,
              level,
              target,
              record.line().unwrap_or(0),
              record.file().unwrap_or("unknown"),
              message
          )
      }
  }
  
  // 日志配置管理器 | Log configuration manager
  pub struct LogConfig {
      pub level: log::LevelFilter,
      pub target: LogTarget,
      pub format: LogFormat,
  }
  
  #[derive(Debug, Clone)]
  pub enum LogTarget {
      Console,
      File(String),
      Both(String),
  }
  
  #[derive(Debug, Clone)]
  pub enum LogFormat {
      Simple,
      Detailed,
      Json,
      Custom,
  }
  
  impl LogConfig {
      pub fn new() -> Self {
          Self {
              level: log::LevelFilter::Info,
              target: LogTarget::Console,
              format: LogFormat::Simple,
          }
      }
      
      // 从环境变量初始化 | Initialize from environment variables
      pub fn from_env() -> Self {
          let mut config = Self::new();
          
          // 从RUST_LOG读取日志级别 | Read log level from RUST_LOG
          if let Ok(env_log) = std::env::var("RUST_LOG") {
              config.level = match env_log.to_lowercase().as_str() {
                  "trace" => log::LevelFilter::Trace,
                  "debug" => log::LevelFilter::Debug,
                  "info" => log::LevelFilter::Info,
                  "warn" => log::LevelFilter::Warn,
                  "error" => log::LevelFilter::Error,
                  _ => log::LevelFilter::Info,
              };
          }
          
          // 从LOG_TARGET读取输出目标 | Read output target from LOG_TARGET
          if let Ok(target) = std::env::var("LOG_TARGET") {
              config.target = match target.as_str() {
                  "console" => LogTarget::Console,
                  path if path.starts_with("file:") => {
                      LogTarget::File(path.strip_prefix("file:").unwrap().to_string())
                  }
                  path => LogTarget::Both(path.to_string()),
              };
          }
          
          // 从LOG_FORMAT读取格式 | Read format from LOG_FORMAT
          if let Ok(format) = std::env::var("LOG_FORMAT") {
              config.format = match format.as_str() {
                  "simple" => LogFormat::Simple,
                  "detailed" => LogFormat::Detailed,
                  "json" => LogFormat::Json,
                  "custom" => LogFormat::Custom,
                  _ => LogFormat::Simple,
              };
          }
          
          config
      }
      
      // 初始化日志系统 | Initialize logging system
      pub fn init(self) -> Result<(), fern::InitError> {
          let mut dispatch = fern::Dispatch::new()
              .level(self.level)
              .chain(std::io::stderr()); // 默认输出到stderr | Default output to stderr
          
          // 配置格式 | Configure format
          dispatch = match self.format {
              LogFormat::Simple => {
                  dispatch.format(|out, message, record| {
                      out.finish(format_args!(
                          "{}[{}][{}] {}",
                          chrono::Local::now().format("%H:%M:%S"),
                          record.target(),
                          record.level(),
                          message
                      ))
                  })
              }
              LogFormat::Detailed => {
                  dispatch.format(|out, message, record| {
                      out.finish(format_args!(
                          "{}[{}][{}][{}:{}] {}",
                          chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                          record.target(),
                          record.level(),
                          record.file().unwrap_or("unknown"),
                          record.line().unwrap_or(0),
                          message
                      ))
                  })
              }
              LogFormat::Json => {
                  dispatch.format(|out, message, record| {
                      let log_entry = serde_json::json!({
                          "timestamp": chrono::Utc::now().to_rfc3339(),
                          "level": record.level().to_string(),
                          "target": record.target(),
                          "file": record.file(),
                          "line": record.line(),
                          "message": message.to_string(),
                      });
                      out.finish(format_args!("{}", log_entry))
                  })
              }
              LogFormat::Custom => {
                  dispatch.format(|out, message, record| {
                      out.finish(format_args!(
                          "[{}] 🦀 {} [{}] 📝 {}",
                          chrono::Local::now().format("%H:%M:%S"),
                          record.level(),
                          record.target(),
                          message
                      ))
                  })
              }
          };
          
          // 配置输出目标 | Configure output target
          dispatch = match self.target {
              LogTarget::Console => dispatch,
              LogTarget::File(path) => {
                  dispatch
                      .chain(std::io::stderr())
                      .chain(fern::log_file(&path)?)
              }
              LogTarget::Both(path) => {
                  dispatch.chain(fern::log_file(&path)?)
              }
          };
          
          dispatch.apply()?;
          Ok(())
      }
  }
  
  // 日志工具函数 | Logging utility functions
  pub mod log_utils {
      use super::*;
      
      // 记录函数执行时间 | Log function execution time
      pub fn time_function<F, T>(name: &str, func: F) -> T
      where
          F: FnOnce() -> T,
      {
          let start = std::time::Instant::now();
          debug!("开始执行函数: {} | Starting function execution: {}", name, name);
          
          let result = func();
          
          let duration = start.elapsed();
          info!("函数 {} 执行完成，耗时: {:?} | Function {} completed, duration: {:?}", 
                name, duration, name, duration);
          
          result
      }
      
      // 记录错误上下文 | Log error context
      pub fn log_error_context<E: std::fmt::Display>(error: &E, context: &str) {
          error!("错误上下文: {} | Error context: {} - 错误: {} | Error: {}", 
                 context, context, error, error);
      }
      
      // 记录性能指标 | Log performance metrics
      pub fn log_metrics(name: &str, value: f64, unit: &str) {
          info!("指标: {} = {:.2} {} | Metric: {} = {:.2} {}", 
                name, value, unit, name, value, unit);
      }
      
      // 结构化日志记录 | Structured logging
      pub fn log_request(method: &str, path: &str, status: u16, duration_ms: u64) {
          info!(
              target: "http_requests",
              "HTTP请求 | HTTP Request: {} {} -> {} ({}ms)",
              method, path, status, duration_ms
          );
      }
  }
  
  // 日志使用演示 | Logging usage demonstration
  pub fn demonstrate_logging() {
      println!("=== 日志系统演示 | Logging System Demo ===");
      
      // 初始化日志配置 | Initialize log configuration
      let config = LogConfig::from_env();
      if let Err(e) = config.init() {
          eprintln!("日志初始化失败 | Log initialization failed: {}", e);
          return;
      }
      
      // 各级别日志演示 | Various log level demonstrations
      trace!("这是trace级别日志 - 最详细的调试信息 | This is trace level log - most detailed debug info");
      debug!("这是debug级别日志 - 调试信息 | This is debug level log - debug information");
      info!("这是info级别日志 - 一般信息 | This is info level log - general information");
      warn!("这是warn级别日志 - 警告信息 | This is warn level log - warning information");
      error!("这是error级别日志 - 错误信息 | This is error level log - error information");
      
      // 结构化日志 | Structured logging
      log_utils::log_request("GET", "/api/users", 200, 145);
      log_utils::log_request("POST", "/api/login", 401, 89);
      
      // 性能监控日志 | Performance monitoring logs
      log_utils::log_metrics("response_time", 0.145, "seconds");
      log_utils::log_metrics("memory_usage", 256.7, "MB");
      
      // 函数执行时间监控 | Function execution time monitoring
      let result = log_utils::time_function("data_processing", || {
          std::thread::sleep(std::time::Duration::from_millis(100));
              42
          });
      info!("处理结果: {} | Processing result: {}", result, result);
      
      // 错误上下文记录 | Error context logging
      let error = std::io::Error::new(std::io::ErrorKind::NotFound, "文件未找到 | File not found");
      log_utils::log_error_context(&error, "配置文件加载 | Configuration file loading");
  }
  ```
  
  **综合应用检查 | Comprehensive Application Check:**
  - 如何在生产环境中动态调整日志级别？| How to dynamically adjust log levels in production?
    **答案 | Answer:** 使用环境变量或配置文件，结合信号处理重载配置 | Use environment variables or config files, combine with signal handling to reload configuration
  - JSON格式日志的优势是什么？| What are the advantages of JSON format logs?
    **答案 | Answer:** 结构化，便于日志聚合和分析工具处理 | Structured, easier for log aggregation and analysis tools to process

### 4. Tracing系统深入应用 | Deep Application of Tracing System (1小时 | 1 hour)

- **分布式追踪和可观测性 | Distributed Tracing and Observability**
  
  **概念定义 | Concept Definition:**
  Tracing是一种高级的可观测性技术，通过跨服务跟踪请求流程，提供比传统日志更丰富的上下文信息，支持分布式系统的性能分析和问题诊断。 | Tracing is an advanced observability technique that tracks request flows across services, providing richer contextual information than traditional logging, supporting performance analysis and problem diagnosis in distributed systems.
  
  **核心特征 | Key Characteristics:**
  - 分层追踪：span嵌套形成调用链路 | Hierarchical tracing: nested spans form call chains
  - 结构化数据：丰富的元数据和上下文信息 | Structured data: rich metadata and contextual information
  - 异步友好：天然支持异步操作的追踪 | Async-friendly: native support for tracing async operations
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. span和event的区别是什么？| What's the difference between span and event?
     **答案 | Answer:** span代表一段时间的操作，event是瞬时的记录点 | Span represents an operation over time, event is an instantaneous record point
  2. tracing相比传统日志的优势是什么？| What are the advantages of tracing over traditional logging?
     **答案 | Answer:** 提供结构化上下文、调用关系和性能数据 | Provides structured context, call relationships, and performance data
  3. 在异步环境中如何传递tracing上下文？| How to propagate tracing context in async environments?
     **答案 | Answer:** 使用instrument宏或手动管理span | Use instrument macro or manually manage spans
  4. subscriber的作用是什么？| What is the role of a subscriber?
     **答案 | Answer:** 接收和处理tracing数据，类似于log的后端 | Receives and processes tracing data, similar to log backends
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use tracing::{debug, error, info, info_span, instrument, warn, Instrument};
  use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
  use std::time::Duration;
  use tokio::time::sleep;
  
  // Tracing配置和初始化 | Tracing configuration and initialization
  pub fn init_tracing() {
      tracing_subscriber::registry()
          .with(
              fmt::layer()
                  .with_target(true)
                  .with_thread_ids(true)
                  .with_level(true)
                  .with_file(true)
                  .with_line_number(true)
          )
          .with(
              tracing_subscriber::filter::EnvFilter::try_from_default_env()
                  .unwrap_or_else(|_| "debug".into())
          )
          .init();
  }
  
  // 网络服务示例 | Network service example
  pub struct NetworkService {
      name: String,
  }
  
  impl NetworkService {
      pub fn new(name: String) -> Self {
          Self { name }
      }
      
      // 使用instrument宏自动创建span | Use instrument macro to automatically create span
      #[instrument(skip(self), fields(service_name = %self.name))]
      pub async fn process_request(&self, request_id: u32, user_id: u32) -> Result<String, NetworkError> {
          info!("开始处理请求 | Starting request processing");
          
          // 验证用户 | Validate user
          self.validate_user(user_id).await?;
          
          // 处理业务逻辑 | Process business logic
          let result = self.business_logic(request_id).await?;
          
          // 记录成功处理 | Log successful processing
          info!(result = %result, "请求处理完成 | Request processing completed");
          
          Ok(result)
      }
      
      #[instrument(skip(self), fields(service = %self.name))]
      async fn validate_user(&self, user_id: u32) -> Result<(), NetworkError> {
          let span = info_span!("user_validation", user_id = user_id);
          
          async move {
              debug!("验证用户权限 | Validating user permissions");
              
              // 模拟数据库查询 | Simulate database query
              self.query_database("users", user_id).await?;
              
              // 模拟权限检查 | Simulate permission check
              if user_id == 999 {
                  warn!("用户权限不足 | User has insufficient permissions");
                  return Err(NetworkError::Authentication);
              }
              
              info!("用户验证通过 | User validation passed");
              Ok(())
          }
          .instrument(span)
          .await
      }
      
      #[instrument(skip(self), fields(service = %self.name, request_id = request_id))]
      async fn business_logic(&self, request_id: u32) -> Result<String, NetworkError> {
          info!("执行业务逻辑 | Executing business logic");
          
          // 并行执行多个子任务 | Execute multiple subtasks in parallel
          let (data1, data2, data3) = tokio::try_join!(
              self.fetch_data("service_a", request_id),
              self.fetch_data("service_b", request_id),
              self.fetch_data("service_c", request_id)
          )?;
          
          let result = format!("合并结果: {}, {}, {} | Combined result: {}, {}, {}", 
                              data1, data2, data3, data1, data2, data3);
          
          info!(
              data_count = 3,
              result_length = result.len(),
              "业务逻辑执行完成 | Business logic execution completed"
          );
          
          Ok(result)
      }
      
      #[instrument(skip(self), fields(service = %self.name, target_service = service_name))]
      async fn fetch_data(&self, service_name: &str, request_id: u32) -> Result<String, NetworkError> {
          let span = tracing::Span::current();
          span.record("request_id", &request_id);
          
          debug!("开始获取数据 | Starting data fetch");
          
          // 模拟网络延迟 | Simulate network latency
          let delay = match service_name {
              "service_a" => Duration::from_millis(100),
              "service_b" => Duration::from_millis(200),
              "service_c" => Duration::from_millis(150),
              _ => Duration::from_millis(100),
          };
          
          sleep(delay).await;
          
          // 模拟随机错误 | Simulate random errors
          if service_name == "service_b" && request_id % 5 == 0 {
              error!("服务调用失败 | Service call failed");
              return Err(NetworkError::ServiceUnavailable {
                  reason: format!("服务 {} 暂时不可用 | Service {} temporarily unavailable", 
                                 service_name, service_name)
              });
          }
          
          let result = format!("{}的数据 | Data from {}", service_name, service_name);
          debug!(result = %result, "数据获取成功 | Data fetch successful");
          
          Ok(result)
      }
      
      #[instrument(skip(self), fields(service = %self.name, table = table_name, record_id = id))]
      async fn query_database(&self, table_name: &str, id: u32) -> Result<String, NetworkError> {
          debug!("查询数据库 | Querying database");
          
          // 模拟数据库查询延迟 | Simulate database query delay
          sleep(Duration::from_millis(50)).await;
          
          let result = format!("{}表中ID{}的记录 | Record with ID {} from {} table", 
                              table_name, id, id, table_name);
          
          info!(
              table = table_name,
              record_id = id,
              "数据库查询完成 | Database query completed"
          );
          
          Ok(result)
      }
  }
  
  // 错误类型重用前面定义的NetworkError | Reuse previously defined NetworkError type
  
  // HTTP请求处理器 | HTTP request handler
  pub struct HttpHandler {
      network_service: NetworkService,
  }
  
  impl HttpHandler {
      pub fn new() -> Self {
          Self {
              network_service: NetworkService::new("http_service".to_string()),
          }
      }
      
      #[instrument(skip(self), fields(method = method, path = path))]
      pub async fn handle_request(
          &self, 
          method: &str, 
          path: &str, 
          request_id: u32,
          user_id: u32
      ) -> Result<(u16, String), NetworkError> {
          let _span = info_span!(
              "http_request",
              method = method,
              path = path,
              request_id = request_id,
              user_id = user_id
          ).entered();
          
          info!("收到HTTP请求 | Received HTTP request");
          
          let start_time = std::time::Instant::now();
          
          let result = match path {
              "/api/health" => {
                  info!("健康检查请求 | Health check request");
                  Ok((200, "健康 | Healthy".to_string()))
              }
              "/api/data" => {
                  info!("数据请求 | Data request");
                  match self.network_service.process_request(request_id, user_id).await {
                      Ok(data) => Ok((200, data)),
                      Err(NetworkError::Authentication) => Ok((401, "未授权 | Unauthorized".to_string())),
                      Err(e) => {
                          error!(error = %e, "请求处理失败 | Request processing failed");
                          Ok((500, "内部服务器错误 | Internal Server Error".to_string()))
                      }
                  }
              }
              _ => {
                  warn!(path = path, "未找到路径 | Path not found");
                  Ok((404, "未找到 | Not Found".to_string()))
              }
          };
          
          let duration = start_time.elapsed();
          
          match &result {
              Ok((status_code, _)) => {
                  info!(
                      status_code = *status_code,
                      duration_ms = duration.as_millis() as u64,
                      "请求处理完成 | Request processing completed"
                  );
              }
              Err(e) => {
                  error!(
                      error = %e,
                      duration_ms = duration.as_millis() as u64,
                      "请求处理出错 | Request processing error"
                  );
              }
          }
          
          result
      }
  }
  
  // Tracing演示函数 | Tracing demonstration function
  pub async fn demonstrate_tracing() {
      println!("=== Tracing系统演示 | Tracing System Demo ===");
      
      init_tracing();
      
      let handler = HttpHandler::new();
      
      // 模拟多个并发请求 | Simulate multiple concurrent requests
      let requests = vec![
          ("GET", "/api/health", 1, 100),
          ("GET", "/api/data", 2, 200),
          ("GET", "/api/data", 3, 999), // 这个会权限失败 | This will fail authorization
          ("GET", "/api/data", 4, 300),
          ("GET", "/api/nonexistent", 5, 400),
          ("POST", "/api/data", 10, 500), // 这个可能会触发service_b错误 | This might trigger service_b error
      ];
      
      // 并发处理所有请求 | Process all requests concurrently
      let mut handles = Vec::new();
      
      for (method, path, request_id, user_id) in requests {
          let handler = &handler;
          let handle = tokio::spawn(async move {
              let result = handler.handle_request(method, path, request_id, user_id).await;
              (request_id, result)
          });
          handles.push(handle);
      }
      
      // 等待所有请求完成 | Wait for all requests to complete
      for handle in handles {
          match handle.await {
              Ok((request_id, result)) => {
                  match result {
                      Ok((status, body)) => {
                          println!("请求 {} 完成: {} - {} | Request {} completed: {} - {}", 
                                  request_id, status, body, request_id, status, body);
                      }
                      Err(e) => {
                          println!("请求 {} 失败: {:?} | Request {} failed: {:?}", 
                                  request_id, e, request_id, e);
                      }
                  }
              }
              Err(e) => {
                  println!("任务执行失败: {:?} | Task execution failed: {:?}", e, e);
              }
          }
      }
      
      // 等待一段时间让所有tracing数据输出 | Wait for all tracing data to output
      sleep(Duration::from_millis(100)).await;
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - instrument宏的主要好处是什么？| What are the main benefits of the instrument macro?
    **答案 | Answer:** 自动创建span，减少样板代码，提供函数级别追踪 | Automatically creates spans, reduces boilerplate, provides function-level tracing
  - 如何在异步任务中传递tracing上下文？| How to propagate tracing context in async tasks?
    **答案 | Answer:** 使用.instrument(span)或在spawn时捕获当前span | Use .instrument(span) or capture current span when spawning
  
  **常见误区检查 | Common Misconception Checks:**
  - tracing是否会显著影响性能？| Does tracing significantly impact performance?
    **答案 | Answer:** 正确配置下影响很小，可以在编译时优化掉 | With proper configuration, impact is minimal, can be optimized away at compile time
  - 是否应该为每个函数都添加tracing？| Should tracing be added to every function?
    **答案 | Answer:** 否，应该聚焦于关键路径和边界 | No, should focus on critical paths and boundaries

### 5. 结构化日志设计实践 | Structured Logging Design Practice (45分钟 | 45 minutes)

- **生产级日志架构设计 | Production-Grade Log Architecture Design**
  
  **关键原则 | Key Principles:**
  - 一致性：统一的日志格式和字段命名 | Consistency: unified log format and field naming
  - 可搜索性：结构化字段便于查询和分析 | Searchability: structured fields for easy querying and analysis
  - 性能考虑：异步日志避免阻塞主线程 | Performance consideration: async logging to avoid blocking main thread
  
  **实践验证问题 | Practice Verification Questions:**
  1. 什么是结构化日志的最佳实践？| What are the best practices for structured logging?
     **答案 | Answer:** 使用一致的字段名，包含足够上下文，避免敏感信息 | Use consistent field names, include sufficient context, avoid sensitive information
  2. 如何平衡日志详细程度和性能？| How to balance log detail level and performance?
     **答案 | Answer:** 根据环境调整级别，使用异步写入，避免过度日志 | Adjust level by environment, use async writing, avoid excessive logging
  3. 生产环境日志应该包含哪些关键信息？| What key information should production logs include?
     **答案 | Answer:** 时间戳、请求ID、用户ID、操作结果、耗时等 | Timestamp, request ID, user ID, operation result, duration, etc.

### 6. 日志和追踪集成最佳实践 | Logging and Tracing Integration Best Practices (15分钟 | 15 minutes)

- **可观测性统一架构 | Unified Observability Architecture**
  
  **综合概念检查 | Comprehensive Concept Check:**
  1. 如何在微服务架构中实现统一日志？| How to implement unified logging in microservice architecture?
     **答案 | Answer:** 使用统一格式，服务发现，中央化日志聚合 | Use unified format, service discovery, centralized log aggregation
  2. 日志、指标和追踪的关系是什么？| What's the relationship between logs, metrics, and traces?
     **答案 | Answer:** 三者构成可观测性的三大支柱，相互补充 | Three pillars of observability, complementing each other
  3. 如何处理敏感信息的日志记录？| How to handle logging of sensitive information?
     **答案 | Answer:** 使用脱敏、加密或避免记录敏感字段 | Use desensitization, encryption, or avoid logging sensitive fields
  4. 生产环境日志监控的最佳实践是什么？| What are the best practices for production log monitoring?
     **答案 | Answer:** 实时告警、日志聚合、异常检测、容量规划 | Real-time alerting, log aggregation, anomaly detection, capacity planning
  5. 如何设计可扩展的日志系统？| How to design a scalable logging system?
     **答案 | Answer:** 分布式存储、负载均衡、数据分区、异步处理 | Distributed storage, load balancing, data partitioning, async processing

## 实践项目：带日志的网络服务 | Practical Project: Network Service with Logging

### 目标 | Objective
构建一个具备完整错误处理和日志系统的网络服务，综合应用错误处理策略、结构化日志记录和分布式追踪技术，实现生产级的可观测性。 | Build a network service with complete error handling and logging system, comprehensively applying error handling strategies, structured logging, and distributed tracing technologies to achieve production-level observability.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 网络错误应该如何分类和处理？| How should network errors be classified and handled?
   **答案 | Answer:** 按瞬时性分类，使用重试和降级策略 | Classify by transience, use retry and fallback strategies
2. 结构化日志的核心要素是什么？| What are the core elements of structured logging?
   **答案 | Answer:** 一致的格式、丰富的上下文、可搜索的字段 | Consistent format, rich context, searchable fields
3. tracing如何提升系统可观测性？| How does tracing improve system observability?
   **答案 | Answer:** 提供调用链、性能数据和上下文关联 | Provides call chains, performance data, and contextual correlation

### 步骤 | Steps
1. 设计错误处理架构，定义网络错误类型和处理策略 | Design error handling architecture, define network error types and handling strategies
2. 实现结构化日志系统，支持多种输出格式和目标 | Implement structured logging system supporting multiple output formats and targets
3. 集成tracing系统，提供分布式追踪能力 | Integrate tracing system to provide distributed tracing capabilities
4. 构建网络服务，演示错误处理和日志记录的实际应用 | Build network service demonstrating practical application of error handling and logging
5. 添加监控和告警机制，完善可观测性体系 | Add monitoring and alerting mechanisms to complete observability system

### 示例代码 | Example Code
```rust
"""
带日志的网络服务 | Network Service with Logging
生产级网络服务错误处理和可观测性实现 | Production-grade network service error handling and observability implementation

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 网络错误分类和处理策略 | Network error classification and handling strategies
- 结构化日志记录和配置管理 | Structured logging and configuration management
- 分布式追踪和性能监控 | Distributed tracing and performance monitoring
- 生产级可观测性最佳实践 | Production-level observability best practices
"""

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, error, info, info_span, instrument, warn, Instrument};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use serde_json::json;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use thiserror::Error;

// 错误类型定义 | Error type definitions
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("IO错误: {0} | IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("解析错误: {message} | Parse error: {message}")]
    Parse { message: String },
    
    #[error("业务逻辑错误: {code} - {message} | Business logic error: {code} - {message}")]
    Business { code: String, message: String },
    
    #[error("外部服务错误: {service} - {reason} | External service error: {service} - {reason}")]
    ExternalService { service: String, reason: String },
    
    #[error("限流错误: 超过速率限制 | Rate limit error: rate limit exceeded")]
    RateLimit,
    
    #[error("认证错误: {message} | Authentication error: {message}")]
    Authentication { message: String },
}

impl ServiceError {
    pub fn error_code(&self) -> &str {
        match self {
            ServiceError::Io(_) => "IO_ERROR",
            ServiceError::Parse { .. } => "PARSE_ERROR",
            ServiceError::Business { .. } => "BUSINESS_ERROR",
            ServiceError::ExternalService { .. } => "EXTERNAL_SERVICE_ERROR",
            ServiceError::RateLimit => "RATE_LIMIT_ERROR",
            ServiceError::Authentication { .. } => "AUTH_ERROR",
        }
    }
    
    pub fn http_status(&self) -> u16 {
        match self {
            ServiceError::Io(_) => 500,
            ServiceError::Parse { .. } => 400,
            ServiceError::Business { .. } => 422,
            ServiceError::ExternalService { .. } => 502,
            ServiceError::RateLimit => 429,
            ServiceError::Authentication { .. } => 401,
        }
    }
    
    pub fn is_retryable(&self) -> bool {
        matches!(self, ServiceError::Io(_) | ServiceError::ExternalService { .. })
    }
}

// 请求上下文 | Request context
#[derive(Debug, Clone)]
pub struct RequestContext {
    pub request_id: String,
    pub client_ip: String,
    pub user_agent: Option<String>,
    pub user_id: Option<String>,
    pub start_time: Instant,
}

impl RequestContext {
    pub fn new(request_id: String, client_ip: String) -> Self {
        Self {
            request_id,
            client_ip,
            user_agent: None,
            user_id: None,
            start_time: Instant::now(),
        }
    }
    
    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }
}

// HTTP请求解析 | HTTP request parsing
#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    #[instrument(skip(data))]
    pub fn parse(data: &str) -> Result<Self, ServiceError> {
        debug!(data_length = data.len(), "解析HTTP请求 | Parsing HTTP request");
        
        let lines: Vec<&str> = data.split("\r\n").collect();
        if lines.is_empty() {
            return Err(ServiceError::Parse {
                message: "空请求数据 | Empty request data".to_string()
            });
        }
        
        // 解析请求行 | Parse request line
        let request_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
        if request_line_parts.len() != 3 {
            return Err(ServiceError::Parse {
                message: "无效的请求行格式 | Invalid request line format".to_string()
            });
        }
        
        let method = request_line_parts[0].to_string();
        let path = request_line_parts[1].to_string();
        let version = request_line_parts[2].to_string();
        
        // 解析头部 | Parse headers
        let mut headers = HashMap::new();
        let mut i = 1;
        while i < lines.len() && !lines[i].is_empty() {
            if let Some(colon_pos) = lines[i].find(':') {
                let key = lines[i][..colon_pos].trim().to_lowercase();
                let value = lines[i][colon_pos + 1..].trim().to_string();
                headers.insert(key, value);
            }
            i += 1;
        }
        
        // 解析消息体 | Parse body
        let body = if i + 1 < lines.len() {
            lines[i + 1..].join("\r\n")
        } else {
            String::new()
        };
        
        let request = HttpRequest {
            method,
            path,
            version,
            headers,
            body,
        };
        
        debug!(
            method = %request.method,
            path = %request.path,
            headers_count = request.headers.len(),
            body_length = request.body.len(),
            "HTTP请求解析完成 | HTTP request parsing completed"
        );
        
        Ok(request)
    }
}

// HTTP响应构建 | HTTP response building
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status_code: u16) -> Self {
        let status_text = match status_code {
            200 => "OK",
            400 => "Bad Request",
            401 => "Unauthorized",
            404 => "Not Found",
            422 => "Unprocessable Entity",
            429 => "Too Many Requests",
            500 => "Internal Server Error",
            502 => "Bad Gateway",
            _ => "Unknown",
        }.to_string();
        
        let mut headers = HashMap::new();
        headers.insert("server".to_string(), "rust-network-service/1.0".to_string());
        headers.insert("content-type".to_string(), "application/json".to_string());
        
        Self {
            status_code,
            status_text,
            headers,
            body: String::new(),
        }
    }
    
    pub fn with_body(mut self, body: String) -> Self {
        self.headers.insert("content-length".to_string(), body.len().to_string());
        self.body = body;
        self
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        response.push_str("\r\n");
        response.push_str(&self.body);
        
        response.into_bytes()
    }
}

// 业务逻辑服务 | Business logic service
pub struct BusinessService {
    name: String,
}

impl BusinessService {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    
    #[instrument(skip(self), fields(service = %self.name))]
    pub async fn process_data(&self, data: &str, ctx: &RequestContext) -> Result<serde_json::Value, ServiceError> {
        info!(
            request_id = %ctx.request_id,
            data_length = data.len(),
            "开始处理业务数据 | Starting business data processing"
        );
        
        // 模拟数据验证 | Simulate data validation
        if data.is_empty() {
            return Err(ServiceError::Business {
                code: "EMPTY_DATA".to_string(),
                message: "数据不能为空 | Data cannot be empty".to_string(),
            });
        }
        
        // 模拟外部服务调用 | Simulate external service call
        let external_data = self.call_external_service("user-service", &ctx.request_id).await?;
        
        // 模拟业务计算 | Simulate business computation
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        let result = json!({
            "processed_data": data,
            "external_data": external_data,
            "processed_at": chrono::Utc::now().to_rfc3339(),
            "service": self.name
        });
        
        info!(
            request_id = %ctx.request_id,
            result_size = result.to_string().len(),
            "业务数据处理完成 | Business data processing completed"
        );
        
        Ok(result)
    }
    
    #[instrument(skip(self), fields(service = %self.name, target_service = service_name))]
    async fn call_external_service(&self, service_name: &str, request_id: &str) -> Result<String, ServiceError> {
        debug!(
            request_id = request_id,
            target = service_name,
            "调用外部服务 | Calling external service"
        );
        
        // 模拟网络延迟 | Simulate network latency
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // 模拟偶发错误 | Simulate occasional errors
        if request_id.ends_with("error") {
            return Err(ServiceError::ExternalService {
                service: service_name.to_string(),
                reason: "服务暂时不可用 | Service temporarily unavailable".to_string(),
            });
        }
        
        let response = format!("{}的响应数据 | Response data from {}", service_name, service_name);
        
        debug!(
            request_id = request_id,
            target = service_name,
            response_length = response.len(),
            "外部服务调用成功 | External service call successful"
        );
        
        Ok(response)
    }
}

// 网络服务器 | Network server
pub struct NetworkServer {
    address: String,
    business_service: Arc<BusinessService>,
}

impl NetworkServer {
    pub fn new(address: String) -> Self {
        Self {
            address,
            business_service: Arc::new(BusinessService::new("main-service".to_string())),
        }
    }
    
    #[instrument(skip(self), fields(server_address = %self.address))]
    pub async fn start(&self) -> Result<(), ServiceError> {
        info!("启动网络服务器 | Starting network server at {}", self.address);
        
        let listener = TcpListener::bind(&self.address).await?;
        info!("服务器监听端口: {} | Server listening on: {}", self.address, self.address);
        
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let business_service = Arc::clone(&self.business_service);
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, addr.to_string(), business_service).await {
                            error!(
                                client_addr = %addr,
                                error = %e,
                                "连接处理失败 | Connection handling failed"
                            );
                        }
                    });
                }
                Err(e) => {
                    error!(error = %e, "接受连接失败 | Failed to accept connection");
                }
            }
        }
    }
    
    #[instrument(skip(stream, business_service), fields(client_addr = %client_addr))]
    async fn handle_connection(
        mut stream: TcpStream, 
        client_addr: String, 
        business_service: Arc<BusinessService>
    ) -> Result<(), ServiceError> {
        let request_id = uuid::Uuid::new_v4().to_string();
        let ctx = RequestContext::new(request_id.clone(), client_addr.clone());
        
        let span = info_span!(
            "http_connection",
            request_id = %ctx.request_id,
            client_ip = %ctx.client_ip
        );
        
        async move {
            info!("处理新连接 | Handling new connection");
            
            // 读取请求数据 | Read request data
            let mut buffer = vec![0; 4096];
            let bytes_read = match tokio::time::timeout(Duration::from_secs(30), stream.read(&mut buffer)).await {
                Ok(Ok(n)) => n,
                Ok(Err(e)) => return Err(ServiceError::Io(e)),
                Err(_) => {
                    warn!("请求读取超时 | Request read timeout");
                    return Err(ServiceError::Io(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "请求读取超时 | Request read timeout"
                    )));
                }
            };
            
            if bytes_read == 0 {
                debug!("客户端关闭连接 | Client closed connection");
                return Ok(());
            }
            
            let request_data = String::from_utf8_lossy(&buffer[..bytes_read]);
            debug!(
                bytes_read = bytes_read,
                "接收到请求数据 | Received request data"
            );
            
            // 解析HTTP请求 | Parse HTTP request
            let request = HttpRequest::parse(&request_data)?;
            
            // 处理请求 | Process request
            let response = Self::process_request(request, &ctx, business_service).await;
            
            // 发送响应 | Send response
            let response_bytes = response.to_bytes();
            if let Err(e) = stream.write_all(&response_bytes).await {
                error!(error = %e, "发送响应失败 | Failed to send response");
                return Err(ServiceError::Io(e));
            }
            
            // 记录请求完成 | Log request completion
            info!(
                method = request.method,
                path = request.path,
                status_code = response.status_code,
                duration_ms = ctx.duration().as_millis() as u64,
                response_size = response_bytes.len(),
                "请求处理完成 | Request processing completed"
            );
            
            Ok(())
        }
        .instrument(span)
        .await
    }
    
    #[instrument(skip(request, business_service), fields(
        method = %request.method,
        path = %request.path,
        request_id = %ctx.request_id
    ))]
    async fn process_request(
        request: HttpRequest,
        ctx: &RequestContext,
        business_service: Arc<BusinessService>
    ) -> HttpResponse {
        info!("处理HTTP请求 | Processing HTTP request");
        
        let result = match (request.method.as_str(), request.path.as_str()) {
            ("GET", "/health") => {
                debug!("健康检查请求 | Health check request");
                let health_info = json!({
                    "status": "healthy",
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "service": "network-service"
                });
                Ok(HttpResponse::new(200).with_body(health_info.to_string()))
            }
            ("POST", "/api/process") => {
                debug!("数据处理请求 | Data processing request");
                match business_service.process_data(&request.body, ctx).await {
                    Ok(result) => {
                        let response_body = json!({
                            "success": true,
                            "data": result,
                            "request_id": ctx.request_id
                        });
                        Ok(HttpResponse::new(200).with_body(response_body.to_string()))
                    }
                    Err(e) => {
                        error!(
                            error = %e,
                            error_code = e.error_code(),
                            "业务处理失败 | Business processing failed"
                        );
                        
                        let error_response = json!({
                            "success": false,
                            "error": {
                                "code": e.error_code(),
                                "message": e.to_string()
                            },
                            "request_id": ctx.request_id
                        });
                        
                        Ok(HttpResponse::new(e.http_status()).with_body(error_response.to_string()))
                    }
                }
            }
            (method, path) => {
                warn!(
                    method = method,
                    path = path,
                    "未找到路由 | Route not found"
                );
                
                let not_found_response = json!({
                    "success": false,
                    "error": {
                        "code": "NOT_FOUND",
                        "message": format!("路径未找到: {} {} | Path not found: {} {}", method, path, method, path)
                    },
                    "request_id": ctx.request_id
                });
                
                Ok(HttpResponse::new(404).with_body(not_found_response.to_string()))
            }
        };
        
        match result {
            Ok(response) => response,
            Err(e) => {
                error!(
                    error = %e,
                    "请求处理出现未预期错误 | Unexpected error in request processing"
                );
                
                let error_response = json!({
                    "success": false,
                    "error": {
                        "code": "INTERNAL_ERROR",
                        "message": "内部服务器错误 | Internal server error"
                    },
                    "request_id": ctx.request_id
                });
                
                HttpResponse::new(500).with_body(error_response.to_string())
            }
        }
    }
}

// 日志配置初始化 | Logging configuration initialization
pub fn init_logging() {
    // 创建多层subscriber | Create multi-layer subscriber
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .json(); // 使用JSON格式 | Use JSON format
    
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(
            tracing_subscriber::filter::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into())
        )
        .init();
    
    info!("日志系统初始化完成 | Logging system initialized");
}

// 主函数 | Main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统 | Initialize logging system
    init_logging();
    
    info!("启动带日志的网络服务 | Starting network service with logging");
    
    // 创建并启动服务器 | Create and start server
    let server = NetworkServer::new("127.0.0.1:8080".to_string());
    
    // 在另一个任务中启动服务器 | Start server in another task
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            error!(error = %e, "服务器启动失败 | Server startup failed");
        }
    });
    
    // 等待一段时间让服务器启动 | Wait for server to start
    tokio::time::sleep(Duration::from_millis(1000)).await;
    
    // 发送测试请求 | Send test requests
    test_client().await?;
    
    // 等待服务器任务 | Wait for server task
    server_handle.await?;
    
    Ok(())
}

// 测试客户端 | Test client
async fn test_client() -> Result<(), Box<dyn std::error::Error>> {
    info!("开始客户端测试 | Starting client tests");
    
    let test_requests = vec![
        ("GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n", "健康检查 | Health check"),
        ("POST /api/process HTTP/1.1\r\nHost: localhost\r\nContent-Length: 26\r\n\r\n{\"data\": \"test data\"}", "正常处理 | Normal processing"),
        ("POST /api/process HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n", "空数据测试 | Empty data test"),
        ("GET /nonexistent HTTP/1.1\r\nHost: localhost\r\n\r\n", "404测试 | 404 test"),
    ];
    
    for (i, (request_data, description)) in test_requests.iter().enumerate() {
        info!(test_case = i + 1, description = description, "发送测试请求 | Sending test request");
        
        match TcpStream::connect("127.0.0.1:8080").await {
            Ok(mut stream) => {
                if let Err(e) = stream.write_all(request_data.as_bytes()).await {
                    error!(error = %e, "发送请求失败 | Failed to send request");
                    continue;
                }
                
                let mut response = Vec::new();
                if let Err(e) = stream.read_to_end(&mut response).await {
                    error!(error = %e, "读取响应失败 | Failed to read response");
                    continue;
                }
                
                let response_str = String::from_utf8_lossy(&response);
                debug!(
                    test_case = i + 1,
                    response_length = response.len(),
                    "收到响应 | Received response"
                );
                
                // 只显示响应的前200个字符 | Only show first 200 characters of response
                let preview = if response_str.len() > 200 {
                    format!("{}...", &response_str[..200])
                } else {
                    response_str.to_string()
                };
                
                info!(
                    test_case = i + 1,
                    description = description,
                    response_preview = preview,
                    "测试完成 | Test completed"
                );
            }
            Err(e) => {
                error!(
                    test_case = i + 1,
                    error = %e,
                    "连接失败 | Connection failed"
                );
            }
        }
        
        // 在测试之间稍作等待 | Wait between tests
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    info!("客户端测试完成 | Client tests completed");
    Ok(())
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否实现了完整的错误处理体系？| Does the project implement a complete error handling system?
2. 结构化日志是否提供了足够的上下文信息？| Do structured logs provide sufficient contextual information?
3. tracing系统是否正确追踪了请求的完整生命周期？| Does the tracing system correctly track the complete request lifecycle?
4. 错误处理策略是否适合生产环境使用？| Are the error handling strategies suitable for production use?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **错误重试机制优化练习 | Error Retry Mechanism Optimization Exercise**
   - **练习描述 | Exercise Description:** 实现带有熔断器和自适应退避的高级重试机制
   - **概念检查 | Concept Check:** 如何避免重试风暴和级联故障？
   - **学习目标 | Learning Objective:** 掌握生产级错误恢复策略

2. **分布式日志聚合练习 | Distributed Log Aggregation Exercise**
   - **练习描述 | Exercise Description:** 设计多服务日志收集和分析系统
   - **概念检查 | Concept Check:** 如何处理大量日志数据的存储和查询？
   - **学习目标 | Learning Objective:** 理解大规模日志系统架构

3. **性能监控仪表板练习 | Performance Monitoring Dashboard Exercise**
   - **练习描述 | Exercise Description:** 构建实时性能指标收集和展示系统
   - **概念检查 | Concept Check:** 哪些指标对系统健康最关键？
   - **学习目标 | Learning Objective:** 掌握系统可观测性设计

4. **日志安全脱敏练习 | Log Security Desensitization Exercise**
   - **练习描述 | Exercise Description:** 实现敏感信息自动识别和脱敏机制
   - **概念检查 | Concept Check:** 如何在保持调试能力的同时保护隐私？
   - **学习目标 | Learning Objective:** 学习安全日志记录最佳实践

5. **异常检测算法练习 | Anomaly Detection Algorithm Exercise**
   - **练习描述 | Exercise Description:** 基于日志模式实现异常行为检测
   - **概念检查 | Concept Check:** 如何区分正常波动和真实异常？
   - **学习目标 | Learning Objective:** 了解智能运维技术

6. **多租户日志隔离练习 | Multi-tenant Log Isolation Exercise**
   - **练习描述 | Exercise Description:** 设计SaaS环境下的日志隔离和访问控制
   - **概念检查 | Concept Check:** 如何确保租户数据的安全隔离？
   - **学习目标 | Learning Objective:** 掌握多租户架构设计

7. **云原生日志架构练习 | Cloud-Native Logging Architecture Exercise**
   - **练习描述 | Exercise Description:** 构建适合Kubernetes环境的日志收集方案
   - **概念检查 | Concept Check:** 容器化环境下的日志收集有什么特殊考虑？
   - **学习目标 | Learning Objective:** 理解云原生运维最佳实践

## 学习资源 | Learning Resources
- [Rust错误处理指南 - The Rust Programming Language Book](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Tracing框架文档](https://docs.rs/tracing/)
- [生产级日志记录最佳实践](https://12factor.net/logs)
- [可观测性工程实践指南](https://www.honeycomb.io/guide-achieving-observability/)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解Rust错误处理核心概念和最佳实践 | Understand Rust error handling core concepts and best practices
- [ ] 掌握网络错误分类和处理策略 | Master network error classification and handling strategies
- [ ] 熟练使用log生态系统进行日志记录 | Proficiently use log ecosystem for logging
- [ ] 深入应用tracing系统进行分布式追踪 | Deeply apply tracing system for distributed tracing
- [ ] 设计和实现结构化日志系统 | Design and implement structured logging system
- [ ] 构建生产级网络服务错误处理架构 | Build production-grade network service error handling architecture
- [ ] 正确回答所有CCQs概念检查问题 | Correctly answer all CCQs concept checking questions
- [ ] 理解可观测性在生产环境中的重要性 | Understand the importance of observability in production environments
- [ ] 掌握日志性能优化和安全考虑 | Master log performance optimization and security considerations
- [ ] 完成至少3个扩展练习 | Complete at least 3 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释错误处理策略、日志系统设计和可观测性最佳实践。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain error handling strategies, logging system design, and observability best practices to others.