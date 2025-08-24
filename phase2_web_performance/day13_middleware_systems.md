# Rust入门 - 第13天：中间件系统 | Rust Introduction - Day 13: Middleware Systems

## 学习目标 | Learning Objectives
- 理解中间件的概念、作用机制和应用场景 | Understand middleware concepts, mechanisms, and application scenarios
- 掌握Tower Service trait的核心原理和实现方法 | Master Tower Service trait core principles and implementation methods
- 学会设计和开发自定义中间件组件 | Learn to design and develop custom middleware components
- 实现常用的日志记录和认证中间件功能 | Implement common logging and authentication middleware functionality
- 掌握中间件组合和链式调用的最佳实践 | Master middleware composition and chaining best practices
- 理解中间件在Web框架中的架构作用和性能影响 | Understand middleware architectural role and performance impact in web frameworks

## 详细内容 | Detailed Content

### 1. 中间件概念和作用机制 | Middleware Concepts and Mechanisms (1.5小时 | 1.5 hours)

- **中间件核心概念 | Core Middleware Concepts**
  
  **概念定义 | Concept Definition:**
  中间件是位于请求-响应流程中的软件组件，可以在请求到达处理器之前或响应返回客户端之前执行特定逻辑，如日志记录、认证、数据转换等。中间件采用洋葱模型（Onion Model），支持请求预处理和响应后处理。
  
  Middleware is a software component positioned in the request-response flow that can execute specific logic before requests reach handlers or before responses return to clients, such as logging, authentication, data transformation, etc. Middleware follows the Onion Model, supporting request preprocessing and response post-processing.
  
  **核心特征 | Key Characteristics:**
  - 请求拦截：在请求到达最终处理器前执行预处理逻辑 | Request interception: execute preprocessing logic before requests reach final handlers
  - 响应处理：在响应返回前进行后处理操作如头部添加、数据转换 | Response processing: perform post-processing operations like header addition and data transformation before response return
  - 链式组合：多个中间件可以组合成处理链，按顺序执行 | Chain composition: multiple middleware can be combined into processing chains, executing in sequence
  - 透明性：中间件对应用程序业务逻辑应该是透明的，不影响核心功能 | Transparency: middleware should be transparent to application business logic, not affecting core functionality
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 中间件是否总是在请求到达处理器之前执行？| Does middleware always execute before requests reach handlers?
     **答案 | Answer:** 否 | No - 中间件可以在请求前、响应后或两者都执行 | middleware can execute before requests, after responses, or both
  2. 中间件可以修改请求和响应内容吗？| Can middleware modify request and response content?
     **答案 | Answer:** 是 | Yes - 这是中间件的主要功能之一 | this is one of the main functions of middleware
  3. 多个中间件的执行顺序是否重要？| Is the execution order of multiple middleware important?
     **答案 | Answer:** 是 | Yes - 顺序影响处理逻辑，如认证应在授权之前 | order affects processing logic, such as authentication should come before authorization
  4. 中间件是否必须调用下一个中间件？| Must middleware call the next middleware?
     **答案 | Answer:** 否 | No - 中间件可以提前终止请求处理并直接返回响应 | middleware can terminate request processing early and return response directly
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 中间件概念演示 - 洋葱模型
  // Middleware concept demonstration - Onion Model
  use axum::{
      http::{Request, Response},
      middleware::{self, Next},
      response::IntoResponse,
      Router,
  };
  use std::time::Instant;
  
  // 日志中间件示例 - 展示请求前后处理
  // Logging middleware example - showing pre and post request processing
  async fn logging_middleware<B>(
      request: Request<B>,
      next: Next<B>,
  ) -> impl IntoResponse {
      let start = Instant::now();
      let method = request.method().clone();
      let uri = request.uri().clone();
      
      println!("📥 Request: {} {}", method, uri); // 请求预处理 | Request preprocessing
      
      let response = next.run(request).await; // 调用下一个中间件/处理器 | Call next middleware/handler
      
      let duration = start.elapsed();
      println!("📤 Response: {:?} in {:?}", response.status(), duration); // 响应后处理 | Response post-processing
      
      response
  }
  
  // 中间件应用演示
  // Middleware application demonstration
  fn create_app_with_middleware() -> Router {
      Router::new()
          .route("/", axum::routing::get(|| async { "Hello World" }))
          .layer(middleware::from_fn(logging_middleware)) // 添加日志中间件层 | Add logging middleware layer
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这个日志中间件在什么时候记录请求信息？| When does this logging middleware record request information?
    **答案 | Answer:** 在调用next.run()之前，即请求处理前 | Before calling next.run(), i.e., before request processing
  - 如果next.run()返回错误，duration还会被记录吗？| If next.run() returns an error, will duration still be recorded?
    **答案 | Answer:** 会，因为await会等待完成无论成功或失败 | Yes, because await will wait for completion regardless of success or failure
  
  **常见误区检查 | Common Misconception Checks:**
  - 中间件是否只能处理HTTP请求？| Can middleware only handle HTTP requests?
    **答案 | Answer:** 否，中间件概念适用于任何请求-响应模式 | No, middleware concept applies to any request-response pattern
  - 中间件是否会显著影响性能？| Does middleware significantly impact performance?
    **答案 | Answer:** 取决于实现，轻量级中间件影响minimal | Depends on implementation, lightweight middleware has minimal impact

- **中间件应用场景分析 | Middleware Application Scenarios**
  
  **概念定义 | Concept Definition:**
  中间件在Web应用中承担横切关注点（Cross-cutting Concerns）处理，包括安全性、日志记录、性能监控、错误处理、数据验证等非业务逻辑功能。这些功能需要在多个端点间共享，通过中间件实现代码复用和关注点分离。
  
  Middleware handles cross-cutting concerns in web applications, including security, logging, performance monitoring, error handling, data validation, and other non-business logic functions. These functions need to be shared across multiple endpoints, achieving code reuse and separation of concerns through middleware.
  
  **核心特征 | Key Characteristics:**
  - 横切关注点：处理跨越多个业务功能的通用需求 | Cross-cutting concerns: handle common needs across multiple business functions
  - 代码复用：避免在每个处理器中重复相同的逻辑 | Code reuse: avoid duplicating same logic in every handler
  - 关注点分离：将技术关注点从业务逻辑中分离出来 | Separation of concerns: separate technical concerns from business logic
  - 可配置性：中间件应支持灵活的配置和开关控制 | Configurability: middleware should support flexible configuration and toggle control
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 认证中间件应该在业务逻辑处理之前还是之后执行？| Should authentication middleware execute before or after business logic processing?
     **答案 | Answer:** 之前 | Before - 只有验证通过的请求才应该到达业务逻辑 | only validated requests should reach business logic
  2. 错误处理中间件通常放在中间件链的什么位置？| Where in the middleware chain is error handling middleware typically placed?
     **答案 | Answer:** 最外层 | Outermost layer - 以捕获所有可能的错误 | to catch all possible errors
  3. 是否所有中间件都需要访问数据库？| Do all middleware need database access?
     **答案 | Answer:** 否 | No - 如日志、CORS等可以是无状态的 | such as logging, CORS can be stateless
  4. 中间件是否可以跳过某些请求？| Can middleware skip certain requests?
     **答案 | Answer:** 是 | Yes - 基于路径、方法或其他条件进行过滤 | filter based on path, method, or other conditions
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 多场景中间件应用示例
  // Multi-scenario middleware application example
  use axum::{
      http::{HeaderMap, Method, StatusCode},
      middleware::{self, Next},
      response::Response,
      Router, Json,
  };
  use serde_json::json;
  
  // 1. CORS中间件 - 跨域资源共享
  // CORS middleware - Cross-Origin Resource Sharing
  async fn cors_middleware<B>(
      request: axum::http::Request<B>,
      next: Next<B>,
  ) -> Response {
      let method = request.method().clone();
      
      let mut response = next.run(request).await;
      let headers = response.headers_mut();
      
      // 添加CORS头部 | Add CORS headers
      headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
      headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE".parse().unwrap());
      headers.insert("Access-Control-Allow-Headers", "Content-Type, Authorization".parse().unwrap());
      
      // 处理预检请求 | Handle preflight requests
      if method == Method::OPTIONS {
          *response.status_mut() = StatusCode::OK;
      }
      
      response
  }
  
  // 2. API版本控制中间件
  // API versioning middleware
  async fn api_versioning_middleware<B>(
      mut request: axum::http::Request<B>,
      next: Next<B>,
  ) -> Result<Response, StatusCode> {
      let headers = request.headers();
      let version = headers.get("API-Version")
          .and_then(|v| v.to_str().ok())
          .unwrap_or("v1");
      
      // 检查支持的版本 | Check supported versions
      if !["v1", "v2"].contains(&version) {
          return Err(StatusCode::BAD_REQUEST);
      }
      
      // 将版本信息添加到请求扩展中 | Add version info to request extensions
      request.extensions_mut().insert(version.to_string());
      
      Ok(next.run(request).await)
  }
  
  // 3. 速率限制中间件基础结构
  // Rate limiting middleware basic structure
  use std::collections::HashMap;
  use std::sync::{Arc, Mutex};
  use std::time::{Duration, Instant};
  
  struct RateLimiter {
      requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
      max_requests: usize,
      window: Duration,
  }
  
  impl RateLimiter {
      fn new(max_requests: usize, window: Duration) -> Self {
          Self {
              requests: Arc::new(Mutex::new(HashMap::new())),
              max_requests,
              window,
          }
      }
      
      fn is_allowed(&self, client_id: &str) -> bool {
          let mut requests = self.requests.lock().unwrap();
          let now = Instant::now();
          let client_requests = requests.entry(client_id.to_string()).or_insert_with(Vec::new);
          
          // 清理过期记录 | Clean expired records
          client_requests.retain(|&time| now.duration_since(time) <= self.window);
          
          if client_requests.len() < self.max_requests {
              client_requests.push(now);
              true
          } else {
              false
          }
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - CORS中间件为什么要特殊处理OPTIONS方法？| Why does CORS middleware specially handle the OPTIONS method?
    **答案 | Answer:** OPTIONS是浏览器发送的预检请求，需要返回允许的方法和头部 | OPTIONS is a preflight request sent by browsers, needs to return allowed methods and headers
  - 速率限制中间件中的清理过期记录操作在什么时候执行？| When does the expired record cleanup operation in rate limiting middleware execute?
    **答案 | Answer:** 每次检查请求是否被允许时 | Every time checking if a request is allowed

### 2. Tower Service trait深入理解 | Deep Understanding of Tower Service trait (1.5小时 | 1.5 hours)

- **Tower Service trait架构 | Tower Service trait Architecture**
  
  **概念定义 | Concept Definition:**
  Tower Service trait是Rust异步生态系统中用于构建可组合、可重用服务的核心抽象。它定义了一个异步服务的基本接口，接受请求并返回future。Tower框架基于此trait构建了完整的中间件生态系统，支持负载均衡、重试、超时等功能。
  
  Tower Service trait is the core abstraction in Rust's async ecosystem for building composable, reusable services. It defines the basic interface for an async service that accepts requests and returns futures. The Tower framework builds a complete middleware ecosystem based on this trait, supporting load balancing, retry, timeout, and other features.
  
  **核心特征 | Key Characteristics:**
  - 异步性：所有操作都是异步的，返回Future而不是直接值 | Asynchronous: all operations are async, returning Futures instead of direct values
  - 可组合性：服务可以包装其他服务，形成服务链 | Composability: services can wrap other services, forming service chains
  - 背压处理：通过poll_ready机制处理服务负载和背压 | Backpressure handling: handle service load and backpressure through poll_ready mechanism
  - 类型安全：通过泛型确保请求和响应类型的一致性 | Type safety: ensure consistency of request and response types through generics
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Service trait的call方法是否立即返回结果？| Does the Service trait's call method immediately return results?
     **答案 | Answer:** 否 | No - 返回Future，需要await才能获得结果 | returns Future, needs await to get results
  2. poll_ready方法的作用是什么？| What is the purpose of the poll_ready method?
     **答案 | Answer:** 检查服务是否准备好处理下一个请求 | Check if service is ready to handle the next request
  3. 一个Service可以包装另一个Service吗？| Can one Service wrap another Service?
     **答案 | Answer:** 是 | Yes - 这是中间件实现的基础 | this is the foundation of middleware implementation
  4. Service trait是否要求实现Clone？| Does the Service trait require implementing Clone?
     **答案 | Answer:** 否 | No - 但很多应用场景中需要Clone以支持并发 | but many application scenarios need Clone for concurrency support
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // Tower Service trait实现示例
  // Tower Service trait implementation example
  use tower::{Service, ServiceExt, Layer};
  use std::task::{Context, Poll};
  use std::pin::Pin;
  use std::future::Future;
  use std::convert::Infallible;
  
  // 1. 基础Service实现 - 简单的echo服务
  // Basic Service implementation - simple echo service
  #[derive(Clone)]
  struct EchoService;
  
  impl<Request> Service<Request> for EchoService 
  where
      Request: Clone + Send + 'static,
  {
      type Response = Request;
      type Error = Infallible;
      type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
      
      fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
          Poll::Ready(Ok(())) // 总是准备就绪 | Always ready
      }
      
      fn call(&mut self, request: Request) -> Self::Future {
          let response = request.clone();
          Box::pin(async move { Ok(response) })
      }
  }
  
  // 2. Service包装器 - 添加日志功能
  // Service wrapper - add logging functionality
  #[derive(Clone)]
  struct LoggingService<S> {
      inner: S,
      service_name: String,
  }
  
  impl<S, Request> Service<Request> for LoggingService<S>
  where
      S: Service<Request> + Clone,
      Request: std::fmt::Debug,
      S::Response: std::fmt::Debug,
  {
      type Response = S::Response;
      type Error = S::Error;
      type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
      
      fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
          self.inner.poll_ready(cx) // 委托给内部服务 | Delegate to inner service
      }
      
      fn call(&mut self, request: Request) -> Self::Future {
          println!("🔍 [{}] Processing request: {:?}", self.service_name, request);
          let future = self.inner.call(request);
          let service_name = self.service_name.clone();
          
          Box::pin(async move {
              let response = future.await;
              match &response {
                  Ok(resp) => println!("✅ [{}] Success: {:?}", service_name, resp),
                  Err(_) => println!("❌ [{}] Error occurred", service_name),
              }
              response
          })
      }
  }
  
  // 3. Layer trait实现 - 用于构建服务栈
  // Layer trait implementation - for building service stacks
  #[derive(Clone)]
  struct LoggingLayer {
      service_name: String,
  }
  
  impl LoggingLayer {
      fn new(service_name: impl Into<String>) -> Self {
          Self {
              service_name: service_name.into(),
          }
      }
  }
  
  impl<S> Layer<S> for LoggingLayer {
      type Service = LoggingService<S>;
      
      fn layer(&self, service: S) -> Self::Service {
          LoggingService {
              inner: service,
              service_name: self.service_name.clone(),
          }
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - LoggingService如何确保内部服务准备就绪？| How does LoggingService ensure the inner service is ready?
    **答案 | Answer:** 通过poll_ready方法委托给内部服务检查 | Delegate to inner service check through poll_ready method
  - Layer trait与Service trait的区别是什么？| What's the difference between Layer trait and Service trait?
    **答案 | Answer:** Layer用于构建Service，Service用于处理请求 | Layer is used to build Services, Service is used to handle requests

- **自定义Service开发实践 | Custom Service Development Practice**
  
  **概念定义 | Concept Definition:**
  自定义Service开发涉及实现Service trait的完整接口，包括正确处理背压、错误传播、资源管理等。开发者需要理解异步编程模型，合理使用Poll机制，确保服务的并发安全性和性能。
  
  Custom Service development involves implementing the complete Service trait interface, including proper handling of backpressure, error propagation, resource management, etc. Developers need to understand the async programming model, properly use the Poll mechanism, and ensure service concurrency safety and performance.
  
  **核心特征 | Key Characteristics:**
  - 状态管理：正确管理服务内部状态，避免竞态条件 | State management: properly manage service internal state, avoid race conditions
  - 错误处理：合适的错误类型定义和传播机制 | Error handling: appropriate error type definition and propagation mechanism
  - 资源管理：及时释放资源，避免内存泄漏 | Resource management: timely resource release, avoid memory leaks
  - 性能优化：minimize allocations，避免不必要的克隆操作 | Performance optimization: minimize allocations, avoid unnecessary clone operations
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 自定义Service是否必须支持Clone？| Must custom Services support Clone?
     **答案 | Answer:** 取决于使用场景 | Depends on usage scenarios - 并发场景通常需要 | concurrent scenarios usually require it
  2. poll_ready返回Pending时会发生什么？| What happens when poll_ready returns Pending?
     **答案 | Answer:** 调用者会稍后重新轮询，不会调用call | caller will poll again later, won't call call
  3. Service的Future是否需要实现Send？| Does the Service's Future need to implement Send?
     **答案 | Answer:** 通常需要 | Usually yes - 以支持跨任务传递 | to support cross-task transfer
  4. 在call方法中访问&mut self是否线程安全？| Is accessing &mut self in the call method thread-safe?
     **答案 | Answer:** 不是 | No - 需要额外的同步机制 | requires additional synchronization mechanisms
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 高级自定义Service实现
  // Advanced custom Service implementation
  use tower::Service;
  use std::task::{Context, Poll};
  use std::pin::Pin;
  use std::future::Future;
  use std::sync::{Arc, Mutex};
  use std::collections::VecDeque;
  use std::time::{Duration, Instant};
  
  // 1. 带缓存的Service实现
  // Service implementation with caching
  #[derive(Clone)]
  struct CachingService<S, K, V> 
  where
      K: Clone + Eq + std::hash::Hash,
      V: Clone,
  {
      inner: S,
      cache: Arc<Mutex<std::collections::HashMap<K, (V, Instant)>>>,
      ttl: Duration,
  }
  
  impl<S, K, V> CachingService<S, K, V>
  where
      K: Clone + Eq + std::hash::Hash,
      V: Clone,
  {
      fn new(inner: S, ttl: Duration) -> Self {
          Self {
              inner,
              cache: Arc::new(Mutex::new(std::collections::HashMap::new())),
              ttl,
          }
      }
      
      fn get_from_cache(&self, key: &K) -> Option<V> {
          let mut cache = self.cache.lock().unwrap();
          if let Some((value, timestamp)) = cache.get(key) {
              if timestamp.elapsed() < self.ttl {
                  return Some(value.clone());
              } else {
                  cache.remove(key); // 清理过期条目 | Clean expired entries
              }
          }
          None
      }
      
      fn store_in_cache(&self, key: K, value: V) {
          let mut cache = self.cache.lock().unwrap();
          cache.insert(key, (value, Instant::now()));
      }
  }
  
  // 为String请求实现缓存Service
  // Implement caching Service for String requests
  impl<S> Service<String> for CachingService<S, String, String>
  where
      S: Service<String, Response = String> + Clone,
      S::Error: std::fmt::Debug,
  {
      type Response = String;
      type Error = S::Error;
      type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
      
      fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
          self.inner.poll_ready(cx)
      }
      
      fn call(&mut self, request: String) -> Self::Future {
          // 检查缓存 | Check cache
          if let Some(cached_response) = self.get_from_cache(&request) {
              println!("💾 Cache hit for: {}", request);
              return Box::pin(async move { Ok(cached_response) });
          }
          
          println!("🔄 Cache miss, calling inner service for: {}", request);
          let future = self.inner.call(request.clone());
          let cache_handle = self.cache.clone();
          let ttl = self.ttl;
          
          Box::pin(async move {
              match future.await {
                  Ok(response) => {
                      // 存储到缓存 | Store in cache
                      let mut cache = cache_handle.lock().unwrap();
                      cache.insert(request, (response.clone(), Instant::now()));
                      Ok(response)
                  },
                  Err(error) => Err(error),
              }
          })
      }
  }
  
  // 2. 批处理Service实现
  // Batch processing Service implementation
  struct BatchingService<S> {
      inner: S,
      pending_requests: VecDeque<String>,
      batch_size: usize,
      timer: Option<Instant>,
      timeout: Duration,
  }
  
  impl<S> BatchingService<S> {
      fn new(inner: S, batch_size: usize, timeout: Duration) -> Self {
          Self {
              inner,
              pending_requests: VecDeque::new(),
              batch_size,
              timer: None,
              timeout,
          }
      }
      
      fn should_flush(&self) -> bool {
          self.pending_requests.len() >= self.batch_size ||
          self.timer.map(|t| t.elapsed() >= self.timeout).unwrap_or(false)
      }
  }
  
  // 使用示例和测试
  // Usage example and testing
  #[tokio::main]
  async fn demonstrate_custom_services() -> Result<(), Box<dyn std::error::Error>> {
      use tower::ServiceExt;
      
      // 创建基础echo服务 | Create basic echo service
      let base_service = EchoService;
      
      // 添加缓存层 | Add caching layer
      let cached_service = CachingService::new(base_service.clone(), Duration::from_secs(60));
      
      // 添加日志层 | Add logging layer
      let logged_service = LoggingService {
          inner: cached_service,
          service_name: "CachedEcho".to_string(),
      };
      
      // 测试服务调用 | Test service calls
      let mut service = logged_service;
      
      // 第一次调用 - 缓存未命中 | First call - cache miss
      let response1 = service.ready().await?.call("test".to_string()).await?;
      println!("Response 1: {}", response1);
      
      // 第二次调用 - 缓存命中 | Second call - cache hit
      let response2 = service.ready().await?.call("test".to_string()).await?;
      println!("Response 2: {}", response2);
      
      Ok(())
  }
  ```

### 3. 自定义中间件开发最佳实践 | Custom Middleware Development Best Practices (1.5小时 | 1.5 hours)

- **中间件设计模式 | Middleware Design Patterns**
  
  **概念定义 | Concept Definition:**
  中间件设计模式包括装饰器模式、职责链模式和策略模式等。装饰器模式用于为现有服务添加功能，职责链模式用于构建中间件链，策略模式用于实现可配置的中间件行为。良好的中间件设计应遵循单一职责原则，提供清晰的配置接口。
  
  Middleware design patterns include Decorator Pattern, Chain of Responsibility Pattern, and Strategy Pattern. Decorator pattern is used to add functionality to existing services, Chain of Responsibility pattern is used to build middleware chains, and Strategy pattern is used to implement configurable middleware behavior. Good middleware design should follow the Single Responsibility Principle and provide clear configuration interfaces.
  
  **核心特征 | Key Characteristics:**
  - 单一职责：每个中间件只负责一个特定的横切关注点 | Single responsibility: each middleware handles only one specific cross-cutting concern
  - 可配置性：支持灵活的配置选项和开关控制 | Configurability: support flexible configuration options and toggle controls
  - 可组合性：中间件之间可以任意组合而不产生冲突 | Composability: middleware can be combined arbitrarily without conflicts
  - 性能优化：最小化运行时开销和内存占用 | Performance optimization: minimize runtime overhead and memory usage
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 一个中间件是否应该处理多种不同类型的关注点？| Should one middleware handle multiple different types of concerns?
     **答案 | Answer:** 否 | No - 应遵循单一职责原则 | should follow Single Responsibility Principle
  2. 中间件的配置信息应该如何传递？| How should middleware configuration information be passed?
     **答案 | Answer:** 通过构造函数参数或配置结构体 | Through constructor parameters or configuration structs
  3. 中间件是否可以有条件地跳过处理？| Can middleware conditionally skip processing?
     **答案 | Answer:** 是 | Yes - 基于请求特征或配置条件 | based on request characteristics or configuration conditions
  4. 多个相同类型的中间件可以同时使用吗？| Can multiple middleware of the same type be used simultaneously?
     **答案 | Answer:** 是 | Yes - 如多个不同规则的验证中间件 | such as multiple validation middleware with different rules
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 可配置中间件设计模式实现
  // Configurable middleware design pattern implementation
  use axum::{
      http::{Request, StatusCode, HeaderMap, HeaderValue},
      middleware::Next,
      response::{IntoResponse, Response},
      Json,
  };
  use serde::{Deserialize, Serialize};
  use std::collections::HashSet;
  
  // 1. 可配置的CORS中间件
  // Configurable CORS middleware
  #[derive(Clone)]
  pub struct CorsConfig {
      pub allowed_origins: Vec<String>,
      pub allowed_methods: Vec<String>,
      pub allowed_headers: Vec<String>,
      pub expose_headers: Vec<String>,
      pub max_age: Option<u32>,
      pub allow_credentials: bool,
  }
  
  impl Default for CorsConfig {
      fn default() -> Self {
          Self {
              allowed_origins: vec!["*".to_string()],
              allowed_methods: vec!["GET".to_string(), "POST".to_string()],
              allowed_headers: vec!["Content-Type".to_string()],
              expose_headers: vec![],
              max_age: Some(3600),
              allow_credentials: false,
          }
      }
  }
  
  impl CorsConfig {
      pub fn new() -> Self {
          Self::default()
      }
      
      pub fn allow_origin(mut self, origin: impl Into<String>) -> Self {
          if self.allowed_origins == vec!["*"] {
              self.allowed_origins.clear();
          }
          self.allowed_origins.push(origin.into());
          self
      }
      
      pub fn allow_methods(mut self, methods: Vec<impl Into<String>>) -> Self {
          self.allowed_methods = methods.into_iter().map(|m| m.into()).collect();
          self
      }
      
      pub fn allow_credentials(mut self, allow: bool) -> Self {
          self.allow_credentials = allow;
          self
      }
  }
  
  // CORS中间件实现
  // CORS middleware implementation
  pub async fn cors_middleware<B>(
      config: CorsConfig,
      request: Request<B>,
      next: Next<B>,
  ) -> Response {
      let origin = request.headers().get("origin")
          .and_then(|v| v.to_str().ok())
          .unwrap_or("");
      
      let method = request.method().as_str();
      
      // 检查Origin是否被允许 | Check if Origin is allowed
      let origin_allowed = config.allowed_origins.contains(&"*".to_string()) ||
          config.allowed_origins.contains(&origin.to_string());
      
      let mut response = if origin_allowed {
          next.run(request).await
      } else {
          StatusCode::FORBIDDEN.into_response()
      };
      
      // 添加CORS头部 | Add CORS headers
      let headers = response.headers_mut();
      
      if origin_allowed {
          if config.allowed_origins.contains(&"*".to_string()) && !config.allow_credentials {
              headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
          } else {
              headers.insert("Access-Control-Allow-Origin", origin.parse().unwrap_or(HeaderValue::from_static("")));
          }
          
          headers.insert(
              "Access-Control-Allow-Methods",
              config.allowed_methods.join(", ").parse().unwrap()
          );
          
          headers.insert(
              "Access-Control-Allow-Headers", 
              config.allowed_headers.join(", ").parse().unwrap()
          );
          
          if config.allow_credentials {
              headers.insert("Access-Control-Allow-Credentials", "true".parse().unwrap());
          }
          
          if let Some(max_age) = config.max_age {
              headers.insert("Access-Control-Max-Age", max_age.to_string().parse().unwrap());
          }
      }
      
      response
  }
  
  // 2. 条件中间件包装器 - 装饰器模式
  // Conditional middleware wrapper - Decorator pattern
  pub struct ConditionalMiddleware<F, M> {
      condition: F,
      middleware: M,
  }
  
  impl<F, M> ConditionalMiddleware<F, M> {
      pub fn new(condition: F, middleware: M) -> Self {
          Self { condition, middleware }
      }
  }
  
  // 路径基础的条件中间件
  // Path-based conditional middleware
  pub async fn path_conditional_middleware<B>(
      path_patterns: Vec<String>,
      request: Request<B>,
      next: Next<B>,
  ) -> Response {
      let request_path = request.uri().path();
      
      // 检查路径是否匹配模式 | Check if path matches patterns
      let should_apply = path_patterns.iter().any(|pattern| {
          if pattern.ends_with("*") {
              request_path.starts_with(&pattern[..pattern.len()-1])
          } else {
              request_path == pattern
          }
      });
      
      if should_apply {
          println!("🎯 Applying middleware to path: {}", request_path);
      } else {
          println!("⏩ Skipping middleware for path: {}", request_path);
      }
      
      next.run(request).await
  }
  
  // 3. 中间件组合器 - 职责链模式
  // Middleware compositor - Chain of Responsibility pattern
  pub struct MiddlewareChain<T> {
      pub middlewares: Vec<T>,
  }
  
  impl<T> MiddlewareChain<T> {
      pub fn new() -> Self {
          Self {
              middlewares: Vec::new(),
          }
      }
      
      pub fn add(mut self, middleware: T) -> Self {
          self.middlewares.push(middleware);
          self
      }
  }
  
  // 4. 策略模式中间件 - 可插拔验证策略
  // Strategy pattern middleware - Pluggable validation strategies
  pub trait ValidationStrategy: Send + Sync {
      fn validate<B>(&self, request: &Request<B>) -> Result<(), String>;
  }
  
  // JWT验证策略
  // JWT validation strategy
  pub struct JwtValidationStrategy {
      pub secret: String,
  }
  
  impl ValidationStrategy for JwtValidationStrategy {
      fn validate<B>(&self, request: &Request<B>) -> Result<(), String> {
          let auth_header = request.headers().get("Authorization")
              .and_then(|v| v.to_str().ok())
              .ok_or("Missing Authorization header")?;
          
          if !auth_header.starts_with("Bearer ") {
              return Err("Invalid Authorization header format".to_string());
          }
          
          let token = &auth_header[7..];
          // 简化的JWT验证逻辑 | Simplified JWT validation logic
          if token.len() < 10 {
              return Err("Invalid JWT token".to_string());
          }
          
          Ok(())
      }
  }
  
  // API Key验证策略
  // API Key validation strategy
  pub struct ApiKeyValidationStrategy {
      pub valid_keys: HashSet<String>,
  }
  
  impl ValidationStrategy for ApiKeyValidationStrategy {
      fn validate<B>(&self, request: &Request<B>) -> Result<(), String> {
          let api_key = request.headers().get("X-API-Key")
              .and_then(|v| v.to_str().ok())
              .ok_or("Missing API key")?;
          
          if self.valid_keys.contains(api_key) {
              Ok(())
          } else {
              Err("Invalid API key".to_string())
          }
      }
  }
  
  // 可配置验证中间件
  // Configurable validation middleware
  pub async fn validation_middleware<B>(
      strategy: Box<dyn ValidationStrategy>,
      request: Request<B>,
      next: Next<B>,
  ) -> Result<Response, StatusCode> {
      match strategy.validate(&request) {
          Ok(()) => Ok(next.run(request).await),
          Err(error) => {
              println!("🚫 Validation failed: {}", error);
              Err(StatusCode::UNAUTHORIZED)
          }
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - CorsConfig的builder模式有什么优势？| What are the advantages of CorsConfig's builder pattern?
    **答案 | Answer:** 提供fluent API，支持链式调用，配置更直观 | Provides fluent API, supports method chaining, more intuitive configuration
  - 为什么ValidationStrategy使用trait而不是enum？| Why does ValidationStrategy use trait instead of enum?
    **答案 | Answer:** 支持运行时策略替换和用户自定义策略 | Supports runtime strategy replacement and user-defined strategies

### 4. 日志中间件深入实现 | Advanced Logging Middleware Implementation (1小时 | 1 hour)

- **结构化日志系统设计 | Structured Logging System Design**
  
  **概念定义 | Concept Definition:**
  结构化日志使用键值对或JSON格式记录信息，便于日志分析和搜索。日志中间件应支持不同级别的日志记录、请求ID跟踪、性能指标收集和敏感信息过滤。通过结构化格式，日志可以被自动化工具处理和分析。
  
  Structured logging uses key-value pairs or JSON format to record information, facilitating log analysis and search. Logging middleware should support different logging levels, request ID tracking, performance metrics collection, and sensitive information filtering. Through structured format, logs can be processed and analyzed by automated tools.
  
  **核心特征 | Key Characteristics:**
  - 结构化格式：使用JSON或键值对格式，便于机器解析 | Structured format: use JSON or key-value format for machine parsing
  - 上下文信息：包含请求ID、用户ID、会话信息等上下文 | Contextual information: include request ID, user ID, session info and other context
  - 性能指标：记录响应时间、数据库查询次数等性能数据 | Performance metrics: record response time, database query count and other performance data
  - 安全过滤：自动过滤敏感信息如密码、令牌等 | Security filtering: automatically filter sensitive information like passwords, tokens
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 结构化日志比普通文本日志有什么优势？| What advantages do structured logs have over plain text logs?
     **答案 | Answer:** 便于搜索、过滤、聚合和自动分析 | Easier to search, filter, aggregate and automatically analyze
  2. 请求ID在日志中的作用是什么？| What is the role of request ID in logs?
     **答案 | Answer:** 追踪单个请求的完整处理流程 | Track the complete processing flow of a single request
  3. 是否应该记录所有HTTP头部信息？| Should all HTTP header information be logged?
     **答案 | Answer:** 否 | No - 应过滤敏感信息如Authorization头 | should filter sensitive information like Authorization headers
  4. 日志级别应该如何选择？| How should log levels be chosen?
     **答案 | Answer:** 根据信息重要性：ERROR > WARN > INFO > DEBUG | Based on information importance: ERROR > WARN > INFO > DEBUG
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 高级结构化日志中间件实现
  // Advanced structured logging middleware implementation
  use axum::{
      extract::ConnectInfo,
      http::{Request, Method, Uri, HeaderMap},
      middleware::Next,
      response::Response,
  };
  use serde_json::{json, Value};
  use std::time::{Instant, SystemTime, UNIX_EPOCH};
  use uuid::Uuid;
  use std::net::SocketAddr;
  
  // 日志配置结构
  // Log configuration structure
  #[derive(Clone)]
  pub struct LoggingConfig {
      pub level: LogLevel,
      pub include_headers: bool,
      pub include_body: bool,
      pub sensitive_headers: Vec<String>,
      pub max_body_size: usize,
      pub enable_performance_metrics: bool,
  }
  
  #[derive(Clone, Debug, PartialEq, PartialOrd)]
  pub enum LogLevel {
      ERROR = 0,
      WARN = 1,
      INFO = 2,
      DEBUG = 3,
  }
  
  impl Default for LoggingConfig {
      fn default() -> Self {
          Self {
              level: LogLevel::INFO,
              include_headers: true,
              include_body: false,
              sensitive_headers: vec![
                  "authorization".to_string(),
                  "cookie".to_string(),
                  "x-api-key".to_string(),
              ],
              max_body_size: 1024,
              enable_performance_metrics: true,
          }
      }
  }
  
  // 请求上下文信息
  // Request context information
  #[derive(Debug)]
  pub struct RequestContext {
      pub request_id: String,
      pub timestamp: u64,
      pub method: String,
      pub uri: String,
      pub user_agent: Option<String>,
      pub client_ip: Option<String>,
      pub headers: Value,
  }
  
  impl RequestContext {
      pub fn new<B>(request: &Request<B>) -> Self {
          let request_id = Uuid::new_v4().to_string();
          let timestamp = SystemTime::now()
              .duration_since(UNIX_EPOCH)
              .unwrap()
              .as_secs();
          
          let method = request.method().to_string();
          let uri = request.uri().to_string();
          
          let user_agent = request.headers()
              .get("user-agent")
              .and_then(|v| v.to_str().ok())
              .map(|s| s.to_string());
          
          // 提取客户端IP地址
          // Extract client IP address
          let client_ip = request.extensions().get::<ConnectInfo<SocketAddr>>()
              .map(|connect_info| connect_info.0.ip().to_string());
          
          Self {
              request_id,
              timestamp,
              method,
              uri,
              user_agent,
              client_ip,
              headers: json!({}), // 稍后填充 | Fill later
          }
      }
  }
  
  // 响应信息
  // Response information
  #[derive(Debug)]
  pub struct ResponseInfo {
      pub status_code: u16,
      pub response_size: Option<usize>,
      pub duration_ms: u128,
  }
  
  // 高级日志中间件
  // Advanced logging middleware
  pub async fn advanced_logging_middleware<B>(
      config: LoggingConfig,
      mut request: Request<B>,
      next: Next<B>,
  ) -> Response 
  where
      B: axum::body::HttpBody + Send + 'static,
      B::Data: Send,
      B::Error: std::fmt::Display,
  {
      let start_time = Instant::now();
      let mut context = RequestContext::new(&request);
      
      // 过滤并记录请求头部 | Filter and record request headers
      if config.include_headers {
          context.headers = filter_sensitive_headers(&request.headers(), &config.sensitive_headers);
      }
      
      // 添加请求ID到扩展中，供下游中间件使用 | Add request ID to extensions for downstream middleware
      request.extensions_mut().insert(context.request_id.clone());
      
      // 记录请求开始 | Log request start
      log_request_start(&config, &context);
      
      // 处理请求 | Process request
      let response = next.run(request).await;
      
      // 计算处理时间 | Calculate processing time
      let duration = start_time.elapsed();
      
      // 构建响应信息 | Build response information
      let response_info = ResponseInfo {
          status_code: response.status().as_u16(),
          response_size: response.headers().get("content-length")
              .and_then(|v| v.to_str().ok())
              .and_then(|s| s.parse().ok()),
          duration_ms: duration.as_millis(),
      };
      
      // 记录请求完成 | Log request completion
      log_request_completion(&config, &context, &response_info);
      
      // 如果启用性能指标，记录详细信息 | If performance metrics enabled, log detailed info
      if config.enable_performance_metrics {
          log_performance_metrics(&context, &response_info);
      }
      
      response
  }
  
  // 过滤敏感头部信息
  // Filter sensitive header information
  fn filter_sensitive_headers(headers: &HeaderMap, sensitive_headers: &[String]) -> Value {
      let mut filtered_headers = serde_json::Map::new();
      
      for (key, value) in headers.iter() {
          let key_lower = key.as_str().to_lowercase();
          
          if sensitive_headers.contains(&key_lower) {
              filtered_headers.insert(key.as_str().to_string(), json!("[FILTERED]"));
          } else {
              if let Ok(value_str) = value.to_str() {
                  filtered_headers.insert(key.as_str().to_string(), json!(value_str));
              }
          }
      }
      
      Value::Object(filtered_headers)
  }
  
  // 记录请求开始
  // Log request start
  fn log_request_start(config: &LoggingConfig, context: &RequestContext) {
      if config.level >= LogLevel::INFO {
          let log_entry = json!({
              "event": "request_start",
              "request_id": context.request_id,
              "timestamp": context.timestamp,
              "method": context.method,
              "uri": context.uri,
              "user_agent": context.user_agent,
              "client_ip": context.client_ip,
              "headers": context.headers
          });
          
          println!("📥 {}", log_entry);
      }
  }
  
  // 记录请求完成
  // Log request completion
  fn log_request_completion(config: &LoggingConfig, context: &RequestContext, response: &ResponseInfo) {
      let log_level = match response.status_code {
          200..=299 => LogLevel::INFO,
          400..=499 => LogLevel::WARN,
          500..=599 => LogLevel::ERROR,
          _ => LogLevel::DEBUG,
      };
      
      if config.level >= log_level {
          let log_entry = json!({
              "event": "request_complete",
              "request_id": context.request_id,
              "timestamp": context.timestamp,
              "method": context.method,
              "uri": context.uri,
              "status_code": response.status_code,
              "duration_ms": response.duration_ms,
              "response_size": response.response_size,
              "client_ip": context.client_ip
          });
          
          let emoji = match response.status_code {
              200..=299 => "✅",
              400..=499 => "⚠️ ",
              500..=599 => "❌",
              _ => "📤",
          };
          
          println!("{} {}", emoji, log_entry);
      }
  }
  
  // 记录性能指标
  // Log performance metrics
  fn log_performance_metrics(context: &RequestContext, response: &ResponseInfo) {
      let metrics = json!({
          "event": "performance_metrics",
          "request_id": context.request_id,
          "endpoint": context.uri,
          "method": context.method,
          "response_time_ms": response.duration_ms,
          "status_code": response.status_code,
          "timestamp": context.timestamp
      });
      
      println!("📊 {}", metrics);
      
      // 性能警告 | Performance warnings
      if response.duration_ms > 1000 {
          let warning = json!({
              "event": "slow_request_warning",
              "request_id": context.request_id,
              "uri": context.uri,
              "duration_ms": response.duration_ms,
              "threshold_ms": 1000
          });
          println!("🐌 {}", warning);
      }
  }
  
  // 日志聚合和分析工具
  // Log aggregation and analysis tools
  pub struct LogAnalyzer {
      pub request_count: std::collections::HashMap<String, u32>,
      pub avg_response_time: std::collections::HashMap<String, f64>,
      pub error_count: std::collections::HashMap<u16, u32>,
  }
  
  impl LogAnalyzer {
      pub fn new() -> Self {
          Self {
              request_count: std::collections::HashMap::new(),
              avg_response_time: std::collections::HashMap::new(),
              error_count: std::collections::HashMap::new(),
          }
      }
      
      pub fn analyze_request(&mut self, context: &RequestContext, response: &ResponseInfo) {
          // 统计请求数量 | Count requests
          *self.request_count.entry(context.uri.clone()).or_insert(0) += 1;
          
          // 统计平均响应时间 | Count average response time
          let current_avg = *self.avg_response_time.entry(context.uri.clone()).or_insert(0.0);
          let count = *self.request_count.get(&context.uri).unwrap() as f64;
          let new_avg = (current_avg * (count - 1.0) + response.duration_ms as f64) / count;
          self.avg_response_time.insert(context.uri.clone(), new_avg);
          
          // 统计错误状态码 | Count error status codes
          if response.status_code >= 400 {
              *self.error_count.entry(response.status_code).or_insert(0) += 1;
          }
      }
      
      pub fn generate_report(&self) -> Value {
          json!({
              "total_endpoints": self.request_count.len(),
              "top_endpoints": self.get_top_endpoints(5),
              "slowest_endpoints": self.get_slowest_endpoints(5),
              "error_summary": self.error_count
          })
      }
      
      fn get_top_endpoints(&self, limit: usize) -> Vec<(String, u32)> {
          let mut endpoints: Vec<_> = self.request_count.iter().collect();
          endpoints.sort_by(|a, b| b.1.cmp(a.1));
          endpoints.into_iter()
              .take(limit)
              .map(|(k, v)| (k.clone(), *v))
              .collect()
      }
      
      fn get_slowest_endpoints(&self, limit: usize) -> Vec<(String, f64)> {
          let mut endpoints: Vec<_> = self.avg_response_time.iter().collect();
          endpoints.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
          endpoints.into_iter()
              .take(limit)
              .map(|(k, v)| (k.clone(), *v))
              .collect()
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么要过滤敏感头部信息？| Why filter sensitive header information?
    **答案 | Answer:** 防止敏感信息如令牌、密码泄露到日志中 | Prevent sensitive information like tokens and passwords from leaking into logs
  - LogAnalyzer如何计算平均响应时间？| How does LogAnalyzer calculate average response time?
    **答案 | Answer:** 使用累积平均值公式，避免存储所有历史数据 | Use cumulative average formula to avoid storing all historical data

### 5. 认证中间件实现与安全实践 | Authentication Middleware Implementation and Security Practices (1小时 | 1 hour)

- **多策略认证系统 | Multi-Strategy Authentication System**
  
  **概念定义 | Concept Definition:**
  多策略认证系统支持多种认证方式，如JWT、API Key、OAuth、Session等。系统应能够根据请求特征自动选择合适的认证策略，支持策略组合和降级，提供统一的认证结果接口。认证中间件需要处理令牌验证、用户信息提取、权限检查等功能。
  
  Multi-strategy authentication system supports various authentication methods like JWT, API Key, OAuth, Session, etc. The system should automatically select appropriate authentication strategies based on request characteristics, support strategy combination and fallback, and provide unified authentication result interfaces. Authentication middleware needs to handle token validation, user information extraction, permission checking, and other functions.
  
  **核心特征 | Key Characteristics:**
  - 多策略支持：同时支持多种认证机制，如JWT、API Key、OAuth等 | Multi-strategy support: simultaneously support multiple authentication mechanisms
  - 自动策略选择：根据请求头或路径自动选择认证策略 | Automatic strategy selection: automatically choose authentication strategies based on request headers or paths
  - 权限分层：支持角色基础访问控制（RBAC）和细粒度权限 | Permission layering: support Role-Based Access Control (RBAC) and fine-grained permissions
  - 安全最佳实践：令牌过期、刷新机制、暴力攻击防护 | Security best practices: token expiration, refresh mechanisms, brute force attack protection
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. JWT认证和Session认证的主要区别是什么？| What's the main difference between JWT authentication and Session authentication?
     **答案 | Answer:** JWT是无状态的，Session需要服务器存储状态 | JWT is stateless, Session requires server-side state storage
  2. 认证中间件应该在什么位置处理权限检查？| Where should authentication middleware handle permission checking?
     **答案 | Answer:** 在用户身份验证成功之后 | After user identity verification succeeds
  3. API Key认证适用于什么场景？| What scenarios is API Key authentication suitable for?
     **答案 | Answer:** 服务间调用、第三方API接入等 | Service-to-service calls, third-party API integration, etc.
  4. 认证失败时是否应该暴露具体的失败原因？| Should specific failure reasons be exposed when authentication fails?
     **答案 | Answer:** 否 | No - 避免信息泄露，统一返回认证失败 | avoid information leakage, uniformly return authentication failure
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 多策略认证中间件实现
  // Multi-strategy authentication middleware implementation
  use axum::{
      async_trait,
      extract::{FromRequestParts, Request},
      http::{request::Parts, StatusCode, HeaderMap},
      middleware::Next,
      response::{IntoResponse, Response},
      RequestPartsExt,
  };
  use serde::{Deserialize, Serialize};
  use std::collections::{HashMap, HashSet};
  use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
  use std::time::{SystemTime, UNIX_EPOCH};
  
  // 用户信息和权限
  // User information and permissions
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct User {
      pub id: u64,
      pub username: String,
      pub email: String,
      pub roles: Vec<String>,
      pub permissions: Vec<String>,
  }
  
  // JWT Claims
  #[derive(Debug, Serialize, Deserialize)]
  pub struct Claims {
      pub sub: String,          // 用户ID | User ID
      pub username: String,
      pub roles: Vec<String>,
      pub permissions: Vec<String>,
      pub exp: u64,            // 过期时间 | Expiration time
      pub iat: u64,            // 签发时间 | Issued at
  }
  
  // 认证结果
  // Authentication result
  #[derive(Debug, Clone)]
  pub struct AuthResult {
      pub user: User,
      pub auth_method: String,
      pub token_expires: Option<u64>,
  }
  
  // 认证策略Trait
  // Authentication strategy trait
  #[async_trait]
  pub trait AuthStrategy: Send + Sync {
      async fn authenticate(&self, headers: &HeaderMap) -> Result<AuthResult, AuthError>;
      fn name(&self) -> &str;
      fn priority(&self) -> u8; // 优先级，数字越小优先级越高 | Priority, lower numbers have higher priority
  }
  
  // 认证错误
  // Authentication errors
  #[derive(Debug, thiserror::Error)]
  pub enum AuthError {
      #[error("Missing authorization header")]
      MissingAuth,
      #[error("Invalid token format")]
      InvalidFormat,
      #[error("Token expired")]
      TokenExpired,
      #[error("Invalid token")]
      InvalidToken,
      #[error("Insufficient permissions")]
      InsufficientPermissions,
      #[error("User not found")]
      UserNotFound,
  }
  
  // JWT认证策略
  // JWT authentication strategy
  pub struct JwtAuthStrategy {
      pub secret: String,
      pub issuer: String,
  }
  
  #[async_trait]
  impl AuthStrategy for JwtAuthStrategy {
      async fn authenticate(&self, headers: &HeaderMap) -> Result<AuthResult, AuthError> {
          let auth_header = headers.get("Authorization")
              .and_then(|v| v.to_str().ok())
              .ok_or(AuthError::MissingAuth)?;
          
          if !auth_header.starts_with("Bearer ") {
              return Err(AuthError::InvalidFormat);
          }
          
          let token = &auth_header[7..];
          
          let decoding_key = DecodingKey::from_secret(self.secret.as_ref());
          let validation = Validation::default();
          
          let token_data = decode::<Claims>(token, &decoding_key, &validation)
              .map_err(|_| AuthError::InvalidToken)?;
          
          let claims = token_data.claims;
          
          // 检查过期时间 | Check expiration
          let now = SystemTime::now()
              .duration_since(UNIX_EPOCH)
              .unwrap()
              .as_secs();
          
          if claims.exp < now {
              return Err(AuthError::TokenExpired);
          }
          
          let user = User {
              id: claims.sub.parse().unwrap_or(0),
              username: claims.username,
              email: format!("{}@example.com", claims.sub), // 简化示例 | Simplified example
              roles: claims.roles,
              permissions: claims.permissions,
          };
          
          Ok(AuthResult {
              user,
              auth_method: "JWT".to_string(),
              token_expires: Some(claims.exp),
          })
      }
      
      fn name(&self) -> &str {
          "JWT"
      }
      
      fn priority(&self) -> u8 {
          1
      }
  }
  
  // API Key认证策略
  // API Key authentication strategy
  pub struct ApiKeyAuthStrategy {
      pub valid_keys: HashMap<String, User>,
  }
  
  #[async_trait]
  impl AuthStrategy for ApiKeyAuthStrategy {
      async fn authenticate(&self, headers: &HeaderMap) -> Result<AuthResult, AuthError> {
          let api_key = headers.get("X-API-Key")
              .and_then(|v| v.to_str().ok())
              .ok_or(AuthError::MissingAuth)?;
          
          let user = self.valid_keys.get(api_key)
              .cloned()
              .ok_or(AuthError::InvalidToken)?;
          
          Ok(AuthResult {
              user,
              auth_method: "API_KEY".to_string(),
              token_expires: None,
          })
      }
      
      fn name(&self) -> &str {
          "API_KEY"
      }
      
      fn priority(&self) -> u8 {
          2
      }
  }
  
  // Basic Auth认证策略
  // Basic Auth authentication strategy  
  pub struct BasicAuthStrategy {
      pub users: HashMap<String, String>, // username -> password
      pub user_info: HashMap<String, User>,
  }
  
  #[async_trait]
  impl AuthStrategy for BasicAuthStrategy {
      async fn authenticate(&self, headers: &HeaderMap) -> Result<AuthResult, AuthError> {
          let auth_header = headers.get("Authorization")
              .and_then(|v| v.to_str().ok())
              .ok_or(AuthError::MissingAuth)?;
          
          if !auth_header.starts_with("Basic ") {
              return Err(AuthError::InvalidFormat);
          }
          
          let encoded = &auth_header[6..];
          let decoded = base64::decode(encoded)
              .map_err(|_| AuthError::InvalidFormat)?;
          
          let credentials = String::from_utf8(decoded)
              .map_err(|_| AuthError::InvalidFormat)?;
          
          let parts: Vec<&str> = credentials.splitn(2, ':').collect();
          if parts.len() != 2 {
              return Err(AuthError::InvalidFormat);
          }
          
          let username = parts[0];
          let password = parts[1];
          
          // 验证用户名密码 | Validate username and password
          if let Some(stored_password) = self.users.get(username) {
              if stored_password == password {
                  let user = self.user_info.get(username)
                      .cloned()
                      .ok_or(AuthError::UserNotFound)?;
                  
                  return Ok(AuthResult {
                      user,
                      auth_method: "BASIC".to_string(),
                      token_expires: None,
                  });
              }
          }
          
          Err(AuthError::InvalidToken)
      }
      
      fn name(&self) -> &str {
          "BASIC"
      }
      
      fn priority(&self) -> u8 {
          3
      }
  }
  
  // 多策略认证管理器
  // Multi-strategy authentication manager
  pub struct AuthManager {
      strategies: Vec<Box<dyn AuthStrategy>>,
  }
  
  impl AuthManager {
      pub fn new() -> Self {
          Self {
              strategies: Vec::new(),
          }
      }
      
      pub fn add_strategy(mut self, strategy: Box<dyn AuthStrategy>) -> Self {
          self.strategies.push(strategy);
          // 按优先级排序 | Sort by priority
          self.strategies.sort_by_key(|s| s.priority());
          self
      }
      
      pub async fn authenticate(&self, headers: &HeaderMap) -> Result<AuthResult, AuthError> {
          for strategy in &self.strategies {
              if let Ok(result) = strategy.authenticate(headers).await {
                  println!("✅ Authentication successful with strategy: {}", strategy.name());
                  return Ok(result);
              }
          }
          
          Err(AuthError::InvalidToken)
      }
  }
  
  // 权限检查器
  // Permission checker
  pub struct PermissionChecker;
  
  impl PermissionChecker {
      pub fn check_permission(user: &User, required_permission: &str) -> bool {
          user.permissions.contains(&required_permission.to_string())
      }
      
      pub fn check_role(user: &User, required_role: &str) -> bool {
          user.roles.contains(&required_role.to_string())
      }
      
      pub fn check_any_role(user: &User, required_roles: &[&str]) -> bool {
          required_roles.iter().any(|role| user.roles.contains(&role.to_string()))
      }
  }
  
  // 认证中间件
  // Authentication middleware
  pub async fn auth_middleware<B>(
      auth_manager: AuthManager,
      required_permission: Option<String>,
      request: Request<B>,
      next: Next<B>,
  ) -> Result<Response, StatusCode> {
      // 提取请求头 | Extract request headers
      let headers = request.headers();
      
      // 尝试认证 | Attempt authentication
      let auth_result = match auth_manager.authenticate(headers).await {
          Ok(result) => result,
          Err(error) => {
              println!("🚫 Authentication failed: {}", error);
              return Err(StatusCode::UNAUTHORIZED);
          }
      };
      
      // 权限检查 | Permission check
      if let Some(permission) = &required_permission {
          if !PermissionChecker::check_permission(&auth_result.user, permission) {
              println!("🚫 Permission denied for user: {}", auth_result.user.username);
              return Err(StatusCode::FORBIDDEN);
          }
      }
      
      // 将用户信息添加到请求扩展中 | Add user info to request extensions
      let (mut parts, body) = request.into_parts();
      parts.extensions.insert(auth_result);
      let request = Request::from_parts(parts, body);
      
      Ok(next.run(request).await)
  }
  
  // 便捷的用户信息提取器
  // Convenient user information extractor
  #[async_trait]
  impl<S> FromRequestParts<S> for User
  where
      S: Send + Sync,
  {
      type Rejection = StatusCode;
      
      async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
          parts.extensions
              .get::<AuthResult>()
              .map(|auth| auth.user.clone())
              .ok_or(StatusCode::UNAUTHORIZED)
      }
  }
  
  // JWT令牌生成工具
  // JWT token generation utility
  pub struct JwtTokenGenerator {
      pub secret: String,
      pub issuer: String,
      pub expiration_hours: u64,
  }
  
  impl JwtTokenGenerator {
      pub fn new(secret: String, issuer: String, expiration_hours: u64) -> Self {
          Self {
              secret,
              issuer,
              expiration_hours,
          }
      }
      
      pub fn generate_token(&self, user: &User) -> Result<String, jsonwebtoken::errors::Error> {
          let now = SystemTime::now()
              .duration_since(UNIX_EPOCH)
              .unwrap()
              .as_secs();
          
          let claims = Claims {
              sub: user.id.to_string(),
              username: user.username.clone(),
              roles: user.roles.clone(),
              permissions: user.permissions.clone(),
              exp: now + (self.expiration_hours * 3600),
              iat: now,
          };
          
          let encoding_key = EncodingKey::from_secret(self.secret.as_ref());
          encode(&Header::default(), &claims, &encoding_key)
      }
  }
  ```

### 6. 中间件性能优化和最佳实践 | Middleware Performance Optimization and Best Practices (30分钟 | 30 minutes)

- **性能优化策略 | Performance Optimization Strategies**
  
  **概念定义 | Concept Definition:**
  中间件性能优化涉及减少内存分配、优化计算复杂度、使用高效的数据结构和算法。关键优化点包括避免不必要的克隆、使用引用传递、缓存昂贵计算结果、批处理操作等。性能测试和监控是确保优化效果的重要手段。
  
  Middleware performance optimization involves reducing memory allocations, optimizing computational complexity, and using efficient data structures and algorithms. Key optimization points include avoiding unnecessary cloning, using reference passing, caching expensive computation results, batch processing operations, etc. Performance testing and monitoring are important means to ensure optimization effectiveness.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 中间件中频繁的字符串拼接应该如何优化？| How should frequent string concatenation in middleware be optimized?
     **答案 | Answer:** 使用format!宏或StringBuilder避免重复分配 | Use format! macro or StringBuilder to avoid repeated allocations
  2. 是否应该在每个请求中重新创建中间件实例？| Should middleware instances be recreated for each request?
     **答案 | Answer:** 否 | No - 应复用实例，只在必要时创建新状态 | should reuse instances, only create new state when necessary
  3. 中间件中的异步操作应该如何优化？| How should async operations in middleware be optimized?
     **答案 | Answer:** 使用并发处理、连接池、批处理减少延迟 | Use concurrent processing, connection pools, batching to reduce latency
  4. 缓存在中间件中有什么作用？| What role does caching play in middleware?
     **答案 | Answer:** 避免重复计算，如认证结果、配置信息等 | Avoid repeated computations like authentication results, configuration info, etc.
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 性能优化的中间件实现示例
  // Performance-optimized middleware implementation examples
  use std::sync::{Arc, RwLock};
  use std::collections::HashMap;
  use std::time::{Duration, Instant};
  use tokio::sync::Semaphore;
  
  // 1. 高性能缓存中间件
  // High-performance caching middleware
  pub struct OptimizedCache<K, V> 
  where
      K: Clone + Eq + std::hash::Hash,
      V: Clone,
  {
      data: Arc<RwLock<HashMap<K, (V, Instant)>>>,
      ttl: Duration,
      max_size: usize,
  }
  
  impl<K, V> OptimizedCache<K, V>
  where
      K: Clone + Eq + std::hash::Hash,
      V: Clone,
  {
      pub fn new(ttl: Duration, max_size: usize) -> Self {
          Self {
              data: Arc::new(RwLock::new(HashMap::new())),
              ttl,
              max_size,
          }
      }
      
      pub fn get(&self, key: &K) -> Option<V> {
          // 使用读锁进行快速查找 | Use read lock for fast lookup
          let data = self.data.read().unwrap();
          if let Some((value, timestamp)) = data.get(key) {
              if timestamp.elapsed() < self.ttl {
                  return Some(value.clone());
              }
          }
          None
      }
      
      pub fn insert(&self, key: K, value: V) {
          let mut data = self.data.write().unwrap();
          
          // LRU淘汰策略（简化版）| LRU eviction policy (simplified)
          if data.len() >= self.max_size {
              // 找到最老的条目进行删除 | Find and remove oldest entry
              let oldest_key = data.iter()
                  .min_by_key(|(_, (_, timestamp))| timestamp)
                  .map(|(k, _)| k.clone());
              
              if let Some(key_to_remove) = oldest_key {
                  data.remove(&key_to_remove);
              }
          }
          
          data.insert(key, (value, Instant::now()));
      }
      
      pub fn cleanup_expired(&self) {
          let mut data = self.data.write().unwrap();
          let now = Instant::now();
          data.retain(|_, (_, timestamp)| now.duration_since(*timestamp) < self.ttl);
      }
  }
  
  // 2. 连接池优化的HTTP客户端中间件
  // Connection pool optimized HTTP client middleware
  use reqwest::Client;
  use std::sync::Arc;
  
  pub struct OptimizedHttpClient {
      client: Arc<Client>,
      semaphore: Arc<Semaphore>,
  }
  
  impl OptimizedHttpClient {
      pub fn new(max_concurrent_requests: usize) -> Self {
          let client = Arc::new(
              Client::builder()
                  .pool_max_idle_per_host(10)
                  .pool_idle_timeout(Duration::from_secs(90))
                  .timeout(Duration::from_secs(30))
                  .build()
                  .unwrap()
          );
          
          Self {
              client,
              semaphore: Arc::new(Semaphore::new(max_concurrent_requests)),
          }
      }
      
      pub async fn get(&self, url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
          let _permit = self.semaphore.acquire().await?;
          let response = self.client.get(url).send().await?;
          let text = response.text().await?;
          Ok(text)
      }
  }
  
  // 3. 零分配字符串处理
  // Zero-allocation string processing
  use std::fmt::Write;
  
  pub struct StringProcessor {
      buffer: String,
  }
  
  impl StringProcessor {
      pub fn new() -> Self {
          Self {
              buffer: String::with_capacity(1024), // 预分配容量 | Pre-allocate capacity
          }
      }
      
      pub fn process_headers(&mut self, headers: &axum::http::HeaderMap) -> &str {
          self.buffer.clear(); // 清空但保持容量 | Clear but keep capacity
          
          for (key, value) in headers.iter() {
              write!(self.buffer, "{}={};", key.as_str(), value.to_str().unwrap_or("")).unwrap();
          }
          
          &self.buffer
      }
  }
  
  // 4. 批处理日志写入器
  // Batch log writer
  use tokio::sync::mpsc;
  use tokio::time::{interval, Duration};
  
  pub struct BatchLogWriter {
      sender: mpsc::UnboundedSender<String>,
  }
  
  impl BatchLogWriter {
      pub fn new() -> Self {
          let (sender, mut receiver) = mpsc::unbounded_channel();
          
          // 启动批处理任务 | Start batch processing task
          tokio::spawn(async move {
              let mut buffer = Vec::with_capacity(100);
              let mut timer = interval(Duration::from_millis(100));
              
              loop {
                  tokio::select! {
                      // 定时写入 | Timed write
                      _ = timer.tick() => {
                          if !buffer.is_empty() {
                              Self::flush_logs(&mut buffer).await;
                          }
                      }
                      
                      // 接收新日志 | Receive new logs
                      Some(log) = receiver.recv() => {
                          buffer.push(log);
                          
                          // 缓冲区满时立即写入 | Immediate write when buffer is full
                          if buffer.len() >= 100 {
                              Self::flush_logs(&mut buffer).await;
                          }
                      }
                      
                      else => break,
                  }
              }
          });
          
          Self { sender }
      }
      
      pub fn log(&self, message: String) {
          let _ = self.sender.send(message);
      }
      
      async fn flush_logs(buffer: &mut Vec<String>) {
          if buffer.is_empty() {
              return;
          }
          
          // 批量写入文件或发送到日志服务 | Batch write to file or send to logging service
          println!("📝 Writing {} logs in batch", buffer.len());
          for log in buffer.iter() {
              println!("{}", log);
          }
          
          buffer.clear();
      }
  }
  
  // 5. 性能监控和测量工具
  // Performance monitoring and measurement tools
  pub struct PerformanceMonitor {
      request_times: Arc<RwLock<Vec<Duration>>>,
      memory_usage: Arc<RwLock<Vec<usize>>>,
  }
  
  impl PerformanceMonitor {
      pub fn new() -> Self {
          Self {
              request_times: Arc::new(RwLock::new(Vec::new())),
              memory_usage: Arc::new(RwLock::new(Vec::new())),
          }
      }
      
      pub fn record_request_time(&self, duration: Duration) {
          let mut times = self.request_times.write().unwrap();
          times.push(duration);
          
          // 保持最近1000条记录 | Keep recent 1000 records
          if times.len() > 1000 {
              times.remove(0);
          }
      }
      
      pub fn get_avg_response_time(&self) -> Option<Duration> {
          let times = self.request_times.read().unwrap();
          if times.is_empty() {
              return None;
          }
          
          let total: Duration = times.iter().sum();
          Some(total / times.len() as u32)
      }
      
      pub fn get_percentile(&self, percentile: f64) -> Option<Duration> {
          let mut times = self.request_times.read().unwrap().clone();
          if times.is_empty() {
              return None;
          }
          
          times.sort();
          let index = ((times.len() as f64) * percentile / 100.0) as usize;
          times.get(index.min(times.len() - 1)).copied()
      }
  }
  
  // 性能测试示例
  // Performance testing example
  #[cfg(test)]
  mod performance_tests {
      use super::*;
      use std::time::Instant;
      
      #[tokio::test]
      async fn test_cache_performance() {
          let cache = OptimizedCache::new(Duration::from_secs(60), 1000);
          let start = Instant::now();
          
          // 写入测试 | Write test
          for i in 0..10000 {
              cache.insert(i, format!("value_{}", i));
          }
          
          let write_time = start.elapsed();
          println!("Write 10k items: {:?}", write_time);
          
          // 读取测试 | Read test
          let start = Instant::now();
          for i in 0..10000 {
              let _ = cache.get(&i);
          }
          
          let read_time = start.elapsed();
          println!("Read 10k items: {:?}", read_time);
          
          assert!(read_time < Duration::from_millis(100));
      }
  }
  ```

## 实践项目：企业级中间件系统 | Practical Project: Enterprise Middleware System

### 目标 | Objective
构建一个完整的企业级中间件系统，包含认证、授权、日志记录、限流、缓存、CORS等功能。项目将演示如何组合多个中间件，处理复杂的业务场景，实现高性能和安全性要求。

Build a complete enterprise-level middleware system including authentication, authorization, logging, rate limiting, caching, CORS, and other functions. The project will demonstrate how to combine multiple middleware, handle complex business scenarios, and achieve high performance and security requirements.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 中间件的洋葱模型执行顺序是什么？| What is the onion model execution order of middleware?
   **答案 | Answer:** 请求时从外到内，响应时从内到外 | Request goes from outer to inner, response goes from inner to outer

2. Tower Service trait的核心作用是什么？| What is the core role of the Tower Service trait?
   **答案 | Answer:** 提供异步服务的统一抽象，支持可组合的中间件架构 | Provide unified abstraction for async services, support composable middleware architecture

3. 多策略认证系统如何选择认证方式？| How does a multi-strategy authentication system choose authentication methods?
   **答案 | Answer:** 根据请求头特征和策略优先级自动选择 | Automatically choose based on request header characteristics and strategy priorities

### 步骤 | Steps
1. **项目架构设计**：设计中间件系统架构，定义各组件职责
2. **核心中间件实现**：实现认证、日志、限流等核心中间件
3. **中间件组合器**：创建灵活的中间件组合和配置系统
4. **性能优化**：实现缓存、连接池等性能优化功能
5. **安全加固**：添加安全防护机制和监控功能

### 示例代码 | Example Code
```rust
"""
企业级中间件系统 | Enterprise Middleware System
综合应用中间件设计模式和性能优化技术 | Comprehensive application of middleware design patterns and performance optimization techniques

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- Tower Service trait架构 | Tower Service trait architecture
- 多策略认证系统 | Multi-strategy authentication system
- 结构化日志记录 | Structured logging
- 性能优化技术 | Performance optimization techniques
- 安全防护机制 | Security protection mechanisms
"""

use axum::{
    extract::{State, Path, Query, Json},
    http::{StatusCode, Method},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};

// 应用状态 | Application state
#[derive(Clone)]
pub struct AppState {
    pub auth_manager: AuthManager,
    pub rate_limiter: Arc<RateLimiter>,
    pub cache: Arc<OptimizedCache<String, String>>,
    pub logger: BatchLogWriter,
    pub performance_monitor: Arc<PerformanceMonitor>,
}

// API请求/响应模型 | API request/response models
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub request_id: String,
}

// 业务处理器 | Business handlers
pub async fn create_user(
    State(state): State<AppState>,
    user: User,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // 权限检查 - 需要admin角色 | Permission check - requires admin role
    if !PermissionChecker::check_role(&user, "admin") {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // 业务逻辑 | Business logic
    let new_user = UserResponse {
        id: 12345,
        username: payload.username,
        email: payload.email,
        roles: vec!["user".to_string()],
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };
    
    let response = ApiResponse {
        success: true,
        data: Some(new_user),
        message: "User created successfully".to_string(),
        request_id: uuid::Uuid::new_v4().to_string(),
    };
    
    state.logger.log(format!("User created by: {}", user.username));
    
    Ok(Json(response))
}

pub async fn get_user(
    State(state): State<AppState>,
    user: User,
    Path(user_id): Path<u64>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // 缓存检查 | Cache check
    let cache_key = format!("user_{}", user_id);
    if let Some(cached) = state.cache.get(&cache_key) {
        state.logger.log(format!("Cache hit for user: {}", user_id));
    }
    
    // 模拟用户数据 | Mock user data
    let user_response = UserResponse {
        id: user_id,
        username: format!("user_{}", user_id),
        email: format!("user{}@example.com", user_id),
        roles: vec!["user".to_string()],
        created_at: 1640995200, // 2022-01-01
    };
    
    // 更新缓存 | Update cache
    state.cache.insert(cache_key, serde_json::to_string(&user_response).unwrap());
    
    let response = ApiResponse {
        success: true,
        data: Some(user_response),
        message: "User retrieved successfully".to_string(),
        request_id: uuid::Uuid::new_v4().to_string(),
    };
    
    Ok(Json(response))
}

pub async fn update_user(
    State(state): State<AppState>,
    user: User,
    Path(user_id): Path<u64>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // 权限检查 - 只能更新自己或管理员 | Permission check - can only update self or admin
    if user.id != user_id && !PermissionChecker::check_role(&user, "admin") {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // 清除缓存 | Clear cache
    let cache_key = format!("user_{}", user_id);
    // state.cache.remove(&cache_key); // 实际实现中需要remove方法 | Need remove method in actual implementation
    
    let updated_user = UserResponse {
        id: user_id,
        username: format!("user_{}", user_id),
        email: payload.email.unwrap_or_else(|| format!("user{}@example.com", user_id)),
        roles: vec!["user".to_string()],
        created_at: 1640995200,
    };
    
    let response = ApiResponse {
        success: true,
        data: Some(updated_user),
        message: "User updated successfully".to_string(),
        request_id: uuid::Uuid::new_v4().to_string(),
    };
    
    state.logger.log(format!("User {} updated by: {}", user_id, user.username));
    
    Ok(Json(response))
}

pub async fn delete_user(
    State(state): State<AppState>,
    user: User,
    Path(user_id): Path<u64>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // 权限检查 - 需要admin角色 | Permission check - requires admin role
    if !PermissionChecker::check_role(&user, "admin") {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // 清除缓存 | Clear cache
    let cache_key = format!("user_{}", user_id);
    // state.cache.remove(&cache_key); // 实际实现中需要remove方法 | Need remove method in actual implementation
    
    let response = ApiResponse {
        success: true,
        data: None,
        message: "User deleted successfully".to_string(),
        request_id: uuid::Uuid::new_v4().to_string(),
    };
    
    state.logger.log(format!("User {} deleted by: {}", user_id, user.username));
    
    Ok(Json(response))
}

// 健康检查端点 | Health check endpoint
pub async fn health_check(
    State(state): State<AppState>,
) -> Json<ApiResponse<HashMap<String, String>>> {
    let mut health_info = HashMap::new();
    health_info.insert("status".to_string(), "healthy".to_string());
    health_info.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
    
    if let Some(avg_time) = state.performance_monitor.get_avg_response_time() {
        health_info.insert("avg_response_time".to_string(), format!("{:?}", avg_time));
    }
    
    if let Some(p95) = state.performance_monitor.get_percentile(95.0) {
        health_info.insert("p95_response_time".to_string(), format!("{:?}", p95));
    }
    
    let response = ApiResponse {
        success: true,
        data: Some(health_info),
        message: "System healthy".to_string(),
        request_id: uuid::Uuid::new_v4().to_string(),
    };
    
    Json(response)
}

// 中间件组合工厂 | Middleware composition factory
pub fn create_middleware_stack() -> ServiceBuilder<
    tower::util::Stack<
        axum::middleware::FromFnLayer<
            fn(axum::http::Request<axum::body::Body>, Next<axum::body::Body>) 
                -> impl std::future::Future<Output = Response> + Send,
            axum::body::Body,
        >,
        tower::util::Stack<
            axum::middleware::FromFnLayer<
                fn(AppState, axum::http::Request<axum::body::Body>, Next<axum::body::Body>) 
                    -> impl std::future::Future<Output = Result<Response, StatusCode>> + Send,
                axum::body::Body,
            >,
            tower::util::Identity,
        >,
    >,
> {
    ServiceBuilder::new()
        // 性能监控中间件 | Performance monitoring middleware
        .layer(middleware::from_fn(performance_monitoring_middleware))
        // 限流中间件 | Rate limiting middleware  
        .layer(middleware::from_fn_with_state(rate_limiting_middleware))
        // 认证中间件 | Authentication middleware
        .layer(middleware::from_fn_with_state(authentication_middleware))
        // 日志中间件 | Logging middleware
        .layer(middleware::from_fn(logging_middleware))
}

// 性能监控中间件 | Performance monitoring middleware
async fn performance_monitoring_middleware<B>(
    request: axum::http::Request<B>,
    next: Next<B>,
) -> Response {
    let start = std::time::Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed();
    
    // 记录请求时间 | Record request time
    println!("⏱️  Request completed in: {:?}", duration);
    
    response
}

// 限流中间件 | Rate limiting middleware
async fn rate_limiting_middleware<B>(
    State(state): State<AppState>,
    request: axum::http::Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let client_ip = request.extensions()
        .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
        .map(|connect_info| connect_info.0.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    
    if !state.rate_limiter.is_allowed(&client_ip) {
        state.logger.log(format!("Rate limit exceeded for IP: {}", client_ip));
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    Ok(next.run(request).await)
}

// 认证中间件 | Authentication middleware
async fn authentication_middleware<B>(
    State(state): State<AppState>,
    request: axum::http::Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let uri = request.uri().path();
    
    // 跳过公开端点 | Skip public endpoints
    if uri.starts_with("/health") || uri.starts_with("/login") {
        return Ok(next.run(request).await);
    }
    
    let headers = request.headers();
    let auth_result = state.auth_manager.authenticate(headers).await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // 将认证结果添加到请求中 | Add auth result to request
    let (mut parts, body) = request.into_parts();
    parts.extensions.insert(auth_result);
    let request = axum::http::Request::from_parts(parts, body);
    
    Ok(next.run(request).await)
}

// 日志中间件 | Logging middleware
async fn logging_middleware<B>(
    request: axum::http::Request<B>,
    next: Next<B>,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    println!("📋 {} {} - {} ({:?})", method, uri, status, duration);
    
    response
}

// 应用构建器 | Application builder
pub async fn build_app() -> Router {
    // 初始化认证管理器 | Initialize auth manager
    let auth_manager = AuthManager::new()
        .add_strategy(Box::new(JwtAuthStrategy {
            secret: "your-secret-key".to_string(),
            issuer: "your-app".to_string(),
        }))
        .add_strategy(Box::new(ApiKeyAuthStrategy {
            valid_keys: {
                let mut keys = HashMap::new();
                keys.insert("api-key-123".to_string(), User {
                    id: 1,
                    username: "api-user".to_string(),
                    email: "api@example.com".to_string(),
                    roles: vec!["api".to_string()],
                    permissions: vec!["read".to_string(), "write".to_string()],
                });
                keys
            },
        }));
    
    // 初始化其他组件 | Initialize other components
    let rate_limiter = Arc::new(RateLimiter::new(100, std::time::Duration::from_secs(60)));
    let cache = Arc::new(OptimizedCache::new(
        std::time::Duration::from_secs(300),
        1000,
    ));
    let logger = BatchLogWriter::new();
    let performance_monitor = Arc::new(PerformanceMonitor::new());
    
    let state = AppState {
        auth_manager,
        rate_limiter,
        cache,
        logger,
        performance_monitor,
    };
    
    Router::new()
        // API路由 | API routes
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .route("/health", get(health_check))
        
        // 应用中间件栈 | Apply middleware stack
        .layer(create_middleware_stack())
        
        // CORS配置 | CORS configuration
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(Any)
        )
        
        .with_state(state)
}

// 主函数 | Main function
#[tokio::main]
async fn main() {
    println!("🚀 Starting Enterprise Middleware System...");
    
    let app = build_app().await;
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🌐 Server running on http://localhost:3000");
    println!("📚 Available endpoints:");
    println!("  POST /users - Create user (requires admin role)");
    println!("  GET /users/:id - Get user");
    println!("  PUT /users/:id - Update user");
    println!("  DELETE /users/:id - Delete user (requires admin role)");
    println!("  GET /health - Health check");
    
    axum::serve(listener, app).await.unwrap();
}

// 集成测试 | Integration tests
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    
    #[tokio::test]
    async fn test_health_endpoint() {
        let app = build_app().await;
        
        let response = app
            .oneshot(Request::builder()
                .uri("/health")
                .body(axum::body::Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_authentication_required() {
        let app = build_app().await;
        
        let response = app
            .oneshot(Request::builder()
                .uri("/users/1")
                .body(axum::body::Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    
    #[tokio::test]
    async fn test_api_key_authentication() {
        let app = build_app().await;
        
        let response = app
            .oneshot(Request::builder()
                .uri("/users/1")
                .header("X-API-Key", "api-key-123")
                .body(axum::body::Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确实现了多层中间件架构？| Does the project correctly implement multi-layer middleware architecture?
2. 认证系统是否支持多种策略并能正确选择？| Does the authentication system support multiple strategies and correctly select them?
3. 性能监控和缓存功能是否正常工作？| Are performance monitoring and caching functions working properly?
4. 日志系统是否记录了完整的请求处理信息？| Does the logging system record complete request processing information?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **Tower Service深度实践 | Tower Service Deep Practice**
   - **练习描述 | Exercise Description:** 实现一个支持熔断器模式的自定义Service，包含健康检查、自动恢复等功能
   - **概念检查 | Concept Check:** 熔断器模式如何通过Service trait实现状态管理？
   - **学习目标 | Learning Objective:** 深入理解Service trait的状态管理和错误处理能力

2. **异步中间件性能优化 | Async Middleware Performance Optimization**
   - **练习描述 | Exercise Description:** 对现有日志中间件进行性能优化，实现零分配的字符串处理和批量I/O操作
   - **概念检查 | Concept Check:** 异步中间件中如何避免不必要的内存分配？
   - **学习目标 | Learning Objective:** 掌握异步Rust中的性能优化技术

3. **复杂认证场景处理 | Complex Authentication Scenario Handling**
   - **练习描述 | Exercise Description:** 实现支持JWT刷新、多租户、动态权限的复杂认证系统
   - **概念检查 | Concept Check:** 多租户认证如何与中间件链结合？
   - **学习目标 | Learning Objective:** 处理企业级认证的复杂需求

4. **中间件监控和观测性 | Middleware Monitoring and Observability**
   - **练习描述 | Exercise Description:** 添加分布式追踪、指标收集和告警功能到中间件系统
   - **概念检查 | Concept Check:** 如何在中间件中传递追踪上下文？
   - **学习目标 | Learning Objective:** 实现完整的可观测性方案

5. **自适应中间件系统 | Adaptive Middleware System**
   - **练习描述 | Exercise Description:** 创建能根据负载和错误率自动调整行为的智能中间件
   - **概念检查 | Concept Check:** 中间件如何收集和响应运行时指标？
   - **学习目标 | Learning Objective:** 实现自适应和智能化的中间件行为

6. **中间件配置管理 | Middleware Configuration Management**
   - **练习描述 | Exercise Description:** 实现支持热重载、环境隔离的配置管理系统
   - **概念检查 | Concept Check:** 如何在不重启服务的情况下更新中间件配置？
   - **学习目标 | Learning Objective:** 掌握动态配置管理技术

7. **中间件测试框架 | Middleware Testing Framework**
   - **练习描述 | Exercise Description:** 开发专门用于测试中间件行为的测试框架和工具
   - **概念检查 | Concept Check:** 如何模拟和测试中间件的各种边界条件？
   - **学习目标 | Learning Objective:** 建立完整的中间件测试体系

## 学习资源 | Learning Resources
- [Tower官方文档 - Service trait](https://docs.rs/tower/latest/tower/)
- [Axum中间件指南](https://docs.rs/axum/latest/axum/middleware/)
- [Rust异步编程权威指南](https://rust-lang.github.io/async-book/)
- [企业级Web服务架构设计](https://microservices.io/patterns/)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解中间件概念和洋葱模型执行机制 | Understand middleware concepts and onion model execution mechanism
- [ ] 掌握Tower Service trait的设计原理和实现方法 | Master Tower Service trait design principles and implementation methods
- [ ] 能够设计和开发自定义中间件组件 | Able to design and develop custom middleware components
- [ ] 实现了完整的日志记录中间件系统 | Implemented complete logging middleware system
- [ ] 构建了多策略认证中间件架构 | Built multi-strategy authentication middleware architecture
- [ ] 掌握中间件性能优化技术和最佳实践 | Master middleware performance optimization techniques and best practices
- [ ] 完成企业级中间件系统项目开发 | Completed enterprise middleware system project development
- [ ] 理解中间件在Web架构中的重要作用 | Understand the important role of middleware in web architecture
- [ ] 能够处理复杂的中间件组合和配置场景 | Able to handle complex middleware composition and configuration scenarios
- [ ] 具备中间件系统的监控和调试能力 | Have middleware system monitoring and debugging capabilities

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释每个核心概念，特别是Tower Service trait的工作原理、中间件的组合模式和性能优化策略。

Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain each core concept to others, especially the working principles of Tower Service trait, middleware composition patterns, and performance optimization strategies.