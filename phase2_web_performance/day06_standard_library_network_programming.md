# Rust入门 - 第6天：标准库网络编程 | Rust Introduction - Day 6: Standard Library Network Programming

## 学习目标 | Learning Objectives
- 深入掌握Rust std::net模块的核心组件和使用方法 | Master core components and usage methods of Rust std::net module
- 熟练进行TCP socket编程，包括客户端和服务端开发 | Proficiently perform TCP socket programming including client and server development
- 理解网络通信的底层机制和最佳实践 | Understand underlying mechanisms and best practices of network communication
- 能够从零构建简单但功能完整的HTTP服务器 | Build simple but functionally complete HTTP servers from scratch
- 掌握网络错误处理、连接管理和性能优化技巧 | Master network error handling, connection management, and performance optimization techniques
- 通过实践项目建立扎实的网络编程基础 | Build solid network programming foundation through practical projects

## 详细内容 | Detailed Content

### 1. std::net模块深度解析 | std::net Module In-Depth Analysis (1.5小时 | 1.5 hours)

- **std::net模块核心概念 | std::net Module Core Concepts**
  
  **概念定义 | Concept Definition:**
  std::net是Rust标准库中专门处理网络通信的模块，提供了TCP、UDP等协议的底层抽象，是构建网络应用程序的基础。它遵循Rust的安全性和零成本抽象原则。| std::net is the network communication module in Rust's standard library, providing low-level abstractions for TCP, UDP and other protocols. It's the foundation for building network applications and follows Rust's safety and zero-cost abstraction principles.
  
  **核心特征 | Key Characteristics:**
  - 类型安全：编译时防止常见网络编程错误 | Type safety: prevents common network programming errors at compile time
  - 跨平台兼容：统一的API适用于不同操作系统 | Cross-platform compatibility: unified API works across different operating systems
  - 零成本抽象：高级API不带来运行时开销 | Zero-cost abstraction: high-level API without runtime overhead
  - 阻塞式IO：简单直观的同步网络操作 | Blocking I/O: simple and intuitive synchronous network operations
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. std::net模块提供的是同步还是异步的网络API？| Does std::net module provide synchronous or asynchronous network APIs?
     **答案 | Answer:** 同步 | Synchronous - std::net提供阻塞式同步API，异步需要使用tokio等第三方库 | std::net provides blocking synchronous APIs, asynchronous operations require third-party libraries like tokio
  2. TcpStream和TcpListener有什么区别？| What's the difference between TcpStream and TcpListener?
     **答案 | Answer:** TcpStream表示已建立的连接，TcpListener监听传入连接 | TcpStream represents an established connection, TcpListener listens for incoming connections
  3. SocketAddr可以同时表示IPv4和IPv6地址吗？| Can SocketAddr represent both IPv4 and IPv6 addresses?
     **答案 | Answer:** 是 | Yes - SocketAddr是一个枚举，包含SocketAddrV4和SocketAddrV6两个变体 | SocketAddr is an enum containing SocketAddrV4 and SocketAddrV6 variants
  4. std::net中的操作是线程安全的吗？| Are operations in std::net thread-safe?
     **答案 | Answer:** 部分是 | Partially - 单个连接操作是线程安全的，但同时读写需要额外同步 | Individual connection operations are thread-safe, but concurrent read/write requires additional synchronization
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // std::net模块核心类型和功能演示 | std::net module core types and functionality demonstration
  use std::net::{TcpListener, TcpStream, SocketAddr, Ipv4Addr, IpAddr, SocketAddrV4};
  use std::io::{Read, Write, Result};
  use std::time::Duration;
  
  /// 网络地址工具类 | Network address utilities
  pub struct NetworkUtils;
  
  impl NetworkUtils {
      /// 创建和解析网络地址 | Create and parse network addresses
      pub fn demonstrate_address_types() {
          println!("=== 网络地址类型演示 | Network Address Type Demonstration ===");
          
          // 1. IPv4地址创建 | IPv4 address creation
          let ipv4 = Ipv4Addr::new(127, 0, 0, 1);
          let localhost_ipv4 = Ipv4Addr::LOCALHOST;
          let any_ipv4 = Ipv4Addr::UNSPECIFIED; // 0.0.0.0
          
          println!("IPv4地址 | IPv4 addresses:");
          println!("  自定义IPv4: {} | Custom IPv4: {}", ipv4, ipv4);
          println!("  本地回环: {} | Localhost: {}", localhost_ipv4, localhost_ipv4);
          println!("  任意地址: {} | Any address: {}", any_ipv4, any_ipv4);
          
          // 2. 套接字地址创建 | Socket address creation
          let socket_addr_v4 = SocketAddrV4::new(localhost_ipv4, 8080);
          let socket_addr: SocketAddr = SocketAddr::V4(socket_addr_v4);
          
          println!("\n套接字地址 | Socket addresses:");
          println!("  IPv4套接字地址: {} | IPv4 socket address: {}", socket_addr, socket_addr);
          
          // 3. 地址解析 | Address parsing
          match "127.0.0.1:8080".parse::<SocketAddr>() {
              Ok(addr) => println!("  解析成功: {} | Parsed successfully: {}", addr, addr),
              Err(e) => println!("  解析失败: {} | Parse failed: {}", e, e),
          }
          
          // 4. 地址属性检查 | Address property checking
          println!("\n地址属性检查 | Address property checking:");
          println!("  是否为回环地址: {} | Is loopback: {}", ipv4.is_loopback(), ipv4.is_loopback());
          println!("  是否为私有地址: {} | Is private: {}", ipv4.is_private(), ipv4.is_private());
          println!("  是否为组播地址: {} | Is multicast: {}", ipv4.is_multicast(), ipv4.is_multicast());
      }
      
      /// 演示基础连接选项 | Demonstrate basic connection options
      pub fn demonstrate_connection_options() -> Result<()> {
          println!("\n=== 连接选项演示 | Connection Options Demonstration ===");
          
          // 1. 创建TCP监听器 | Create TCP listener
          let listener = TcpListener::bind("127.0.0.1:0")?; // 使用系统分配端口 | Use system-assigned port
          let local_addr = listener.local_addr()?;
          println!("监听器绑定到: {} | Listener bound to: {}", local_addr, local_addr);
          
          // 2. 设置监听器选项 | Set listener options
          listener.set_nonblocking(false)?; // 设置为阻塞模式 | Set to blocking mode
          println!("监听器设置为阻塞模式 | Listener set to blocking mode");
          
          // 3. 创建客户端连接 | Create client connection
          let mut stream = TcpStream::connect(local_addr)?;
          println!("客户端连接建立 | Client connection established");
          
          // 4. 设置连接选项 | Set connection options
          stream.set_read_timeout(Some(Duration::from_secs(10)))?;
          stream.set_write_timeout(Some(Duration::from_secs(10)))?;
          stream.set_nodelay(true)?; // 禁用Nagle算法 | Disable Nagle algorithm
          
          println!("连接选项设置完成 | Connection options configured");
          println!("  读取超时: 10秒 | Read timeout: 10 seconds");
          println!("  写入超时: 10秒 | Write timeout: 10 seconds");
          println!("  禁用Nagle: 是 | Nagle disabled: Yes");
          
          // 5. 获取连接信息 | Get connection information
          let local_addr = stream.local_addr()?;
          let peer_addr = stream.peer_addr()?;
          println!("连接信息 | Connection info:");
          println!("  本地地址: {} | Local address: {}", local_addr, local_addr);
          println!("  对端地址: {} | Peer address: {}", peer_addr, peer_addr);
          
          Ok(())
      }
      
      /// 错误处理演示 | Error handling demonstration
      pub fn demonstrate_error_handling() {
          println!("\n=== 网络错误处理演示 | Network Error Handling Demonstration ===");
          
          // 1. 连接超时错误 | Connection timeout error
          match TcpStream::connect_timeout(
              &"192.0.2.1:80".parse().unwrap(), // 不存在的测试地址 | Non-existent test address
              Duration::from_millis(100)
          ) {
              Ok(_) => println!("连接成功（不太可能）| Connection successful (unlikely)"),
              Err(e) => println!("连接超时错误: {} | Connection timeout error: {}", e, e),
          }
          
          // 2. 地址绑定错误 | Address binding error
          match TcpListener::bind("256.256.256.256:8080") {
              Ok(_) => println!("绑定成功（不可能）| Binding successful (impossible)"),
              Err(e) => println!("地址无效错误: {} | Invalid address error: {}", e, e),
          }
          
          // 3. 端口占用错误处理 | Port occupation error handling
          if let Ok(listener1) = TcpListener::bind("127.0.0.1:8081") {
              println!("第一个监听器绑定成功 | First listener bound successfully");
              
              match TcpListener::bind("127.0.0.1:8081") {
                  Ok(_) => println!("第二个监听器绑定成功（不应该发生）| Second listener bound (shouldn't happen)"),
                  Err(e) => println!("端口已占用错误: {} | Port already in use error: {}", e, e),
              }
          }
      }
  }
  
  /// 网络实用函数 | Network utility functions
  impl NetworkUtils {
      /// 获取本机IP地址 | Get local IP address
      pub fn get_local_ip() -> Result<IpAddr> {
          // 通过连接到远程地址获取本机IP | Get local IP by connecting to remote address
          let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
          socket.connect("8.8.8.8:80")?; // Google DNS
          Ok(socket.local_addr()?.ip())
      }
      
      /// 检查端口是否可用 | Check if port is available
      pub fn is_port_available(port: u16) -> bool {
          TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
      }
      
      /// 查找可用端口 | Find available port
      pub fn find_available_port(start_port: u16, end_port: u16) -> Option<u16> {
          for port in start_port..=end_port {
              if Self::is_port_available(port) {
                  return Some(port);
              }
          }
          None
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码中，set_nodelay(true)的作用是什么？| What does set_nodelay(true) do in this code?
    **答案 | Answer:** 禁用Nagle算法，立即发送小数据包而不等待缓冲区填满 | Disables Nagle algorithm, immediately sends small packets without waiting for buffer to fill
  - 为什么使用"127.0.0.1:0"绑定监听器？| Why bind listener to "127.0.0.1:0"?
    **答案 | Answer:** 端口号0表示让操作系统自动分配可用端口 | Port number 0 means let OS automatically assign available port
  
  **常见误区检查 | Common Misconception Checks:**
  - TcpStream的读写操作是否总是成功传输所有数据？| Do TcpStream read/write operations always successfully transfer all data?
    **答案 | Answer:** 否 | No - 可能出现部分读写，需要循环处理直到传输完成 | Partial reads/writes may occur, need to loop until transfer is complete
  - 网络连接是否需要手动关闭？| Do network connections need to be manually closed?
    **答案 | Answer:** 否 | No - Rust的RAII机制会在变量离开作用域时自动关闭连接 | Rust's RAII mechanism automatically closes connections when variables go out of scope

### 2. TCP客户端编程实战 | TCP Client Programming Practice (45分钟 | 45 minutes)

- **TCP客户端核心技术 | TCP Client Core Technologies**
  
  **概念定义 | Concept Definition:**
  TCP客户端负责主动建立与服务器的连接，发送请求数据并接收响应。在Rust中，TcpStream提供了完整的客户端功能，包括连接建立、数据传输和连接管理。| TCP client actively establishes connections with servers, sends request data and receives responses. In Rust, TcpStream provides complete client functionality including connection establishment, data transfer, and connection management.
  
  **核心操作流程 | Core Operation Flow:**
  - 连接建立：使用connect()方法连接到服务器 | Connection establishment: use connect() method to connect to server
  - 数据发送：通过Write trait发送数据到服务器 | Data sending: send data to server through Write trait
  - 数据接收：通过Read trait从服务器读取响应 | Data receiving: read responses from server through Read trait
  - 连接关闭：自动或手动释放网络资源 | Connection closing: automatically or manually release network resources
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. TcpStream::connect()是阻塞操作还是非阻塞操作？| Is TcpStream::connect() a blocking or non-blocking operation?
     **答案 | Answer:** 阻塞 | Blocking - 默认情况下会一直等待直到连接建立或失败 | By default waits until connection is established or fails
  2. 如何设置连接超时？| How to set connection timeout?
     **答案 | Answer:** 使用connect_timeout()方法 | Use connect_timeout() method - 可以指定最大等待时间 | Can specify maximum wait time
  3. TCP客户端可以同时连接多个服务器吗？| Can a TCP client connect to multiple servers simultaneously?
     **答案 | Answer:** 是 | Yes - 可以创建多个TcpStream实例分别连接不同服务器 | Can create multiple TcpStream instances connecting to different servers
  4. 写入数据后是否需要调用flush()？| Do you need to call flush() after writing data?
     **答案 | Answer:** 建议调用 | Recommended - 确保数据立即发送而不是缓存在本地 | Ensures data is sent immediately rather than buffered locally
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // TCP客户端完整实现 | Complete TCP client implementation
  use std::net::{TcpStream, SocketAddr};
  use std::io::{Read, Write, BufRead, BufReader, Result, ErrorKind};
  use std::time::{Duration, Instant};
  use std::thread;
  
  /// TCP客户端构建器 | TCP client builder
  #[derive(Debug)]
  pub struct TcpClientBuilder {
      connect_timeout: Option<Duration>,
      read_timeout: Option<Duration>,
      write_timeout: Option<Duration>,
      nodelay: bool,
      buffer_size: usize,
  }
  
  impl Default for TcpClientBuilder {
      fn default() -> Self {
          Self {
              connect_timeout: Some(Duration::from_secs(10)),
              read_timeout: Some(Duration::from_secs(30)),
              write_timeout: Some(Duration::from_secs(30)),
              nodelay: true,
              buffer_size: 4096,
          }
      }
  }
  
  impl TcpClientBuilder {
      /// 创建新的客户端构建器 | Create new client builder
      pub fn new() -> Self {
          Self::default()
      }
      
      /// 设置连接超时 | Set connection timeout
      pub fn connect_timeout(mut self, timeout: Duration) -> Self {
          self.connect_timeout = Some(timeout);
          self
      }
      
      /// 设置读取超时 | Set read timeout
      pub fn read_timeout(mut self, timeout: Duration) -> Self {
          self.read_timeout = Some(timeout);
          self
      }
      
      /// 设置写入超时 | Set write timeout
      pub fn write_timeout(mut self, timeout: Duration) -> Self {
          self.write_timeout = Some(timeout);
          self
      }
      
      /// 设置Nagle算法 | Set Nagle algorithm
      pub fn nodelay(mut self, nodelay: bool) -> Self {
          self.nodelay = nodelay;
          self
      }
      
      /// 构建并连接客户端 | Build and connect client
      pub fn connect<A: std::net::ToSocketAddrs>(self, addr: A) -> Result<TcpClient> {
          // 解析地址 | Parse address
          let addr = addr.to_socket_addrs()?.next()
              .ok_or_else(|| std::io::Error::new(ErrorKind::InvalidInput, "无效地址 | Invalid address"))?;
          
          // 建立连接 | Establish connection
          let stream = if let Some(timeout) = self.connect_timeout {
              TcpStream::connect_timeout(&addr, timeout)?
          } else {
              TcpStream::connect(addr)?
          };
          
          // 设置连接选项 | Set connection options
          if let Some(timeout) = self.read_timeout {
              stream.set_read_timeout(Some(timeout))?;
          }
          
          if let Some(timeout) = self.write_timeout {
              stream.set_write_timeout(Some(timeout))?;
          }
          
          stream.set_nodelay(self.nodelay)?;
          
          println!("成功连接到服务器: {} | Successfully connected to server: {}", addr, addr);
          
          Ok(TcpClient {
              stream,
              buffer_size: self.buffer_size,
              local_addr: stream.local_addr()?,
              peer_addr: stream.peer_addr()?,
          })
      }
  }
  
  /// TCP客户端 | TCP Client
  pub struct TcpClient {
      stream: TcpStream,
      buffer_size: usize,
      local_addr: SocketAddr,
      peer_addr: SocketAddr,
  }
  
  impl TcpClient {
      /// 发送文本消息 | Send text message
      pub fn send_text(&mut self, message: &str) -> Result<usize> {
          println!("发送消息: {} | Sending message: {}", message, message);
          
          let bytes = message.as_bytes();
          let mut total_sent = 0;
          
          // 确保所有数据都被发送 | Ensure all data is sent
          while total_sent < bytes.len() {
              match self.stream.write(&bytes[total_sent..]) {
                  Ok(sent) => {
                      total_sent += sent;
                      println!("发送了 {} 字节 | Sent {} bytes", sent, sent);
                  }
                  Err(e) if e.kind() == ErrorKind::WouldBlock => {
                      // 非阻塞模式下的重试 | Retry in non-blocking mode
                      thread::sleep(Duration::from_millis(10));
                      continue;
                  }
                  Err(e) => return Err(e),
              }
          }
          
          // 确保数据立即发送 | Ensure data is sent immediately
          self.stream.flush()?;
          println!("消息发送完成，总计 {} 字节 | Message sent complete, total {} bytes", total_sent, total_sent);
          
          Ok(total_sent)
      }
      
      /// 接收文本消息 | Receive text message
      pub fn receive_text(&mut self) -> Result<String> {
          let mut buffer = vec![0u8; self.buffer_size];
          
          match self.stream.read(&mut buffer) {
              Ok(0) => {
                  println!("连接已被服务器关闭 | Connection closed by server");
                  Err(std::io::Error::new(ErrorKind::UnexpectedEof, "连接已关闭 | Connection closed"))
              }
              Ok(bytes_read) => {
                  buffer.truncate(bytes_read);
                  let message = String::from_utf8_lossy(&buffer).to_string();
                  println!("接收到消息 ({} 字节): {} | Received message ({} bytes): {}", bytes_read, message, bytes_read, message);
                  Ok(message)
              }
              Err(e) => {
                  println!("接收消息时出错: {} | Error receiving message: {}", e, e);
                  Err(e)
              }
          }
      }
      
      /// 发送HTTP GET请求 | Send HTTP GET request
      pub fn send_http_get(&mut self, path: &str, host: &str) -> Result<String> {
          let request = format!(
              "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: Rust-TcpClient/1.0\r\n\r\n",
              path, host
          );
          
          println!("发送HTTP请求 | Sending HTTP request:");
          println!("{}", request);
          
          self.send_text(&request)?;
          
          // 读取完整响应 | Read complete response
          let mut response = String::new();
          let mut reader = BufReader::new(&self.stream);
          
          loop {
              let mut line = String::new();
              match reader.read_line(&mut line) {
                  Ok(0) => break, // EOF
                  Ok(_) => response.push_str(&line),
                  Err(e) if e.kind() == ErrorKind::TimedOut => {
                      println!("读取超时，返回已接收的数据 | Read timeout, returning received data");
                      break;
                  }
                  Err(e) => return Err(e),
              }
          }
          
          println!("接收到HTTP响应 ({} 字节) | Received HTTP response ({} bytes)", response.len(), response.len());
          Ok(response)
      }
      
      /// 执行回显测试 | Perform echo test
      pub fn echo_test(&mut self, test_messages: Vec<&str>) -> Result<Vec<(String, String)>> {
          let mut results = Vec::new();
          
          println!("开始回显测试 | Starting echo test");
          
          for (i, message) in test_messages.iter().enumerate() {
              println!("\n测试 {}: | Test {}:", i + 1, i + 1);
              
              // 发送消息 | Send message
              self.send_text(message)?;
              
              // 接收回显 | Receive echo
              let echo = self.receive_text()?;
              
              // 验证回显 | Verify echo
              let is_match = echo.trim() == message.trim();
              println!("回显验证: {} | Echo verification: {}", 
                      if is_match { "通过 | PASS" } else { "失败 | FAIL" },
                      if is_match { "通过 | PASS" } else { "失败 | FAIL" });
              
              results.push((message.to_string(), echo));
          }
          
          println!("回显测试完成 | Echo test completed");
          Ok(results)
      }
      
      /// 获取连接信息 | Get connection info
      pub fn connection_info(&self) -> (SocketAddr, SocketAddr) {
          (self.local_addr, self.peer_addr)
      }
      
      /// 性能测试 | Performance test
      pub fn performance_test(&mut self, data_size: usize, iterations: usize) -> Result<Duration> {
          let test_data = "A".repeat(data_size);
          let start_time = Instant::now();
          
          println!("开始性能测试: {} 次迭代，每次 {} 字节 | Starting performance test: {} iterations, {} bytes each", 
                  iterations, data_size, iterations, data_size);
          
          for i in 0..iterations {
              if i % 100 == 0 {
                  println!("进度: {}/{} | Progress: {}/{}", i, iterations, i, iterations);
              }
              
              self.send_text(&test_data)?;
              let _response = self.receive_text()?;
          }
          
          let elapsed = start_time.elapsed();
          let total_bytes = data_size * iterations * 2; // 发送和接收 | Send and receive
          let throughput = (total_bytes as f64) / elapsed.as_secs_f64() / 1024.0 / 1024.0; // MB/s
          
          println!("性能测试完成 | Performance test completed:");
          println!("  总时间: {:?} | Total time: {:?}", elapsed, elapsed);
          println!("  总数据量: {} 字节 | Total data: {} bytes", total_bytes, total_bytes);
          println!("  吞吐量: {:.2} MB/s | Throughput: {:.2} MB/s", throughput, throughput);
          
          Ok(elapsed)
      }
  }
  
  impl Drop for TcpClient {
      fn drop(&mut self) {
          println!("TCP客户端连接关闭 | TCP client connection closed");
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么在send_text中使用循环发送数据？| Why use a loop to send data in send_text?
    **答案 | Answer:** 防止部分写入，确保所有数据都被发送完毕 | Prevents partial writes, ensures all data is sent completely
  - 什么情况下会收到0字节的读取结果？| When would you receive a 0-byte read result?
    **答案 | Answer:** 对端关闭连接时 | When peer closes the connection - 表示EOF（文件结束） | Indicates EOF (End of File)

### 3. TCP服务器编程核心 | TCP Server Programming Core (1小时 | 1 hour)

- **TCP服务器架构设计 | TCP Server Architecture Design**
  
  **概念定义 | Concept Definition:**
  TCP服务器通过TcpListener监听指定端口，接受客户端连接请求，为每个连接创建处理逻辑。服务器需要处理并发连接、错误恢复和资源管理等复杂问题。| TCP server listens on specified port through TcpListener, accepts client connection requests, and creates handling logic for each connection. Server needs to handle concurrent connections, error recovery, and resource management.
  
  **服务器模式选择 | Server Mode Selection:**
  - 单线程阻塞：简单但性能有限 | Single-threaded blocking: simple but limited performance
  - 多线程：为每个连接创建独立线程 | Multi-threaded: create independent thread for each connection
  - 线程池：复用有限线程处理连接 | Thread pool: reuse limited threads to handle connections
  - 异步非阻塞：高并发低资源消耗 | Async non-blocking: high concurrency with low resource consumption
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. TcpListener::bind()失败的常见原因是什么？| What are common reasons for TcpListener::bind() failure?
     **答案 | Answer:** 端口被占用、权限不足、地址无效 | Port in use, insufficient permissions, invalid address
  2. accept()方法返回的是什么？| What does the accept() method return?
     **答案 | Answer:** Result<(TcpStream, SocketAddr)> | Result containing TcpStream and client address
  3. 服务器如何处理多个并发连接？| How does server handle multiple concurrent connections?
     **答案 | Answer:** 通过多线程或异步编程模型 | Through multi-threading or asynchronous programming models
  4. 服务器意外停止时，已建立的连接会怎样？| What happens to established connections when server stops unexpectedly?
     **答案 | Answer:** 连接会被操作系统自动关闭 | Connections are automatically closed by operating system
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // TCP服务器完整实现 | Complete TCP server implementation
  use std::net::{TcpListener, TcpStream, SocketAddr};
  use std::io::{Read, Write, BufRead, BufReader, Result, ErrorKind};
  use std::thread;
  use std::sync::{Arc, Mutex, mpsc};
  use std::time::{Duration, Instant};
  use std::collections::HashMap;
  
  /// 服务器统计信息 | Server statistics
  #[derive(Debug, Clone, Default)]
  pub struct ServerStats {
      pub total_connections: u64,
      pub active_connections: u64,
      pub messages_received: u64,
      pub messages_sent: u64,
      pub bytes_received: u64,
      pub bytes_sent: u64,
      pub errors: u64,
  }
  
  /// 客户端连接信息 | Client connection info
  #[derive(Debug, Clone)]
  pub struct ConnectionInfo {
      pub id: u32,
      pub addr: SocketAddr,
      pub connected_at: Instant,
      pub last_activity: Instant,
  }
  
  /// TCP服务器配置 | TCP server configuration
  #[derive(Debug, Clone)]
  pub struct ServerConfig {
      pub bind_addr: String,
      pub max_connections: usize,
      pub read_timeout: Option<Duration>,
      pub write_timeout: Option<Duration>,
      pub buffer_size: usize,
      pub enable_stats: bool,
  }
  
  impl Default for ServerConfig {
      fn default() -> Self {
          Self {
              bind_addr: "127.0.0.1:8080".to_string(),
              max_connections: 100,
              read_timeout: Some(Duration::from_secs(60)),
              write_timeout: Some(Duration::from_secs(60)),
              buffer_size: 4096,
              enable_stats: true,
          }
      }
  }
  
  /// TCP服务器 | TCP Server
  pub struct TcpServer {
      config: ServerConfig,
      listener: Option<TcpListener>,
      stats: Arc<Mutex<ServerStats>>,
      connections: Arc<Mutex<HashMap<u32, ConnectionInfo>>>,
      next_connection_id: Arc<Mutex<u32>>,
      shutdown_signal: Option<mpsc::Sender<()>>,
  }
  
  impl TcpServer {
      /// 创建新服务器 | Create new server
      pub fn new(config: ServerConfig) -> Self {
          Self {
              config,
              listener: None,
              stats: Arc::new(Mutex::new(ServerStats::default())),
              connections: Arc::new(Mutex::new(HashMap::new())),
              next_connection_id: Arc::new(Mutex::new(1)),
              shutdown_signal: None,
          }
      }
      
      /// 启动服务器 | Start server
      pub fn start(&mut self) -> Result<()> {
          println!("启动TCP服务器... | Starting TCP server...");
          
          // 绑定监听器 | Bind listener
          let listener = TcpListener::bind(&self.config.bind_addr)?;
          let local_addr = listener.local_addr()?;
          
          println!("服务器监听地址: {} | Server listening on: {}", local_addr, local_addr);
          println!("最大连接数: {} | Max connections: {}", self.config.max_connections, self.config.max_connections);
          
          self.listener = Some(listener);
          
          // 创建关闭信号通道 | Create shutdown signal channel
          let (shutdown_tx, shutdown_rx) = mpsc::channel();
          self.shutdown_signal = Some(shutdown_tx);
          
          // 启动统计监控线程 | Start statistics monitoring thread
          if self.config.enable_stats {
              self.start_stats_monitor();
          }
          
          // 主循环：接受连接 | Main loop: accept connections
          loop {
              match self.accept_connection() {
                  Ok(_) => {}
                  Err(e) => {
                      println!("接受连接时出错: {} | Error accepting connection: {}", e, e);
                      self.increment_error_count();
                      
                      // 检查是否收到关闭信号 | Check for shutdown signal
                      if shutdown_rx.try_recv().is_ok() {
                          println!("收到关闭信号，停止服务器 | Received shutdown signal, stopping server");
                          break;
                      }
                  }
              }
          }
          
          Ok(())
      }
      
      /// 接受新连接 | Accept new connection
      fn accept_connection(&self) -> Result<()> {
          let listener = self.listener.as_ref()
              .ok_or_else(|| std::io::Error::new(ErrorKind::Other, "服务器未启动 | Server not started"))?;
          
          let (stream, addr) = listener.accept()?;
          
          // 检查连接数限制 | Check connection limit
          let active_connections = {
              let connections = self.connections.lock().unwrap();
              connections.len()
          };
          
          if active_connections >= self.config.max_connections {
              println!("拒绝连接 {} - 已达到最大连接数 | Rejecting connection {} - max connections reached", addr, addr);
              return Ok(());
          }
          
          // 分配连接ID | Assign connection ID
          let connection_id = {
              let mut next_id = self.next_connection_id.lock().unwrap();
              let id = *next_id;
              *next_id += 1;
              id
          };
          
          // 记录连接信息 | Record connection info
          let connection_info = ConnectionInfo {
              id: connection_id,
              addr,
              connected_at: Instant::now(),
              last_activity: Instant::now(),
          };
          
          {
              let mut connections = self.connections.lock().unwrap();
              connections.insert(connection_id, connection_info);
          }
          
          // 更新统计 | Update statistics
          self.increment_connection_count();
          
          println!("新连接 #{}: {} | New connection #{}: {}", connection_id, addr, connection_id, addr);
          
          // 为连接创建处理线程 | Create handling thread for connection
          self.spawn_connection_handler(stream, connection_id, addr);
          
          Ok(())
      }
      
      /// 为连接创建处理线程 | Spawn connection handler thread
      fn spawn_connection_handler(&self, mut stream: TcpStream, connection_id: u32, addr: SocketAddr) {
          let stats = Arc::clone(&self.stats);
          let connections = Arc::clone(&self.connections);
          let config = self.config.clone();
          
          thread::spawn(move || {
              // 设置超时 | Set timeouts
              if let Some(timeout) = config.read_timeout {
                  let _ = stream.set_read_timeout(Some(timeout));
              }
              if let Some(timeout) = config.write_timeout {
                  let _ = stream.set_write_timeout(Some(timeout));
              }
              
              // 处理连接 | Handle connection
              if let Err(e) = Self::handle_connection(&mut stream, connection_id, &config, &stats, &connections) {
                  println!("连接 #{} 处理错误: {} | Connection #{} handling error: {}", connection_id, e, connection_id, e);
              }
              
              // 清理连接信息 | Cleanup connection info
              {
                  let mut conns = connections.lock().unwrap();
                  conns.remove(&connection_id);
              }
              
              println!("连接 #{} ({}) 已关闭 | Connection #{} ({}) closed", connection_id, addr, connection_id, addr);
          });
      }
      
      /// 处理单个连接 | Handle single connection
      fn handle_connection(
          stream: &mut TcpStream,
          connection_id: u32,
          config: &ServerConfig,
          stats: &Arc<Mutex<ServerStats>>,
          connections: &Arc<Mutex<HashMap<u32, ConnectionInfo>>>,
      ) -> Result<()> {
          let mut buffer = vec![0u8; config.buffer_size];
          let mut reader = BufReader::new(stream);
          
          loop {
              // 读取客户端消息 | Read client message
              let mut line = String::new();
              match reader.read_line(&mut line) {
                  Ok(0) => {
                      println!("客户端 #{} 关闭连接 | Client #{} closed connection", connection_id, connection_id);
                      break;
                  }
                  Ok(bytes_read) => {
                      // 更新活动时间 | Update activity time
                      {
                          let mut conns = connections.lock().unwrap();
                          if let Some(info) = conns.get_mut(&connection_id) {
                              info.last_activity = Instant::now();
                          }
                      }
                      
                      // 更新统计 | Update statistics
                      {
                          let mut stats = stats.lock().unwrap();
                          stats.messages_received += 1;
                          stats.bytes_received += bytes_read as u64;
                      }
                      
                      let message = line.trim();
                      println!("连接 #{} 收到: {} | Connection #{} received: {}", connection_id, message, connection_id, message);
                      
                      // 处理特殊命令 | Handle special commands
                      let response = match message {
                          "PING" => "PONG".to_string(),
                          "TIME" => format!("{:?}", std::time::SystemTime::now()),
                          "INFO" => format!("Connection #{}", connection_id),
                          "QUIT" => {
                              println!("客户端 #{} 请求断开连接 | Client #{} requested disconnect", connection_id, connection_id);
                              break;
                          }
                          _ => {
                              // 回显消息 | Echo message
                              format!("Echo: {}", message)
                          }
                      };
                      
                      // 发送响应 | Send response
                      let response_line = format!("{}\n", response);
                      stream.write_all(response_line.as_bytes())?;
                      stream.flush()?;
                      
                      // 更新统计 | Update statistics
                      {
                          let mut stats = stats.lock().unwrap();
                          stats.messages_sent += 1;
                          stats.bytes_sent += response_line.len() as u64;
                      }
                      
                      println!("连接 #{} 发送: {} | Connection #{} sent: {}", connection_id, response, connection_id, response);
                  }
                  Err(e) if e.kind() == ErrorKind::TimedOut => {
                      println!("连接 #{} 读取超时 | Connection #{} read timeout", connection_id, connection_id);
                      break;
                  }
                  Err(e) => {
                      println!("连接 #{} 读取错误: {} | Connection #{} read error: {}", connection_id, e, connection_id, e);
                      return Err(e);
                  }
              }
          }
          
          Ok(())
      }
      
      /// 启动统计监控 | Start statistics monitoring
      fn start_stats_monitor(&self) {
          let stats = Arc::clone(&self.stats);
          let connections = Arc::clone(&self.connections);
          
          thread::spawn(move || {
              loop {
                  thread::sleep(Duration::from_secs(10));
                  
                  let stats = stats.lock().unwrap();
                  let connections = connections.lock().unwrap();
                  
                  println!("\n=== 服务器统计信息 | Server Statistics ===");
                  println!("总连接数: {} | Total connections: {}", stats.total_connections, stats.total_connections);
                  println!("当前活跃连接: {} | Active connections: {}", connections.len(), connections.len());
                  println!("接收消息数: {} | Messages received: {}", stats.messages_received, stats.messages_received);
                  println!("发送消息数: {} | Messages sent: {}", stats.messages_sent, stats.messages_sent);
                  println!("接收字节数: {} | Bytes received: {}", stats.bytes_received, stats.bytes_received);
                  println!("发送字节数: {} | Bytes sent: {}", stats.bytes_sent, stats.bytes_sent);
                  println!("错误次数: {} | Errors: {}", stats.errors, stats.errors);
                  println!("=======================================\n");
              }
          });
      }
      
      /// 增加连接计数 | Increment connection count
      fn increment_connection_count(&self) {
          let mut stats = self.stats.lock().unwrap();
          stats.total_connections += 1;
          stats.active_connections += 1;
      }
      
      /// 增加错误计数 | Increment error count
      fn increment_error_count(&self) {
          let mut stats = self.stats.lock().unwrap();
          stats.errors += 1;
      }
      
      /// 获取服务器统计 | Get server statistics
      pub fn get_stats(&self) -> ServerStats {
          self.stats.lock().unwrap().clone()
      }
      
      /// 获取活跃连接列表 | Get active connections list
      pub fn get_connections(&self) -> Vec<ConnectionInfo> {
          self.connections.lock().unwrap().values().cloned().collect()
      }
  }
  ```

### 4. HTTP服务器从零构建 | Building HTTP Server from Scratch (1小时 | 1 hour)

- **HTTP协议基础实现 | HTTP Protocol Basic Implementation**
  
  **概念定义 | Concept Definition:**
  HTTP服务器在TCP服务器基础上实现HTTP协议解析，包括请求行解析、头部处理、响应生成等功能。理解HTTP协议的文本格式对于构建Web服务器至关重要。| HTTP server implements HTTP protocol parsing on top of TCP server foundation, including request line parsing, header handling, response generation. Understanding HTTP protocol's text format is crucial for building web servers.
  
  **HTTP消息结构 | HTTP Message Structure:**
  - 请求行：方法、路径、协议版本 | Request line: method, path, protocol version
  - 请求头：键值对形式的元数据 | Request headers: metadata in key-value format
  - 空行：分隔头部和主体 | Empty line: separates headers and body
  - 请求主体：POST数据等内容 | Request body: POST data and other content
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. HTTP请求的第一行包含哪些信息？| What information does the first line of HTTP request contain?
     **答案 | Answer:** 方法、路径、协议版本 | Method, path, protocol version - 例如"GET /index.html HTTP/1.1" | e.g., "GET /index.html HTTP/1.1"
  2. HTTP响应的状态码200表示什么？| What does HTTP response status code 200 mean?
     **答案 | Answer:** 请求成功 | Request successful - OK状态，请求已被成功处理 | OK status, request has been successfully processed
  3. HTTP头部和主体之间用什么分隔？| What separates HTTP headers and body?
     **答案 | Answer:** 空行（\\r\\n\\r\\n）| Empty line (\\r\\n\\r\\n) - 两个连续的换行符 | Two consecutive line breaks
  4. Content-Length头部的作用是什么？| What is the purpose of Content-Length header?
     **答案 | Answer:** 指定主体内容的字节长度 | Specifies byte length of body content - 帮助接收方知道何时停止读取 | Helps receiver know when to stop reading
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 简单HTTP服务器实现 | Simple HTTP server implementation
  use std::net::{TcpListener, TcpStream};
  use std::io::{Read, Write, BufRead, BufReader, Result};
  use std::collections::HashMap;
  use std::thread;
  use std::fs;
  use std::path::Path;
  
  /// HTTP请求方法 | HTTP request methods
  #[derive(Debug, Clone, PartialEq)]
  pub enum HttpMethod {
      GET,
      POST,
      PUT,
      DELETE,
      HEAD,
      OPTIONS,
      PATCH,
  }
  
  impl std::str::FromStr for HttpMethod {
      type Err = String;
      
      fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
          match s.to_uppercase().as_str() {
              "GET" => Ok(HttpMethod::GET),
              "POST" => Ok(HttpMethod::POST),
              "PUT" => Ok(HttpMethod::PUT),
              "DELETE" => Ok(HttpMethod::DELETE),
              "HEAD" => Ok(HttpMethod::HEAD),
              "OPTIONS" => Ok(HttpMethod::OPTIONS),
              "PATCH" => Ok(HttpMethod::PATCH),
              _ => Err(format!("不支持的HTTP方法: {} | Unsupported HTTP method: {}", s, s)),
          }
      }
  }
  
  /// HTTP请求 | HTTP Request
  #[derive(Debug)]
  pub struct HttpRequest {
      pub method: HttpMethod,
      pub path: String,
      pub version: String,
      pub headers: HashMap<String, String>,
      pub body: Vec<u8>,
  }
  
  impl HttpRequest {
      /// 从TCP流解析HTTP请求 | Parse HTTP request from TCP stream
      pub fn parse(stream: &mut TcpStream) -> Result<Self> {
          let mut reader = BufReader::new(stream);
          
          // 读取请求行 | Read request line
          let mut request_line = String::new();
          reader.read_line(&mut request_line)?;
          
          println!("请求行: {} | Request line: {}", request_line.trim(), request_line.trim());
          
          // 解析请求行 | Parse request line
          let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
          if parts.len() != 3 {
              return Err(std::io::Error::new(
                  std::io::ErrorKind::InvalidData,
                  "无效的HTTP请求行 | Invalid HTTP request line"
              ));
          }
          
          let method: HttpMethod = parts[0].parse()
              .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
          let path = parts[1].to_string();
          let version = parts[2].to_string();
          
          // 读取头部 | Read headers
          let mut headers = HashMap::new();
          loop {
              let mut line = String::new();
              reader.read_line(&mut line)?;
              
              let line = line.trim();
              if line.is_empty() {
                  break; // 空行表示头部结束 | Empty line indicates end of headers
              }
              
              if let Some(pos) = line.find(':') {
                  let key = line[..pos].trim().to_lowercase();
                  let value = line[pos + 1..].trim().to_string();
                  headers.insert(key, value);
                  println!("请求头: {} = {} | Request header: {} = {}", key, value, key, value);
              }
          }
          
          // 读取主体（如果有Content-Length）| Read body (if Content-Length exists)
          let mut body = Vec::new();
          if let Some(content_length_str) = headers.get("content-length") {
              if let Ok(content_length) = content_length_str.parse::<usize>() {
                  if content_length > 0 {
                      body.resize(content_length, 0);
                      reader.read_exact(&mut body)?;
                      println!("读取主体: {} 字节 | Read body: {} bytes", content_length, content_length);
                  }
              }
          }
          
          Ok(HttpRequest {
              method,
              path,
              version,
              headers,
              body,
          })
      }
      
      /// 获取查询参数 | Get query parameters
      pub fn get_query_params(&self) -> HashMap<String, String> {
          let mut params = HashMap::new();
          
          if let Some(query_start) = self.path.find('?') {
              let query_string = &self.path[query_start + 1..];
              for pair in query_string.split('&') {
                  if let Some(eq_pos) = pair.find('=') {
                      let key = &pair[..eq_pos];
                      let value = &pair[eq_pos + 1..];
                      params.insert(key.to_string(), value.to_string());
                  }
              }
          }
          
          params
      }
      
      /// 获取清理后的路径（去除查询参数）| Get cleaned path (remove query parameters)
      pub fn get_clean_path(&self) -> &str {
          if let Some(query_start) = self.path.find('?') {
              &self.path[..query_start]
          } else {
              &self.path
          }
      }
  }
  
  /// HTTP响应 | HTTP Response
  #[derive(Debug)]
  pub struct HttpResponse {
      pub status_code: u16,
      pub status_text: String,
      pub headers: HashMap<String, String>,
      pub body: Vec<u8>,
  }
  
  impl HttpResponse {
      /// 创建新的HTTP响应 | Create new HTTP response
      pub fn new(status_code: u16) -> Self {
          let status_text = match status_code {
              200 => "OK",
              201 => "Created",
              204 => "No Content",
              400 => "Bad Request",
              401 => "Unauthorized",
              403 => "Forbidden",
              404 => "Not Found",
              405 => "Method Not Allowed",
              500 => "Internal Server Error",
              _ => "Unknown",
          }.to_string();
          
          let mut headers = HashMap::new();
          headers.insert("server".to_string(), "Rust-HTTP-Server/1.0".to_string());
          headers.insert("connection".to_string(), "close".to_string());
          
          Self {
              status_code,
              status_text,
              headers,
              body: Vec::new(),
          }
      }
      
      /// 设置响应头 | Set response header
      pub fn header(mut self, key: &str, value: &str) -> Self {
          self.headers.insert(key.to_lowercase(), value.to_string());
          self
      }
      
      /// 设置JSON响应体 | Set JSON response body
      pub fn json(mut self, json: &str) -> Self {
          self.body = json.as_bytes().to_vec();
          self.headers.insert("content-type".to_string(), "application/json".to_string());
          self.headers.insert("content-length".to_string(), self.body.len().to_string());
          self
      }
      
      /// 设置HTML响应体 | Set HTML response body
      pub fn html(mut self, html: &str) -> Self {
          self.body = html.as_bytes().to_vec();
          self.headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
          self.headers.insert("content-length".to_string(), self.body.len().to_string());
          self
      }
      
      /// 设置文本响应体 | Set text response body
      pub fn text(mut self, text: &str) -> Self {
          self.body = text.as_bytes().to_vec();
          self.headers.insert("content-type".to_string(), "text/plain; charset=utf-8".to_string());
          self.headers.insert("content-length".to_string(), self.body.len().to_string());
          self
      }
      
      /// 发送响应到TCP流 | Send response to TCP stream
      pub fn send(self, stream: &mut TcpStream) -> Result<()> {
          // 构建响应行 | Build response line
          let response_line = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
          
          // 构建头部 | Build headers
          let mut header_lines = String::new();
          for (key, value) in &self.headers {
              header_lines.push_str(&format!("{}: {}\r\n", key, value));
          }
          
          // 发送响应 | Send response
          stream.write_all(response_line.as_bytes())?;
          stream.write_all(header_lines.as_bytes())?;
          stream.write_all(b"\r\n")?; // 空行分隔头部和主体 | Empty line separates headers and body
          stream.write_all(&self.body)?;
          stream.flush()?;
          
          println!("发送HTTP响应: {} {} ({} 字节) | Sent HTTP response: {} {} ({} bytes)", 
                  self.status_code, self.status_text, self.body.len(), 
                  self.status_code, self.status_text, self.body.len());
          
          Ok(())
      }
  }
  
  /// HTTP服务器 | HTTP Server
  pub struct HttpServer {
      pub_root: String,
  }
  
  impl HttpServer {
      /// 创建新的HTTP服务器 | Create new HTTP server
      pub fn new(pub_root: &str) -> Self {
          Self {
              pub_root: pub_root.to_string(),
          }
      }
      
      /// 启动服务器 | Start server
      pub fn start(&self, bind_addr: &str) -> Result<()> {
          let listener = TcpListener::bind(bind_addr)?;
          let local_addr = listener.local_addr()?;
          
          println!("HTTP服务器启动，监听地址: {} | HTTP server started, listening on: {}", local_addr, local_addr);
          println!("静态文件根目录: {} | Static file root: {}", self.pub_root, self.pub_root);
          
          for stream in listener.incoming() {
              match stream {
                  Ok(stream) => {
                      let pub_root = self.pub_root.clone();
                      thread::spawn(move || {
                          if let Err(e) = Self::handle_request(stream, &pub_root) {
                              println!("处理请求时出错: {} | Error handling request: {}", e, e);
                          }
                      });
                  }
                  Err(e) => {
                      println!("接受连接时出错: {} | Error accepting connection: {}", e, e);
                  }
              }
          }
          
          Ok(())
      }
      
      /// 处理HTTP请求 | Handle HTTP request
      fn handle_request(mut stream: TcpStream, pub_root: &str) -> Result<()> {
          // 解析请求 | Parse request
          let request = HttpRequest::parse(&mut stream)?;
          
          println!("处理请求: {} {} | Handling request: {} {}", 
                  format!("{:?}", request.method), request.path, 
                  format!("{:?}", request.method), request.path);
          
          // 路由处理 | Route handling
          let response = match (&request.method, request.get_clean_path()) {
              (HttpMethod::GET, "/") => {
                  Self::handle_index(&request)
              }
              (HttpMethod::GET, "/api/status") => {
                  Self::handle_api_status(&request)
              }
              (HttpMethod::GET, "/api/echo") => {
                  Self::handle_api_echo(&request)
              }
              (HttpMethod::POST, "/api/echo") => {
                  Self::handle_api_post_echo(&request)
              }
              (HttpMethod::GET, path) => {
                  Self::handle_static_file(path, pub_root)
              }
              (method, path) => {
                  println!("不支持的方法或路径: {:?} {} | Unsupported method or path: {:?} {}", method, path, method, path);
                  HttpResponse::new(404).html("<h1>404 Not Found</h1><p>请求的资源不存在 | The requested resource does not exist</p>")
              }
          };
          
          // 发送响应 | Send response
          response.send(&mut stream)?;
          
          Ok(())
      }
      
      /// 处理首页请求 | Handle index request
      fn handle_index(_request: &HttpRequest) -> HttpResponse {
          let html = r#"
  <!DOCTYPE html>
  <html>
  <head>
      <meta charset="UTF-8">
      <title>Rust HTTP Server</title>
      <style>
          body { font-family: Arial, sans-serif; margin: 40px; }
          .container { max-width: 800px; margin: 0 auto; }
          .api-section { background: #f5f5f5; padding: 20px; margin: 20px 0; border-radius: 5px; }
      </style>
  </head>
  <body>
      <div class="container">
          <h1>🦀 Rust HTTP Server | Rust HTTP 服务器</h1>
          <p>Welcome to the Rust HTTP Server! | 欢迎使用 Rust HTTP 服务器！</p>
          
          <div class="api-section">
              <h2>Available APIs | 可用的 API</h2>
              <ul>
                  <li><a href="/api/status">GET /api/status</a> - Server status | 服务器状态</li>
                  <li><a href="/api/echo?message=hello">GET /api/echo</a> - Echo service | 回显服务</li>
                  <li>POST /api/echo - Post echo service | POST 回显服务</li>
              </ul>
          </div>
          
          <div class="api-section">
              <h2>Test Commands | 测试命令</h2>
              <pre>
  # GET request test | GET 请求测试
  curl http://localhost:8080/api/status
  
  # Echo test | 回显测试
  curl "http://localhost:8080/api/echo?message=Hello%20World"
  
  # POST test | POST 测试
  curl -X POST -d "Hello from POST" http://localhost:8080/api/echo
              </pre>
          </div>
      </div>
  </body>
  </html>
          "#;
          
          HttpResponse::new(200).html(html)
      }
      
      /// 处理状态API | Handle status API
      fn handle_api_status(_request: &HttpRequest) -> HttpResponse {
          let status = format!(
              r#"{{
      "status": "ok",
      "message": "Server is running | 服务器正在运行",
      "timestamp": "{}",
      "version": "1.0.0"
  }}"#,
              chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
          );
          
          HttpResponse::new(200).json(&status)
      }
      
      /// 处理GET回显API | Handle GET echo API
      fn handle_api_echo(request: &HttpRequest) -> HttpResponse {
          let params = request.get_query_params();
          let message = params.get("message").unwrap_or(&"Hello World".to_string());
          
          let response = format!(
              r#"{{
      "method": "GET",
      "echo": "{}",
      "timestamp": "{}",
      "query_params": {}
  }}"#,
              message,
              chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
              serde_json::to_string(&params).unwrap_or("{}".to_string())
          );
          
          HttpResponse::new(200).json(&response)
      }
      
      /// 处理POST回显API | Handle POST echo API
      fn handle_api_post_echo(request: &HttpRequest) -> HttpResponse {
          let body_text = String::from_utf8_lossy(&request.body);
          
          let response = format!(
              r#"{{
      "method": "POST",
      "echo": "{}",
      "timestamp": "{}",
      "content_length": {}
  }}"#,
              body_text,
              chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
              request.body.len()
          );
          
          HttpResponse::new(200).json(&response)
      }
      
      /// 处理静态文件请求 | Handle static file request
      fn handle_static_file(path: &str, pub_root: &str) -> HttpResponse {
          let file_path = format!("{}{}", pub_root, path);
          let file_path = Path::new(&file_path);
          
          // 安全检查：防止路径遍历攻击 | Security check: prevent path traversal attacks
          if !file_path.starts_with(pub_root) {
              return HttpResponse::new(403).html("<h1>403 Forbidden</h1><p>路径不被允许 | Path not allowed</p>");
          }
          
          match fs::read(&file_path) {
              Ok(content) => {
                  let content_type = Self::get_content_type(&file_path);
                  HttpResponse::new(200)
                      .header("content-type", &content_type)
                      .header("content-length", &content.len().to_string())
                      .body = content;
                      
                  HttpResponse {
                      status_code: 200,
                      status_text: "OK".to_string(),
                      headers: {
                          let mut headers = HashMap::new();
                          headers.insert("content-type".to_string(), content_type);
                          headers.insert("content-length".to_string(), content.len().to_string());
                          headers.insert("server".to_string(), "Rust-HTTP-Server/1.0".to_string());
                          headers
                      },
                      body: content,
                  }
              }
              Err(_) => {
                  HttpResponse::new(404).html("<h1>404 Not Found</h1><p>文件不存在 | File not found</p>")
              }
          }
      }
      
      /// 根据文件扩展名获取Content-Type | Get Content-Type based on file extension
      fn get_content_type(path: &Path) -> String {
          match path.extension().and_then(|ext| ext.to_str()) {
              Some("html") | Some("htm") => "text/html; charset=utf-8",
              Some("css") => "text/css",
              Some("js") => "application/javascript",
              Some("json") => "application/json",
              Some("png") => "image/png",
              Some("jpg") | Some("jpeg") => "image/jpeg",
              Some("gif") => "image/gif",
              Some("svg") => "image/svg+xml",
              Some("txt") => "text/plain; charset=utf-8",
              _ => "application/octet-stream",
          }.to_string()
      }
  }
  ```

### 5. 错误处理与连接管理 | Error Handling and Connection Management (30分钟 | 30 minutes)

- **网络编程错误处理最佳实践 | Network Programming Error Handling Best Practices**
  
  **概念定义 | Concept Definition:**
  网络编程中的错误处理涉及连接失败、超时、数据传输错误、资源耗尽等多种情况。良好的错误处理策略能够提高应用程序的健壮性和用户体验。| Error handling in network programming involves connection failures, timeouts, data transmission errors, resource exhaustion, and various other situations. Good error handling strategies improve application robustness and user experience.
  
  **错误类型分类 | Error Type Classification:**
  - 连接错误：无法建立连接、连接被拒绝 | Connection errors: unable to establish connection, connection refused
  - 传输错误：数据发送失败、接收中断 | Transmission errors: data send failure, receive interruption
  - 超时错误：操作执行时间过长 | Timeout errors: operations taking too long
  - 资源错误：内存不足、文件描述符耗尽 | Resource errors: insufficient memory, file descriptor exhaustion
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 网络操作返回EWOULDBLOCK错误表示什么？| What does EWOULDBLOCK error mean in network operations?
     **答案 | Answer:** 非阻塞操作暂时无法完成 | Non-blocking operation temporarily cannot complete - 需要稍后重试 | Need to retry later
  2. 连接突然中断时应该如何处理？| How should sudden connection interruption be handled?
     **答案 | Answer:** 清理资源并记录错误 | Clean up resources and log error - 可能需要重新连接或通知用户 | May need to reconnect or notify user
  3. 设置超时的目的是什么？| What is the purpose of setting timeouts?
     **答案 | Answer:** 防止无限等待，提高响应性 | Prevent infinite waiting, improve responsiveness
  4. 如何优雅地关闭网络连接？| How to gracefully close network connections?
     **答案 | Answer:** 发送关闭信号，等待确认，释放资源 | Send close signal, wait for acknowledgment, release resources
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 网络错误处理和连接管理工具 | Network error handling and connection management utilities
  use std::net::{TcpStream, TcpListener};
  use std::io::{Result, Error, ErrorKind};
  use std::time::{Duration, Instant};
  use std::thread;
  use std::sync::{Arc, Mutex, mpsc};
  
  /// 网络错误类型 | Network error types
  #[derive(Debug, Clone)]
  pub enum NetworkError {
      ConnectionFailed(String),
      ConnectionTimeout,
      ReadTimeout,
      WriteTimeout,
      ConnectionClosed,
      InvalidData(String),
      ResourceExhausted,
      Unknown(String),
  }
  
  impl std::fmt::Display for NetworkError {
      fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
          match self {
              NetworkError::ConnectionFailed(addr) => write!(f, "连接失败: {} | Connection failed: {}", addr, addr),
              NetworkError::ConnectionTimeout => write!(f, "连接超时 | Connection timeout"),
              NetworkError::ReadTimeout => write!(f, "读取超时 | Read timeout"),
              NetworkError::WriteTimeout => write!(f, "写入超时 | Write timeout"),
              NetworkError::ConnectionClosed => write!(f, "连接已关闭 | Connection closed"),
              NetworkError::InvalidData(msg) => write!(f, "无效数据: {} | Invalid data: {}", msg, msg),
              NetworkError::ResourceExhausted => write!(f, "资源耗尽 | Resource exhausted"),
              NetworkError::Unknown(msg) => write!(f, "未知错误: {} | Unknown error: {}", msg, msg),
          }
      }
  }
  
  impl std::error::Error for NetworkError {}
  
  /// 连接池管理器 | Connection pool manager
  pub struct ConnectionPool {
      max_connections: usize,
      active_connections: Arc<Mutex<Vec<TcpStream>>>,
      connection_timeout: Duration,
  }
  
  impl ConnectionPool {
      /// 创建连接池 | Create connection pool
      pub fn new(max_connections: usize, connection_timeout: Duration) -> Self {
          Self {
              max_connections,
              active_connections: Arc::new(Mutex::new(Vec::new())),
              connection_timeout,
          }
      }
      
      /// 获取连接 | Get connection
      pub fn get_connection(&self, addr: &str) -> std::result::Result<TcpStream, NetworkError> {
          // 检查连接池大小 | Check pool size
          {
              let connections = self.active_connections.lock().unwrap();
              if connections.len() >= self.max_connections {
                  return Err(NetworkError::ResourceExhausted);
              }
          }
          
          // 尝试建立连接 | Try to establish connection
          match TcpStream::connect_timeout(
              &addr.parse().map_err(|_| NetworkError::InvalidData("无效地址 | Invalid address".to_string()))?,
              self.connection_timeout
          ) {
              Ok(stream) => {
                  println!("连接池：成功连接到 {} | Connection pool: successfully connected to {}", addr, addr);
                  
                  // 将连接添加到池中 | Add connection to pool
                  {
                      let mut connections = self.active_connections.lock().unwrap();
                      connections.push(stream.try_clone().map_err(|e| NetworkError::Unknown(e.to_string()))?);
                  }
                  
                  Ok(stream)
              }
              Err(e) => {
                  match e.kind() {
                      ErrorKind::TimedOut => Err(NetworkError::ConnectionTimeout),
                      ErrorKind::ConnectionRefused => Err(NetworkError::ConnectionFailed(addr.to_string())),
                      _ => Err(NetworkError::Unknown(e.to_string())),
                  }
              }
          }
      }
      
      /// 释放连接 | Release connection
      pub fn release_connection(&self, _stream: TcpStream) {
          // 简化实现：从池中移除连接 | Simplified implementation: remove connection from pool
          let mut connections = self.active_connections.lock().unwrap();
          if !connections.is_empty() {
              connections.remove(0);
              println!("连接池：释放一个连接 | Connection pool: released one connection");
          }
      }
      
      /// 获取连接统计 | Get connection statistics
      pub fn get_stats(&self) -> (usize, usize) {
          let connections = self.active_connections.lock().unwrap();
          (connections.len(), self.max_connections)
      }
  }
  
  /// 重连客户端 | Reconnecting client
  pub struct ReconnectingClient {
      target_addr: String,
      connection: Option<TcpStream>,
      max_retries: u32,
      retry_delay: Duration,
      connection_timeout: Duration,
  }
  
  impl ReconnectingClient {
      /// 创建重连客户端 | Create reconnecting client
      pub fn new(target_addr: String) -> Self {
          Self {
              target_addr,
              connection: None,
              max_retries: 3,
              retry_delay: Duration::from_secs(1),
              connection_timeout: Duration::from_secs(10),
          }
      }
      
      /// 设置重试参数 | Set retry parameters
      pub fn with_retry_config(mut self, max_retries: u32, retry_delay: Duration) -> Self {
          self.max_retries = max_retries;
          self.retry_delay = retry_delay;
          self
      }
      
      /// 确保连接可用 | Ensure connection is available
      pub fn ensure_connected(&mut self) -> std::result::Result<(), NetworkError> {
          // 检查现有连接 | Check existing connection
          if let Some(ref mut stream) = self.connection {
              if self.test_connection(stream).is_ok() {
                  return Ok(());
              } else {
                  println!("现有连接已失效，需要重新连接 | Existing connection failed, need to reconnect");
                  self.connection = None;
              }
          }
          
          // 尝试重新连接 | Try to reconnect
          for attempt in 1..=self.max_retries {
              println!("尝试连接 {}/{} | Connection attempt {}/{}", attempt, self.max_retries, attempt, self.max_retries);
              
              match TcpStream::connect_timeout(
                  &self.target_addr.parse().map_err(|_| NetworkError::InvalidData("无效地址 | Invalid address".to_string()))?,
                  self.connection_timeout
              ) {
                  Ok(stream) => {
                      println!("重连成功！| Reconnection successful!");
                      self.connection = Some(stream);
                      return Ok(());
                  }
                  Err(e) => {
                      let error = match e.kind() {
                          ErrorKind::TimedOut => NetworkError::ConnectionTimeout,
                          ErrorKind::ConnectionRefused => NetworkError::ConnectionFailed(self.target_addr.clone()),
                          _ => NetworkError::Unknown(e.to_string()),
                      };
                      
                      println!("连接失败: {} | Connection failed: {}", error, error);
                      
                      if attempt < self.max_retries {
                          println!("等待 {:?} 后重试 | Waiting {:?} before retry", self.retry_delay, self.retry_delay);
                          thread::sleep(self.retry_delay);
                      } else {
                          return Err(error);
                      }
                  }
              }
          }
          
          Err(NetworkError::ConnectionFailed(self.target_addr.clone()))
      }
      
      /// 测试连接有效性 | Test connection validity
      fn test_connection(&mut self, stream: &mut TcpStream) -> std::result::Result<(), NetworkError> {
          // 尝试发送小数据包测试连接 | Try sending small packet to test connection
          match stream.write(&[0]) {
              Ok(_) => Ok(()),
              Err(e) => match e.kind() {
                  ErrorKind::BrokenPipe | ErrorKind::ConnectionAborted | ErrorKind::ConnectionReset => {
                      Err(NetworkError::ConnectionClosed)
                  }
                  _ => Err(NetworkError::Unknown(e.to_string())),
              }
          }
      }
      
      /// 发送数据（带重连） | Send data (with reconnection)
      pub fn send_data(&mut self, data: &[u8]) -> std::result::Result<usize, NetworkError> {
          self.ensure_connected()?;
          
          if let Some(ref mut stream) = self.connection {
              match stream.write(data) {
                  Ok(bytes_sent) => {
                      stream.flush().map_err(|e| NetworkError::Unknown(e.to_string()))?;
                      println!("发送数据: {} 字节 | Sent data: {} bytes", bytes_sent, bytes_sent);
                      Ok(bytes_sent)
                  }
                  Err(e) => {
                      match e.kind() {
                          ErrorKind::BrokenPipe | ErrorKind::ConnectionAborted | ErrorKind::ConnectionReset => {
                              println!("连接中断，清理连接状态 | Connection interrupted, cleaning connection state");
                              self.connection = None;
                              Err(NetworkError::ConnectionClosed)
                          }
                          ErrorKind::TimedOut => Err(NetworkError::WriteTimeout),
                          _ => Err(NetworkError::Unknown(e.to_string())),
                      }
                  }
              }
          } else {
              Err(NetworkError::ConnectionFailed(self.target_addr.clone()))
          }
      }
      
      /// 接收数据（带超时处理）| Receive data (with timeout handling)
      pub fn receive_data(&mut self, buffer: &mut [u8]) -> std::result::Result<usize, NetworkError> {
          self.ensure_connected()?;
          
          if let Some(ref mut stream) = self.connection {
              match stream.read(buffer) {
                  Ok(0) => {
                      println!("连接被对端关闭 | Connection closed by peer");
                      self.connection = None;
                      Err(NetworkError::ConnectionClosed)
                  }
                  Ok(bytes_read) => {
                      println!("接收数据: {} 字节 | Received data: {} bytes", bytes_read, bytes_read);
                      Ok(bytes_read)
                  }
                  Err(e) => {
                      match e.kind() {
                          ErrorKind::TimedOut => Err(NetworkError::ReadTimeout),
                          ErrorKind::ConnectionAborted | ErrorKind::ConnectionReset => {
                              println!("连接异常中断 | Connection abnormally interrupted");
                              self.connection = None;
                              Err(NetworkError::ConnectionClosed)
                          }
                          _ => Err(NetworkError::Unknown(e.to_string())),
                      }
                  }
              }
          } else {
              Err(NetworkError::ConnectionFailed(self.target_addr.clone()))
          }
      }
  }
  
  /// 错误处理工具函数 | Error handling utility functions
  pub struct ErrorHandler;
  
  impl ErrorHandler {
      /// 网络错误分类处理 | Network error classification handling
      pub fn handle_network_error(error: &std::io::Error) -> NetworkError {
          match error.kind() {
              ErrorKind::TimedOut => NetworkError::ConnectionTimeout,
              ErrorKind::ConnectionRefused => NetworkError::ConnectionFailed("连接被拒绝 | Connection refused".to_string()),
              ErrorKind::ConnectionAborted => NetworkError::ConnectionClosed,
              ErrorKind::ConnectionReset => NetworkError::ConnectionClosed,
              ErrorKind::BrokenPipe => NetworkError::ConnectionClosed,
              ErrorKind::UnexpectedEof => NetworkError::ConnectionClosed,
              ErrorKind::InvalidData => NetworkError::InvalidData("数据格式错误 | Invalid data format".to_string()),
              _ => NetworkError::Unknown(error.to_string()),
          }
      }
      
      /// 错误恢复策略 | Error recovery strategy
      pub fn should_retry(error: &NetworkError) -> bool {
          match error {
              NetworkError::ConnectionTimeout => true,
              NetworkError::ConnectionFailed(_) => true,
              NetworkError::ConnectionClosed => true,
              NetworkError::ReadTimeout => true,
              NetworkError::WriteTimeout => true,
              NetworkError::ResourceExhausted => false,
              NetworkError::InvalidData(_) => false,
              NetworkError::Unknown(_) => false,
          }
      }
      
      /// 获取重试延迟 | Get retry delay
      pub fn get_retry_delay(attempt: u32) -> Duration {
          // 指数退避策略 | Exponential backoff strategy
          let base_delay = Duration::from_millis(100);
          let max_delay = Duration::from_secs(30);
          
          let delay = base_delay * (2_u32.pow(attempt.min(10)));
          delay.min(max_delay)
      }
  }
  ```

### 6. 实践整合与优化技巧 | Practice Integration and Optimization Techniques (15分钟 | 15 minutes)

- **网络编程性能优化策略 | Network Programming Performance Optimization Strategies**
  
  **概念定义 | Concept Definition:**
  网络编程性能优化涉及减少延迟、提高吞吐量、优化资源使用等方面。通过合理的缓冲区大小、连接复用、批量处理等技术可以显著提升性能。| Network programming performance optimization involves reducing latency, improving throughput, optimizing resource usage. Through proper buffer sizes, connection reuse, batch processing and other techniques, performance can be significantly improved.
  
  **优化维度 | Optimization Dimensions:**
  - 内存优化：减少内存分配和拷贝 | Memory optimization: reduce memory allocation and copying
  - IO优化：批量读写、缓冲区调优 | I/O optimization: batch read/write, buffer tuning
  - 连接优化：连接池、长连接复用 | Connection optimization: connection pools, persistent connection reuse
  - 并发优化：多线程、异步处理 | Concurrency optimization: multi-threading, asynchronous processing
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 增大缓冲区大小一定能提高性能吗？| Does increasing buffer size always improve performance?
     **答案 | Answer:** 不一定 | Not necessarily - 过大的缓冲区可能增加内存使用和延迟 | Overly large buffers may increase memory usage and latency
  2. TCP_NODELAY选项的作用是什么？| What does the TCP_NODELAY option do?
     **答案 | Answer:** 禁用Nagle算法 | Disables Nagle algorithm - 立即发送小数据包，减少延迟 | Immediately sends small packets, reduces latency
  3. 连接池的主要好处是什么？| What are the main benefits of connection pooling?
     **答案 | Answer:** 减少连接建立开销，提高资源利用率 | Reduces connection establishment overhead, improves resource utilization
  4. 什么情况下应该使用长连接？| When should persistent connections be used?
     **答案 | Answer:** 频繁通信的场景 | Scenarios with frequent communication - 避免重复建立连接的开销 | Avoids overhead of repeatedly establishing connections

## 实践项目：多功能Echo服务器 | Practical Project: Multi-functional Echo Server

### 目标 | Objective
构建一个功能完整的Echo服务器，集成TCP服务器、HTTP服务器、错误处理、连接管理等多种技术，展示std::net模块的综合应用能力。| Build a fully functional Echo server integrating TCP server, HTTP server, error handling, connection management and other technologies, demonstrating comprehensive application capabilities of std::net module.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. TcpListener和TcpStream的区别和用途是什么？| What are the differences and uses of TcpListener and TcpStream?
   **答案 | Answer:** TcpListener用于监听连接，TcpStream用于数据通信 | TcpListener for listening connections, TcpStream for data communication
2. HTTP请求的基本结构包含哪些部分？| What parts does the basic structure of HTTP request contain?
   **答案 | Answer:** 请求行、请求头、空行、请求体 | Request line, headers, empty line, body
3. 网络错误处理中最重要的考虑因素是什么？| What are the most important considerations in network error handling?
   **答案 | Answer:** 错误分类、重试策略、资源清理 | Error classification, retry strategy, resource cleanup

### 步骤 | Steps
1. 创建基础项目结构和配置管理 | Create basic project structure and configuration management
2. 实现TCP Echo服务器核心功能 | Implement TCP Echo server core functionality
3. 添加HTTP服务器支持和API端点 | Add HTTP server support and API endpoints
4. 集成错误处理和连接管理机制 | Integrate error handling and connection management mechanisms
5. 实现性能监控和统计功能 | Implement performance monitoring and statistics functionality

### 示例代码 | Example Code
```rust
"""
多功能Echo服务器 | Multi-functional Echo Server
展示Rust标准库网络编程的综合应用 | Demonstrates comprehensive application of Rust standard library network programming

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- std::net模块的核心功能 | Core functionality of std::net module
- TCP和HTTP服务器实现 | TCP and HTTP server implementation
- 错误处理和连接管理 | Error handling and connection management
- 性能优化和监控 | Performance optimization and monitoring
"""

// main.rs - 主程序入口 | Main program entry
use clap::{App, Arg, SubCommand};
use std::thread;
use std::time::Duration;

mod tcp_server;
mod http_server;
mod connection_manager;
mod error_handling;

use tcp_server::TcpEchoServer;
use http_server::HttpEchoServer;
use connection_manager::ConnectionPool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("多功能Echo服务器 | Multi-functional Echo Server")
        .version("1.0")
        .author("Rust Developer")
        .about("基于std::net的Echo服务器 | Echo server based on std::net")
        .subcommand(SubCommand::with_name("tcp")
            .about("启动TCP Echo服务器 | Start TCP Echo server")
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("监听端口 | Listen port")
                .takes_value(true)
                .default_value("8080"))
            .arg(Arg::with_name("max-connections")
                .short("m")
                .long("max-connections")
                .value_name("MAX")
                .help("最大连接数 | Maximum connections")
                .takes_value(true)
                .default_value("100")))
        .subcommand(SubCommand::with_name("http")
            .about("启动HTTP Echo服务器 | Start HTTP Echo server")
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("监听端口 | Listen port")
                .takes_value(true)
                .default_value("8080"))
            .arg(Arg::with_name("static-dir")
                .short("s")
                .long("static")
                .value_name("DIR")
                .help("静态文件目录 | Static files directory")
                .takes_value(true)
                .default_value("./static")))
        .subcommand(SubCommand::with_name("both")
            .about("同时启动TCP和HTTP服务器 | Start both TCP and HTTP servers")
            .arg(Arg::with_name("tcp-port")
                .short("t")
                .long("tcp-port")
                .value_name("PORT")
                .help("TCP端口 | TCP port")
                .takes_value(true)
                .default_value("8080"))
            .arg(Arg::with_name("http-port")
                .short("h")
                .long("http-port")
                .value_name("PORT")
                .help("HTTP端口 | HTTP port")
                .takes_value(true)
                .default_value("8081")))
        .get_matches();

    match matches.subcommand() {
        ("tcp", Some(tcp_matches)) => {
            let port: u16 = tcp_matches.value_of("port").unwrap().parse()?;
            let max_connections: usize = tcp_matches.value_of("max-connections").unwrap().parse()?;
            
            println!("🚀 启动TCP Echo服务器... | Starting TCP Echo server...");
            println!("📡 监听端口: {} | Listen port: {}", port, port);
            println!("🔗 最大连接数: {} | Max connections: {}", max_connections, max_connections);
            
            let mut server = TcpEchoServer::new(format!("127.0.0.1:{}", port), max_connections);
            server.start()?;
        }
        
        ("http", Some(http_matches)) => {
            let port: u16 = http_matches.value_of("port").unwrap().parse()?;
            let static_dir = http_matches.value_of("static-dir").unwrap();
            
            println!("🌐 启动HTTP Echo服务器... | Starting HTTP Echo server...");
            println!("📡 监听端口: {} | Listen port: {}", port, port);
            println!("📁 静态文件目录: {} | Static files directory: {}", static_dir, static_dir);
            
            let server = HttpEchoServer::new(static_dir);
            server.start(&format!("127.0.0.1:{}", port))?;
        }
        
        ("both", Some(both_matches)) => {
            let tcp_port: u16 = both_matches.value_of("tcp-port").unwrap().parse()?;
            let http_port: u16 = both_matches.value_of("http-port").unwrap().parse()?;
            
            println!("🚀 启动双服务器模式... | Starting dual server mode...");
            println!("📡 TCP端口: {} | TCP port: {}", tcp_port, tcp_port);
            println!("🌐 HTTP端口: {} | HTTP port: {}", http_port, http_port);
            
            // TCP服务器线程 | TCP server thread
            let tcp_addr = format!("127.0.0.1:{}", tcp_port);
            let tcp_handle = thread::spawn(move || {
                let mut server = TcpEchoServer::new(tcp_addr, 100);
                if let Err(e) = server.start() {
                    eprintln!("TCP服务器错误: {} | TCP server error: {}", e, e);
                }
            });
            
            // HTTP服务器线程 | HTTP server thread
            let http_addr = format!("127.0.0.1:{}", http_port);
            let http_handle = thread::spawn(move || {
                let server = HttpEchoServer::new("./static");
                if let Err(e) = server.start(&http_addr) {
                    eprintln!("HTTP服务器错误: {} | HTTP server error: {}", e, e);
                }
            });
            
            // 等待用户中断 | Wait for user interrupt
            println!("按 Ctrl+C 停止服务器 | Press Ctrl+C to stop servers");
            let _ = tcp_handle.join();
            let _ = http_handle.join();
        }
        
        _ => {
            println!("使用 --help 查看可用命令 | Use --help to see available commands");
            println!("\n示例 | Examples:");
            println!("  cargo run tcp -p 8080              # 启动TCP服务器");
            println!("  cargo run http -p 8081             # 启动HTTP服务器");  
            println!("  cargo run both -t 8080 -h 8081     # 启动双服务器");
        }
    }

    Ok(())
}

// tcp_server.rs - TCP Echo服务器实现
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Result, BufRead, BufReader};
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// TCP Echo服务器 | TCP Echo Server
pub struct TcpEchoServer {
    bind_addr: String,
    max_connections: usize,
    stats: Arc<Mutex<ServerStats>>,
    active_connections: Arc<Mutex<HashMap<u32, ConnectionInfo>>>,
    next_connection_id: Arc<Mutex<u32>>,
}

#[derive(Debug, Clone, Default)]
pub struct ServerStats {
    pub total_connections: u64,
    pub messages_processed: u64,
    pub bytes_received: u64,
    pub bytes_sent: u64,
    pub errors: u64,
    pub uptime_start: Option<Instant>,
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub id: u32,
    pub addr: String,
    pub connected_at: Instant,
    pub last_activity: Instant,
    pub messages_count: u32,
}

impl TcpEchoServer {
    /// 创建新的TCP Echo服务器 | Create new TCP Echo server
    pub fn new(bind_addr: String, max_connections: usize) -> Self {
        Self {
            bind_addr,
            max_connections,
            stats: Arc::new(Mutex::new(ServerStats {
                uptime_start: Some(Instant::now()),
                ..Default::default()
            })),
            active_connections: Arc::new(Mutex::new(HashMap::new())),
            next_connection_id: Arc::new(Mutex::new(1)),
        }
    }
    
    /// 启动服务器 | Start server
    pub fn start(&mut self) -> Result<()> {
        let listener = TcpListener::bind(&self.bind_addr)?;
        let local_addr = listener.local_addr()?;
        
        println!("✅ TCP Echo服务器启动成功 | TCP Echo server started successfully");
        println!("📡 监听地址: {} | Listening on: {}", local_addr, local_addr);
        
        // 启动统计监控 | Start statistics monitoring
        self.start_stats_monitoring();
        
        // 主循环 | Main loop
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if self.should_accept_connection() {
                        self.handle_new_connection(stream);
                    } else {
                        println!("⚠️  拒绝连接 - 已达到最大连接数 | Connection rejected - max connections reached");
                    }
                }
                Err(e) => {
                    println!("❌ 接受连接时出错: {} | Error accepting connection: {}", e, e);
                    self.increment_error_count();
                }
            }
        }
        
        Ok(())
    }
    
    /// 检查是否应该接受新连接 | Check if should accept new connection
    fn should_accept_connection(&self) -> bool {
        let connections = self.active_connections.lock().unwrap();
        connections.len() < self.max_connections
    }
    
    /// 处理新连接 | Handle new connection
    fn handle_new_connection(&self, stream: TcpStream) {
        let connection_id = {
            let mut next_id = self.next_connection_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };
        
        let peer_addr = stream.peer_addr().unwrap().to_string();
        println!("🔗 新连接 #{}: {} | New connection #{}: {}", connection_id, peer_addr, connection_id, peer_addr);
        
        // 记录连接信息 | Record connection info
        {
            let mut connections = self.active_connections.lock().unwrap();
            connections.insert(connection_id, ConnectionInfo {
                id: connection_id,
                addr: peer_addr.clone(),
                connected_at: Instant::now(),
                last_activity: Instant::now(),
                messages_count: 0,
            });
        }
        
        // 更新统计 | Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_connections += 1;
        }
        
        // 创建连接处理线程 | Create connection handler thread
        let stats = Arc::clone(&self.stats);
        let connections = Arc::clone(&self.active_connections);
        
        thread::spawn(move || {
            if let Err(e) = Self::handle_connection(stream, connection_id, stats, connections) {
                println!("❌ 连接 #{} 处理错误: {} | Connection #{} handling error: {}", connection_id, e, connection_id, e);
            }
        });
    }
    
    /// 处理连接通信 | Handle connection communication
    fn handle_connection(
        mut stream: TcpStream,
        connection_id: u32,
        stats: Arc<Mutex<ServerStats>>,
        connections: Arc<Mutex<HashMap<u32, ConnectionInfo>>>,
    ) -> Result<()> {
        // 设置超时 | Set timeouts
        stream.set_read_timeout(Some(Duration::from_secs(300)))?; // 5分钟读取超时 | 5 minute read timeout
        stream.set_write_timeout(Some(Duration::from_secs(60)))?; // 1分钟写入超时 | 1 minute write timeout
        stream.set_nodelay(true)?; // 禁用Nagle算法 | Disable Nagle algorithm
        
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();
        
        // 发送欢迎消息 | Send welcome message
        let welcome = format!("🎉 欢迎连接到TCP Echo服务器! 连接ID: {} | Welcome to TCP Echo Server! Connection ID: {}\n", connection_id, connection_id);
        stream.write_all(welcome.as_bytes())?;
        stream.flush()?;
        
        println!("📝 连接 #{} 已发送欢迎消息 | Connection #{} welcome message sent", connection_id, connection_id);
        
        // 主通信循环 | Main communication loop
        loop {
            line.clear();
            
            match reader.read_line(&mut line) {
                Ok(0) => {
                    println!("👋 连接 #{} 被客户端关闭 | Connection #{} closed by client", connection_id, connection_id);
                    break;
                }
                Ok(bytes_read) => {
                    // 更新活动时间和统计 | Update activity time and statistics
                    {
                        let mut connections = connections.lock().unwrap();
                        if let Some(info) = connections.get_mut(&connection_id) {
                            info.last_activity = Instant::now();
                            info.messages_count += 1;
                        }
                    }
                    
                    {
                        let mut stats = stats.lock().unwrap();
                        stats.messages_processed += 1;
                        stats.bytes_received += bytes_read as u64;
                    }
                    
                    let message = line.trim();
                    println!("📨 连接 #{} 收到: {} | Connection #{} received: {}", connection_id, message, connection_id, message);
                    
                    // 处理特殊命令 | Handle special commands
                    let response = match message.to_uppercase().as_str() {
                        "PING" => "PONG".to_string(),
                        "TIME" => format!("{:?}", std::time::SystemTime::now()),
                        "STATS" => Self::get_connection_stats(&connections, connection_id),
                        "HELP" => Self::get_help_message(),
                        "QUIT" | "EXIT" | "BYE" => {
                            println!("👋 连接 #{} 请求断开 | Connection #{} requested disconnect", connection_id, connection_id);
                            break;
                        }
                        _ => {
                            // 普通Echo响应 | Regular echo response
                            format!("Echo #{}: {}", connection_id, message)
                        }
                    };
                    
                    // 发送响应 | Send response
                    let response_line = format!("{}\n", response);
                    match stream.write_all(response_line.as_bytes()) {
                        Ok(_) => {
                            stream.flush()?;
                            
                            // 更新发送统计 | Update send statistics
                            {
                                let mut stats = stats.lock().unwrap();
                                stats.bytes_sent += response_line.len() as u64;
                            }
                            
                            println!("📤 连接 #{} 发送: {} | Connection #{} sent: {}", connection_id, response, connection_id, response);
                        }
                        Err(e) => {
                            println!("❌ 连接 #{} 发送失败: {} | Connection #{} send failed: {}", connection_id, e, connection_id, e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("❌ 连接 #{} 读取错误: {} | Connection #{} read error: {}", connection_id, e, connection_id, e);
                    break;
                }
            }
        }
        
        // 清理连接信息 | Cleanup connection info
        {
            let mut connections = connections.lock().unwrap();
            if let Some(info) = connections.remove(&connection_id) {
                let duration = info.connected_at.elapsed();
                println!("🔌 连接 #{} 已关闭 - 持续时间: {:?}, 消息数: {} | Connection #{} closed - duration: {:?}, messages: {}", 
                        connection_id, duration, info.messages_count, connection_id, duration, info.messages_count);
            }
        }
        
        Ok(())
    }
    
    /// 获取连接统计信息 | Get connection statistics
    fn get_connection_stats(connections: &Arc<Mutex<HashMap<u32, ConnectionInfo>>>, connection_id: u32) -> String {
        let connections = connections.lock().unwrap();
        if let Some(info) = connections.get(&connection_id) {
            format!(
                "连接统计 | Connection Stats: ID={}, 持续时间={:?}, 消息数={}, 最后活动={:?}",
                info.id,
                info.connected_at.elapsed(),
                info.messages_count,
                info.last_activity.elapsed()
            )
        } else {
            "未找到连接信息 | Connection info not found".to_string()
        }
    }
    
    /// 获取帮助信息 | Get help message
    fn get_help_message() -> String {
        r#"可用命令 | Available Commands:
- PING: 返回PONG | Returns PONG
- TIME: 返回当前时间 | Returns current time
- STATS: 显示连接统计 | Show connection statistics
- HELP: 显示此帮助信息 | Show this help message
- QUIT/EXIT/BYE: 断开连接 | Disconnect
- 其他消息将被回显 | Other messages will be echoed"#.to_string()
    }
    
    /// 启动统计监控 | Start statistics monitoring
    fn start_stats_monitoring(&self) {
        let stats = Arc::clone(&self.stats);
        let connections = Arc::clone(&self.active_connections);
        
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(30));
                
                let stats = stats.lock().unwrap();
                let connections = connections.lock().unwrap();
                
                let uptime = if let Some(start) = stats.uptime_start {
                    start.elapsed()
                } else {
                    Duration::from_secs(0)
                };
                
                println!("\n📊 ======== TCP服务器统计 | TCP Server Statistics ========");
                println!("⏱️  运行时间: {:?} | Uptime: {:?}", uptime, uptime);
                println!("🔗 总连接数: {} | Total connections: {}", stats.total_connections, stats.total_connections);
                println!("📡 当前活跃: {} | Currently active: {}", connections.len(), connections.len());
                println!("📨 处理消息: {} | Messages processed: {}", stats.messages_processed, stats.messages_processed);
                println!("📥 接收字节: {} | Bytes received: {}", stats.bytes_received, stats.bytes_received);
                println!("📤 发送字节: {} | Bytes sent: {}", stats.bytes_sent, stats.bytes_sent);
                println!("❌ 错误次数: {} | Errors: {}", stats.errors, stats.errors);
                println!("========================================================\n");
            }
        });
    }
    
    /// 增加错误计数 | Increment error count
    fn increment_error_count(&self) {
        let mut stats = self.stats.lock().unwrap();
        stats.errors += 1;
    }
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确应用了std::net模块的核心功能？| Does the project correctly apply core functionality of std::net module?
2. TCP和HTTP服务器的实现是否符合最佳实践？| Do the TCP and HTTP server implementations follow best practices?
3. 错误处理和连接管理的逻辑是否完善？| Are the error handling and connection management logic comprehensive?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **Socket选项深度配置练习 | Socket Options Deep Configuration Exercise**
   - **练习描述 | Exercise Description:** 研究和实现TCP socket的各种选项配置，如SO_REUSEADDR、TCP_KEEPALIVE等
   - **概念检查 | Concept Check:** 理解不同socket选项对网络性能和行为的影响
   - **学习目标 | Learning Objective:** 掌握socket选项的实际应用场景

2. **多协议服务器练习 | Multi-protocol Server Exercise**
   - **练习描述 | Exercise Description:** 扩展Echo服务器支持UDP协议，实现TCP/UDP双协议服务器
   - **概念检查 | Concept Check:** 理解TCP和UDP协议的差异和适用场景
   - **学习目标 | Learning Objective:** 提高协议选择和实现能力

3. **网络性能测试工具练习 | Network Performance Testing Tool Exercise**
   - **练习描述 | Exercise Description:** 开发网络延迟、吞吐量、并发连接数的测试工具
   - **概念检查 | Concept Check:** 理解网络性能指标的测量方法
   - **学习目标 | Learning Objective:** 发展性能测试和分析技能

4. **HTTP协议扩展练习 | HTTP Protocol Extension Exercise**
   - **练习描述 | Exercise Description:** 实现HTTP/1.1的Keep-Alive、分块传输编码等特性
   - **概念检查 | Concept Check:** 理解HTTP协议的高级特性和优化技术
   - **学习目标 | Learning Objective:** 深化HTTP协议理解和实现能力

5. **网络安全加固练习 | Network Security Hardening Exercise**
   - **练习描述 | Exercise Description:** 为服务器添加TLS/SSL支持、访问控制、DDoS防护等安全功能
   - **概念检查 | Concept Check:** 理解网络安全的基础概念和防护措施
   - **学习目标 | Learning Objective:** 培养网络安全开发意识

6. **集群负载均衡练习 | Cluster Load Balancing Exercise**
   - **练习描述 | Exercise Description:** 实现简单的负载均衡器，支持多种均衡算法
   - **概念检查 | Concept Check:** 理解负载均衡的原理和实现方式
   - **学习目标 | Learning Objective:** 发展分布式系统设计能力

7. **网络调试工具练习 | Network Debugging Tool Exercise**
   - **练习描述 | Exercise Description:** 开发网络数据包捕获和分析工具，类似简化版tcpdump
   - **概念检查 | Concept Check:** 理解网络数据包的结构和传输过程
   - **学习目标 | Learning Objective:** 提高网络问题诊断和调试能力

## 学习资源 | Learning Resources
- [Rust std::net官方文档 | Rust std::net Official Documentation](https://doc.rust-lang.org/std/net/)
- [TCP/IP协议详解 | TCP/IP Illustrated](https://www.tcpipguide.com/)
- [HTTP协议规范 | HTTP Protocol Specifications](https://tools.ietf.org/html/rfc7230)
- [Rust网络编程指南 | Rust Network Programming Guide](https://rust-lang-nursery.github.io/rust-cookbook/net.html)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] std::net模块核心类型和功能掌握 | std::net module core types and functionality mastered
- [ ] TCP socket编程技能熟练应用 | TCP socket programming skills proficiently applied
- [ ] TcpListener和TcpStream的使用方法理解 | TcpListener and TcpStream usage methods understood
- [ ] HTTP协议解析和响应生成实现 | HTTP protocol parsing and response generation implemented
- [ ] 网络错误处理策略正确应用 | Network error handling strategies correctly applied
- [ ] 连接管理和资源清理机制建立 | Connection management and resource cleanup mechanisms established
- [ ] 简单HTTP服务器从零构建完成 | Simple HTTP server built from scratch completed
- [ ] 多功能Echo服务器项目实现 | Multi-functional Echo server project implemented
- [ ] 网络性能优化技巧理解和应用 | Network performance optimization techniques understood and applied
- [ ] 扩展练习至少完成3个 | At least 3 extension exercises completed

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释std::net模块的核心概念、TCP编程的基本流程、HTTP协议的实现原理，以及网络编程中的错误处理和性能优化策略。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain to others the core concepts of std::net module, basic TCP programming flow, HTTP protocol implementation principles, and error handling and performance optimization strategies in network programming.