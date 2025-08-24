# Rust入门 - 第10天：第一个里程碑项目 | Rust Introduction - Day 10: First Milestone Project

## 学习目标 | Learning Objectives
- 综合应用前9天所学的网络编程、异步处理、HTTP解析和错误处理知识 | Comprehensively apply the knowledge of network programming, async processing, HTTP parsing, and error handling learned in the first 9 days
- 构建一个完整的多线程HTTP服务器，支持基本的Web服务功能 | Build a complete multi-threaded HTTP server supporting basic web service functions
- 实现静态文件服务、请求路由和响应生成机制 | Implement static file serving, request routing, and response generation mechanisms
- 掌握生产级服务器的基础错误处理和日志记录策略 | Master basic error handling and logging strategies for production-grade servers
- 理解多线程模型与异步模型在HTTP服务器中的应用场景 | Understand application scenarios of multi-threaded vs async models in HTTP servers
- 建立对Web服务器架构和性能优化的初步认知 | Establish initial understanding of web server architecture and performance optimization

## 详细内容 | Detailed Content

### 1. HTTP服务器架构设计 | HTTP Server Architecture Design (1.5小时 | 1.5 hours)

- **多线程HTTP服务器架构 | Multi-threaded HTTP Server Architecture**
  
  **概念定义 | Concept Definition:**
  多线程HTTP服务器是一种使用线程池处理并发请求的服务器架构模式，每个连接或请求由独立的线程处理，通过线程间的并行执行来提高服务器的吞吐量和响应能力。 | A multi-threaded HTTP server is a server architecture pattern that uses thread pools to handle concurrent requests, where each connection or request is processed by an independent thread, improving server throughput and responsiveness through parallel execution between threads.
  
  **核心特征 | Key Characteristics:**
  - 线程池管理：预分配固定数量的工作线程，避免频繁创建销毁线程的开销 | Thread pool management: pre-allocate a fixed number of worker threads to avoid the overhead of frequent thread creation and destruction
  - 请求分发机制：将传入的HTTP请求分配给空闲线程进行处理 | Request dispatch mechanism: distribute incoming HTTP requests to idle threads for processing
  - 资源隔离：每个线程拥有独立的栈空间，避免数据竞争问题 | Resource isolation: each thread has independent stack space, avoiding data race issues
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 多线程服务器中，每个HTTP请求都需要创建新线程吗？| Does a multi-threaded server need to create a new thread for each HTTP request?
     **答案 | Answer:** 否 | No - 使用线程池复用线程，避免创建开销 | Uses thread pool to reuse threads, avoiding creation overhead
  2. 线程池的大小应该设置为无限大吗？| Should the thread pool size be set to unlimited?
     **答案 | Answer:** 否 | No - 需要根据系统资源和负载特性合理配置 | Need to configure reasonably based on system resources and load characteristics
  3. 多线程模型比异步模型在所有场景下都更好吗？| Is the multi-threaded model always better than the async model in all scenarios?
     **答案 | Answer:** 否 | No - 取决于I/O密集程度和并发需求 | Depends on I/O intensity and concurrency requirements
  4. 多个线程可以同时修改共享的服务器配置吗？| Can multiple threads simultaneously modify shared server configuration?
     **答案 | Answer:** 否 | No - 需要同步机制保护共享资源 | Need synchronization mechanisms to protect shared resources
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::thread;
  use std::sync::{Arc, Mutex};
  use std::net::{TcpListener, TcpStream};
  use std::sync::mpsc;
  
  // 线程池结构设计 | Thread pool structure design
  pub struct ThreadPool {
      workers: Vec<Worker>,           // 工作线程集合 | Collection of worker threads
      sender: mpsc::Sender<Job>,      // 任务发送器 | Task sender
  }
  
  type Job = Box<dyn FnOnce() + Send + 'static>;
  
  impl ThreadPool {
      // 创建指定大小的线程池 | Create thread pool with specified size
      pub fn new(size: usize) -> Result<ThreadPool, &'static str> {
          if size == 0 {
              return Err("Thread pool size must be greater than 0");
          }
          
          let (sender, receiver) = mpsc::channel();
          let receiver = Arc::new(Mutex::new(receiver));
          
          let mut workers = Vec::with_capacity(size);
          for id in 0..size {
              workers.push(Worker::new(id, Arc::clone(&receiver)));
          }
          
          Ok(ThreadPool { workers, sender })
      }
      
      // 执行任务 | Execute task
      pub fn execute<F>(&self, f: F)
      where
          F: FnOnce() + Send + 'static,
      {
          let job = Box::new(f);
          self.sender.send(job).unwrap();
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这个线程池设计中，为什么使用Arc<Mutex<Receiver>>？| Why use Arc<Mutex<Receiver>> in this thread pool design?
    **答案 | Answer:** 允许多个worker线程安全共享同一个receiver | Allows multiple worker threads to safely share the same receiver
  - 如果线程池大小为0会发生什么？| What happens if the thread pool size is 0?
    **答案 | Answer:** 返回错误，因为无法处理任何请求 | Returns an error because it cannot handle any requests
  
  **常见误区检查 | Common Misconception Checks:**
  - 线程越多服务器性能越好吗？| Do more threads always mean better server performance?
    **答案 | Answer:** 否，过多线程会导致上下文切换开销增加 | No, too many threads lead to increased context switching overhead
  - 每个HTTP连接都需要独立的线程吗？| Does each HTTP connection need an independent thread?
    **答案 | Answer:** 不一定，可以用线程池复用线程处理多个连接 | Not necessarily, can use thread pool to reuse threads for multiple connections

- **请求路由系统 | Request Routing System**
  
  **概念定义 | Concept Definition:**
  请求路由系统是HTTP服务器中负责根据请求的URL路径、HTTP方法等信息，将请求分发到对应处理函数的核心组件，它定义了URL模式与处理逻辑之间的映射关系。 | A request routing system is the core component in an HTTP server responsible for dispatching requests to corresponding handler functions based on request URL paths, HTTP methods, and other information, defining the mapping relationship between URL patterns and processing logic.
  
  **核心特征 | Key Characteristics:**
  - 路径匹配：支持精确匹配、前缀匹配和通配符匹配等多种模式 | Path matching: supports exact matching, prefix matching, wildcard matching, and other patterns
  - 方法过滤：区分GET、POST、PUT、DELETE等不同HTTP方法 | Method filtering: distinguishes between different HTTP methods like GET, POST, PUT, DELETE
  - 参数提取：从URL中提取动态参数用于处理逻辑 | Parameter extraction: extract dynamic parameters from URLs for processing logic
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 路由系统必须支持正则表达式匹配吗？| Must a routing system support regex matching?
     **答案 | Answer:** 否 | No - 基础路由可以用简单的字符串匹配实现 | Basic routing can be implemented with simple string matching
  2. 同一个URL路径可以支持多个HTTP方法吗？| Can the same URL path support multiple HTTP methods?
     **答案 | Answer:** 是 | Yes - 通过方法过滤区分不同的处理逻辑 | Different processing logic is distinguished through method filtering
  3. 路由匹配的顺序重要吗？| Does the order of route matching matter?
     **答案 | Answer:** 是 | Yes - 通常按照注册顺序或优先级进行匹配 | Usually matches according to registration order or priority
  4. 路由系统可以完全不使用吗？| Can a routing system be completely omitted?
     **答案 | Answer:** 可以 | Yes - 但会使请求处理逻辑变得复杂和难维护 | But it makes request processing logic complex and hard to maintain

### 2. HTTP协议处理实现 | HTTP Protocol Processing Implementation (1.5小时 | 1.5 hours)

- **HTTP请求解析器 | HTTP Request Parser**
  
  **概念定义 | Concept Definition:**
  HTTP请求解析器是负责将原始TCP数据流解析为结构化HTTP请求对象的组件，它需要处理请求行、请求头、请求体等不同部分，并验证协议格式的正确性。 | An HTTP request parser is a component responsible for parsing raw TCP data streams into structured HTTP request objects, handling different parts like request line, headers, and body, while validating protocol format correctness.
  
  **与基础概念的关系 | Relationship to Basic Concepts:**
  建立在第8天学习的HTTP解析基础上，本节将解析器集成到完整的服务器架构中，处理真实的网络连接和数据流。 | Building on the HTTP parsing foundation learned on Day 8, this section integrates the parser into a complete server architecture, handling real network connections and data streams.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. HTTP请求解析器是否需要处理不完整的数据包？| Does an HTTP request parser need to handle incomplete data packets?
     **答案 | Answer:** 是 | Yes - TCP是流协议，数据可能分多个包到达 | TCP is a stream protocol, data may arrive in multiple packets
  2. 解析器应该如何处理格式错误的HTTP请求？| How should a parser handle malformed HTTP requests?
     **答案 | Answer:** 返回适当的HTTP错误响应码（如400 Bad Request）| Return appropriate HTTP error response code (like 400 Bad Request)
  3. 请求体的大小限制是必需的吗？| Is request body size limit necessary?
     **答案 | Answer:** 是 | Yes - 防止内存耗尽和DoS攻击 | Prevents memory exhaustion and DoS attacks
  4. 解析器需要支持HTTP/2协议吗？| Does the parser need to support HTTP/2 protocol?
     **答案 | Answer:** 不是必需的 | Not required - 可以先实现HTTP/1.1支持 | Can implement HTTP/1.1 support first
  5. 解析失败时应该关闭连接吗？| Should the connection be closed when parsing fails?
     **答案 | Answer:** 取决于错误类型 | Depends on error type - 协议错误通常需要关闭连接 | Protocol errors usually require closing the connection
  
  **复杂代码示例 | Complex Code Examples:**
  ```rust
  use std::collections::HashMap;
  use std::str;
  
  #[derive(Debug, Clone)]
  pub struct HttpRequest {
      pub method: String,
      pub path: String,
      pub version: String,
      pub headers: HashMap<String, String>,
      pub body: Vec<u8>,
  }
  
  pub struct HttpParser {
      buffer: Vec<u8>,
      state: ParserState,
  }
  
  #[derive(Debug, PartialEq)]
  enum ParserState {
      RequestLine,
      Headers,
      Body(usize), // 期望的body长度 | Expected body length
      Complete,
  }
  
  impl HttpParser {
      pub fn new() -> Self {
          HttpParser {
              buffer: Vec::new(),
              state: ParserState::RequestLine,
          }
      }
      
      // 处理新接收的数据 | Process newly received data
      pub fn feed(&mut self, data: &[u8]) -> Result<Option<HttpRequest>, ParseError> {
          self.buffer.extend_from_slice(data);
          self.parse()
      }
      
      fn parse(&mut self) -> Result<Option<HttpRequest>, ParseError> {
          loop {
              match self.state {
                  ParserState::RequestLine => {
                      if let Some(line_end) = self.find_line_end() {
                          let line = str::from_utf8(&self.buffer[..line_end])
                              .map_err(|_| ParseError::InvalidEncoding)?;
                          
                          let mut request = self.parse_request_line(line)?;
                          self.buffer.drain(..line_end + 2); // 移除已解析的数据 | Remove parsed data
                          self.state = ParserState::Headers;
                          continue;
                      } else {
                          return Ok(None); // 需要更多数据 | Need more data
                      }
                  },
                  ParserState::Headers => {
                      // 解析头部的实现... | Header parsing implementation...
                      // 此处简化处理 | Simplified handling here
                      return Ok(None);
                  },
                  ParserState::Body(expected_length) => {
                      if self.buffer.len() >= expected_length {
                          // 完成解析 | Complete parsing
                          self.state = ParserState::Complete;
                          return Ok(Some(self.build_request()));
                      } else {
                          return Ok(None); // 需要更多数据 | Need more data
                      }
                  },
                  ParserState::Complete => {
                      return Err(ParseError::AlreadyComplete);
                  }
              }
          }
      }
      
      fn parse_request_line(&self, line: &str) -> Result<HttpRequest, ParseError> {
          let parts: Vec<&str> = line.split_whitespace().collect();
          if parts.len() != 3 {
              return Err(ParseError::InvalidRequestLine);
          }
          
          Ok(HttpRequest {
              method: parts[0].to_string(),
              path: parts[1].to_string(),
              version: parts[2].to_string(),
              headers: HashMap::new(),
              body: Vec::new(),
          })
      }
      
      fn find_line_end(&self) -> Option<usize> {
          self.buffer.windows(2).position(|window| window == b"\r\n")
      }
      
      fn build_request(&self) -> HttpRequest {
          // 构建完整的请求对象 | Build complete request object
          HttpRequest {
              method: "GET".to_string(), // 简化示例 | Simplified example
              path: "/".to_string(),
              version: "HTTP/1.1".to_string(),
              headers: HashMap::new(),
              body: self.buffer.clone(),
          }
      }
  }
  
  #[derive(Debug)]
  pub enum ParseError {
      InvalidEncoding,
      InvalidRequestLine,
      InvalidHeader,
      BodyTooLarge,
      AlreadyComplete,
  }
  ```
  
  **综合应用检查 | Comprehensive Application Check:**
  - 如何处理keep-alive连接中的多个请求？| How to handle multiple requests in keep-alive connections?
  - 大文件上传时的内存管理策略是什么？| What is the memory management strategy for large file uploads?
  - 解析器的性能瓶颈通常在哪里？| Where are the performance bottlenecks of parsers usually located?

### 3. 静态文件服务实现 | Static File Serving Implementation (1小时 | 1 hour)

- **文件系统访问与安全 | File System Access and Security**
  
  **概念定义 | Concept Definition:**
  静态文件服务是Web服务器的基础功能，负责根据HTTP请求的路径直接返回文件系统中对应的文件内容，需要处理文件读取、MIME类型识别、缓存控制和路径安全等问题。 | Static file serving is a fundamental feature of web servers, responsible for directly returning corresponding file content from the file system based on HTTP request paths, handling file reading, MIME type identification, cache control, and path security.
  
  **解决的问题 | Problems It Solves:**
  - 路径遍历攻击：防止通过../等方式访问服务器上的敏感文件 | Path traversal attacks: prevent accessing sensitive files on the server through ../ and similar methods
  - MIME类型识别：根据文件扩展名返回正确的Content-Type头 | MIME type identification: return correct Content-Type header based on file extension
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 客户端请求"/../../etc/passwd"时应该如何处理？| How should the request "/../../etc/passwd" be handled?
     **答案 | Answer:** 应该拒绝请求并返回403 Forbidden | Should reject the request and return 403 Forbidden
  2. 文件不存在时应该返回什么状态码？| What status code should be returned when a file doesn't exist?
     **答案 | Answer:** 404 Not Found
  3. 大文件应该一次性读入内存吗？| Should large files be read into memory all at once?
     **答案 | Answer:** 否 | No - 应该使用流式读取避免内存耗尽 | Should use streaming reads to avoid memory exhaustion
  4. 文件修改时间可以用于缓存控制吗？| Can file modification time be used for cache control?
     **答案 | Answer:** 是 | Yes - 可以生成ETag或Last-Modified头 | Can generate ETag or Last-Modified headers
  
  **实际应用示例 | Real-world Application Examples:**
  ```rust
  use std::path::{Path, PathBuf};
  use std::fs;
  use std::io::Read;
  
  pub struct StaticFileHandler {
      root_dir: PathBuf,
      default_file: String,
  }
  
  impl StaticFileHandler {
      pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
          StaticFileHandler {
              root_dir: root_dir.as_ref().to_path_buf(),
              default_file: "index.html".to_string(),
          }
      }
      
      // 处理静态文件请求 | Handle static file request
      pub fn handle_request(&self, path: &str) -> Result<FileResponse, FileError> {
          // 安全路径检查 | Security path check
          let safe_path = self.sanitize_path(path)?;
          let file_path = self.root_dir.join(&safe_path);
          
          // 检查路径是否在根目录内 | Check if path is within root directory
          if !file_path.starts_with(&self.root_dir) {
              return Err(FileError::Forbidden);
          }
          
          // 处理目录请求 | Handle directory request
          if file_path.is_dir() {
              let index_path = file_path.join(&self.default_file);
              if index_path.exists() {
                  return self.read_file(&index_path);
              } else {
                  return Err(FileError::NotFound);
              }
          }
          
          // 处理文件请求 | Handle file request
          if file_path.exists() && file_path.is_file() {
              self.read_file(&file_path)
          } else {
              Err(FileError::NotFound)
          }
      }
      
      fn sanitize_path(&self, path: &str) -> Result<PathBuf, FileError> {
          // 移除查询参数和锚点 | Remove query parameters and anchors
          let path = path.split('?').next().unwrap_or(path);
          let path = path.split('#').next().unwrap_or(path);
          
          // URL解码 | URL decode
          let path = urlencoding::decode(path)
              .map_err(|_| FileError::BadRequest)?;
          
          // 规范化路径 | Normalize path
          let path = Path::new(path.as_ref());
          let mut safe_path = PathBuf::new();
          
          for component in path.components() {
              match component {
                  std::path::Component::Normal(name) => {
                      safe_path.push(name);
                  },
                  std::path::Component::ParentDir => {
                      safe_path.pop(); // 安全地处理../ | Safely handle ../
                  },
                  std::path::Component::RootDir | 
                  std::path::Component::CurDir => {
                      // 忽略根目录和当前目录标记 | Ignore root and current directory markers
                  },
                  _ => return Err(FileError::BadRequest),
              }
          }
          
          Ok(safe_path)
      }
      
      fn read_file(&self, file_path: &Path) -> Result<FileResponse, FileError> {
          let metadata = fs::metadata(file_path)
              .map_err(|_| FileError::InternalError)?;
          
          let mut file = fs::File::open(file_path)
              .map_err(|_| FileError::InternalError)?;
          
          let mut content = Vec::new();
          file.read_to_end(&mut content)
              .map_err(|_| FileError::InternalError)?;
          
          let mime_type = self.get_mime_type(file_path);
          
          Ok(FileResponse {
              content,
              mime_type,
              last_modified: metadata.modified().ok(),
              size: metadata.len(),
          })
      }
      
      fn get_mime_type(&self, path: &Path) -> String {
          match path.extension().and_then(|ext| ext.to_str()) {
              Some("html") => "text/html; charset=utf-8".to_string(),
              Some("css") => "text/css".to_string(),
              Some("js") => "application/javascript".to_string(),
              Some("json") => "application/json".to_string(),
              Some("png") => "image/png".to_string(),
              Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
              Some("svg") => "image/svg+xml".to_string(),
              Some("ico") => "image/x-icon".to_string(),
              _ => "application/octet-stream".to_string(),
          }
      }
  }
  
  pub struct FileResponse {
      pub content: Vec<u8>,
      pub mime_type: String,
      pub last_modified: Option<std::time::SystemTime>,
      pub size: u64,
  }
  
  #[derive(Debug)]
  pub enum FileError {
      NotFound,
      Forbidden,
      BadRequest,
      InternalError,
  }
  ```

### 4. 响应生成与错误处理 | Response Generation and Error Handling (1小时 | 1 hour)

- **HTTP响应构建器 | HTTP Response Builder**
  
  **多概念整合 | Multi-concept Integration:**
  响应生成需要整合HTTP协议知识、错误处理策略、内容编码、缓存控制等多个概念，形成完整的响应处理流水线。 | Response generation needs to integrate multiple concepts including HTTP protocol knowledge, error handling strategies, content encoding, cache control, forming a complete response processing pipeline.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. HTTP响应是否必须包含Content-Length头？| Must HTTP responses include Content-Length header?
     **答案 | Answer:** 不是必须 | Not required - 可以使用Transfer-Encoding: chunked | Can use Transfer-Encoding: chunked
  2. 错误响应是否需要包含响应体？| Do error responses need to include response body?
     **答案 | Answer:** 不是必须但推荐 | Not required but recommended - 提供错误信息有助于调试 | Providing error information helps debugging
  3. 响应头的顺序重要吗？| Does the order of response headers matter?
     **答案 | Answer:** 一般不重要 | Generally not important - 但某些代理可能有特殊要求 | But some proxies may have special requirements
  4. 如何判断客户端是否支持gzip压缩？| How to determine if client supports gzip compression?
     **答案 | Answer:** 检查Accept-Encoding请求头 | Check Accept-Encoding request header

### 5. 性能优化与监控 | Performance Optimization and Monitoring (45分钟 | 45 minutes)

- **连接管理与资源控制 | Connection Management and Resource Control**
  
  **关键原则 | Key Principles:**
  - 连接池化：复用TCP连接减少建立连接的开销 | Connection pooling: reuse TCP connections to reduce connection establishment overhead
  - 超时控制：设置合理的读写超时避免连接泄露 | Timeout control: set reasonable read/write timeouts to avoid connection leaks
  - 资源限制：限制同时处理的请求数量和内存使用 | Resource limits: limit the number of concurrent requests and memory usage
  
  **实践验证问题 | Practice Verification Questions:**
  1. 如何检测空闲连接并及时关闭？| How to detect idle connections and close them in time?
  2. 线程池大小应该如何确定？| How should thread pool size be determined?
  3. 什么情况下需要启用连接keep-alive？| When should connection keep-alive be enabled?

### 6. 测试与部署准备 | Testing and Deployment Preparation (30分钟 | 30 minutes)

- **集成测试与性能验证 | Integration Testing and Performance Validation**
  
  **综合概念检查 | Comprehensive Concept Check:**
  1. 如何测试服务器在高并发情况下的表现？| How to test server performance under high concurrency?
  2. 服务器启动失败的常见原因有哪些？| What are common causes of server startup failures?
  3. 如何验证静态文件服务的安全性？| How to verify the security of static file serving?
  4. 生产环境部署需要考虑哪些因素？| What factors need to be considered for production deployment?
  5. 如何实现优雅关闭服务器？| How to implement graceful server shutdown?

## 实践项目：多线程HTTP服务器 | Practical Project: Multi-threaded HTTP Server

### 目标 | Objective
构建一个功能完整的多线程HTTP服务器，综合应用线程池、HTTP解析、静态文件服务、错误处理和日志记录等核心概念，为后续的Web框架学习打下坚实基础。 | Build a fully functional multi-threaded HTTP server that comprehensively applies core concepts including thread pools, HTTP parsing, static file serving, error handling, and logging, laying a solid foundation for subsequent web framework learning.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 你能解释线程池的工作原理和优势吗？| Can you explain the working principle and advantages of thread pools?
   **答案 | Answer:** 线程池预创建固定数量的线程，通过任务队列分发工作，避免频繁创建销毁线程的开销，提高性能和资源利用率 | Thread pools pre-create a fixed number of threads and distribute work through task queues, avoiding the overhead of frequent thread creation/destruction, improving performance and resource utilization

2. HTTP请求解析中最重要的安全考虑是什么？| What is the most important security consideration in HTTP request parsing?
   **答案 | Answer:** 输入验证和资源限制，防止恶意请求导致服务器崩溃或资源耗尽 | Input validation and resource limits to prevent malicious requests from causing server crashes or resource exhaustion

3. 静态文件服务如何防止路径遍历攻击？| How does static file serving prevent path traversal attacks?
   **答案 | Answer:** 通过路径标准化、限制访问根目录范围、过滤危险路径组件如../ | Through path normalization, restricting access to root directory scope, filtering dangerous path components like ../

### 步骤 | Steps
1. **基础架构搭建**：实现线程池和基本的TCP服务器框架 | **Basic Architecture Setup**: Implement thread pool and basic TCP server framework
2. **HTTP协议处理**：集成请求解析器和响应生成器 | **HTTP Protocol Processing**: Integrate request parser and response generator  
3. **静态文件服务**：实现安全的文件服务功能 | **Static File Serving**: Implement secure file serving functionality
4. **错误处理完善**：添加全面的错误处理和日志记录 | **Error Handling Enhancement**: Add comprehensive error handling and logging
5. **性能测试验证**：进行负载测试和性能分析 | **Performance Testing Validation**: Conduct load testing and performance analysis

### 示例代码 | Example Code
```rust
"""
多线程HTTP服务器 | Multi-threaded HTTP Server
一个功能完整的HTTP/1.1服务器实现，支持静态文件服务、多线程并发处理和完善的错误处理机制

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 线程池管理和任务调度 | Thread pool management and task scheduling
- HTTP协议解析和响应生成 | HTTP protocol parsing and response generation
- 静态文件服务和安全控制 | Static file serving and security control
- 错误处理和结构化日志 | Error handling and structured logging
- 性能优化和资源管理 | Performance optimization and resource management
"""

use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use log::{info, warn, error, debug};
use chrono::{DateTime, Utc};

// 主服务器结构 | Main server structure
pub struct HttpServer {
    address: String,
    thread_pool: ThreadPool,
    static_handler: StaticFileHandler,
    config: ServerConfig,
}

// 服务器配置 | Server configuration
#[derive(Clone)]
pub struct ServerConfig {
    pub max_request_size: usize,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub static_root: PathBuf,
    pub default_file: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            max_request_size: 1024 * 1024, // 1MB
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            static_root: PathBuf::from("./static"),
            default_file: "index.html".to_string(),
        }
    }
}

impl HttpServer {
    // 创建新的服务器实例 | Create new server instance
    pub fn new(address: String, config: ServerConfig) -> Result<Self, ServerError> {
        let thread_pool = ThreadPool::new(4)
            .map_err(|e| ServerError::InitializationError(e.to_string()))?;
        
        let static_handler = StaticFileHandler::new(&config.static_root);
        
        // 确保静态文件目录存在 | Ensure static file directory exists
        if !config.static_root.exists() {
            fs::create_dir_all(&config.static_root)
                .map_err(|e| ServerError::InitializationError(format!("Failed to create static directory: {}", e)))?;
            info!("Created static file directory: {:?}", config.static_root);
        }
        
        Ok(HttpServer {
            address,
            thread_pool,
            static_handler,
            config,
        })
    }
    
    // 启动服务器 | Start server
    pub fn run(&self) -> Result<(), ServerError> {
        let listener = TcpListener::bind(&self.address)
            .map_err(|e| ServerError::BindError(format!("Failed to bind to {}: {}", self.address, e)))?;
        
        info!("HTTP Server listening on {}", self.address);
        info!("Static files served from: {:?}", self.config.static_root);
        
        // 处理连接循环 | Connection handling loop
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let config = self.config.clone();
                    let static_handler = self.static_handler.clone();
                    
                    // 将连接处理任务提交给线程池 | Submit connection handling task to thread pool
                    self.thread_pool.execute(move || {
                        if let Err(e) = handle_connection(stream, config, static_handler) {
                            error!("Error handling connection: {:?}", e);
                        }
                    });
                },
                Err(e) => {
                    warn!("Failed to accept connection: {}", e);
                }
            }
        }
        
        Ok(())
    }
}

// 处理单个连接 | Handle single connection
fn handle_connection(
    mut stream: TcpStream, 
    config: ServerConfig, 
    static_handler: StaticFileHandler
) -> Result<(), ConnectionError> {
    // 设置超时 | Set timeouts
    stream.set_read_timeout(Some(config.read_timeout))?;
    stream.set_write_timeout(Some(config.write_timeout))?;
    
    let peer_addr = stream.peer_addr().unwrap_or_else(|_| "unknown".parse().unwrap());
    debug!("New connection from: {}", peer_addr);
    
    // 解析HTTP请求 | Parse HTTP request
    let request = parse_http_request(&mut stream, config.max_request_size)?;
    info!("{} {} {} - {}", peer_addr, request.method, request.path, request.version);
    
    // 生成响应 | Generate response
    let response = match request.method.as_str() {
        "GET" => handle_get_request(&request, &static_handler),
        "POST" => handle_post_request(&request),
        "HEAD" => handle_head_request(&request, &static_handler),
        _ => create_error_response(405, "Method Not Allowed", Some("Unsupported HTTP method")),
    };
    
    // 发送响应 | Send response
    send_response(&mut stream, &response)?;
    
    debug!("Connection handled successfully for {}", peer_addr);
    Ok(())
}

// HTTP请求结构 | HTTP request structure
#[derive(Debug, Clone)]
struct HttpRequest {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

// HTTP响应结构 | HTTP response structure
#[derive(Debug)]
struct HttpResponse {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpResponse {
    // 创建成功响应 | Create success response
    fn ok(content_type: &str, body: Vec<u8>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), content_type.to_string());
        headers.insert("Content-Length".to_string(), body.len().to_string());
        headers.insert("Server".to_string(), "RustHttpServer/1.0".to_string());
        headers.insert("Date".to_string(), Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string());
        
        HttpResponse {
            status_code: 200,
            status_text: "OK".to_string(),
            headers,
            body,
        }
    }
    
    // 转换为字节流 | Convert to byte stream
    fn to_bytes(&self) -> Vec<u8> {
        let mut response = Vec::new();
        
        // 状态行 | Status line
        response.extend_from_slice(format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text).as_bytes());
        
        // 响应头 | Response headers
        for (name, value) in &self.headers {
            response.extend_from_slice(format!("{}: {}\r\n", name, value).as_bytes());
        }
        
        // 空行分隔头部和主体 | Empty line separating headers and body
        response.extend_from_slice(b"\r\n");
        
        // 响应主体 | Response body
        response.extend_from_slice(&self.body);
        
        response
    }
}

// 解析HTTP请求 | Parse HTTP request
fn parse_http_request(stream: &mut TcpStream, max_size: usize) -> Result<HttpRequest, ConnectionError> {
    let mut buffer = vec![0; 4096];
    let mut total_data = Vec::new();
    let mut headers_end = None;
    
    // 读取请求头 | Read request headers
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => return Err(ConnectionError::UnexpectedEof),
            Ok(n) => {
                total_data.extend_from_slice(&buffer[..n]);
                
                // 检查是否读取完整请求头 | Check if complete request headers are read
                if let Some(pos) = find_headers_end(&total_data) {
                    headers_end = Some(pos);
                    break;
                }
                
                // 防止请求过大 | Prevent oversized requests
                if total_data.len() > max_size {
                    return Err(ConnectionError::RequestTooLarge);
                }
            },
            Err(e) => return Err(ConnectionError::IoError(e)),
        }
    }
    
    let headers_end = headers_end.ok_or(ConnectionError::InvalidRequest)?;
    let headers_data = &total_data[..headers_end];
    let headers_str = std::str::from_utf8(headers_data)
        .map_err(|_| ConnectionError::InvalidEncoding)?;
    
    // 解析请求行和头部 | Parse request line and headers
    let lines: Vec<&str> = headers_str.lines().collect();
    if lines.is_empty() {
        return Err(ConnectionError::InvalidRequest);
    }
    
    // 解析请求行 | Parse request line
    let request_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
    if request_line_parts.len() != 3 {
        return Err(ConnectionError::InvalidRequest);
    }
    
    let method = request_line_parts[0].to_string();
    let path = request_line_parts[1].to_string();
    let version = request_line_parts[2].to_string();
    
    // 解析头部 | Parse headers
    let mut headers = HashMap::new();
    for line in &lines[1..] {
        if let Some(colon_pos) = line.find(':') {
            let name = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            headers.insert(name.to_lowercase(), value);
        }
    }
    
    // 处理请求体 | Handle request body
    let body = if let Some(content_length) = headers.get("content-length") {
        let content_length: usize = content_length.parse()
            .map_err(|_| ConnectionError::InvalidRequest)?;
        
        if content_length > max_size {
            return Err(ConnectionError::RequestTooLarge);
        }
        
        let body_start = headers_end + 4; // 跳过\r\n\r\n | Skip \r\n\r\n
        let mut body = Vec::new();
        
        // 检查是否已经读取了部分body | Check if part of body is already read
        if total_data.len() > body_start {
            let already_read = &total_data[body_start..];
            body.extend_from_slice(already_read);
        }
        
        // 读取剩余的body | Read remaining body
        while body.len() < content_length {
            let remaining = content_length - body.len();
            let to_read = remaining.min(buffer.len());
            
            match stream.read(&mut buffer[..to_read]) {
                Ok(0) => return Err(ConnectionError::UnexpectedEof),
                Ok(n) => body.extend_from_slice(&buffer[..n]),
                Err(e) => return Err(ConnectionError::IoError(e)),
            }
        }
        
        body
    } else {
        Vec::new()
    };
    
    Ok(HttpRequest {
        method,
        path,
        version,
        headers,
        body,
    })
}

// 查找请求头结束位置 | Find end of request headers
fn find_headers_end(data: &[u8]) -> Option<usize> {
    data.windows(4).position(|window| window == b"\r\n\r\n")
}

// 处理GET请求 | Handle GET request
fn handle_get_request(request: &HttpRequest, static_handler: &StaticFileHandler) -> HttpResponse {
    match static_handler.handle_request(&request.path) {
        Ok(file_response) => {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), file_response.mime_type);
            headers.insert("Content-Length".to_string(), file_response.content.len().to_string());
            headers.insert("Server".to_string(), "RustHttpServer/1.0".to_string());
            headers.insert("Date".to_string(), Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string());
            
            // 添加缓存头 | Add cache headers
            if let Some(last_modified) = file_response.last_modified {
                if let Ok(duration) = last_modified.duration_since(std::time::UNIX_EPOCH) {
                    let datetime = DateTime::from_timestamp(duration.as_secs() as i64, 0);
                    if let Some(dt) = datetime {
                        headers.insert("Last-Modified".to_string(), 
                                     dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string());
                    }
                }
            }
            
            HttpResponse {
                status_code: 200,
                status_text: "OK".to_string(),
                headers,
                body: file_response.content,
            }
        },
        Err(FileError::NotFound) => create_error_response(404, "Not Found", Some("The requested file was not found")),
        Err(FileError::Forbidden) => create_error_response(403, "Forbidden", Some("Access to this resource is forbidden")),
        Err(FileError::BadRequest) => create_error_response(400, "Bad Request", Some("Invalid request path")),
        Err(_) => create_error_response(500, "Internal Server Error", Some("An internal server error occurred")),
    }
}

// 处理POST请求 | Handle POST request
fn handle_post_request(request: &HttpRequest) -> HttpResponse {
    // 简单的POST处理示例 | Simple POST handling example
    let response_body = format!(
        "POST request received\nPath: {}\nBody length: {} bytes\n",
        request.path,
        request.body.len()
    );
    
    HttpResponse::ok("text/plain; charset=utf-8", response_body.into_bytes())
}

// 处理HEAD请求 | Handle HEAD request
fn handle_head_request(request: &HttpRequest, static_handler: &StaticFileHandler) -> HttpResponse {
    let mut get_response = handle_get_request(request, static_handler);
    get_response.body.clear(); // HEAD请求不返回body | HEAD request doesn't return body
    get_response
}

// 创建错误响应 | Create error response
fn create_error_response(status_code: u16, status_text: &str, message: Option<&str>) -> HttpResponse {
    let body_content = message.unwrap_or(status_text);
    let html_body = format!(
        "<!DOCTYPE html>\
        <html><head><title>{} {}</title></head>\
        <body><h1>{} {}</h1><p>{}</p>\
        <hr><i>RustHttpServer/1.0</i></body></html>",
        status_code, status_text, status_code, status_text, body_content
    );
    
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
    headers.insert("Content-Length".to_string(), html_body.len().to_string());
    headers.insert("Server".to_string(), "RustHttpServer/1.0".to_string());
    headers.insert("Date".to_string(), Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string());
    
    HttpResponse {
        status_code,
        status_text: status_text.to_string(),
        headers,
        body: html_body.into_bytes(),
    }
}

// 发送响应 | Send response
fn send_response(stream: &mut TcpStream, response: &HttpResponse) -> Result<(), ConnectionError> {
    let response_bytes = response.to_bytes();
    stream.write_all(&response_bytes)?;
    stream.flush()?;
    Ok(())
}

// 静态文件处理器 | Static file handler
#[derive(Clone)]
pub struct StaticFileHandler {
    root_dir: PathBuf,
}

impl StaticFileHandler {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        StaticFileHandler {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }
    
    pub fn handle_request(&self, path: &str) -> Result<FileResponse, FileError> {
        let safe_path = self.sanitize_path(path)?;
        let file_path = self.root_dir.join(&safe_path);
        
        // 安全检查：确保文件在根目录内 | Security check: ensure file is within root directory
        if !file_path.starts_with(&self.root_dir) {
            return Err(FileError::Forbidden);
        }
        
        if file_path.is_dir() {
            let index_path = file_path.join("index.html");
            if index_path.exists() {
                self.read_file(&index_path)
            } else {
                Err(FileError::NotFound)
            }
        } else if file_path.exists() && file_path.is_file() {
            self.read_file(&file_path)
        } else {
            Err(FileError::NotFound)
        }
    }
    
    fn sanitize_path(&self, path: &str) -> Result<PathBuf, FileError> {
        let path = path.trim_start_matches('/');
        let path = urlencoding::decode(path)
            .map_err(|_| FileError::BadRequest)?;
        
        let path = Path::new(path.as_ref());
        let mut safe_path = PathBuf::new();
        
        for component in path.components() {
            match component {
                std::path::Component::Normal(name) => {
                    safe_path.push(name);
                },
                std::path::Component::ParentDir => {
                    safe_path.pop();
                },
                std::path::Component::CurDir => {
                    // 忽略 | Ignore
                },
                _ => return Err(FileError::BadRequest),
            }
        }
        
        Ok(safe_path)
    }
    
    fn read_file(&self, file_path: &Path) -> Result<FileResponse, FileError> {
        let metadata = fs::metadata(file_path)
            .map_err(|_| FileError::InternalError)?;
        
        let content = fs::read(file_path)
            .map_err(|_| FileError::InternalError)?;
        
        let mime_type = self.get_mime_type(file_path);
        
        Ok(FileResponse {
            content,
            mime_type,
            last_modified: metadata.modified().ok(),
            size: metadata.len(),
        })
    }
    
    fn get_mime_type(&self, path: &Path) -> String {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("html") => "text/html; charset=utf-8",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            Some("ico") => "image/x-icon",
            Some("txt") => "text/plain; charset=utf-8",
            _ => "application/octet-stream",
        }.to_string()
    }
}

pub struct FileResponse {
    pub content: Vec<u8>,
    pub mime_type: String,
    pub last_modified: Option<std::time::SystemTime>,
    pub size: u64,
}

// 线程池实现 | Thread pool implementation
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, &'static str> {
        if size == 0 {
            return Err("Thread pool size must be greater than 0");
        }
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        Ok(ThreadPool { workers, sender })
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            debug!("Worker {} executing job", id);
            job();
        });
        
        Worker { id, thread }
    }
}

// 错误类型定义 | Error type definitions
#[derive(Debug)]
pub enum ServerError {
    InitializationError(String),
    BindError(String),
}

#[derive(Debug)]
pub enum ConnectionError {
    IoError(std::io::Error),
    InvalidRequest,
    InvalidEncoding,
    RequestTooLarge,
    UnexpectedEof,
}

impl From<std::io::Error> for ConnectionError {
    fn from(error: std::io::Error) -> Self {
        ConnectionError::IoError(error)
    }
}

#[derive(Debug)]
pub enum FileError {
    NotFound,
    Forbidden,
    BadRequest,
    InternalError,
}

// 主函数示例 | Main function example
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志 | Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    // 创建静态文件目录 | Create static file directory
    let static_dir = PathBuf::from("./static");
    if !static_dir.exists() {
        fs::create_dir_all(&static_dir)?;
        
        // 创建示例HTML文件 | Create sample HTML file
        let index_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>Rust HTTP Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .container { max-width: 800px; margin: 0 auto; }
        .header { color: #333; border-bottom: 2px solid #ddd; padding-bottom: 10px; }
        .content { margin-top: 20px; line-height: 1.6; }
        .footer { margin-top: 40px; text-align: center; color: #666; font-size: 0.9em; }
    </style>
</head>
<body>
    <div class="container">
        <h1 class="header">Welcome to Rust HTTP Server!</h1>
        <div class="content">
            <p>🎉 Congratulations! Your multi-threaded HTTP server is running successfully.</p>
            <h2>Features:</h2>
            <ul>
                <li>✅ Multi-threaded request handling</li>
                <li>✅ Static file serving</li>
                <li>✅ HTTP/1.1 protocol support</li>
                <li>✅ Error handling and logging</li>
                <li>✅ Security path validation</li>
                <li>✅ MIME type detection</li>
            </ul>
            <h2>Supported Methods:</h2>
            <ul>
                <li><strong>GET</strong> - Retrieve static files</li>
                <li><strong>POST</strong> - Handle form submissions</li>
                <li><strong>HEAD</strong> - Get response headers only</li>
            </ul>
        </div>
        <div class="footer">
            <p>Powered by Rust Multi-threaded HTTP Server v1.0</p>
        </div>
    </div>
</body>
</html>"#;
        
        fs::write(static_dir.join("index.html"), index_content)?;
        println!("Created sample index.html file");
    }
    
    // 服务器配置 | Server configuration
    let config = ServerConfig {
        static_root: static_dir,
        max_request_size: 2 * 1024 * 1024, // 2MB
        read_timeout: Duration::from_secs(30),
        write_timeout: Duration::from_secs(30),
        default_file: "index.html".to_string(),
    };
    
    // 创建并启动服务器 | Create and start server
    let server = HttpServer::new("127.0.0.1:8080".to_string(), config)?;
    
    println!("Starting HTTP server...");
    println!("Server running at: http://127.0.0.1:8080");
    println!("Press Ctrl+C to stop the server");
    
    server.run()?;
    
    Ok(())
}

// Cargo.toml 依赖配置 | Cargo.toml dependencies configuration
/*
[dependencies]
log = "0.4"
env_logger = "0.10"
chrono = { version = "0.4", features = ["serde"] }
urlencoding = "2.1"

[dev-dependencies]
tokio-test = "0.4"
*/
```

### 项目完成检查 | Project Completion Check
1. 服务器是否能正确处理并发请求？| Can the server correctly handle concurrent requests?
2. 静态文件服务是否实现了安全的路径验证？| Does static file serving implement secure path validation?
3. 错误处理是否覆盖了所有可能的异常情况？| Does error handling cover all possible exception scenarios?
4. 日志记录是否提供了足够的调试信息？| Does logging provide sufficient debugging information?
5. 性能测试结果是否满足预期？| Do performance test results meet expectations?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **线程池性能优化练习 | Thread Pool Performance Optimization Exercise**
   - **练习描述 | Exercise Description:** 实现动态线程池，根据负载情况自动调整线程数量，并添加线程利用率监控
   - **概念检查 | Concept Check:** 你能解释线程池大小对性能的影响吗？如何平衡线程数量和系统资源？
   - **学习目标 | Learning Objective:** 深入理解线程池的工作原理和性能调优策略

2. **HTTP Keep-Alive实现练习 | HTTP Keep-Alive Implementation Exercise**
   - **练习描述 | Exercise Description:** 实现HTTP/1.1的连接复用功能，支持在同一TCP连接上处理多个HTTP请求
   - **概念检查 | Concept Check:** Keep-Alive如何减少连接建立开销？如何处理连接超时？
   - **学习目标 | Learning Objective:** 理解HTTP连接管理和性能优化技术

3. **请求限流机制练习 | Request Rate Limiting Exercise**
   - **练习描述 | Exercise Description:** 实现基于IP地址的请求限流，防止单个客户端过度占用服务器资源
   - **概念检查 | Concept Check:** 常见的限流算法有哪些？如何选择合适的限流策略？
   - **学习目标 | Learning Objective:** 掌握服务器保护机制和资源管理策略

4. **HTTPS支持添加练习 | HTTPS Support Addition Exercise**
   - **练习描述 | Exercise Description:** 为HTTP服务器添加TLS/SSL支持，实现HTTPS协议处理
   - **概念检查 | Concept Check:** TLS握手过程是怎样的？如何管理SSL证书？
   - **学习目标 | Learning Objective:** 理解Web安全和加密通信原理

5. **虚拟主机支持练习 | Virtual Host Support Exercise**
   - **练习描述 | Exercise Description:** 实现基于Host头的虚拟主机功能，支持单个服务器托管多个网站
   - **概念检查 | Concept Check:** 虚拟主机如何根据域名路由请求？配置管理需要考虑什么？
   - **学习目标 | Learning Objective:** 掌握多租户Web服务架构设计

6. **缓存机制实现练习 | Caching Mechanism Implementation Exercise**
   - **练习描述 | Exercise Description:** 为静态文件添加ETag和Last-Modified缓存机制，支持条件请求
   - **概念检查 | Concept Check:** HTTP缓存头的作用机制是什么？如何验证缓存有效性？
   - **学习目标 | Learning Objective:** 理解Web缓存策略和性能优化技术

7. **压力测试工具开发练习 | Load Testing Tool Development Exercise**
   - **练习描述 | Exercise Description:** 开发一个多线程的HTTP压力测试工具，评估服务器性能
   - **概念检查 | Concept Check:** 如何设计有效的性能测试？关键性能指标有哪些？
   - **学习目标 | Learning Objective:** 掌握性能测试方法和服务器性能分析技能

## 学习资源 | Learning Resources
- [Rust官方文档 - 网络编程](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) | [Rust Official Documentation - Network Programming](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [HTTP/1.1规范RFC 2616](https://tools.ietf.org/html/rfc2616) | [HTTP/1.1 Specification RFC 2616](https://tools.ietf.org/html/rfc2616)
- [多线程编程最佳实践](https://doc.rust-lang.org/book/ch16-00-concurrency.html) | [Multi-threading Programming Best Practices](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Web服务器性能优化指南](https://developer.mozilla.org/en-US/docs/Web/Performance) | [Web Server Performance Optimization Guide](https://developer.mozilla.org/en-US/docs/Web/Performance)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解多线程HTTP服务器架构原理 | Understand multi-threaded HTTP server architecture principles
- [ ] 掌握线程池的设计和实现方法 | Master thread pool design and implementation methods  
- [ ] 能够解析和处理HTTP协议消息 | Able to parse and process HTTP protocol messages
- [ ] 实现安全的静态文件服务功能 | Implement secure static file serving functionality
- [ ] 建立完善的错误处理和日志系统 | Establish comprehensive error handling and logging system
- [ ] 完成多线程HTTP服务器项目开发 | Complete multi-threaded HTTP server project development
- [ ] 能够进行基础的性能测试和优化 | Able to conduct basic performance testing and optimization
- [ ] 理解Web服务器的安全考虑因素 | Understand security considerations for web servers
- [ ] 掌握HTTP响应的生成和发送机制 | Master HTTP response generation and sending mechanisms
- [ ] 具备继续学习Web框架的知识基础 | Have the knowledge foundation to continue learning web frameworks

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够独立实现一个基础的多线程HTTP服务器，处理静态文件请求和基本的错误情况。
Before marking as complete, ensure you can correctly answer all CCQs from today and independently implement a basic multi-threaded HTTP server that handles static file requests and basic error scenarios.