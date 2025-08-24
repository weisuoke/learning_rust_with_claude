# Rust入门 - 第2天：HTTP/2与现代协议 | Rust Introduction - Day 2: HTTP/2 and Modern Protocols

## 学习目标 | Learning Objectives
- 深入理解HTTP/2协议的核心特性和性能优势 | Deeply understand HTTP/2 protocol's core features and performance advantages
- 掌握多路复用、服务器推送等现代Web通信机制 | Master modern web communication mechanisms like multiplexing and server push
- 了解HTTP/3和QUIC协议的基础概念 | Learn basic concepts of HTTP/3 and QUIC protocol
- 学会协议升级机制的实现原理 | Learn implementation principles of protocol upgrade mechanisms
- 能够分析和对比不同HTTP版本的性能差异 | Be able to analyze and compare performance differences between HTTP versions
- 使用Rust实现基础的HTTP/2功能演示 | Implement basic HTTP/2 functionality demonstration using Rust

## 详细内容 | Detailed Content

### 1. HTTP/2协议深度理解 | HTTP/2 Protocol Deep Understanding (1.5小时 | 1.5 hours)

- **HTTP/2协议核心概念 | HTTP/2 Protocol Core Concepts**
  
  **概念定义 | Concept Definition:**
  HTTP/2是HTTP协议的第二个主要版本，设计目标是改善Web性能，通过多路复用、头部压缩、服务器推送等技术显著提升网页加载速度和用户体验。| HTTP/2 is the second major version of the HTTP protocol, designed to improve web performance through technologies like multiplexing, header compression, and server push to significantly enhance page loading speed and user experience.
  
  **核心特征 | Key Characteristics:**
  - 二进制协议：使用二进制格式传输，相比HTTP/1.1的文本格式更高效 | Binary protocol: uses binary format for transmission, more efficient than HTTP/1.1's text format
  - 多路复用：在单个TCP连接上并行处理多个请求和响应 | Multiplexing: parallel processing of multiple requests and responses over a single TCP connection
  - 头部压缩：使用HPACK算法压缩HTTP头部，减少冗余数据 | Header compression: uses HPACK algorithm to compress HTTP headers, reducing redundant data
  - 服务器推送：服务器主动推送资源给客户端，无需等待请求 | Server push: server proactively pushes resources to client without waiting for requests
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. HTTP/2使用什么格式传输数据？| What format does HTTP/2 use for data transmission?
     **答案 | Answer:** 二进制格式 | Binary format - 这使得解析更快速且占用更少带宽 | This makes parsing faster and uses less bandwidth
  2. HTTP/2可以在单个TCP连接上同时处理多个请求吗？| Can HTTP/2 handle multiple requests simultaneously on a single TCP connection?  
     **答案 | Answer:** 是 | Yes - 这是多路复用特性的核心优势 | This is the core advantage of the multiplexing feature
  3. HTTP/2的头部压缩使用什么算法？| What algorithm does HTTP/2 use for header compression?
     **答案 | Answer:** HPACK算法 | HPACK algorithm - 专门为HTTP/2设计的高效压缩算法 | A efficient compression algorithm specifically designed for HTTP/2
  4. 服务器推送功能需要客户端发起请求吗？| Does server push functionality require client-initiated requests?
     **答案 | Answer:** 否 | No - 服务器可以主动推送资源，提前满足客户端需求 | Server can proactively push resources to meet client needs in advance
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // HTTP/2连接示例 | HTTP/2 connection example
  use hyper::{Body, Request, Response, Server};
  use hyper::service::{make_service_fn, service_fn};
  use std::convert::Infallible;
  
  // HTTP/2服务器基础结构 | Basic HTTP/2 server structure
  async fn handle_http2_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
      println!("处理HTTP/2请求: {} {}", req.method(), req.uri());
      println!("Processing HTTP/2 request: {} {}", req.method(), req.uri());
      
      // HTTP/2自动处理多路复用 | HTTP/2 automatically handles multiplexing
      let response = Response::builder()
          .status(200)
          .header("content-type", "text/plain")
          // HTTP/2自动进行头部压缩 | HTTP/2 automatically performs header compression
          .body(Body::from("HTTP/2 响应 | HTTP/2 Response"))
          .unwrap();
      
      Ok(response)
  }
  
  #[tokio::main]
  async fn main() {
      let make_svc = make_service_fn(|_conn| async {
          Ok::<_, Infallible>(service_fn(handle_http2_request))
      });
      
      let addr = ([127, 0, 0, 1], 3000).into();
      let server = Server::bind(&addr)
          .http2_only(true)  // 强制使用HTTP/2 | Force HTTP/2 usage
          .serve(make_svc);
      
      println!("HTTP/2服务器运行在 http://{}", addr);
      println!("HTTP/2 server running at http://{}", addr);
      
      if let Err(e) = server.await {
          eprintln!("服务器错误: {}", e);
          eprintln!("Server error: {}", e);
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码如何强制使用HTTP/2协议？| How does this code force the use of HTTP/2 protocol?
    **答案 | Answer:** 使用`.http2_only(true)`方法 | Using the `.http2_only(true)` method
  - HTTP/2的多路复用在这个例子中是如何体现的？| How is HTTP/2 multiplexing demonstrated in this example?
    **答案 | Answer:** 单个服务器实例可以并发处理多个请求，无需为每个请求创建新连接 | A single server instance can handle multiple requests concurrently without creating new connections for each request
  
  **常见误区检查 | Common Misconception Checks:**
  - HTTP/2比HTTP/1.1总是更快吗？| Is HTTP/2 always faster than HTTP/1.1?
    **答案 | Answer:** 不一定，在单个大文件传输等场景下差异可能不明显 | Not necessarily, differences may not be significant in scenarios like single large file transfers
  - HTTP/2需要HTTPS才能工作吗？| Does HTTP/2 require HTTPS to work?
    **答案 | Answer:** 理论上不需要，但大多数浏览器只支持基于HTTPS的HTTP/2 | Theoretically no, but most browsers only support HTTP/2 over HTTPS

### 2. 多路复用技术深入 | Multiplexing Technology Deep Dive (1小时 | 1 hour)

- **多路复用工作原理 | Multiplexing Working Principles**
  
  **概念定义 | Concept Definition:**
  多路复用允许在单个TCP连接上同时传输多个独立的数据流(stream)，每个流都有唯一标识符，可以独立发送、接收和处理，避免了HTTP/1.1的队头阻塞问题。| Multiplexing allows simultaneous transmission of multiple independent data streams over a single TCP connection, with each stream having a unique identifier and can be sent, received, and processed independently, avoiding HTTP/1.1's head-of-line blocking problem.
  
  **核心特征 | Key Characteristics:**
  - 流独立性：每个HTTP请求/响应对应一个独立的流 | Stream independence: each HTTP request/response corresponds to an independent stream
  - 流优先级：可以为不同流设置优先级，优化资源分配 | Stream prioritization: different streams can be assigned priorities to optimize resource allocation
  - 流量控制：防止快速发送方压垮慢速接收方 | Flow control: prevents fast senders from overwhelming slow receivers
  - 无队头阻塞：一个流的延迟不会影响其他流的处理 | No head-of-line blocking: delay in one stream doesn't affect processing of other streams
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 一个TCP连接可以同时处理多少个HTTP/2流？| How many HTTP/2 streams can a single TCP connection handle simultaneously?
     **答案 | Answer:** 理论上可以处理2^31-1个流，实际受服务器配置限制 | Theoretically up to 2^31-1 streams, actually limited by server configuration
  2. HTTP/2的流有优先级控制吗？| Do HTTP/2 streams have priority control?
     **答案 | Answer:** 是 | Yes - 客户端可以指定流的优先级和依赖关系 | Clients can specify stream priorities and dependencies
  3. 如果一个流发生错误，会影响其他流吗？| If one stream encounters an error, will it affect other streams?
     **答案 | Answer:** 否 | No - 流是独立的，一个流的错误不会影响同一连接上的其他流 | Streams are independent, error in one stream doesn't affect other streams on the same connection
  4. 多路复用解决了什么HTTP/1.1问题？| What HTTP/1.1 problem does multiplexing solve?
     **答案 | Answer:** 队头阻塞问题 | Head-of-line blocking problem - 避免了慢请求阻塞后续请求 | Avoids slow requests blocking subsequent requests
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 多路复用演示：并发处理多个请求 | Multiplexing demo: concurrent handling of multiple requests
  use hyper::{Body, Method, Request, Response, Server, StatusCode};
  use hyper::service::{make_service_fn, service_fn};
  use std::convert::Infallible;
  use std::time::Duration;
  use tokio::time::sleep;
  
  async fn handle_multiplexed_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
      let path = req.uri().path();
      let method = req.method();
      
      println!("处理请求: {} {}", method, path);
      println!("Processing request: {} {}", method, path);
      
      match (method, path) {
          // 快速响应的资源 | Fast responding resource
          (&Method::GET, "/fast") => {
              let response = Response::builder()
                  .status(StatusCode::OK)
                  .header("content-type", "application/json")
                  .body(Body::from(r#"{"message":"快速响应","type":"fast"}"#))
                  .unwrap();
              Ok(response)
          },
          
          // 慢速响应的资源（模拟数据库查询） | Slow responding resource (simulating database query)
          (&Method::GET, "/slow") => {
              // 模拟耗时操作，但不会阻塞其他请求 | Simulate time-consuming operation without blocking other requests
              sleep(Duration::from_secs(3)).await;
              let response = Response::builder()
                  .status(StatusCode::OK)
                  .header("content-type", "application/json")
                  .body(Body::from(r#"{"message":"慢速响应","type":"slow","delay":"3s"}"#))
                  .unwrap();
              Ok(response)
          },
          
          // 中等速度响应 | Medium speed response
          (&Method::GET, "/medium") => {
              sleep(Duration::from_millis(500)).await;
              let response = Response::builder()
                  .status(StatusCode::OK)
                  .header("content-type", "application/json")
                  .body(Body::from(r#"{"message":"中等响应","type":"medium","delay":"500ms"}"#))
                  .unwrap();
              Ok(response)
          },
          
          _ => {
              let response = Response::builder()
                  .status(StatusCode::NOT_FOUND)
                  .body(Body::from("未找到资源 | Resource not found"))
                  .unwrap();
              Ok(response)
          }
      }
  }
  
  #[tokio::main]
  async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
      let make_svc = make_service_fn(|_conn| async {
          Ok::<_, Infallible>(service_fn(handle_multiplexed_request))
      });
      
      let addr = ([127, 0, 0, 1], 3000).into();
      let server = Server::bind(&addr)
          .http2_only(true)
          .serve(make_svc);
      
      println!("多路复用演示服务器启动: http://{}", addr);
      println!("Multiplexing demo server started: http://{}", addr);
      println!("测试端点:");
      println!("Test endpoints:");
      println!("  GET /fast   - 快速响应 | Fast response");
      println!("  GET /medium - 中等响应 | Medium response"); 
      println!("  GET /slow   - 慢速响应 | Slow response");
      
      server.await?;
      Ok(())
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 当同时请求`/slow`和`/fast`端点时会发生什么？| What happens when requesting `/slow` and `/fast` endpoints simultaneously?
    **答案 | Answer:** `/fast`会立即响应，而`/slow`会在3秒后响应，两者不会相互阻塞 | `/fast` responds immediately while `/slow` responds after 3 seconds, neither blocks the other
  - 这个例子如何展示多路复用的优势？| How does this example demonstrate the advantages of multiplexing?
    **答案 | Answer:** 多个不同延迟的请求可以在同一连接上并行处理，慢请求不影响快请求 | Multiple requests with different delays can be processed in parallel on the same connection, slow requests don't affect fast ones
  
  **常见误区检查 | Common Misconception Checks:**
  - 多路复用意味着请求会更快完成吗？| Does multiplexing mean requests will complete faster?
    **答案 | Answer:** 不一定，它主要解决的是并发性问题，而不是单个请求的速度 | Not necessarily, it mainly addresses concurrency issues rather than individual request speed
  - 多路复用需要修改现有的HTTP API吗？| Does multiplexing require modifying existing HTTP APIs?
    **答案 | Answer:** 不需要，HTTP/2在应用层保持与HTTP/1.1的兼容性 | No, HTTP/2 maintains compatibility with HTTP/1.1 at the application layer

### 3. 服务器推送机制 | Server Push Mechanism (1小时 | 1 hour)

- **服务器推送核心概念 | Server Push Core Concepts**
  
  **概念定义 | Concept Definition:**
  服务器推送允许服务器在客户端请求之前主动发送资源，通过预测客户端需要的资源（如CSS、JavaScript文件）来减少往返时间，提升页面加载性能。| Server push allows the server to proactively send resources before client requests, reducing round-trip time by predicting resources the client will need (like CSS, JavaScript files) to improve page loading performance.
  
  **核心特征 | Key Characteristics:**
  - 主动推送：服务器无需等待客户端请求即可发送资源 | Proactive pushing: server can send resources without waiting for client requests
  - 缓存感知：推送的资源遵循HTTP缓存机制 | Cache-aware: pushed resources follow HTTP caching mechanisms
  - 客户端控制：客户端可以拒绝不需要的推送资源 | Client control: clients can reject unwanted pushed resources
  - 依赖优化：可以推送当前请求资源的依赖项 | Dependency optimization: can push dependencies of current requested resources
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 服务器推送需要客户端先发起请求吗？| Does server push require the client to initiate a request first?
     **答案 | Answer:** 不需要 | No - 这正是服务器推送的核心优势，可以预先发送资源 | This is the core advantage of server push, it can send resources in advance
  2. 客户端可以拒绝服务器推送的资源吗？| Can clients reject server-pushed resources?
     **答案 | Answer:** 是 | Yes - 客户端可以发送RST_STREAM帧拒绝推送 | Clients can send RST_STREAM frames to reject pushes
  3. 推送的资源会被浏览器缓存吗？| Will pushed resources be cached by the browser?
     **答案 | Answer:** 是 | Yes - 推送的资源遵循正常的HTTP缓存机制 | Pushed resources follow normal HTTP caching mechanisms
  4. 什么场景下服务器推送最有效？| In what scenarios is server push most effective?
     **答案 | Answer:** 当服务器能够准确预测客户端需要的资源时 | When the server can accurately predict resources the client will need
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 注意：此示例演示服务器推送概念，实际的hyper需要更复杂的配置
  // Note: This example demonstrates server push concepts, actual hyper requires more complex configuration
  use hyper::{Body, Method, Request, Response, Server, StatusCode};
  use hyper::service::{make_service_fn, service_fn};
  use hyper::header::{HeaderValue, CONTENT_TYPE, LINK};
  use std::convert::Infallible;
  
  async fn handle_server_push_demo(req: Request<Body>) -> Result<Response<Body>, Infallible> {
      let path = req.uri().path();
      
      match (req.method(), path) {
          // 主HTML页面 - 这里我们模拟推送指示 | Main HTML page - here we simulate push indication
          (&Method::GET, "/") => {
              // 在HTTP/2中，服务器会在这里推送CSS和JS文件
              // In HTTP/2, server would push CSS and JS files here
              let html = r#"
  <!DOCTYPE html>
  <html>
  <head>
      <title>服务器推送演示 | Server Push Demo</title>
      <link rel="stylesheet" href="/styles.css">
  </head>
  <body>
      <h1>HTTP/2 服务器推送演示</h1>
      <h1>HTTP/2 Server Push Demo</h1>
      <p>CSS和JavaScript文件应该已被推送</p>
      <p>CSS and JavaScript files should have been pushed</p>
      <script src="/app.js"></script>
  </body>
  </html>
              "#;
              
              let response = Response::builder()
                  .status(StatusCode::OK)
                  .header(CONTENT_TYPE, "text/html; charset=utf-8")
                  // Link头用于指示推送资源（HTTP/2服务器推送提示）
                  // Link header indicates push resources (HTTP/2 server push hint)
                  .header(LINK, "</styles.css>; rel=preload; as=style")
                  .header(LINK, "</app.js>; rel=preload; as=script")
                  .body(Body::from(html))
                  .unwrap();
              
              println!("发送主页面，指示推送CSS和JS文件");
              println!("Sending main page, indicating push of CSS and JS files");
              
              Ok(response)
          },
          
          // CSS文件 - 通常会被推送 | CSS file - typically pushed
          (&Method::GET, "/styles.css") => {
              let css = r#"
  body {
      font-family: Arial, sans-serif;
      margin: 40px;
      background-color: #f5f5f5;
  }
  h1 {
      color: #333;
      border-bottom: 2px solid #007acc;
  }
  p {
      line-height: 1.6;
  }
              "#;
              
              let response = Response::builder()
                  .status(StatusCode::OK)
                  .header(CONTENT_TYPE, "text/css")
                  // 指示这个资源可以被推送 | Indicate this resource can be pushed
                  .header("X-Push-Policy", "push")
                  .body(Body::from(css))
                  .unwrap();
              
              println!("发送CSS文件（通常由服务器推送）");
              println!("Sending CSS file (typically pushed by server)");
              
              Ok(response)
          },
          
          // JavaScript文件 - 通常会被推送 | JavaScript file - typically pushed  
          (&Method::GET, "/app.js") => {
              let js = r#"
  console.log('HTTP/2 服务器推送演示加载完成');
  console.log('HTTP/2 Server Push demo loaded');
  
  // 检测资源加载时间 | Detect resource loading time
  window.addEventListener('load', function() {
      const perfEntries = performance.getEntriesByType('navigation');
      if (perfEntries.length > 0) {
          console.log('页面加载时间:', perfEntries[0].loadEventEnd - perfEntries[0].loadEventStart, 'ms');
          console.log('Page load time:', perfEntries[0].loadEventEnd - perfEntries[0].loadEventStart, 'ms');
      }
  });
              "#;
              
              let response = Response::builder()
                  .status(StatusCode::OK)
                  .header(CONTENT_TYPE, "application/javascript")
                  .header("X-Push-Policy", "push")
                  .body(Body::from(js))
                  .unwrap();
              
              println!("发送JavaScript文件（通常由服务器推送）");
              println!("Sending JavaScript file (typically pushed by server)");
              
              Ok(response)
          },
          
          _ => {
              let response = Response::builder()
                  .status(StatusCode::NOT_FOUND)
                  .body(Body::from("资源未找到 | Resource not found"))
                  .unwrap();
              Ok(response)
          }
      }
  }
  
  #[tokio::main]
  async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
      let make_svc = make_service_fn(|_conn| async {
          Ok::<_, Infallible>(service_fn(handle_server_push_demo))
      });
      
      let addr = ([127, 0, 0, 1], 3000).into();
      let server = Server::bind(&addr)
          .http2_only(true)
          .serve(make_svc);
      
      println!("服务器推送演示服务器启动: http://{}", addr);
      println!("Server push demo server started: http://{}", addr);
      println!("访问 http://localhost:3000 查看演示");
      println!("Visit http://localhost:3000 to see the demo");
      
      server.await?;
      Ok(())
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - Link头中的`rel=preload`有什么作用？| What is the purpose of `rel=preload` in the Link header?
    **答案 | Answer:** 它告诉浏览器这些资源应该被预先加载，在HTTP/2中可以触发服务器推送 | It tells the browser these resources should be preloaded, which can trigger server push in HTTP/2
  - 为什么要检查资源加载时间？| Why check resource loading time?
    **答案 | Answer:** 服务器推送的效果可以通过减少的加载时间来衡量 | The effectiveness of server push can be measured by reduced loading times
  
  **常见误区检查 | Common Misconception Checks:**
  - 服务器推送总是能提升性能吗？| Does server push always improve performance?
    **答案 | Answer:** 不一定，如果推送了客户端不需要或已缓存的资源，可能会浪费带宽 | Not necessarily, pushing resources that clients don't need or already have cached may waste bandwidth
  - 服务器推送会替代客户端缓存吗？| Will server push replace client caching?
    **答案 | Answer:** 不会，它们是互补的，推送的资源仍然可以被缓存 | No, they are complementary, pushed resources can still be cached

### 4. HTTP/3与QUIC协议基础 | HTTP/3 and QUIC Protocol Basics (45分钟 | 45 minutes)

- **HTTP/3与QUIC协议概述 | HTTP/3 and QUIC Protocol Overview**
  
  **概念定义 | Concept Definition:**
  HTTP/3是HTTP协议的最新版本，基于QUIC传输协议构建，旨在解决TCP的固有限制，提供更快的连接建立、更好的多路复用和improved移动网络性能。| HTTP/3 is the latest version of HTTP protocol, built on QUIC transport protocol, aiming to solve TCP's inherent limitations and provide faster connection establishment, better multiplexing, and improved mobile network performance.
  
  **核心特征 | Key Characteristics:**
  - 基于UDP：使用UDP代替TCP，减少连接延迟 | UDP-based: uses UDP instead of TCP, reducing connection latency
  - 内置加密：加密是协议的核心部分，不是可选项 | Built-in encryption: encryption is core to the protocol, not optional
  - 连接迁移：支持IP地址变更时保持连接 | Connection migration: supports maintaining connections when IP addresses change
  - 改进的多路复用：解决TCP层面的队头阻塞 | Improved multiplexing: solves head-of-line blocking at TCP level
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. HTTP/3使用什么传输协议？| What transport protocol does HTTP/3 use?
     **答案 | Answer:** QUIC协议（基于UDP）| QUIC protocol (based on UDP) - 这避免了TCP的队头阻塞问题 | This avoids TCP's head-of-line blocking issues
  2. HTTP/3的加密是可选的吗？| Is encryption optional in HTTP/3?
     **答案 | Answer:** 否 | No - 加密是HTTP/3/QUIC协议的内置部分 | Encryption is built into the HTTP/3/QUIC protocol
  3. 当用户从WiFi切换到移动网络时，HTTP/3连接会断开吗？| When a user switches from WiFi to mobile network, will the HTTP/3 connection drop?
     **答案 | Answer:** 不一定 | Not necessarily - QUIC支持连接迁移，可以保持连接状态 | QUIC supports connection migration and can maintain connection state
  4. HTTP/3解决了HTTP/2的什么问题？| What HTTP/2 problem does HTTP/3 solve?
     **答案 | Answer:** TCP层面的队头阻塞问题 | Head-of-line blocking at the TCP level - HTTP/2只解决了HTTP层面的问题 | HTTP/2 only solved it at the HTTP level
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // HTTP/3概念演示（注意：实际HTTP/3支持需要专门的库如quiche）
  // HTTP/3 concept demonstration (Note: actual HTTP/3 support requires specialized libraries like quiche)
  use std::collections::HashMap;
  use serde::{Deserialize, Serialize};
  
  // QUIC连接状态模拟 | QUIC connection state simulation
  #[derive(Debug, Clone)]
  pub struct QuicConnection {
      pub connection_id: u64,
      pub local_addr: String,
      pub peer_addr: String,
      pub is_migrated: bool,
      pub encryption_established: bool,
      pub streams: HashMap<u64, QuicStream>,
  }
  
  #[derive(Debug, Clone)]
  pub struct QuicStream {
      pub stream_id: u64,
      pub is_bidirectional: bool,
      pub state: StreamState,
      pub data: Vec<u8>,
  }
  
  #[derive(Debug, Clone)]
  pub enum StreamState {
      Open,
      HalfClosed,
      Closed,
  }
  
  #[derive(Serialize, Deserialize, Debug)]
  pub struct Http3Frame {
      pub frame_type: String,
      pub stream_id: u64,
      pub payload: Vec<u8>,
  }
  
  impl QuicConnection {
      pub fn new(connection_id: u64, local_addr: String, peer_addr: String) -> Self {
          println!("创建新的QUIC连接 ID: {}", connection_id);
          println!("Creating new QUIC connection ID: {}", connection_id);
          
          Self {
              connection_id,
              local_addr,
              peer_addr,
              is_migrated: false,
              encryption_established: true, // QUIC默认加密 | QUIC encrypted by default
              streams: HashMap::new(),
          }
      }
      
      // 演示连接迁移特性 | Demonstrate connection migration feature
      pub fn migrate_connection(&mut self, new_local_addr: String) {
          println!("连接迁移: {} -> {}", self.local_addr, new_local_addr);
          println!("Connection migration: {} -> {}", self.local_addr, new_local_addr);
          
          self.local_addr = new_local_addr;
          self.is_migrated = true;
          
          println!("连接迁移完成，连接ID保持不变: {}", self.connection_id);
          println!("Connection migration completed, connection ID remains: {}", self.connection_id);
      }
      
      // 创建HTTP/3流 | Create HTTP/3 stream
      pub fn create_stream(&mut self, stream_id: u64, is_bidirectional: bool) -> &mut QuicStream {
          let stream = QuicStream {
              stream_id,
              is_bidirectional,
              state: StreamState::Open,
              data: Vec::new(),
          };
          
          println!("创建HTTP/3流 ID: {}, 双向: {}", stream_id, is_bidirectional);
          println!("Created HTTP/3 stream ID: {}, bidirectional: {}", stream_id, is_bidirectional);
          
          self.streams.insert(stream_id, stream);
          self.streams.get_mut(&stream_id).unwrap()
      }
      
      // 发送HTTP/3帧 | Send HTTP/3 frame
      pub fn send_http3_frame(&mut self, frame: Http3Frame) -> Result<(), String> {
          if !self.encryption_established {
              return Err("加密未建立 | Encryption not established".to_string());
          }
          
          println!("发送HTTP/3帧 - 类型: {}, 流ID: {}", frame.frame_type, frame.stream_id);
          println!("Sending HTTP/3 frame - Type: {}, Stream ID: {}", frame.frame_type, frame.stream_id);
          
          // 检查流是否存在 | Check if stream exists
          if let Some(stream) = self.streams.get_mut(&frame.stream_id) {
              stream.data.extend_from_slice(&frame.payload);
              Ok(())
          } else {
              Err(format!("流不存在 | Stream doesn't exist: {}", frame.stream_id))
          }
      }
      
      // 获取连接统计 | Get connection statistics  
      pub fn get_stats(&self) -> HashMap<String, String> {
          let mut stats = HashMap::new();
          stats.insert("connection_id".to_string(), self.connection_id.to_string());
          stats.insert("local_addr".to_string(), self.local_addr.clone());
          stats.insert("peer_addr".to_string(), self.peer_addr.clone());
          stats.insert("is_migrated".to_string(), self.is_migrated.to_string());
          stats.insert("encryption".to_string(), "enabled".to_string());
          stats.insert("active_streams".to_string(), self.streams.len().to_string());
          stats
      }
  }
  
  #[tokio::main]
  async fn main() {
      // 演示HTTP/3/QUIC特性 | Demonstrate HTTP/3/QUIC features
      println!("=== HTTP/3与QUIC协议特性演示 ===");
      println!("=== HTTP/3 and QUIC Protocol Features Demo ===\n");
      
      // 创建QUIC连接 | Create QUIC connection
      let mut connection = QuicConnection::new(
          12345,
          "192.168.1.100:8080".to_string(),
          "203.0.113.1:443".to_string(),
      );
      
      // 创建多个流（多路复用）| Create multiple streams (multiplexing)
      connection.create_stream(1, true);  // 双向流 | Bidirectional stream
      connection.create_stream(3, true);  // 双向流 | Bidirectional stream
      connection.create_stream(5, false); // 单向流 | Unidirectional stream
      
      // 发送HTTP/3帧 | Send HTTP/3 frames
      let headers_frame = Http3Frame {
          frame_type: "HEADERS".to_string(),
          stream_id: 1,
          payload: b"GET /api/data HTTP/3\r\nHost: example.com\r\n\r\n".to_vec(),
      };
      
      match connection.send_http3_frame(headers_frame) {
          Ok(_) => println!("HTTP/3请求帧发送成功"),
          Err(e) => println!("发送失败: {}", e),
      }
      
      let data_frame = Http3Frame {
          frame_type: "DATA".to_string(),
          stream_id: 3,
          payload: b"Response data payload".to_vec(),
      };
      
      connection.send_http3_frame(data_frame).unwrap();
      
      // 演示连接迁移（从WiFi切换到移动网络）| Demonstrate connection migration (WiFi to mobile)
      println!("\n--- 模拟网络切换 | Simulating Network Switch ---");
      connection.migrate_connection("10.0.0.50:8080".to_string());
      
      // 显示连接统计 | Show connection statistics
      println!("\n--- 连接统计 | Connection Statistics ---");
      let stats = connection.get_stats();
      for (key, value) in stats {
          println!("{}: {}", key, value);
      }
      
      println!("\n=== 演示完成 | Demo Complete ===");
      println!("HTTP/3/QUIC主要优势:");
      println!("HTTP/3/QUIC Main Advantages:");
      println!("1. 减少连接建立时间 | Reduced connection establishment time");
      println!("2. 真正的多路复用（无TCP队头阻塞）| True multiplexing (no TCP head-of-line blocking)");
      println!("3. 连接迁移支持 | Connection migration support");
      println!("4. 内置加密 | Built-in encryption");
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么QUIC连接可以迁移而TCP不能？| Why can QUIC connections migrate while TCP cannot?
    **答案 | Answer:** QUIC使用连接ID而不是IP地址和端口来标识连接 | QUIC uses connection IDs rather than IP addresses and ports to identify connections
  - HTTP/3的加密在什么层面实现？| At what level is HTTP/3 encryption implemented?
    **答案 | Answer:** 在QUIC传输层，不像HTTPS在应用层之上 | At the QUIC transport layer, unlike HTTPS which is above the application layer

### 5. 协议升级机制 | Protocol Upgrade Mechanisms (30分钟 | 30 minutes)

- **HTTP协议升级原理 | HTTP Protocol Upgrade Principles**
  
  **概念定义 | Concept Definition:**
  协议升级机制允许客户端和服务器协商使用更新或不同的协议版本，包括从HTTP/1.1升级到HTTP/2，或者通过Alt-Svc头发现HTTP/3支持，确保最佳的通信效率。| Protocol upgrade mechanisms allow clients and servers to negotiate using newer or different protocol versions, including upgrading from HTTP/1.1 to HTTP/2, or discovering HTTP/3 support through Alt-Svc headers, ensuring optimal communication efficiency.
  
  **核心特征 | Key Characteristics:**
  - 协商机制：客户端和服务器协商最佳协议版本 | Negotiation mechanism: clients and servers negotiate the best protocol version
  - 向后兼容：确保旧客户端仍能正常工作 | Backward compatibility: ensures old clients can still work normally  
  - 透明升级：应用层代码通常无需修改 | Transparent upgrade: application layer code usually requires no modification
  - 发现机制：通过ALPN、Alt-Svc等机制发现协议支持 | Discovery mechanism: discover protocol support through ALPN, Alt-Svc, etc.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. HTTP协议升级需要客户端和服务器都支持新协议吗？| Does HTTP protocol upgrade require both client and server to support the new protocol?
     **答案 | Answer:** 是 | Yes - 协议升级需要双方都支持并协商一致 | Protocol upgrade requires both parties to support and agree
  2. 如果客户端不支持HTTP/2，服务器会怎么处理？| What happens if a client doesn't support HTTP/2?
     **答案 | Answer:** 服务器会回退到HTTP/1.1或客户端支持的最高版本 | Server falls back to HTTP/1.1 or the highest version supported by client
  3. Alt-Svc头的作用是什么？| What is the purpose of the Alt-Svc header?
     **答案 | Answer:** 告知客户端可以通过替代服务（如HTTP/3）访问相同资源 | Informs clients they can access the same resources via alternative services (like HTTP/3)
  4. ALPN协商发生在连接的哪个阶段？| At which stage of connection does ALPN negotiation occur?
     **答案 | Answer:** TLS握手阶段 | During TLS handshake - 在应用数据传输之前 | Before application data transmission
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 协议升级和发现机制演示 | Protocol upgrade and discovery mechanism demonstration
  use hyper::{Body, Method, Request, Response, Server, StatusCode, Version};
  use hyper::service::{make_service_fn, service_fn};
  use hyper::header::{HeaderValue, ALT_SVC, UPGRADE, CONNECTION};
  use std::convert::Infallible;
  
  async fn handle_protocol_upgrade(req: Request<Body>) -> Result<Response<Body>, Infallible> {
      let version = req.version();
      let path = req.uri().path();
      let headers = req.headers();
      
      println!("收到请求: {} {} {:?}", req.method(), path, version);
      println!("Received request: {} {} {:?}", req.method(), path, version);
      
      // 检查客户端是否请求协议升级 | Check if client requests protocol upgrade
      if let Some(upgrade_header) = headers.get(UPGRADE) {
          println!("客户端请求协议升级: {:?}", upgrade_header);
          println!("Client requests protocol upgrade: {:?}", upgrade_header);
      }
      
      match (req.method(), path) {
          (&Method::GET, "/") => {
              let mut response_builder = Response::builder()
                  .status(StatusCode::OK)
                  .header("content-type", "text/html; charset=utf-8");
              
              // 对于HTTP/1.1请求，添加Alt-Svc头指示HTTP/2和HTTP/3支持
              // For HTTP/1.1 requests, add Alt-Svc header indicating HTTP/2 and HTTP/3 support
              if version == Version::HTTP_11 {
                  response_builder = response_builder.header(
                      ALT_SVC,
                      r#"h2=":443"; ma=3600, h3-29=":443"; ma=3600"#
                  );
                  println!("为HTTP/1.1客户端添加Alt-Svc头");
                  println!("Added Alt-Svc header for HTTP/1.1 client");
              }
              
              let html = format!(r#"
  <!DOCTYPE html>
  <html>
  <head>
      <title>协议升级演示 | Protocol Upgrade Demo</title>
      <meta charset="utf-8">
  </head>
  <body>
      <h1>HTTP协议版本检测</h1>
      <h1>HTTP Protocol Version Detection</h1>
      
      <div>
          <h2>当前连接信息 | Current Connection Info</h2>
          <ul>
              <li>协议版本 | Protocol Version: <strong>{:?}</strong></li>
              <li>请求方法 | Request Method: <strong>{}</strong></li>
              <li>请求路径 | Request Path: <strong>{}</strong></li>
          </ul>
      </div>
      
      <div>
          <h2>协议升级状态 | Protocol Upgrade Status</h2>
          <p id="upgrade-status">检查升级可能性...</p>
          <p id="upgrade-status">Checking upgrade possibilities...</p>
      </div>
      
      <div>
          <h2>测试链接 | Test Links</h2>
          <ul>
              <li><a href="/protocol-info">协议信息 | Protocol Info</a></li>
              <li><a href="/performance-test">性能测试 | Performance Test</a></li>
              <li><a href="/upgrade-check">升级检查 | Upgrade Check</a></li>
          </ul>
      </div>
      
      <script>
          // 检测当前使用的协议 | Detect currently used protocol
          document.addEventListener('DOMContentLoaded', function() {{
              const statusElement = document.getElementById('upgrade-status');
              const protocol = location.protocol;
              const isSecure = protocol === 'https:';
              
              if (isSecure) {{
                  statusElement.innerHTML = '使用HTTPS，支持协议升级 | Using HTTPS, protocol upgrade supported';
                  statusElement.style.color = 'green';
              }} else {{
                  statusElement.innerHTML = '使用HTTP，协议升级受限 | Using HTTP, protocol upgrade limited';
                  statusElement.style.color = 'orange';
              }}
          }});
      </script>
  </body>
  </html>
              "#, version, req.method(), path);
              
              let response = response_builder
                  .body(Body::from(html))
                  .unwrap();
              
              Ok(response)
          },
          
          (&Method::GET, "/protocol-info") => {
              let protocol_info = match version {
                  Version::HTTP_09 => "HTTP/0.9 - 原始HTTP协议 | Original HTTP protocol",
                  Version::HTTP_10 => "HTTP/1.0 - 添加头部支持 | Added header support", 
                  Version::HTTP_11 => "HTTP/1.1 - 持久连接和管道化 | Persistent connections and pipelining",
                  Version::HTTP_2 => "HTTP/2 - 二进制、多路复用、服务器推送 | Binary, multiplexing, server push",
                  Version::HTTP_3 => "HTTP/3 - 基于QUIC，更快连接 | QUIC-based, faster connections",
                  _ => "未知协议版本 | Unknown protocol version",
              };
              
              let json_response = format!(r#"
  {{
      "protocol_version": "{:?}",
      "protocol_description": "{}",
      "features": {{
          "multiplexing": {},
          "server_push": {},
          "header_compression": {},
          "binary_protocol": {}
      }},
      "upgrade_available": {}
  }}
              "#,
                  version,
                  protocol_info,
                  version >= Version::HTTP_2,  // 多路复用 | Multiplexing
                  version >= Version::HTTP_2,  // 服务器推送 | Server push
                  version >= Version::HTTP_2,  // 头部压缩 | Header compression
                  version >= Version::HTTP_2,  // 二进制协议 | Binary protocol
                  version < Version::HTTP_2     // 升级可用 | Upgrade available
              );
              
              let mut response_builder = Response::builder()
                  .status(StatusCode::OK)
                  .header("content-type", "application/json");
              
              // 如果是HTTP/1.1，建议升级 | If HTTP/1.1, suggest upgrade
              if version == Version::HTTP_11 {
                  response_builder = response_builder.header(
                      ALT_SVC,
                      r#"h2=":443"; ma=7200"#
                  );
              }
              
              let response = response_builder
                  .body(Body::from(json_response))
                  .unwrap();
              
              Ok(response)
          },
          
          (&Method::GET, "/upgrade-check") => {
              let upgrade_recommendations = match version {
                  Version::HTTP_11 => vec![
                      "建议升级到HTTP/2以获得更好性能 | Recommend upgrading to HTTP/2 for better performance".to_string(),
                      "HTTP/2支持多路复用，减少连接数 | HTTP/2 supports multiplexing, reducing connections".to_string(),
                      "HTTP/2提供头部压缩，节省带宽 | HTTP/2 provides header compression, saving bandwidth".to_string(),
                  ],
                  Version::HTTP_2 => vec![
                      "当前使用HTTP/2，性能良好 | Currently using HTTP/2, good performance".to_string(),
                      "可以考虑HTTP/3以获得更低延迟 | Consider HTTP/3 for even lower latency".to_string(),
                  ],
                  _ => vec![
                      "使用最新协议版本 | Using latest protocol version".to_string(),
                  ]
              };
              
              let recommendations_html = upgrade_recommendations
                  .iter()
                  .map(|rec| format!("<li>{}</li>", rec))
                  .collect::<Vec<_>>()
                  .join("\n");
              
              let html = format!(r#"
  <!DOCTYPE html>
  <html>
  <head>
      <title>协议升级建议 | Protocol Upgrade Recommendations</title>
      <meta charset="utf-8">
  </head>
  <body>
      <h1>协议升级检查结果</h1>
      <h1>Protocol Upgrade Check Results</h1>
      
      <p>当前协议: <strong>{:?}</strong></p>
      <p>Current Protocol: <strong>{:?}</strong></p>
      
      <h2>建议 | Recommendations:</h2>
      <ul>
          {}
      </ul>
      
      <a href="/">返回首页 | Back to Home</a>
  </body>
  </html>
              "#, version, version, recommendations_html);
              
              let response = Response::builder()
                  .status(StatusCode::OK)
                  .header("content-type", "text/html; charset=utf-8")
                  .body(Body::from(html))
                  .unwrap();
              
              Ok(response)
          },
          
          _ => {
              let response = Response::builder()
                  .status(StatusCode::NOT_FOUND)
                  .body(Body::from("页面未找到 | Page not found"))
                  .unwrap();
              Ok(response)
          }
      }
  }
  
  #[tokio::main]
  async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
      let make_svc = make_service_fn(|_conn| async {
          Ok::<_, Infallible>(service_fn(handle_protocol_upgrade))
      });
      
      let addr = ([127, 0, 0, 1], 3000).into();
      let server = Server::bind(&addr)
          .http2_only(false)  // 支持HTTP/1.1和HTTP/2 | Support both HTTP/1.1 and HTTP/2
          .serve(make_svc);
      
      println!("协议升级演示服务器启动: http://{}", addr);
      println!("Protocol upgrade demo server started: http://{}", addr);
      println!("支持HTTP/1.1和HTTP/2，通过Alt-Svc头指示可用协议");
      println!("Supports HTTP/1.1 and HTTP/2, indicates available protocols via Alt-Svc header");
      
      server.await?;
      Ok(())
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - Alt-Svc头中的`ma=3600`表示什么？| What does `ma=3600` in the Alt-Svc header represent?
    **答案 | Answer:** 最大有效期3600秒，告诉客户端该信息的缓存时间 | Maximum age of 3600 seconds, telling the client how long to cache this information
  - 为什么协议升级检查会根据当前版本给出不同建议？| Why does protocol upgrade checking give different recommendations based on current version?
    **答案 | Answer:** 因为不同协议版本有不同的性能特点和升级路径 | Because different protocol versions have different performance characteristics and upgrade paths

### 6. HTTP协议性能对比分析 | HTTP Protocol Performance Comparison Analysis (15分钟 | 15 minutes)

- **协议性能对比总结 | Protocol Performance Comparison Summary**
  
  **概念定义 | Concept Definition:**
  通过对比HTTP/1.1、HTTP/2和HTTP/3在连接建立、数据传输、资源加载等方面的性能差异，了解各协议的优势和适用场景，为实际项目选择最优协议版本提供依据。| By comparing performance differences between HTTP/1.1, HTTP/2, and HTTP/3 in connection establishment, data transmission, and resource loading, understand the advantages and applicable scenarios of each protocol to provide basis for selecting optimal protocol versions in actual projects.
  
  **性能对比维度 | Performance Comparison Dimensions:**
  - 连接建立时间：握手次数和延迟对比 | Connection establishment time: comparison of handshake counts and latency
  - 并发处理能力：同时处理请求的效率 | Concurrent processing capability: efficiency of handling simultaneous requests
  - 带宽利用率：头部压缩和数据传输效率 | Bandwidth utilization: header compression and data transmission efficiency
  - 移动网络性能：网络切换和不稳定连接处理 | Mobile network performance: network switching and unstable connection handling
  
  **综合概念检查 | Comprehensive Concept Check:**
  1. 在什么情况下HTTP/1.1可能比HTTP/2性能更好？| In what situations might HTTP/1.1 perform better than HTTP/2?
     **答案 | Answer:** 单个大文件传输或服务器资源有限时 | When transferring single large files or when server resources are limited
  2. HTTP/3相比HTTP/2的主要性能优势是什么？| What are the main performance advantages of HTTP/3 over HTTP/2?
     **答案 | Answer:** 解决TCP队头阻塞、更快的连接建立、连接迁移支持 | Solves TCP head-of-line blocking, faster connection establishment, connection migration support
  3. 对于包含大量小资源的网页，哪个协议版本最优？| For web pages containing many small resources, which protocol version is optimal?
     **答案 | Answer:** HTTP/2或HTTP/3，因为多路复用可以并行加载多个小资源 | HTTP/2 or HTTP/3, because multiplexing can load multiple small resources in parallel
  4. 服务器推送在什么场景下最有效？| In what scenarios is server push most effective?
     **答案 | Answer:** 当服务器能准确预测客户端需要的关键资源时 | When server can accurately predict critical resources the client will need
  5. 移动应用应该优先选择哪个HTTP版本？| Which HTTP version should mobile applications prioritize?
     **答案 | Answer:** HTTP/3，因为它对网络切换和不稳定连接处理更好 | HTTP/3, because it handles network switching and unstable connections better

## 实践项目：HTTP协议性能分析工具 | Practical Project: HTTP Protocol Performance Analysis Tool

### 目标 | Objective
开发一个综合性的HTTP协议性能分析工具，能够测试和对比不同HTTP版本的性能特征，包括连接时间、传输效率、并发处理能力等关键指标。| Develop a comprehensive HTTP protocol performance analysis tool that can test and compare performance characteristics of different HTTP versions, including connection time, transmission efficiency, and concurrent processing capabilities.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. HTTP/2的多路复用如何解决HTTP/1.1的队头阻塞问题？| How does HTTP/2 multiplexing solve HTTP/1.1's head-of-line blocking problem?
   **答案 | Answer:** HTTP/2在单个TCP连接上创建多个独立的流，每个流可以独立发送和接收数据，一个流的阻塞不会影响其他流

2. 服务器推送的实现原理和性能优势是什么？| What are the implementation principles and performance advantages of server push?
   **答案 | Answer:** 服务器主动发送客户端可能需要的资源，减少往返时间，但需要准确预测客户端需求以避免浪费带宽

3. HTTP/3基于QUIC的主要性能改进有哪些？| What are the main performance improvements of HTTP/3 based on QUIC?
   **答案 | Answer:** 减少连接建立时间、解决TCP层队头阻塞、支持连接迁移、内置加密

### 步骤 | Steps
1. **项目结构设计**：创建模块化的性能测试框架 | **Project Structure Design**: Create modular performance testing framework
2. **协议客户端实现**：实现HTTP/1.1和HTTP/2客户端测试工具 | **Protocol Client Implementation**: Implement HTTP/1.1 and HTTP/2 client testing tools  
3. **性能指标收集**：设计并实现关键性能指标的测量机制 | **Performance Metrics Collection**: Design and implement measurement mechanisms for key performance indicators
4. **对比分析功能**：开发协议版本间的性能对比分析 | **Comparative Analysis Features**: Develop performance comparison analysis between protocol versions
5. **报告生成系统**：创建详细的性能分析报告生成器 | **Report Generation System**: Create detailed performance analysis report generator

### 示例代码 | Example Code
```rust
/*
HTTP协议性能分析工具 | HTTP Protocol Performance Analysis Tool
本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- HTTP/1.1与HTTP/2性能对比 | HTTP/1.1 vs HTTP/2 performance comparison
- 多路复用效率测试 | Multiplexing efficiency testing  
- 连接建立时间测量 | Connection establishment time measurement
- 并发请求处理分析 | Concurrent request processing analysis
*/

use hyper::{Body, Client, Method, Request, Response, Uri};
use hyper::client::HttpConnector;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::timeout;
use serde::{Deserialize, Serialize};

// 性能测试配置 | Performance test configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub target_url: String,
    pub concurrent_requests: usize,
    pub test_duration_seconds: u64,
    pub request_timeout_seconds: u64,
    pub test_endpoints: Vec<String>,
}

// 性能测试结果 | Performance test results
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub protocol_version: String,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub min_response_time_ms: f64,
    pub max_response_time_ms: f64,
    pub requests_per_second: f64,
    pub connection_time_ms: f64,
    pub first_byte_time_ms: f64,
    pub transfer_time_ms: f64,
    pub total_bytes_transferred: u64,
}

// HTTP协议性能分析器 | HTTP protocol performance analyzer
pub struct HttpPerformanceAnalyzer {
    config: TestConfig,
    http1_client: Client<HttpConnector>,
    http2_client: Client<HttpConnector>,
}

impl HttpPerformanceAnalyzer {
    pub fn new(config: TestConfig) -> Self {
        // 创建HTTP/1.1客户端 | Create HTTP/1.1 client
        let http1_connector = HttpConnector::new();
        let http1_client = Client::builder()
            .http1_title_case_headers(true)
            .http1_preserve_header_case(true)
            .build(http1_connector);
        
        // 创建HTTP/2客户端 | Create HTTP/2 client  
        let http2_connector = HttpConnector::new();
        let http2_client = Client::builder()
            .http2_only(true)
            .http2_initial_stream_window_size(65535)
            .http2_initial_connection_window_size(1048576)
            .build(http2_connector);
        
        Self {
            config,
            http1_client,
            http2_client,
        }
    }
    
    // 执行HTTP/1.1性能测试 | Execute HTTP/1.1 performance test
    pub async fn test_http1_performance(&self) -> Result<PerformanceMetrics, Box<dyn std::error::Error>> {
        println!("开始HTTP/1.1性能测试...");
        println!("Starting HTTP/1.1 performance test...");
        
        let start_time = Instant::now();
        let mut response_times = Vec::new();
        let mut successful_requests = 0u64;
        let mut failed_requests = 0u64;
        let mut total_bytes = 0u64;
        
        // 测量连接建立时间 | Measure connection establishment time
        let connection_start = Instant::now();
        let uri: Uri = self.config.target_url.parse()?;
        let req = Request::builder()
            .method(Method::GET)
            .uri(&uri)
            .header("User-Agent", "HTTP-Performance-Analyzer/1.0")
            .body(Body::empty())?;
        
        let connection_response = timeout(
            Duration::from_secs(self.config.request_timeout_seconds),
            self.http1_client.request(req)
        ).await;
        
        let connection_time = connection_start.elapsed().as_millis() as f64;
        
        match connection_response {
            Ok(Ok(_)) => {
                println!("HTTP/1.1连接建立时间: {:.2}ms", connection_time);
                println!("HTTP/1.1 connection establishment time: {:.2}ms", connection_time);
            }
            _ => {
                println!("HTTP/1.1连接失败");
                println!("HTTP/1.1 connection failed");
                return Err("Connection failed".into());
            }
        }
        
        // 并发请求测试 | Concurrent request testing
        let test_duration = Duration::from_secs(self.config.test_duration_seconds);
        let mut tasks = Vec::new();
        
        for i in 0..self.config.concurrent_requests {
            let client = self.http1_client.clone();
            let url = self.config.target_url.clone();
            let timeout_duration = Duration::from_secs(self.config.request_timeout_seconds);
            
            let task = tokio::spawn(async move {
                let mut local_response_times = Vec::new();
                let mut local_successful = 0u64;
                let mut local_failed = 0u64;
                let mut local_bytes = 0u64;
                
                let task_start = Instant::now();
                while task_start.elapsed() < test_duration {
                    let request_start = Instant::now();
                    let uri: Uri = url.parse().unwrap();
                    let req = Request::builder()
                        .method(Method::GET)
                        .uri(&uri)
                        .header("User-Agent", format!("HTTP-Performance-Analyzer-{}/1.0", i))
                        .body(Body::empty())
                        .unwrap();
                    
                    match timeout(timeout_duration, client.request(req)).await {
                        Ok(Ok(response)) => {
                            let response_time = request_start.elapsed().as_millis() as f64;
                            local_response_times.push(response_time);
                            local_successful += 1;
                            
                            // 读取响应体以测量传输时间 | Read response body to measure transfer time
                            match hyper::body::to_bytes(response.into_body()).await {
                                Ok(bytes) => {
                                    local_bytes += bytes.len() as u64;
                                }
                                Err(_) => local_failed += 1,
                            }
                        }
                        _ => {
                            local_failed += 1;
                        }
                    }
                    
                    // 短暂延迟以模拟真实使用场景 | Brief delay to simulate real usage scenarios
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                
                (local_response_times, local_successful, local_failed, local_bytes)
            });
            
            tasks.push(task);
        }
        
        // 收集所有任务结果 | Collect all task results
        for task in tasks {
            match task.await {
                Ok((times, success, failed, bytes)) => {
                    response_times.extend(times);
                    successful_requests += success;
                    failed_requests += failed;
                    total_bytes += bytes;
                }
                Err(e) => {
                    println!("任务执行错误: {}", e);
                    println!("Task execution error: {}", e);
                }
            }
        }
        
        // 计算性能指标 | Calculate performance metrics
        let total_time = start_time.elapsed().as_secs_f64();
        let total_requests = successful_requests + failed_requests;
        
        let (avg_time, min_time, max_time) = if !response_times.is_empty() {
            let avg = response_times.iter().sum::<f64>() / response_times.len() as f64;
            let min = response_times.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = response_times.iter().fold(0.0, |a, &b| a.max(b));
            (avg, min, max)
        } else {
            (0.0, 0.0, 0.0)
        };
        
        Ok(PerformanceMetrics {
            protocol_version: "HTTP/1.1".to_string(),
            total_requests,
            successful_requests,
            failed_requests,
            average_response_time_ms: avg_time,
            min_response_time_ms: min_time,
            max_response_time_ms: max_time,
            requests_per_second: if total_time > 0.0 { successful_requests as f64 / total_time } else { 0.0 },
            connection_time_ms: connection_time,
            first_byte_time_ms: min_time,
            transfer_time_ms: avg_time - min_time,
            total_bytes_transferred: total_bytes,
        })
    }
    
    // 执行HTTP/2性能测试 | Execute HTTP/2 performance test
    pub async fn test_http2_performance(&self) -> Result<PerformanceMetrics, Box<dyn std::error::Error>> {
        println!("开始HTTP/2性能测试...");
        println!("Starting HTTP/2 performance test...");
        
        let start_time = Instant::now();
        let mut response_times = Vec::new();
        let mut successful_requests = 0u64;
        let mut failed_requests = 0u64;
        let mut total_bytes = 0u64;
        
        // 测量连接建立时间（HTTP/2） | Measure connection establishment time (HTTP/2)
        let connection_start = Instant::now();
        let uri: Uri = self.config.target_url.parse()?;
        let req = Request::builder()
            .method(Method::GET)
            .uri(&uri)
            .header("User-Agent", "HTTP-Performance-Analyzer/2.0")
            .body(Body::empty())?;
        
        let connection_response = timeout(
            Duration::from_secs(self.config.request_timeout_seconds),
            self.http2_client.request(req)
        ).await;
        
        let connection_time = connection_start.elapsed().as_millis() as f64;
        
        match connection_response {
            Ok(Ok(_)) => {
                println!("HTTP/2连接建立时间: {:.2}ms", connection_time);
                println!("HTTP/2 connection establishment time: {:.2}ms", connection_time);
            }
            _ => {
                println!("HTTP/2连接失败，回退到HTTP/1.1测试");
                println!("HTTP/2 connection failed, falling back to HTTP/1.1 test");
                return self.test_http1_performance().await;
            }
        }
        
        // HTTP/2多路复用并发测试 | HTTP/2 multiplexing concurrent test
        let test_duration = Duration::from_secs(self.config.test_duration_seconds);
        let mut tasks = Vec::new();
        
        // HTTP/2的优势：可以在单个连接上进行更多并发请求
        // HTTP/2 advantage: can handle more concurrent requests on single connection
        for i in 0..self.config.concurrent_requests {
            let client = self.http2_client.clone();
            let url = self.config.target_url.clone();
            let timeout_duration = Duration::from_secs(self.config.request_timeout_seconds);
            
            let task = tokio::spawn(async move {
                let mut local_response_times = Vec::new();
                let mut local_successful = 0u64;
                let mut local_failed = 0u64;
                let mut local_bytes = 0u64;
                
                let task_start = Instant::now();
                while task_start.elapsed() < test_duration {
                    let request_start = Instant::now();
                    let uri: Uri = url.parse().unwrap();
                    let req = Request::builder()
                        .method(Method::GET)
                        .uri(&uri)
                        .header("User-Agent", format!("HTTP-Performance-Analyzer-H2-{}/2.0", i))
                        .body(Body::empty())
                        .unwrap();
                    
                    match timeout(timeout_duration, client.request(req)).await {
                        Ok(Ok(response)) => {
                            let response_time = request_start.elapsed().as_millis() as f64;
                            local_response_times.push(response_time);
                            local_successful += 1;
                            
                            match hyper::body::to_bytes(response.into_body()).await {
                                Ok(bytes) => {
                                    local_bytes += bytes.len() as u64;
                                }
                                Err(_) => local_failed += 1,
                            }
                        }
                        _ => {
                            local_failed += 1;
                        }
                    }
                    
                    // HTTP/2可以处理更快的请求频率 | HTTP/2 can handle faster request frequency
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
                
                (local_response_times, local_successful, local_failed, local_bytes)
            });
            
            tasks.push(task);
        }
        
        // 收集结果 | Collect results
        for task in tasks {
            match task.await {
                Ok((times, success, failed, bytes)) => {
                    response_times.extend(times);
                    successful_requests += success;
                    failed_requests += failed;
                    total_bytes += bytes;
                }
                Err(e) => {
                    println!("HTTP/2任务执行错误: {}", e);
                    println!("HTTP/2 task execution error: {}", e);
                }
            }
        }
        
        // 计算HTTP/2性能指标 | Calculate HTTP/2 performance metrics
        let total_time = start_time.elapsed().as_secs_f64();
        let total_requests = successful_requests + failed_requests;
        
        let (avg_time, min_time, max_time) = if !response_times.is_empty() {
            let avg = response_times.iter().sum::<f64>() / response_times.len() as f64;
            let min = response_times.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = response_times.iter().fold(0.0, |a, &b| a.max(b));
            (avg, min, max)
        } else {
            (0.0, 0.0, 0.0)
        };
        
        Ok(PerformanceMetrics {
            protocol_version: "HTTP/2".to_string(),
            total_requests,
            successful_requests,
            failed_requests,
            average_response_time_ms: avg_time,
            min_response_time_ms: min_time,
            max_response_time_ms: max_time,
            requests_per_second: if total_time > 0.0 { successful_requests as f64 / total_time } else { 0.0 },
            connection_time_ms: connection_time,
            first_byte_time_ms: min_time,
            transfer_time_ms: avg_time - min_time,
            total_bytes_transferred: total_bytes,
        })
    }
    
    // 对比分析两个协议的性能 | Compare performance between two protocols
    pub fn compare_performance(&self, http1_metrics: &PerformanceMetrics, http2_metrics: &PerformanceMetrics) {
        println!("\n=== HTTP协议性能对比分析 ===");
        println!("=== HTTP Protocol Performance Comparison Analysis ===\n");
        
        println!("📊 基础统计 | Basic Statistics:");
        println!("┌─────────────────────────────────┬─────────────┬─────────────┬─────────────┐");
        println!("│ 指标 | Metric                   │ HTTP/1.1    │ HTTP/2      │ 改进 | Improv │");
        println!("├─────────────────────────────────┼─────────────┼─────────────┼─────────────┤");
        println!("│ 总请求数 | Total Requests        │ {:11} │ {:11} │ {:>11} │", 
                 http1_metrics.total_requests, 
                 http2_metrics.total_requests,
                 if http2_metrics.total_requests > http1_metrics.total_requests { "↗" } else { "↘" });
        println!("│ 成功请求数 | Successful Requests  │ {:11} │ {:11} │ {:>11} │", 
                 http1_metrics.successful_requests, 
                 http2_metrics.successful_requests,
                 if http2_metrics.successful_requests > http1_metrics.successful_requests { "↗" } else { "↘" });
        println!("│ RPS | Requests Per Second      │ {:11.2} │ {:11.2} │ {:>10.1}% │",
                 http1_metrics.requests_per_second,
                 http2_metrics.requests_per_second,
                 ((http2_metrics.requests_per_second - http1_metrics.requests_per_second) / http1_metrics.requests_per_second) * 100.0);
        println!("└─────────────────────────────────┴─────────────┴─────────────┴─────────────┘");
        
        println!("\n⏱️ 延迟分析 | Latency Analysis:");
        println!("┌─────────────────────────────────┬─────────────┬─────────────┬─────────────┐");
        println!("│ 延迟指标 | Latency Metric        │ HTTP/1.1    │ HTTP/2      │ 改进 | Improv │");
        println!("├─────────────────────────────────┼─────────────┼─────────────┼─────────────┤");
        println!("│ 连接时间 | Connection Time (ms)  │ {:11.2} │ {:11.2} │ {:>10.1}% │",
                 http1_metrics.connection_time_ms,
                 http2_metrics.connection_time_ms,
                 ((http1_metrics.connection_time_ms - http2_metrics.connection_time_ms) / http1_metrics.connection_time_ms) * 100.0);
        println!("│ 平均响应 | Average Response (ms) │ {:11.2} │ {:11.2} │ {:>10.1}% │",
                 http1_metrics.average_response_time_ms,
                 http2_metrics.average_response_time_ms,
                 ((http1_metrics.average_response_time_ms - http2_metrics.average_response_time_ms) / http1_metrics.average_response_time_ms) * 100.0);
        println!("│ 最小响应 | Min Response (ms)     │ {:11.2} │ {:11.2} │ {:>10.1}% │",
                 http1_metrics.min_response_time_ms,
                 http2_metrics.min_response_time_ms,
                 ((http1_metrics.min_response_time_ms - http2_metrics.min_response_time_ms) / http1_metrics.min_response_time_ms) * 100.0);
        println!("│ 最大响应 | Max Response (ms)     │ {:11.2} │ {:11.2} │ {:>10.1}% │",
                 http1_metrics.max_response_time_ms,
                 http2_metrics.max_response_time_ms,
                 ((http1_metrics.max_response_time_ms - http2_metrics.max_response_time_ms) / http1_metrics.max_response_time_ms) * 100.0);
        println!("└─────────────────────────────────┴─────────────┴─────────────┴─────────────┘");
        
        println!("\n📈 传输效率 | Transfer Efficiency:");
        println!("┌─────────────────────────────────┬─────────────┬─────────────┬─────────────┐");
        println!("│ 效率指标 | Efficiency Metric     │ HTTP/1.1    │ HTTP/2      │ 改进 | Improv │");
        println!("├─────────────────────────────────┼─────────────┼─────────────┼─────────────┤");
        println!("│ 传输字节 | Bytes Transferred     │ {:11} │ {:11} │ {:>11} │",
                 http1_metrics.total_bytes_transferred,
                 http2_metrics.total_bytes_transferred,
                 if http2_metrics.total_bytes_transferred > http1_metrics.total_bytes_transferred { "↗" } else { "↘" });
        
        let http1_throughput = if http1_metrics.average_response_time_ms > 0.0 {
            http1_metrics.total_bytes_transferred as f64 / (http1_metrics.average_response_time_ms / 1000.0)
        } else { 0.0 };
        let http2_throughput = if http2_metrics.average_response_time_ms > 0.0 {
            http2_metrics.total_bytes_transferred as f64 / (http2_metrics.average_response_time_ms / 1000.0)
        } else { 0.0 };
        
        println!("│ 吞吐量 | Throughput (bytes/s)   │ {:11.0} │ {:11.0} │ {:>10.1}% │",
                 http1_throughput,
                 http2_throughput,
                 if http1_throughput > 0.0 { ((http2_throughput - http1_throughput) / http1_throughput) * 100.0 } else { 0.0 });
        println!("└─────────────────────────────────┴─────────────┴─────────────┴─────────────┘");
        
        println!("\n🎯 性能建议 | Performance Recommendations:");
        
        if http2_metrics.requests_per_second > http1_metrics.requests_per_second * 1.2 {
            println!("✅ HTTP/2在并发处理方面表现出色，建议在生产环境使用");
            println!("✅ HTTP/2 excels in concurrent processing, recommended for production use");
        }
        
        if http2_metrics.average_response_time_ms < http1_metrics.average_response_time_ms * 0.8 {
            println!("✅ HTTP/2响应时间明显更优，有效提升用户体验");
            println!("✅ HTTP/2 response time significantly better, effectively improves user experience");
        }
        
        if http1_metrics.connection_time_ms < http2_metrics.connection_time_ms {
            println!("⚠️  HTTP/1.1连接建立更快，但HTTP/2的多路复用优势会在后续请求中体现");
            println!("⚠️  HTTP/1.1 connection establishment is faster, but HTTP/2 multiplexing advantage shows in subsequent requests");
        }
        
        println!("\n📋 测试总结 | Test Summary:");
        println!("此次测试展示了HTTP/2相对于HTTP/1.1的核心优势：");
        println!("This test demonstrates HTTP/2's core advantages over HTTP/1.1:");
        println!("• 多路复用减少了队头阻塞 | Multiplexing reduces head-of-line blocking");
        println!("• 头部压缩节省了带宽 | Header compression saves bandwidth");
        println!("• 更高的并发处理能力 | Higher concurrent processing capability");
        println!("• 服务器推送潜力（未在此测试中体现）| Server push potential (not demonstrated in this test)");
    }
}

// 主测试函数 | Main test function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 HTTP协议性能分析工具启动");
    println!("🚀 HTTP Protocol Performance Analysis Tool Starting\n");
    
    // 测试配置 | Test configuration
    let config = TestConfig {
        target_url: "https://httpbin.org/json".to_string(),  // 使用公共测试API
        concurrent_requests: 10,
        test_duration_seconds: 30,
        request_timeout_seconds: 10,
        test_endpoints: vec![
            "/json".to_string(),
            "/status/200".to_string(),
            "/delay/1".to_string(),
        ],
    };
    
    println!("📋 测试配置 | Test Configuration:");
    println!("• 目标URL | Target URL: {}", config.target_url);
    println!("• 并发数 | Concurrent Requests: {}", config.concurrent_requests);
    println!("• 测试时长 | Test Duration: {}s", config.test_duration_seconds);
    println!("• 请求超时 | Request Timeout: {}s", config.request_timeout_seconds);
    println!();
    
    // 创建分析器 | Create analyzer
    let analyzer = HttpPerformanceAnalyzer::new(config);
    
    // 执行HTTP/1.1测试 | Execute HTTP/1.1 test
    println!("🔄 执行HTTP/1.1性能测试...");
    println!("🔄 Executing HTTP/1.1 performance test...");
    let http1_results = match analyzer.test_http1_performance().await {
        Ok(results) => results,
        Err(e) => {
            println!("❌ HTTP/1.1测试失败: {}", e);
            println!("❌ HTTP/1.1 test failed: {}", e);
            return Err(e);
        }
    };
    
    println!("✅ HTTP/1.1测试完成");
    println!("✅ HTTP/1.1 test completed\n");
    
    // 执行HTTP/2测试 | Execute HTTP/2 test  
    println!("🔄 执行HTTP/2性能测试...");
    println!("🔄 Executing HTTP/2 performance test...");
    let http2_results = match analyzer.test_http2_performance().await {
        Ok(results) => results,
        Err(e) => {
            println!("❌ HTTP/2测试失败: {}", e);
            println!("❌ HTTP/2 test failed: {}", e);
            return Err(e);
        }
    };
    
    println!("✅ HTTP/2测试完成");
    println!("✅ HTTP/2 test completed\n");
    
    // 性能对比分析 | Performance comparison analysis
    analyzer.compare_performance(&http1_results, &http2_results);
    
    // 保存结果到JSON文件 | Save results to JSON file
    let results = serde_json::json!({
        "test_timestamp": chrono::Utc::now().to_rfc3339(),
        "http1_metrics": http1_results,
        "http2_metrics": http2_results,
        "performance_improvement": {
            "rps_improvement_percent": ((http2_results.requests_per_second - http1_results.requests_per_second) / http1_results.requests_per_second) * 100.0,
            "latency_improvement_percent": ((http1_results.average_response_time_ms - http2_results.average_response_time_ms) / http1_results.average_response_time_ms) * 100.0,
            "connection_time_difference_ms": http2_results.connection_time_ms - http1_results.connection_time_ms,
        }
    });
    
    std::fs::write("http_performance_analysis.json", serde_json::to_string_pretty(&results)?)?;
    println!("\n💾 测试结果已保存到 http_performance_analysis.json");
    println!("💾 Test results saved to http_performance_analysis.json");
    
    Ok(())
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确实现了HTTP/1.1和HTTP/2的性能测试？| Does the project correctly implement HTTP/1.1 and HTTP/2 performance testing?
2. 多路复用的性能优势是否在测试结果中体现？| Are the performance advantages of multiplexing reflected in test results?
3. 性能对比分析是否提供了有价值的洞察？| Does the performance comparison analysis provide valuable insights?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **HTTP/2多路复用理解强化练习 | HTTP/2 Multiplexing Understanding Reinforcement Exercise**
   - **练习描述 | Exercise Description:** 实现一个可视化的HTTP/2流管理器，展示多个并发请求如何在单个连接上独立处理
   - **概念检查 | Concept Check:** 多个HTTP/2流可以在同一连接上并发处理而不相互阻塞吗？
   - **学习目标 | Learning Objective:** 深入理解多路复用机制和流独立性

2. **服务器推送策略练习 | Server Push Strategy Exercise**
   - **练习描述 | Exercise Description:** 设计一个智能的服务器推送策略，根据请求的HTML页面自动推送相关的CSS和JavaScript资源
   - **概念检查 | Concept Check:** 服务器推送需要准确预测客户端需求以避免浪费带宽吗？
   - **学习目标 | Learning Objective:** 掌握服务器推送的最佳实践和性能优化

3. **HTTP/3连接迁移练习 | HTTP/3 Connection Migration Exercise**
   - **练习描述 | Exercise Description:** 模拟移动设备在WiFi和移动网络间切换时的连接迁移过程
   - **概念检查 | Concept Check:** QUIC协议如何支持IP地址变更时保持连接状态？
   - **学习目标 | Learning Objective:** 理解QUIC的连接迁移机制和移动网络优化

4. **协议升级决策练习 | Protocol Upgrade Decision Exercise**
   - **练习描述 | Exercise Description:** 开发一个协议选择工具，根据网络条件和应用特点推荐最适合的HTTP版本
   - **概念检查 | Concept Check:** 不同HTTP版本在什么场景下各有优势？
   - **学习目标 | Learning Objective:** 培养协议选择的决策能力

5. **性能基准测试练习 | Performance Benchmarking Exercise**
   - **练习描述 | Exercise Description:** 扩展性能分析工具，支持更多测试场景和自定义测试参数
   - **概念检查 | Concept Check:** HTTP/2的性能优势在什么条件下最明显？
   - **学习目标 | Learning Objective:** 掌握全面的性能测试方法

6. **协议特性教学练习 | Protocol Features Teaching Exercise**
   - **练习描述 | Exercise Description:** 创建一个交互式演示程序，向他人展示HTTP协议版本间的差异
   - **概念检查 | Concept Check:** 能否清晰解释HTTP/2相对于HTTP/1.1的核心改进？
   - **学习目标 | Learning Objective:** 通过教学深化对协议特性的理解

7. **实际应用优化练习 | Real-world Application Optimization Exercise**
   - **练习描述 | Exercise Description:** 选择一个实际的Web应用，分析其HTTP协议使用情况并提出优化建议
   - **概念检查 | Concept Check:** 如何根据应用特点选择和配置最优的HTTP协议版本？
   - **学习目标 | Learning Objective:** 将协议知识应用到实际项目优化中

## 学习资源 | Learning Resources
- [RFC 7540 - HTTP/2](https://tools.ietf.org/html/rfc7540) - HTTP/2官方规范
- [RFC 9114 - HTTP/3](https://tools.ietf.org/html/rfc9114) - HTTP/3官方规范  
- [QUIC协议文档](https://quicwg.org/) - QUIC工作组官方资源
- [hyper库文档](https://docs.rs/hyper/) - Rust HTTP客户端和服务器库
- [HTTP/2性能最佳实践](https://developers.google.com/web/fundamentals/performance/http2) - Google开发者指南

---

✅ **完成检查清单 | Completion Checklist**
- [ ] HTTP/2协议核心特性理解（二进制、多路复用、头部压缩、服务器推送）| HTTP/2 protocol core features understanding (binary, multiplexing, header compression, server push)
- [ ] 多路复用机制和性能优势掌握 | Multiplexing mechanism and performance advantages mastery
- [ ] 服务器推送原理和应用场景理解 | Server push principles and application scenarios understanding
- [ ] HTTP/3与QUIC协议基础概念掌握 | HTTP/3 and QUIC protocol basic concepts mastery
- [ ] 协议升级机制和发现过程理解 | Protocol upgrade mechanism and discovery process understanding
- [ ] 性能分析工具项目完成 | Performance analysis tool project completion
- [ ] 所有CCQs正确回答 | All CCQs answered correctly
- [ ] 代码示例理解和运行 | Code examples understood and executed
- [ ] HTTP协议版本对比分析能力 | HTTP protocol version comparison analysis capability
- [ ] 至少完成3个扩展练习 | At least 3 extension exercises completed

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释HTTP/2的多路复用、服务器推送、HTTP/3的QUIC基础，以及协议升级机制的工作原理。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain HTTP/2 multiplexing, server push, HTTP/3 QUIC basics, and how protocol upgrade mechanisms work to others.