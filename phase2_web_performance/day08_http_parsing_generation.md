# Rust入门 - 第8天：HTTP解析与生成 | Rust Introduction - Day 8: HTTP Parsing and Generation

## 学习目标 | Learning Objectives
- 深入理解HTTP协议的消息格式和结构 | Deeply understand HTTP protocol message format and structure
- 掌握HTTP请求和响应的完整解析过程 | Master the complete parsing process of HTTP requests and responses
- 熟练处理HTTP头部字段的解析和生成 | Proficiently handle parsing and generation of HTTP header fields
- 能够正确处理不同类型的请求体和响应体 | Be able to correctly handle different types of request and response bodies
- 实现高性能的HTTP消息解析器 | Implement high-performance HTTP message parser
- 理解HTTP协议在网络编程中的实际应用 | Understand practical applications of HTTP protocol in network programming

## 详细内容 | Detailed Content

### 1. HTTP协议消息格式深入 | Deep Dive into HTTP Protocol Message Format (1小时 | 1 hour)

- **HTTP消息结构基础 | HTTP Message Structure Fundamentals**
  
  **概念定义 | Concept Definition:**
  HTTP消息是客户端和服务器之间通信的基本单位，包含起始行、头部字段和可选的消息体三个主要部分，遵循严格的文本格式规范。 | HTTP messages are the basic units of communication between clients and servers, containing three main parts: start line, header fields, and optional message body, following strict text format specifications.
  
  **核心特征 | Key Characteristics:**
  - 文本协议：HTTP消息使用可读的ASCII文本格式 | Text protocol: HTTP messages use readable ASCII text format
  - 分行结构：使用CRLF(\r\n)作为行分隔符 | Line-based structure: uses CRLF(\r\n) as line separator
  - 分层组织：起始行、头部、空行、消息体的固定结构 | Layered organization: fixed structure of start line, headers, blank line, message body
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. HTTP消息的三个主要部分是否都是必需的？| Are all three main parts of HTTP message required?
     **答案 | Answer:** 否 | No - 消息体是可选的，起始行和头部是必需的
  2. HTTP头部字段是否区分大小写？| Are HTTP header fields case-sensitive?
     **答案 | Answer:** 否 | No - HTTP头部字段名不区分大小写
  3. 请求消息和响应消息的起始行格式是否相同？| Do request and response messages have the same start line format?
     **答案 | Answer:** 否 | No - 请求行包含方法、URI和版本，状态行包含版本、状态码和原因短语
  4. 消息体和头部之间必须有什么分隔？| What separation is required between message body and headers?
     **答案 | Answer:** 空行(CRLF) | Blank line (CRLF) - 必须有一个空行分隔头部和消息体
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // HTTP消息格式示例和解析 | HTTP message format examples and parsing
  
  // HTTP请求消息示例 | HTTP request message example
  const HTTP_REQUEST_EXAMPLE: &str = 
  "GET /api/users HTTP/1.1\r\n\
   Host: example.com\r\n\
   User-Agent: Mozilla/5.0\r\n\
   Accept: application/json\r\n\
   Content-Length: 0\r\n\
   \r\n";
  
  // HTTP响应消息示例 | HTTP response message example  
  const HTTP_RESPONSE_EXAMPLE: &str =
  "HTTP/1.1 200 OK\r\n\
   Content-Type: application/json\r\n\
   Content-Length: 27\r\n\
   Server: Rust-Server/1.0\r\n\
   \r\n\
   {\"message\":\"Hello World\"}";
  
  // 基础消息结构分析 | Basic message structure analysis
  #[derive(Debug, PartialEq)]
  pub struct HttpMessageParts {
      pub start_line: String,
      pub headers: Vec<(String, String)>,
      pub body: Vec<u8>,
  }
  
  impl HttpMessageParts {
      // 解析HTTP消息的基本结构 | Parse basic structure of HTTP message
      pub fn parse_basic_structure(message: &str) -> Result<Self, String> {
          let bytes = message.as_bytes();
          let mut pos = 0;
          
          // 1. 解析起始行 | Parse start line
          let start_line = match Self::find_line(&bytes, &mut pos) {
              Some(line) => String::from_utf8_lossy(line).to_string(),
              None => return Err("找不到起始行 | Start line not found".to_string()),
          };
          
          // 2. 解析头部字段 | Parse header fields
          let mut headers = Vec::new();
          while let Some(line) = Self::find_line(&bytes, &mut pos) {
              if line.is_empty() {
                  // 遇到空行，头部解析完成 | Empty line encountered, headers parsing complete
                  break;
              }
              
              // 解析单个头部字段 | Parse individual header field
              if let Some((name, value)) = Self::parse_header_line(line) {
                  headers.push((name, value));
              } else {
                  return Err(format!("无效的头部字段: {:?} | Invalid header field: {:?}", 
                                   String::from_utf8_lossy(line), String::from_utf8_lossy(line)));
              }
          }
          
          // 3. 读取消息体 | Read message body
          let body = bytes[pos..].to_vec();
          
          Ok(HttpMessageParts {
              start_line,
              headers,
              body,
          })
      }
      
      // 查找下一行 | Find next line
      fn find_line<'a>(bytes: &'a [u8], pos: &mut usize) -> Option<&'a [u8]> {
          let start = *pos;
          
          // 查找CRLF或LF | Look for CRLF or LF
          while *pos < bytes.len() {
              if bytes[*pos] == b'\r' && *pos + 1 < bytes.len() && bytes[*pos + 1] == b'\n' {
                  let line = &bytes[start..*pos];
                  *pos += 2; // 跳过CRLF | Skip CRLF
                  return Some(line);
              } else if bytes[*pos] == b'\n' {
                  let line = &bytes[start..*pos];
                  *pos += 1; // 跳过LF | Skip LF
                  return Some(line);
              }
              *pos += 1;
          }
          
          // 处理文件结尾的情况 | Handle end of file case
          if start < bytes.len() {
              *pos = bytes.len();
              Some(&bytes[start..])
          } else {
              None
          }
      }
      
      // 解析头部字段行 | Parse header field line
      fn parse_header_line(line: &[u8]) -> Option<(String, String)> {
          let line_str = String::from_utf8_lossy(line);
          
          // 查找冒号分隔符 | Find colon separator
          if let Some(colon_pos) = line_str.find(':') {
              let name = line_str[..colon_pos].trim().to_string();
              let value = line_str[colon_pos + 1..].trim().to_string();
              Some((name, value))
          } else {
              None
          }
      }
  }
  
  // 测试消息解析 | Test message parsing
  fn test_message_parsing() {
      println!("=== HTTP消息解析测试 | HTTP Message Parsing Test ===");
      
      // 测试请求消息解析 | Test request message parsing
      match HttpMessageParts::parse_basic_structure(HTTP_REQUEST_EXAMPLE) {
          Ok(parts) => {
              println!("请求解析成功 | Request parsing successful:");
              println!("  起始行 | Start line: {}", parts.start_line);
              println!("  头部数量 | Header count: {}", parts.headers.len());
              for (name, value) in &parts.headers {
                  println!("    {}: {}", name, value);
              }
              println!("  消息体长度 | Body length: {} bytes", parts.body.len());
          },
          Err(e) => println!("请求解析失败 | Request parsing failed: {}", e),
      }
      
      // 测试响应消息解析 | Test response message parsing
      match HttpMessageParts::parse_basic_structure(HTTP_RESPONSE_EXAMPLE) {
          Ok(parts) => {
              println!("\n响应解析成功 | Response parsing successful:");
              println!("  起始行 | Start line: {}", parts.start_line);
              println!("  头部数量 | Header count: {}", parts.headers.len());
              println!("  消息体长度 | Body length: {} bytes", parts.body.len());
              println!("  消息体内容 | Body content: {}", String::from_utf8_lossy(&parts.body));
          },
          Err(e) => println!("响应解析失败 | Response parsing failed: {}", e),
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - HTTP_REQUEST_EXAMPLE包含多少个头部字段？| How many header fields does HTTP_REQUEST_EXAMPLE contain?
    **答案 | Answer:** 4个(Host, User-Agent, Accept, Content-Length) | 4 (Host, User-Agent, Accept, Content-Length)
  - 如果消息中只有LF而没有CRLF会如何处理？| How is it handled if message only has LF without CRLF?
    **答案 | Answer:** 代码兼容处理，也会正确解析行 | Code handles compatibility and parses lines correctly
  
  **常见误区检查 | Common Misconception Checks:**
  - HTTP消息是否必须使用CRLF作为行结束符？| Must HTTP messages use CRLF as line terminator?
    **答案 | Answer:** 标准要求CRLF，但实践中常兼容LF | Standard requires CRLF, but LF is commonly supported in practice
  - 头部字段值是否可以包含空格？| Can header field values contain spaces?
    **答案 | Answer:** 可以，但首尾空格通常被忽略 | Yes, but leading/trailing spaces are usually ignored

### 2. HTTP请求消息解析 | HTTP Request Message Parsing (1小时 | 1 hour)

- **HTTP请求行解析机制 | HTTP Request Line Parsing Mechanism**
  
  **概念定义 | Concept Definition:**
  HTTP请求行是请求消息的第一行，包含HTTP方法、请求URI和协议版本三个部分，每个部分由空格分隔，是服务器理解客户端意图的关键信息。 | The HTTP request line is the first line of a request message, containing three parts: HTTP method, request URI, and protocol version, separated by spaces, serving as key information for servers to understand client intent.
  
  **核心特征 | Key Characteristics:**
  - 三段结构：方法、URI、版本的固定格式 | Three-part structure: fixed format of method, URI, version
  - 空格分隔：各部分之间使用单个空格分隔 | Space-separated: parts separated by single spaces
  - 大小写敏感：HTTP方法通常大写，版本号格式固定 | Case-sensitive: HTTP methods usually uppercase, version format fixed
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. HTTP请求行是否可以包含超过三个部分？| Can HTTP request line contain more than three parts?
     **答案 | Answer:** 否 | No - 标准格式只有方法、URI、版本三部分
  2. 请求URI是否可以为空？| Can request URI be empty?
     **答案 | Answer:** 否 | No - URI必须至少包含路径部分，最少是"/"
  3. HTTP版本号是否可以省略？| Can HTTP version number be omitted?
     **答案 | Answer:** 否 | No - 版本号是请求行的必需部分
  4. 请求方法是否区分大小写？| Are request methods case-sensitive?
     **答案 | Answer:** 是 | Yes - HTTP方法区分大小写，通常使用大写
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::collections::HashMap;
  use std::fmt;
  
  // HTTP方法枚举 | HTTP method enumeration
  #[derive(Debug, Clone, PartialEq)]
  pub enum HttpMethod {
      GET,
      POST,
      PUT,
      DELETE,
      HEAD,
      OPTIONS,
      PATCH,
      TRACE,
      CONNECT,
      Custom(String),
  }
  
  impl HttpMethod {
      // 从字符串解析HTTP方法 | Parse HTTP method from string
      pub fn from_str(method: &str) -> Self {
          match method.to_uppercase().as_str() {
              "GET" => HttpMethod::GET,
              "POST" => HttpMethod::POST,
              "PUT" => HttpMethod::PUT,
              "DELETE" => HttpMethod::DELETE,
              "HEAD" => HttpMethod::HEAD,
              "OPTIONS" => HttpMethod::OPTIONS,
              "PATCH" => HttpMethod::PATCH,
              "TRACE" => HttpMethod::TRACE,
              "CONNECT" => HttpMethod::CONNECT,
              _ => HttpMethod::Custom(method.to_string()),
          }
      }
      
      // 判断方法是否安全(只读) | Check if method is safe (read-only)
      pub fn is_safe(&self) -> bool {
          matches!(self, HttpMethod::GET | HttpMethod::HEAD | HttpMethod::OPTIONS)
      }
      
      // 判断方法是否幂等 | Check if method is idempotent
      pub fn is_idempotent(&self) -> bool {
          matches!(self, 
              HttpMethod::GET | HttpMethod::HEAD | HttpMethod::PUT | 
              HttpMethod::DELETE | HttpMethod::OPTIONS
          )
      }
  }
  
  impl fmt::Display for HttpMethod {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          match self {
              HttpMethod::GET => write!(f, "GET"),
              HttpMethod::POST => write!(f, "POST"),
              HttpMethod::PUT => write!(f, "PUT"),
              HttpMethod::DELETE => write!(f, "DELETE"),
              HttpMethod::HEAD => write!(f, "HEAD"),
              HttpMethod::OPTIONS => write!(f, "OPTIONS"),
              HttpMethod::PATCH => write!(f, "PATCH"),
              HttpMethod::TRACE => write!(f, "TRACE"),
              HttpMethod::CONNECT => write!(f, "CONNECT"),
              HttpMethod::Custom(method) => write!(f, "{}", method),
          }
      }
  }
  
  // HTTP版本结构 | HTTP version structure
  #[derive(Debug, Clone, PartialEq)]
  pub struct HttpVersion {
      pub major: u8,
      pub minor: u8,
  }
  
  impl HttpVersion {
      // 从字符串解析版本号 | Parse version from string
      pub fn from_str(version: &str) -> Result<Self, String> {
          if !version.starts_with("HTTP/") {
              return Err("版本必须以HTTP/开头 | Version must start with HTTP/".to_string());
          }
          
          let version_part = &version[5..]; // 跳过"HTTP/" | Skip "HTTP/"
          let parts: Vec<&str> = version_part.split('.').collect();
          
          if parts.len() != 2 {
              return Err("版本格式必须为major.minor | Version format must be major.minor".to_string());
          }
          
          let major = parts[0].parse::<u8>()
              .map_err(|_| "主版本号必须是数字 | Major version must be numeric".to_string())?;
          let minor = parts[1].parse::<u8>()
              .map_err(|_| "次版本号必须是数字 | Minor version must be numeric".to_string())?;
              
          Ok(HttpVersion { major, minor })
      }
      
      // 检查是否支持持久连接 | Check if persistent connections are supported
      pub fn supports_persistent_connection(&self) -> bool {
          self.major > 1 || (self.major == 1 && self.minor >= 1)
      }
  }
  
  impl fmt::Display for HttpVersion {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "HTTP/{}.{}", self.major, self.minor)
      }
  }
  
  // URI组件解析 | URI component parsing
  #[derive(Debug, Clone, PartialEq)]
  pub struct RequestUri {
      pub path: String,
      pub query: Option<String>,
      pub fragment: Option<String>,
  }
  
  impl RequestUri {
      // 从字符串解析URI | Parse URI from string
      pub fn from_str(uri: &str) -> Self {
          let mut path = uri;
          let mut query = None;
          let mut fragment = None;
          
          // 处理fragment(#) | Handle fragment (#)
          if let Some(fragment_pos) = uri.find('#') {
              fragment = Some(uri[fragment_pos + 1..].to_string());
              path = &uri[..fragment_pos];
          }
          
          // 处理query(?) | Handle query (?)
          if let Some(query_pos) = path.find('?') {
              query = Some(path[query_pos + 1..].to_string());
              path = &path[..query_pos];
          }
          
          RequestUri {
              path: path.to_string(),
              query,
              fragment,
          }
      }
      
      // 解析查询参数 | Parse query parameters
      pub fn parse_query_params(&self) -> HashMap<String, String> {
          let mut params = HashMap::new();
          
          if let Some(query) = &self.query {
              for pair in query.split('&') {
                  if let Some(eq_pos) = pair.find('=') {
                      let key = &pair[..eq_pos];
                      let value = &pair[eq_pos + 1..];
                      params.insert(
                          urlencoding::decode(key).unwrap_or_default().to_string(),
                          urlencoding::decode(value).unwrap_or_default().to_string()
                      );
                  } else {
                      params.insert(
                          urlencoding::decode(pair).unwrap_or_default().to_string(),
                          String::new()
                      );
                  }
              }
          }
          
          params
      }
  }
  
  impl fmt::Display for RequestUri {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "{}", self.path)?;
          if let Some(query) = &self.query {
              write!(f, "?{}", query)?;
          }
          if let Some(fragment) = &self.fragment {
              write!(f, "#{}", fragment)?;
          }
          Ok(())
      }
  }
  
  // 完整的HTTP请求结构 | Complete HTTP request structure
  #[derive(Debug, Clone)]
  pub struct HttpRequest {
      pub method: HttpMethod,
      pub uri: RequestUri,
      pub version: HttpVersion,
      pub headers: HashMap<String, String>,
      pub body: Vec<u8>,
  }
  
  impl HttpRequest {
      // 解析HTTP请求 | Parse HTTP request
      pub fn parse(request: &str) -> Result<Self, String> {
          let parts = HttpMessageParts::parse_basic_structure(request)?;
          
          // 解析请求行 | Parse request line
          let request_line_parts: Vec<&str> = parts.start_line.split_whitespace().collect();
          if request_line_parts.len() != 3 {
              return Err("请求行必须包含方法、URI和版本 | Request line must contain method, URI and version".to_string());
          }
          
          let method = HttpMethod::from_str(request_line_parts[0]);
          let uri = RequestUri::from_str(request_line_parts[1]);
          let version = HttpVersion::from_str(request_line_parts[2])?;
          
          // 转换头部格式 | Convert headers format
          let mut headers = HashMap::new();
          for (name, value) in parts.headers {
              headers.insert(name.to_lowercase(), value);
          }
          
          Ok(HttpRequest {
              method,
              uri,
              version,
              headers,
              body: parts.body,
          })
      }
      
      // 获取头部字段值 | Get header field value
      pub fn get_header(&self, name: &str) -> Option<&String> {
          self.headers.get(&name.to_lowercase())
      }
      
      // 获取Content-Length | Get Content-Length
      pub fn content_length(&self) -> Option<usize> {
          self.get_header("content-length")
              .and_then(|v| v.parse().ok())
      }
      
      // 检查是否为持久连接 | Check if persistent connection
      pub fn is_persistent_connection(&self) -> bool {
          if self.version.supports_persistent_connection() {
              // HTTP/1.1默认持久连接 | HTTP/1.1 defaults to persistent connection
              self.get_header("connection")
                  .map(|v| v.to_lowercase() != "close")
                  .unwrap_or(true)
          } else {
              // HTTP/1.0需要显式Connection: keep-alive | HTTP/1.0 needs explicit Connection: keep-alive
              self.get_header("connection")
                  .map(|v| v.to_lowercase() == "keep-alive")
                  .unwrap_or(false)
          }
      }
  }
  
  // 测试请求解析 | Test request parsing
  fn test_request_parsing() {
      println!("=== HTTP请求解析测试 | HTTP Request Parsing Test ===");
      
      let test_requests = vec![
          "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n",
          "POST /api/users?sort=name HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length: 25\r\n\r\n{\"name\":\"John\",\"age\":30}",
          "PUT /resource/123#section1 HTTP/2.0\r\nAuthorization: Bearer token123\r\n\r\n",
      ];
      
      for (i, request_str) in test_requests.iter().enumerate() {
          println!("\n--- 测试请求 {} | Test Request {} ---", i + 1, i + 1);
          match HttpRequest::parse(request_str) {
              Ok(request) => {
                  println!("✅ 解析成功 | Parsing successful:");
                  println!("   方法 | Method: {}", request.method);
                  println!("   路径 | Path: {}", request.uri.path);
                  if let Some(query) = &request.uri.query {
                      println!("   查询 | Query: {}", query);
                      let params = request.uri.parse_query_params();
                      for (key, value) in params {
                          println!("     参数 | Param: {} = {}", key, value);
                      }
                  }
                  if let Some(fragment) = &request.uri.fragment {
                      println!("   片段 | Fragment: {}", fragment);
                  }
                  println!("   版本 | Version: {}", request.version);
                  println!("   头部数量 | Header count: {}", request.headers.len());
                  println!("   消息体长度 | Body length: {} bytes", request.body.len());
                  println!("   持久连接 | Persistent: {}", request.is_persistent_connection());
                  
                  // 展示方法特性 | Show method characteristics
                  println!("   方法安全性 | Method safety: 安全={} 幂等={} | Safe={} Idempotent={}", 
                          request.method.is_safe(), request.method.is_idempotent(),
                          request.method.is_safe(), request.method.is_idempotent());
              },
              Err(e) => println!("❌ 解析失败 | Parsing failed: {}", e),
          }
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - POST方法是否是幂等的？| Is the POST method idempotent?
    **答案 | Answer:** 否，POST不是幂等的 | No, POST is not idempotent
  - HTTP/1.0是否默认支持持久连接？| Does HTTP/1.0 support persistent connections by default?
    **答案 | Answer:** 否，需要显式Connection: keep-alive | No, needs explicit Connection: keep-alive
  
  **常见误区检查 | Common Misconception Checks:**
  - URI中的fragment是否会发送到服务器？| Is the fragment in URI sent to server?
    **答案 | Answer:** 否，fragment只在客户端使用 | No, fragment is only used on client side
  - 请求方法是否可以自定义？| Can request methods be customized?
    **答案 | Answer:** 可以，但需要服务器支持 | Yes, but requires server support

### 3. HTTP响应消息解析 | HTTP Response Message Parsing (1小时 | 1 hour)

- **HTTP状态行和状态码处理 | HTTP Status Line and Status Code Handling**
  
  **概念定义 | Concept Definition:**
  HTTP响应状态行包含协议版本、状态码和原因短语，状态码是三位数字，表示服务器对请求的处理结果，分为五个类别表示不同的响应类型。 | The HTTP response status line contains protocol version, status code, and reason phrase. Status codes are three-digit numbers indicating server's processing result for requests, divided into five categories representing different response types.
  
  **核心特征 | Key Characteristics:**
  - 状态码分类：1xx信息、2xx成功、3xx重定向、4xx客户端错误、5xx服务器错误 | Status code categories: 1xx informational, 2xx success, 3xx redirection, 4xx client error, 5xx server error
  - 原因短语：可读的状态码描述，可自定义 | Reason phrase: human-readable status code description, customizable
  - 版本一致性：响应版本应与请求版本匹配或兼容 | Version consistency: response version should match or be compatible with request version
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 状态码200表示什么含义？| What does status code 200 mean?
     **答案 | Answer:** 成功 | Success - 请求已成功处理
  2. 4xx状态码表示什么类型的错误？| What type of error do 4xx status codes represent?
     **答案 | Answer:** 客户端错误 | Client error - 请求有问题
  3. 原因短语是否必须与标准匹配？| Must reason phrase match the standard?
     **答案 | Answer:** 否 | No - 原因短语可以自定义
  4. 1xx状态码是否表示最终响应？| Do 1xx status codes represent final responses?
     **答案 | Answer:** 否 | No - 1xx是信息性响应，后面还会有最终响应
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::fmt;
  use std::collections::HashMap;
  
  // HTTP状态码枚举 | HTTP status code enumeration
  #[derive(Debug, Clone, PartialEq)]
  pub enum HttpStatusCode {
      // 1xx 信息响应 | 1xx Informational responses
      Continue = 100,
      SwitchingProtocols = 101,
      Processing = 102,
      
      // 2xx 成功 | 2xx Success
      Ok = 200,
      Created = 201,
      Accepted = 202,
      NoContent = 204,
      PartialContent = 206,
      
      // 3xx 重定向 | 3xx Redirection
      MovedPermanently = 301,
      Found = 302,
      SeeOther = 303,
      NotModified = 304,
      TemporaryRedirect = 307,
      PermanentRedirect = 308,
      
      // 4xx 客户端错误 | 4xx Client errors
      BadRequest = 400,
      Unauthorized = 401,
      Forbidden = 403,
      NotFound = 404,
      MethodNotAllowed = 405,
      Conflict = 409,
      Gone = 410,
      UnprocessableEntity = 422,
      TooManyRequests = 429,
      
      // 5xx 服务器错误 | 5xx Server errors
      InternalServerError = 500,
      NotImplemented = 501,
      BadGateway = 502,
      ServiceUnavailable = 503,
      GatewayTimeout = 504,
      
      // 自定义状态码 | Custom status codes
      Custom(u16),
  }
  
  impl HttpStatusCode {
      // 从数字创建状态码 | Create status code from number
      pub fn from_u16(code: u16) -> Self {
          match code {
              100 => HttpStatusCode::Continue,
              101 => HttpStatusCode::SwitchingProtocols,
              102 => HttpStatusCode::Processing,
              200 => HttpStatusCode::Ok,
              201 => HttpStatusCode::Created,
              202 => HttpStatusCode::Accepted,
              204 => HttpStatusCode::NoContent,
              206 => HttpStatusCode::PartialContent,
              301 => HttpStatusCode::MovedPermanently,
              302 => HttpStatusCode::Found,
              303 => HttpStatusCode::SeeOther,
              304 => HttpStatusCode::NotModified,
              307 => HttpStatusCode::TemporaryRedirect,
              308 => HttpStatusCode::PermanentRedirect,
              400 => HttpStatusCode::BadRequest,
              401 => HttpStatusCode::Unauthorized,
              403 => HttpStatusCode::Forbidden,
              404 => HttpStatusCode::NotFound,
              405 => HttpStatusCode::MethodNotAllowed,
              409 => HttpStatusCode::Conflict,
              410 => HttpStatusCode::Gone,
              422 => HttpStatusCode::UnprocessableEntity,
              429 => HttpStatusCode::TooManyRequests,
              500 => HttpStatusCode::InternalServerError,
              501 => HttpStatusCode::NotImplemented,
              502 => HttpStatusCode::BadGateway,
              503 => HttpStatusCode::ServiceUnavailable,
              504 => HttpStatusCode::GatewayTimeout,
              _ => HttpStatusCode::Custom(code),
          }
      }
      
      // 获取状态码数值 | Get status code number
      pub fn as_u16(&self) -> u16 {
          match self {
              HttpStatusCode::Custom(code) => *code,
              _ => *self as u16,
          }
      }
      
      // 获取默认原因短语 | Get default reason phrase
      pub fn default_reason_phrase(&self) -> &'static str {
          match self {
              HttpStatusCode::Continue => "Continue",
              HttpStatusCode::SwitchingProtocols => "Switching Protocols",
              HttpStatusCode::Processing => "Processing",
              HttpStatusCode::Ok => "OK",
              HttpStatusCode::Created => "Created",
              HttpStatusCode::Accepted => "Accepted",
              HttpStatusCode::NoContent => "No Content",
              HttpStatusCode::PartialContent => "Partial Content",
              HttpStatusCode::MovedPermanently => "Moved Permanently",
              HttpStatusCode::Found => "Found",
              HttpStatusCode::SeeOther => "See Other",
              HttpStatusCode::NotModified => "Not Modified",
              HttpStatusCode::TemporaryRedirect => "Temporary Redirect",
              HttpStatusCode::PermanentRedirect => "Permanent Redirect",
              HttpStatusCode::BadRequest => "Bad Request",
              HttpStatusCode::Unauthorized => "Unauthorized",
              HttpStatusCode::Forbidden => "Forbidden",
              HttpStatusCode::NotFound => "Not Found",
              HttpStatusCode::MethodNotAllowed => "Method Not Allowed",
              HttpStatusCode::Conflict => "Conflict",
              HttpStatusCode::Gone => "Gone",
              HttpStatusCode::UnprocessableEntity => "Unprocessable Entity",
              HttpStatusCode::TooManyRequests => "Too Many Requests",
              HttpStatusCode::InternalServerError => "Internal Server Error",
              HttpStatusCode::NotImplemented => "Not Implemented",
              HttpStatusCode::BadGateway => "Bad Gateway",
              HttpStatusCode::ServiceUnavailable => "Service Unavailable",
              HttpStatusCode::GatewayTimeout => "Gateway Timeout",
              HttpStatusCode::Custom(_) => "Unknown",
          }
      }
      
      // 判断状态码类别 | Determine status code category
      pub fn is_informational(&self) -> bool {
          let code = self.as_u16();
          code >= 100 && code < 200
      }
      
      pub fn is_success(&self) -> bool {
          let code = self.as_u16();
          code >= 200 && code < 300
      }
      
      pub fn is_redirection(&self) -> bool {
          let code = self.as_u16();
          code >= 300 && code < 400
      }
      
      pub fn is_client_error(&self) -> bool {
          let code = self.as_u16();
          code >= 400 && code < 500
      }
      
      pub fn is_server_error(&self) -> bool {
          let code = self.as_u16();
          code >= 500 && code < 600
      }
      
      // 是否需要重定向处理 | Whether redirection handling is needed
      pub fn requires_redirect_handling(&self) -> bool {
          matches!(self, 
              HttpStatusCode::MovedPermanently | 
              HttpStatusCode::Found | 
              HttpStatusCode::SeeOther |
              HttpStatusCode::TemporaryRedirect |
              HttpStatusCode::PermanentRedirect
          )
      }
  }
  
  impl fmt::Display for HttpStatusCode {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "{}", self.as_u16())
      }
  }
  
  // HTTP响应结构 | HTTP response structure
  #[derive(Debug, Clone)]
  pub struct HttpResponse {
      pub version: HttpVersion,
      pub status_code: HttpStatusCode,
      pub reason_phrase: String,
      pub headers: HashMap<String, String>,
      pub body: Vec<u8>,
  }
  
  impl HttpResponse {
      // 解析HTTP响应 | Parse HTTP response
      pub fn parse(response: &str) -> Result<Self, String> {
          let parts = HttpMessageParts::parse_basic_structure(response)?;
          
          // 解析状态行 | Parse status line
          let status_line_parts: Vec<&str> = parts.start_line.split_whitespace().collect();
          if status_line_parts.len() < 2 {
              return Err("状态行至少需要版本和状态码 | Status line needs at least version and status code".to_string());
          }
          
          let version = HttpVersion::from_str(status_line_parts[0])?;
          let status_code = status_line_parts[1].parse::<u16>()
              .map_err(|_| "状态码必须是数字 | Status code must be numeric".to_string())?;
          let status_code = HttpStatusCode::from_u16(status_code);
          
          // 原因短语可能包含空格，需要重新组合 | Reason phrase may contain spaces, needs reassembly
          let reason_phrase = if status_line_parts.len() > 2 {
              status_line_parts[2..].join(" ")
          } else {
              status_code.default_reason_phrase().to_string()
          };
          
          // 转换头部格式 | Convert headers format
          let mut headers = HashMap::new();
          for (name, value) in parts.headers {
              headers.insert(name.to_lowercase(), value);
          }
          
          Ok(HttpResponse {
              version,
              status_code,
              reason_phrase,
              headers,
              body: parts.body,
          })
      }
      
      // 创建新响应 | Create new response
      pub fn new(status_code: HttpStatusCode) -> Self {
          HttpResponse {
              version: HttpVersion { major: 1, minor: 1 },
              status_code: status_code.clone(),
              reason_phrase: status_code.default_reason_phrase().to_string(),
              headers: HashMap::new(),
              body: Vec::new(),
          }
      }
      
      // 添加头部字段 | Add header field
      pub fn with_header(mut self, name: &str, value: &str) -> Self {
          self.headers.insert(name.to_lowercase(), value.to_string());
          self
      }
      
      // 设置消息体 | Set message body
      pub fn with_body(mut self, body: Vec<u8>) -> Self {
          // 自动设置Content-Length | Automatically set Content-Length
          self.headers.insert("content-length".to_string(), body.len().to_string());
          self.body = body;
          self
      }
      
      // 设置文本消息体 | Set text message body
      pub fn with_text_body(self, text: &str) -> Self {
          self.with_body(text.as_bytes().to_vec())
      }
      
      // 设置JSON消息体 | Set JSON message body
      pub fn with_json_body(self, json: &str) -> Self {
          self.with_header("content-type", "application/json")
              .with_body(json.as_bytes().to_vec())
      }
      
      // 生成响应字符串 | Generate response string
      pub fn to_string(&self) -> String {
          let mut response = String::new();
          
          // 状态行 | Status line
          response.push_str(&format!("{} {} {}\r\n", 
                                   self.version, self.status_code.as_u16(), self.reason_phrase));
          
          // 头部字段 | Header fields
          for (name, value) in &self.headers {
              response.push_str(&format!("{}: {}\r\n", name, value));
          }
          
          // 空行分隔 | Blank line separator
          response.push_str("\r\n");
          
          // 消息体 | Message body
          if !self.body.is_empty() {
              response.push_str(&String::from_utf8_lossy(&self.body));
          }
          
          response
      }
      
      // 获取重定向位置 | Get redirect location
      pub fn redirect_location(&self) -> Option<&String> {
          if self.status_code.requires_redirect_handling() {
              self.headers.get("location")
          } else {
              None
          }
      }
  }
  
  // 测试响应解析和生成 | Test response parsing and generation
  fn test_response_handling() {
      println!("=== HTTP响应处理测试 | HTTP Response Handling Test ===");
      
      // 测试响应解析 | Test response parsing
      let test_responses = vec![
          "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello, World!",
          "HTTP/1.1 404 Not Found\r\nServer: nginx/1.18.0\r\n\r\n",
          "HTTP/2.0 301 Moved Permanently\r\nLocation: https://example.com/new-path\r\n\r\n",
          "HTTP/1.1 500 Custom Server Error Message\r\nX-Error-Code: E001\r\n\r\n{\"error\":\"Database connection failed\"}",
      ];
      
      for (i, response_str) in test_responses.iter().enumerate() {
          println!("\n--- 解析测试 {} | Parsing Test {} ---", i + 1, i + 1);
          match HttpResponse::parse(response_str) {
              Ok(response) => {
                  println!("✅ 解析成功 | Parsing successful:");
                  println!("   版本 | Version: {}", response.version);
                  println!("   状态码 | Status Code: {} ({})", response.status_code.as_u16(), response.reason_phrase);
                  println!("   状态类别 | Status Category: 成功={} 重定向={} 错误={} | Success={} Redirect={} Error={}", 
                          response.status_code.is_success(),
                          response.status_code.is_redirection(),
                          response.status_code.is_client_error() || response.status_code.is_server_error(),
                          response.status_code.is_success(),
                          response.status_code.is_redirection(),
                          response.status_code.is_client_error() || response.status_code.is_server_error());
                  
                  if let Some(location) = response.redirect_location() {
                      println!("   重定向位置 | Redirect Location: {}", location);
                  }
                  
                  println!("   头部数量 | Header Count: {}", response.headers.len());
                  println!("   消息体长度 | Body Length: {} bytes", response.body.len());
                  
                  if !response.body.is_empty() && response.body.len() <= 50 {
                      println!("   消息体内容 | Body Content: {}", String::from_utf8_lossy(&response.body));
                  }
              },
              Err(e) => println!("❌ 解析失败 | Parsing failed: {}", e),
          }
      }
      
      // 测试响应生成 | Test response generation
      println!("\n--- 响应生成测试 | Response Generation Test ---");
      
      let generated_responses = vec![
          HttpResponse::new(HttpStatusCode::Ok)
              .with_text_body("Hello, World!"),
          
          HttpResponse::new(HttpStatusCode::Created)
              .with_json_body("{\"id\":123,\"name\":\"John\"}")
              .with_header("location", "/users/123"),
          
          HttpResponse::new(HttpStatusCode::NotFound)
              .with_header("server", "Rust-Server/1.0"),
          
          HttpResponse::new(HttpStatusCode::MovedPermanently)
              .with_header("location", "https://example.com/new-location"),
      ];
      
      for (i, response) in generated_responses.iter().enumerate() {
          println!("\n生成响应 {} | Generated Response {}:", i + 1, i + 1);
          println!("{}", response.to_string());
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 状态码301和302的主要区别是什么？| What's the main difference between status codes 301 and 302?
    **答案 | Answer:** 301是永久重定向，302是临时重定向 | 301 is permanent redirect, 302 is temporary redirect
  - 4xx错误是否都表示客户端问题？| Do all 4xx errors indicate client-side problems?
    **答案 | Answer:** 是的，4xx表示客户端错误 | Yes, 4xx indicates client errors
  
  **常见误区检查 | Common Misconception Checks:**
  - 原因短语是否必须与标准匹配？| Must reason phrases match the standard?
    **答案 | Answer:** 否，原因短语可以自定义，只要状态码正确 | No, reason phrases can be customized as long as status code is correct
  - 响应是否必须包含消息体？| Must responses include a message body?
    **答案 | Answer:** 否，很多响应（如204）不包含消息体 | No, many responses (like 204) don't include message body

### 4. HTTP头部字段处理 | HTTP Header Field Processing (45分钟 | 45 minutes)

- **头部字段解析和验证 | Header Field Parsing and Validation**
  
  **概念定义 | Concept Definition:**
  HTTP头部字段承载请求和响应的元数据，包含内容类型、长度、编码、缓存控制等重要信息，需要正确解析、验证和处理以确保HTTP通信的可靠性。 | HTTP header fields carry metadata for requests and responses, containing important information like content type, length, encoding, cache control, requiring correct parsing, validation, and processing to ensure reliable HTTP communication.
  
  **解决的问题 | Problems It Solves:**
  - 内容协商：通过Accept等头部实现内容类型协商 | Content negotiation: achieve content type negotiation through Accept headers
  - 缓存控制：通过Cache-Control等头部控制缓存行为 | Cache control: control caching behavior through Cache-Control headers
  - 安全控制：通过各种安全头部提升应用安全性 | Security control: enhance application security through various security headers
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Content-Type头部是否必需？| Is the Content-Type header required?
     **答案 | Answer:** 对于有消息体的请求/响应通常需要 | Usually needed for requests/responses with message body
  2. 同一个头部名称是否可以出现多次？| Can the same header name appear multiple times?
     **答案 | Answer:** 某些头部可以，如Set-Cookie | Some headers can, like Set-Cookie
  3. 头部字段值是否可以包含非ASCII字符？| Can header field values contain non-ASCII characters?
     **答案 | Answer:** 需要编码，如RFC 2047规定 | Need encoding as specified by RFC 2047
  4. Content-Length为0是否表示没有消息体？| Does Content-Length: 0 mean no message body?
     **答案 | Answer:** 表示消息体长度为0字节 | Indicates message body length is 0 bytes
  
  **实际应用示例 | Real-world Application Examples:**
  ```rust
  use std::collections::HashMap;
  use chrono::{DateTime, Utc};
  use std::str::FromStr;
  
  // 常见头部字段处理 | Common header field processing
  #[derive(Debug, Clone)]
  pub struct HeaderProcessor;
  
  impl HeaderProcessor {
      // 解析Content-Type头部 | Parse Content-Type header
      pub fn parse_content_type(value: &str) -> (String, HashMap<String, String>) {
          let mut parts = value.split(';');
          let media_type = parts.next().unwrap_or("").trim().to_string();
          let mut parameters = HashMap::new();
          
          for part in parts {
              if let Some(eq_pos) = part.find('=') {
                  let key = part[..eq_pos].trim().to_string();
                  let mut val = part[eq_pos + 1..].trim();
                  
                  // 处理引号 | Handle quotes
                  if val.starts_with('"') && val.ends_with('"') {
                      val = &val[1..val.len()-1];
                  }
                  
                  parameters.insert(key, val.to_string());
              }
          }
          
          (media_type, parameters)
      }
      
      // 解析Accept头部 | Parse Accept header
      pub fn parse_accept(value: &str) -> Vec<(String, f32)> {
          let mut media_types = Vec::new();
          
          for item in value.split(',') {
              let item = item.trim();
              let mut parts = item.split(';');
              let media_type = parts.next().unwrap_or("").trim().to_string();
              
              let mut quality = 1.0;
              for part in parts {
                  let part = part.trim();
                  if part.starts_with("q=") {
                      if let Ok(q) = part[2..].parse::<f32>() {
                          quality = q;
                      }
                  }
              }
              
              if !media_type.is_empty() {
                  media_types.push((media_type, quality));
              }
          }
          
          // 按quality值降序排序 | Sort by quality value descending
          media_types.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
          media_types
      }
      
      // 解析Date头部 | Parse Date header
      pub fn parse_date(value: &str) -> Option<DateTime<Utc>> {
          // 尝试不同的日期格式 | Try different date formats
          let formats = [
              "%a, %d %b %Y %H:%M:%S GMT",        // RFC 1123
              "%A, %d-%b-%y %H:%M:%S GMT",        // RFC 1036
              "%a %b %e %H:%M:%S %Y",             // ANSI C asctime()
          ];
          
          for format in &formats {
              if let Ok(dt) = DateTime::parse_from_str(value, format) {
                  return Some(dt.with_timezone(&Utc));
              }
          }
          
          None
      }
      
      // 生成Date头部 | Generate Date header
      pub fn format_date(dt: &DateTime<Utc>) -> String {
          dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
      }
      
      // 解析Cache-Control头部 | Parse Cache-Control header
      pub fn parse_cache_control(value: &str) -> HashMap<String, Option<String>> {
          let mut directives = HashMap::new();
          
          for directive in value.split(',') {
              let directive = directive.trim();
              if let Some(eq_pos) = directive.find('=') {
                  let key = directive[..eq_pos].trim().to_string();
                  let val = directive[eq_pos + 1..].trim();
                  let val = if val.starts_with('"') && val.ends_with('"') {
                      &val[1..val.len()-1]
                  } else {
                      val
                  };
                  directives.insert(key, Some(val.to_string()));
              } else {
                  directives.insert(directive.to_string(), None);
              }
          }
          
          directives
      }
      
      // 验证头部字段 | Validate header fields
      pub fn validate_headers(headers: &HashMap<String, String>) -> Vec<String> {
          let mut errors = Vec::new();
          
          // 检查Content-Length | Check Content-Length
          if let Some(cl_value) = headers.get("content-length") {
              if cl_value.parse::<usize>().is_err() {
                  errors.push("Content-Length必须是非负整数 | Content-Length must be non-negative integer".to_string());
              }
          }
          
          // 检查Host头部(HTTP/1.1必需) | Check Host header (required for HTTP/1.1)
          if !headers.contains_key("host") {
              errors.push("HTTP/1.1请求必须包含Host头部 | HTTP/1.1 requests must include Host header".to_string());
          }
          
          // 检查Content-Type格式 | Check Content-Type format
          if let Some(ct_value) = headers.get("content-type") {
              let (media_type, _) = Self::parse_content_type(ct_value);
              if media_type.is_empty() || !media_type.contains('/') {
                  errors.push("Content-Type格式无效 | Invalid Content-Type format".to_string());
              }
          }
          
          // 检查日期格式 | Check date format
          if let Some(date_value) = headers.get("date") {
              if Self::parse_date(date_value).is_none() {
                  errors.push("Date头部格式无效 | Invalid Date header format".to_string());
              }
          }
          
          errors
      }
  }
  
  // 头部字段构建器 | Header field builder
  #[derive(Debug)]
  pub struct HeaderBuilder {
      headers: HashMap<String, String>,
  }
  
  impl HeaderBuilder {
      pub fn new() -> Self {
          HeaderBuilder {
              headers: HashMap::new(),
          }
      }
      
      // 设置基本头部 | Set basic headers
      pub fn content_type(mut self, media_type: &str) -> Self {
          self.headers.insert("content-type".to_string(), media_type.to_string());
          self
      }
      
      pub fn content_length(mut self, length: usize) -> Self {
          self.headers.insert("content-length".to_string(), length.to_string());
          self
      }
      
      pub fn server(mut self, server: &str) -> Self {
          self.headers.insert("server".to_string(), server.to_string());
          self
      }
      
      pub fn date_now(mut self) -> Self {
          let now = Utc::now();
          self.headers.insert("date".to_string(), HeaderProcessor::format_date(&now));
          self
      }
      
      // 设置缓存控制 | Set cache control
      pub fn cache_control(mut self, directives: &[&str]) -> Self {
          let value = directives.join(", ");
          self.headers.insert("cache-control".to_string(), value);
          self
      }
      
      // 设置安全头部 | Set security headers
      pub fn security_headers(mut self) -> Self {
          self.headers.insert("x-content-type-options".to_string(), "nosniff".to_string());
          self.headers.insert("x-frame-options".to_string(), "DENY".to_string());
          self.headers.insert("x-xss-protection".to_string(), "1; mode=block".to_string());
          self.headers.insert("strict-transport-security".to_string(), 
                            "max-age=31536000; includeSubDomains".to_string());
          self
      }
      
      // 自定义头部 | Custom header
      pub fn custom(mut self, name: &str, value: &str) -> Self {
          self.headers.insert(name.to_lowercase(), value.to_string());
          self
      }
      
      // 构建头部映射 | Build header map
      pub fn build(self) -> HashMap<String, String> {
          self.headers
      }
  }
  
  // 测试头部处理 | Test header processing
  fn test_header_processing() {
      println!("=== HTTP头部处理测试 | HTTP Header Processing Test ===");
      
      // 测试头部解析 | Test header parsing
      println!("\n--- 头部解析测试 | Header Parsing Test ---");
      
      let test_headers = vec![
          ("content-type", "application/json; charset=utf-8"),
          ("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
          ("date", "Mon, 27 Jul 2009 12:28:53 GMT"),
          ("cache-control", "public, max-age=3600, no-cache=\"Set-Cookie\""),
      ];
      
      for (name, value) in test_headers {
          println!("\n处理头部 | Processing header: {}: {}", name, value);
          
          match name {
              "content-type" => {
                  let (media_type, params) = HeaderProcessor::parse_content_type(value);
                  println!("  媒体类型 | Media Type: {}", media_type);
                  for (key, val) in params {
                      println!("  参数 | Parameter: {} = {}", key, val);
                  }
              },
              "accept" => {
                  let media_types = HeaderProcessor::parse_accept(value);
                  println!("  接受类型 | Accept Types:");
                  for (media_type, quality) in media_types {
                      println!("    {} (q={})", media_type, quality);
                  }
              },
              "date" => {
                  if let Some(dt) = HeaderProcessor::parse_date(value) {
                      println!("  解析日期 | Parsed Date: {}", dt);
                  } else {
                      println!("  日期解析失败 | Date parsing failed");
                  }
              },
              "cache-control" => {
                  let directives = HeaderProcessor::parse_cache_control(value);
                  println!("  缓存指令 | Cache Directives:");
                  for (directive, val) in directives {
                      if let Some(v) = val {
                          println!("    {} = {}", directive, v);
                      } else {
                          println!("    {}", directive);
                      }
                  }
              },
              _ => {}
          }
      }
      
      // 测试头部构建 | Test header building
      println!("\n--- 头部构建测试 | Header Building Test ---");
      
      let headers = HeaderBuilder::new()
          .content_type("application/json; charset=utf-8")
          .content_length(42)
          .server("Rust-Server/1.0")
          .date_now()
          .cache_control(&["public", "max-age=3600"])
          .security_headers()
          .custom("x-request-id", "req-123456")
          .build();
      
      println!("构建的头部 | Built Headers:");
      for (name, value) in &headers {
          println!("  {}: {}", name, value);
      }
      
      // 测试头部验证 | Test header validation
      println!("\n--- 头部验证测试 | Header Validation Test ---");
      
      let validation_errors = HeaderProcessor::validate_headers(&headers);
      if validation_errors.is_empty() {
          println!("✅ 头部验证通过 | Header validation passed");
      } else {
          println!("❌ 头部验证失败 | Header validation failed:");
          for error in validation_errors {
              println!("  - {}", error);
          }
      }
  }
  ```
  
  **综合应用检查 | Comprehensive Application Check:**
  - Content-Type头部的charset参数有什么作用？| What role does the charset parameter in Content-Type header play?
    **答案 | Answer:** 指定文本内容的字符编码 | Specifies character encoding of text content
  - Accept头部的q值表示什么？| What does the q value in Accept header represent?
    **答案 | Answer:** 质量因子，表示客户端对该媒体类型的偏好程度 | Quality factor, indicating client preference for that media type
  - 安全头部有哪些常见类型？| What are common types of security headers?
    **答案 | Answer:** X-Content-Type-Options, X-Frame-Options, HSTS等 | X-Content-Type-Options, X-Frame-Options, HSTS, etc.

### 5. HTTP消息体处理 | HTTP Message Body Processing (30分钟 | 30 minutes)

- **消息体编码和传输 | Message Body Encoding and Transfer**
  
  **关键原则 | Key Principles:**
  - 内容编码：支持gzip、deflate等压缩算法 | Content encoding: support compression algorithms like gzip, deflate
  - 传输编码：支持chunked transfer等分块传输 | Transfer encoding: support chunked transfer and other mechanisms
  - 字符集处理：正确处理不同字符编码 | Character set handling: correctly process different character encodings
  
  **实践验证问题 | Practice Verification Questions:**
  1. chunked传输编码的优势是什么？| What are the advantages of chunked transfer encoding?
     **答案 | Answer:** 不需要预先知道内容长度，支持流式传输 | No need to know content length in advance, supports streaming
  2. Content-Encoding和Transfer-Encoding有什么区别？| What's the difference between Content-Encoding and Transfer-Encoding?
     **答案 | Answer:** Content-Encoding是内容压缩，Transfer-Encoding是传输方式 | Content-Encoding is content compression, Transfer-Encoding is transmission method
  3. 如何处理不同字符编码的消息体？| How to handle message bodies with different character encodings?
     **答案 | Answer:** 根据Content-Type头部的charset参数解码 | Decode according to charset parameter in Content-Type header

### 6. 性能优化和最佳实践 | Performance Optimization and Best Practices (15分钟 | 15 minutes)

- **HTTP解析性能优化 | HTTP Parsing Performance Optimization**
  
  **综合概念检查 | Comprehensive Concept Check:**
  1. 如何优化HTTP头部解析的性能？| How to optimize HTTP header parsing performance?
     **答案 | Answer:** 使用零拷贝技术，避免不必要的字符串分配 | Use zero-copy techniques, avoid unnecessary string allocations
  2. 大文件传输时应该注意什么？| What should be noted when transferring large files?
     **答案 | Answer:** 使用流式处理，避免一次性加载到内存 | Use streaming processing, avoid loading everything into memory at once
  3. HTTP/2对消息解析有什么影响？| How does HTTP/2 affect message parsing?
     **答案 | Answer:** 使用二进制格式，头部压缩，需要不同的解析策略 | Uses binary format, header compression, requires different parsing strategies
  4. 如何处理恶意的HTTP请求？| How to handle malicious HTTP requests?
     **答案 | Answer:** 设置大小限制，超时控制，输入验证 | Set size limits, timeout controls, input validation
  5. 解析器的状态管理为什么重要？| Why is state management important for parsers?
     **答案 | Answer:** 支持流式解析，处理不完整的消息 | Supports streaming parsing, handles incomplete messages

## 实践项目：HTTP消息解析器 | Practical Project: HTTP Message Parser

### 目标 | Objective
构建一个高性能、功能完整的HTTP消息解析器，综合应用HTTP协议知识、字符串处理技术和错误处理策略，支持请求和响应的完整解析。 | Build a high-performance, fully functional HTTP message parser that comprehensively applies HTTP protocol knowledge, string processing techniques, and error handling strategies, supporting complete parsing of requests and responses.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. HTTP消息的三个主要部分是什么？| What are the three main parts of HTTP messages?
   **答案 | Answer:** 起始行、头部字段、消息体 | Start line, header fields, message body
2. 如何区分请求消息和响应消息？| How to distinguish between request and response messages?
   **答案 | Answer:** 通过起始行格式：请求行包含方法和URI，状态行包含状态码 | Through start line format: request line contains method and URI, status line contains status code
3. 头部字段解析中需要注意什么？| What needs attention in header field parsing?
   **答案 | Answer:** 大小写不敏感，值可能包含空格，支持多行值 | Case insensitive, values may contain spaces, support multi-line values

### 步骤 | Steps
1. 设计解析器架构，支持流式和批量解析模式 | Design parser architecture supporting streaming and batch parsing modes
2. 实现HTTP消息基础结构解析，验证协议理解 | Implement HTTP message basic structure parsing, verify protocol understanding
3. 添加头部字段专门处理，支持常见头部类型 | Add specialized header field processing, support common header types
4. 集成消息体处理，支持不同编码和传输方式 | Integrate message body processing, support different encodings and transfer methods
5. 优化性能和内存使用，添加完整的错误处理 | Optimize performance and memory usage, add comprehensive error handling

### 示例代码 | Example Code
```rust
"""
HTTP消息解析器 | HTTP Message Parser
高性能HTTP协议消息解析和生成工具 | High-performance HTTP protocol message parsing and generation tool

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- HTTP协议消息格式解析 | HTTP protocol message format parsing
- 头部字段处理和验证 | Header field processing and validation
- 消息体编码和传输处理 | Message body encoding and transfer processing
- 性能优化和错误处理 | Performance optimization and error handling
"""

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::fmt;
use thiserror::Error;

// 解析错误类型 | Parser error types
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("无效的HTTP消息格式 | Invalid HTTP message format")]
    InvalidFormat,
    #[error("缺少必需的头部字段: {0} | Missing required header: {0}")]
    MissingHeader(String),
    #[error("无效的头部字段值: {field} = {value} | Invalid header value: {field} = {value}")]
    InvalidHeaderValue { field: String, value: String },
    #[error("消息体长度不匹配 | Message body length mismatch")]
    BodyLengthMismatch,
    #[error("不支持的传输编码: {0} | Unsupported transfer encoding: {0}")]
    UnsupportedEncoding(String),
    #[error("IO错误: {0} | IO error: {0}")]
    Io(#[from] io::Error),
}

// 解析器状态 | Parser state
#[derive(Debug, Clone, PartialEq)]
pub enum ParserState {
    ReadingStartLine,
    ReadingHeaders,
    ReadingBody,
    Complete,
}

// HTTP消息类型 | HTTP message type
#[derive(Debug, Clone)]
pub enum HttpMessage {
    Request(HttpRequest),
    Response(HttpResponse),
}

// 高性能HTTP解析器 | High-performance HTTP parser
pub struct HttpParser {
    state: ParserState,
    buffer: Vec<u8>,
    position: usize,
    max_header_size: usize,
    max_body_size: usize,
}

impl HttpParser {
    // 创建新解析器 | Create new parser
    pub fn new() -> Self {
        HttpParser {
            state: ParserState::ReadingStartLine,
            buffer: Vec::new(),
            position: 0,
            max_header_size: 8192,  // 8KB头部限制 | 8KB header limit
            max_body_size: 1024 * 1024, // 1MB消息体限制 | 1MB body limit
        }
    }
    
    // 配置限制 | Configure limits
    pub fn with_limits(mut self, max_header_size: usize, max_body_size: usize) -> Self {
        self.max_header_size = max_header_size;
        self.max_body_size = max_body_size;
        self
    }
    
    // 解析HTTP消息 | Parse HTTP message
    pub fn parse(&mut self, data: &[u8]) -> Result<Option<HttpMessage>, ParseError> {
        self.buffer.extend_from_slice(data);
        
        match self.state {
            ParserState::ReadingStartLine => self.parse_start_line(),
            ParserState::ReadingHeaders => self.parse_headers(),
            ParserState::ReadingBody => self.parse_body(),
            ParserState::Complete => Ok(None),
        }
    }
    
    // 解析起始行 | Parse start line
    fn parse_start_line(&mut self) -> Result<Option<HttpMessage>, ParseError> {
        if let Some(line_end) = self.find_line_end() {
            let line = &self.buffer[self.position..line_end];
            let line_str = String::from_utf8_lossy(line);
            
            // 检查是请求还是响应 | Check if request or response
            if line_str.starts_with("HTTP/") {
                // 响应消息 | Response message
                self.parse_response_line(&line_str)
            } else {
                // 请求消息 | Request message
                self.parse_request_line(&line_str)
            }?;
            
            self.position = line_end + 2; // 跳过CRLF | Skip CRLF
            self.state = ParserState::ReadingHeaders;
        }
        
        Ok(None) // 需要更多数据 | Need more data
    }
    
    // 解析请求行 | Parse request line
    fn parse_request_line(&self, line: &str) -> Result<(), ParseError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(ParseError::InvalidFormat);
        }
        
        // 验证HTTP方法 | Validate HTTP method
        let method = HttpMethod::from_str(parts[0]);
        
        // 验证URI | Validate URI
        if parts[1].is_empty() {
            return Err(ParseError::InvalidFormat);
        }
        
        // 验证版本 | Validate version
        HttpVersion::from_str(parts[2])
            .map_err(|_| ParseError::InvalidFormat)?;
        
        Ok(())
    }
    
    // 解析状态行 | Parse status line
    fn parse_response_line(&self, line: &str) -> Result<(), ParseError> {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() < 2 {
            return Err(ParseError::InvalidFormat);
        }
        
        // 验证版本 | Validate version
        HttpVersion::from_str(parts[0])
            .map_err(|_| ParseError::InvalidFormat)?;
        
        // 验证状态码 | Validate status code
        parts[1].parse::<u16>()
            .map_err(|_| ParseError::InvalidFormat)?;
        
        Ok(())
    }
    
    // 解析头部字段 | Parse header fields
    fn parse_headers(&mut self) -> Result<Option<HttpMessage>, ParseError> {
        let mut headers = HashMap::new();
        
        loop {
            if let Some(line_end) = self.find_line_end() {
                let line = &self.buffer[self.position..line_end];
                
                if line.is_empty() {
                    // 空行，头部解析完成 | Empty line, headers complete
                    self.position = line_end + 2; // 跳过CRLF | Skip CRLF
                    self.state = ParserState::ReadingBody;
                    break;
                }
                
                // 检查头部大小限制 | Check header size limit
                if self.position > self.max_header_size {
                    return Err(ParseError::InvalidFormat);
                }
                
                // 解析头部字段 | Parse header field
                let line_str = String::from_utf8_lossy(line);
                if let Some(colon_pos) = line_str.find(':') {
                    let name = line_str[..colon_pos].trim().to_lowercase();
                    let value = line_str[colon_pos + 1..].trim().to_string();
                    headers.insert(name, value);
                } else {
                    return Err(ParseError::InvalidFormat);
                }
                
                self.position = line_end + 2; // 跳过CRLF | Skip CRLF
            } else {
                return Ok(None); // 需要更多数据 | Need more data
            }
        }
        
        // 验证必需的头部 | Validate required headers
        self.validate_headers(&headers)?;
        
        Ok(None) // 继续解析消息体 | Continue to parse body
    }
    
    // 解析消息体 | Parse message body
    fn parse_body(&mut self) -> Result<Option<HttpMessage>, ParseError> {
        // 这里简化处理，实际需要根据Content-Length或Transfer-Encoding | 
        // Simplified handling here, actually needs to handle based on Content-Length or Transfer-Encoding
        let remaining = &self.buffer[self.position..];
        let body = remaining.to_vec();
        
        // 检查消息体大小限制 | Check body size limit
        if body.len() > self.max_body_size {
            return Err(ParseError::InvalidFormat);
        }
        
        self.state = ParserState::Complete;
        
        // 构建完整消息 | Build complete message
        Ok(Some(self.build_message(body)?))
    }
    
    // 构建消息对象 | Build message object
    fn build_message(&self, body: Vec<u8>) -> Result<HttpMessage, ParseError> {
        // 从缓冲区重新解析构建消息 | Re-parse from buffer to build message
        // 这里简化处理，实际实现需要保存解析状态 | 
        // Simplified here, actual implementation needs to save parsing state
        
        // 示例：构建一个基础请求 | Example: build a basic request
        Ok(HttpMessage::Request(HttpRequest {
            method: HttpMethod::GET,
            uri: RequestUri::from_str("/"),
            version: HttpVersion { major: 1, minor: 1 },
            headers: HashMap::new(),
            body,
        }))
    }
    
    // 查找行结束位置 | Find line end position
    fn find_line_end(&self) -> Option<usize> {
        for i in self.position..self.buffer.len().saturating_sub(1) {
            if self.buffer[i] == b'\r' && self.buffer[i + 1] == b'\n' {
                return Some(i);
            }
        }
        None
    }
    
    // 验证头部字段 | Validate header fields
    fn validate_headers(&self, headers: &HashMap<String, String>) -> Result<(), ParseError> {
        // 验证Content-Length | Validate Content-Length
        if let Some(cl) = headers.get("content-length") {
            if cl.parse::<usize>().is_err() {
                return Err(ParseError::InvalidHeaderValue {
                    field: "content-length".to_string(),
                    value: cl.clone(),
                });
            }
        }
        
        // 可以添加更多验证逻辑 | Can add more validation logic
        Ok(())
    }
    
    // 重置解析器 | Reset parser
    pub fn reset(&mut self) {
        self.state = ParserState::ReadingStartLine;
        self.buffer.clear();
        self.position = 0;
    }
}

// HTTP消息生成器 | HTTP message generator
pub struct HttpGenerator;

impl HttpGenerator {
    // 生成HTTP请求 | Generate HTTP request
    pub fn generate_request(request: &HttpRequest) -> Vec<u8> {
        let mut output = Vec::new();
        
        // 请求行 | Request line
        let request_line = format!("{} {} {}\r\n", request.method, request.uri, request.version);
        output.extend_from_slice(request_line.as_bytes());
        
        // 头部字段 | Header fields
        for (name, value) in &request.headers {
            let header_line = format!("{}: {}\r\n", name, value);
            output.extend_from_slice(header_line.as_bytes());
        }
        
        // 空行 | Empty line
        output.extend_from_slice(b"\r\n");
        
        // 消息体 | Message body
        output.extend_from_slice(&request.body);
        
        output
    }
    
    // 生成HTTP响应 | Generate HTTP response
    pub fn generate_response(response: &HttpResponse) -> Vec<u8> {
        let mut output = Vec::new();
        
        // 状态行 | Status line
        let status_line = format!("{} {} {}\r\n", 
                                 response.version, 
                                 response.status_code.as_u16(), 
                                 response.reason_phrase);
        output.extend_from_slice(status_line.as_bytes());
        
        // 头部字段 | Header fields
        for (name, value) in &response.headers {
            let header_line = format!("{}: {}\r\n", name, value);
            output.extend_from_slice(header_line.as_bytes());
        }
        
        // 空行 | Empty line
        output.extend_from_slice(b"\r\n");
        
        // 消息体 | Message body
        output.extend_from_slice(&response.body);
        
        output
    }
}

// 性能测试和演示 | Performance testing and demonstration
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parser_performance() {
        let mut parser = HttpParser::new();
        
        // 测试大量小请求解析 | Test parsing many small requests
        let test_request = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
        
        let start = std::time::Instant::now();
        for _ in 0..10000 {
            parser.reset();
            let _ = parser.parse(test_request);
        }
        let elapsed = start.elapsed();
        
        println!("解析10000个请求耗时: {:?} | Time to parse 10000 requests: {:?}", elapsed, elapsed);
        println!("平均每个请求: {:?} | Average per request: {:?}", 
                elapsed / 10000, elapsed / 10000);
    }
    
    #[test]
    fn test_incremental_parsing() {
        let mut parser = HttpParser::new();
        
        // 模拟网络数据分块到达 | Simulate network data arriving in chunks
        let full_request = b"GET /api/test HTTP/1.1\r\nHost: example.com\r\nContent-Length: 5\r\n\r\nHello";
        
        // 分块发送数据 | Send data in chunks
        let chunks = [
            &full_request[0..10],   // "GET /api/t"
            &full_request[10..30],  // "est HTTP/1.1\r\nHost: "
            &full_request[30..50],  // "example.com\r\nContent"
            &full_request[50..],    // "-Length: 5\r\n\r\nHello"
        ];
        
        for chunk in chunks.iter() {
            match parser.parse(chunk) {
                Ok(Some(message)) => {
                    println!("解析完成 | Parsing complete: {:?}", message);
                    break;
                },
                Ok(None) => {
                    println!("需要更多数据 | Need more data, current state: {:?}", parser.state);
                },
                Err(e) => {
                    println!("解析错误 | Parse error: {}", e);
                    break;
                },
            }
        }
    }
}

// 主函数演示 | Main function demonstration
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== HTTP消息解析器演示 | HTTP Message Parser Demo ===");
    
    // 测试基础解析功能 | Test basic parsing functionality
    test_message_parsing();
    test_request_parsing();
    test_response_handling();
    test_header_processing();
    
    // 性能测试 | Performance testing
    println!("\n=== 性能测试 | Performance Testing ===");
    let mut parser = HttpParser::new().with_limits(16384, 2 * 1024 * 1024);
    
    let test_data = b"GET /benchmark HTTP/1.1\r\n\
                     Host: benchmark.example.com\r\n\
                     User-Agent: Rust-HTTP-Parser/1.0\r\n\
                     Accept: */*\r\n\
                     Content-Length: 0\r\n\
                     \r\n";
    
    let start = std::time::Instant::now();
    let iterations = 100000;
    
    for _ in 0..iterations {
        parser.reset();
        if let Ok(Some(_)) = parser.parse(test_data) {
            // 解析成功 | Parse successful
        }
    }
    
    let elapsed = start.elapsed();
    println!("解析{}个请求耗时: {:?} | Time to parse {} requests: {:?}", 
             iterations, elapsed, iterations, elapsed);
    println!("每秒处理请求数: {:.0} | Requests per second: {:.0}", 
             iterations as f64 / elapsed.as_secs_f64(),
             iterations as f64 / elapsed.as_secs_f64());
    
    Ok(())
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确解析了HTTP消息的所有主要部分？| Does the project correctly parse all major parts of HTTP messages?
2. 头部字段处理是否遵循HTTP协议规范？| Does header field processing follow HTTP protocol specifications?
3. 错误处理是否覆盖了常见的异常情况？| Does error handling cover common exceptional situations?
4. 解析器的性能是否满足实际应用需求？| Does the parser's performance meet practical application requirements?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **HTTP/2消息格式研究练习 | HTTP/2 Message Format Research Exercise**
   - **练习描述 | Exercise Description:** 研究HTTP/2的二进制帧格式，对比HTTP/1.1的文本格式
   - **概念检查 | Concept Check:** HTTP/2如何改进HTTP/1.1的性能问题？
   - **学习目标 | Learning Objective:** 理解协议演进和性能优化思路

2. **Chunked传输编码实现练习 | Chunked Transfer Encoding Implementation Exercise**
   - **练习描述 | Exercise Description:** 实现完整的chunked传输编码解析和生成
   - **概念检查 | Concept Check:** chunked编码如何支持流式传输？
   - **学习目标 | Learning Objective:** 深入理解HTTP传输机制

3. **多部分消息处理练习 | Multipart Message Processing Exercise**
   - **练习描述 | Exercise Description:** 实现multipart/form-data格式的解析
   - **概念检查 | Concept Check:** 如何处理文件上传中的二进制数据？
   - **学习目标 | Learning Objective:** 掌握复杂消息格式处理

4. **HTTP缓存头部练习 | HTTP Cache Headers Exercise**
   - **练习描述 | Exercise Description:** 实现完整的HTTP缓存控制逻辑
   - **概念检查 | Concept Check:** ETag和Last-Modified的区别是什么？
   - **学习目标 | Learning Objective:** 理解Web缓存机制

5. **安全头部检查练习 | Security Headers Check Exercise**
   - **练习描述 | Exercise Description:** 创建HTTP安全头部检查和建议工具
   - **概念检查 | Concept Check:** CORS预检请求的工作原理？
   - **学习目标 | Learning Objective:** 提升Web安全意识

6. **性能基准测试练习 | Performance Benchmarking Exercise**
   - **练习描述 | Exercise Description:** 对比不同HTTP解析库的性能表现
   - **概念检查 | Concept Check:** 影响解析性能的主要因素有哪些？
   - **学习目标 | Learning Objective:** 掌握性能分析和优化方法

7. **HTTP客户端实现练习 | HTTP Client Implementation Exercise**
   - **练习描述 | Exercise Description:** 基于解析器构建完整的HTTP客户端
   - **概念检查 | Concept Check:** 如何处理重定向和持久连接？
   - **学习目标 | Learning Objective:** 综合应用HTTP协议知识

## 学习资源 | Learning Resources
- [HTTP协议官方规范 - RFC 7230-7237](https://tools.ietf.org/rfc/rfc7230.txt)
- [MDN HTTP文档](https://developer.mozilla.org/en-US/docs/Web/HTTP)
- [Rust HTTP解析库源码分析](https://docs.rs/httparse/)
- [Web性能优化指南](https://web.dev/performance/)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解HTTP协议消息格式和结构 | Understand HTTP protocol message format and structure
- [ ] 掌握HTTP请求行和状态行解析 | Master HTTP request line and status line parsing
- [ ] 熟练处理各种HTTP头部字段 | Proficiently handle various HTTP header fields
- [ ] 能够正确解析和生成HTTP消息体 | Correctly parse and generate HTTP message bodies
- [ ] 实现完整的HTTP消息解析器项目 | Implement complete HTTP message parser project
- [ ] 正确回答所有CCQs概念检查问题 | Correctly answer all CCQs concept checking questions
- [ ] 理解HTTP协议的性能优化技巧 | Understand HTTP protocol performance optimization techniques
- [ ] 掌握错误处理和安全考虑 | Master error handling and security considerations
- [ ] 了解HTTP协议的发展和未来趋势 | Understand HTTP protocol development and future trends
- [ ] 完成至少3个扩展练习 | Complete at least 3 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释HTTP协议的消息格式、解析过程和实际应用场景。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain HTTP protocol message format, parsing process, and practical application scenarios to others.