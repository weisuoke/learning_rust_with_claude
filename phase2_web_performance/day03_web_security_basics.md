# Rust Web入门 - 第3天：Web安全基础 | Rust Web Introduction - Day 3: Web Security Basics

## 学习目标 | Learning Objectives
- 深入理解HTTPS与TLS握手过程原理 | Deeply understand HTTPS and TLS handshake process principles
- 掌握常见安全头设置和最佳实践 | Master common security header settings and best practices
- 熟练配置CORS、CSP、HSTS等安全机制 | Proficiently configure CORS, CSP, HSTS and other security mechanisms
- 理解现代Web认证与授权机制 | Understand modern web authentication and authorization mechanisms
- 具备配置生产级安全HTTPS服务的能力 | Have the ability to configure production-level secure HTTPS services
- 掌握Web安全漏洞防护技术 | Master web security vulnerability protection techniques

## 详细内容 | Detailed Content

### 1. HTTPS与TLS握手过程 | HTTPS and TLS Handshake Process (1.5小时 | 1.5 hours)

- **TLS/SSL协议基础 | TLS/SSL Protocol Fundamentals**
  
  **概念定义 | Concept Definition:**
  TLS (Transport Layer Security) 是一种加密协议，用于在网络通信中提供数据完整性和隐私保护。它是SSL的继任者，通过公钥加密、对称加密和数字证书验证来确保通信安全。| TLS (Transport Layer Security) is an encryption protocol used to provide data integrity and privacy protection in network communications. It is the successor to SSL, ensuring communication security through public key encryption, symmetric encryption, and digital certificate verification.
  
  **核心特征 | Key Characteristics:**
  - TLS提供三层安全保护：认证、完整性、机密性 | TLS provides three layers of security protection: authentication, integrity, confidentiality
  - 使用非对称加密交换密钥，对称加密传输数据 | Uses asymmetric encryption for key exchange, symmetric encryption for data transmission
  - 支持向前保密，确保历史通信不被破解 | Supports forward secrecy, ensuring historical communications cannot be compromised
  - 通过数字证书验证服务器身份 | Verifies server identity through digital certificates
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. TLS握手过程中客户端和服务器都需要验证身份吗？| Do both client and server need to verify identity during TLS handshake?
     **答案 | Answer:** 否 | No - 通常只验证服务器身份，客户端身份验证是可选的 | Usually only server identity is verified, client identity verification is optional
  2. TLS使用对称加密还是非对称加密传输实际数据？| Does TLS use symmetric or asymmetric encryption to transmit actual data?
     **答案 | Answer:** 对称加密 | Symmetric encryption - 非对称加密只用于密钥交换 | Asymmetric encryption is only used for key exchange
  3. 数字证书的主要作用是什么？| What is the main purpose of digital certificates?
     **答案 | Answer:** 验证服务器身份和公钥的真实性 | Verify server identity and authenticity of public keys
  4. TLS的向前保密特性能防止什么安全威胁？| What security threats can TLS forward secrecy prevent?
     **答案 | Answer:** 防止私钥泄露后历史通信被解密 | Prevent historical communications from being decrypted after private key compromise
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // TLS服务器配置示例 | TLS server configuration example
  use rustls::{Certificate, PrivateKey, ServerConfig};
  use rustls_pemfile::{certs, pkcs8_private_keys};
  use std::fs::File;
  use std::io::BufReader;
  
  // 加载TLS证书和私钥 | Load TLS certificate and private key
  fn load_tls_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
      // 读取证书文件 | Read certificate file
      let cert_file = File::open("cert.pem")?;
      let mut cert_reader = BufReader::new(cert_file);
      let certs = certs(&mut cert_reader)?
          .into_iter()
          .map(Certificate)
          .collect();
  
      // 读取私钥文件 | Read private key file
      let key_file = File::open("key.pem")?;
      let mut key_reader = BufReader::new(key_file);
      let mut keys = pkcs8_private_keys(&mut key_reader)?;
      if keys.is_empty() {
          return Err("No private keys found".into());
      }
      let key = PrivateKey(keys.remove(0));
  
      // 配置TLS服务器 | Configure TLS server
      let config = ServerConfig::builder()
          .with_safe_defaults()
          .with_no_client_auth()
          .with_single_cert(certs, key)?;
  
      Ok(config)
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这个配置启用了客户端证书验证吗？| Does this configuration enable client certificate verification?
    **答案 | Answer:** 没有，使用了with_no_client_auth() | No, it uses with_no_client_auth()
  - 如果证书文件损坏会发生什么？| What happens if the certificate file is corrupted?
    **答案 | Answer:** 函数会返回错误，服务器无法启动TLS | The function will return an error and the server cannot start TLS
  
  **常见误区检查 | Common Misconception Checks:**
  - TLS加密是否会显著影响服务器性能？| Does TLS encryption significantly impact server performance?
    **答案 | Answer:** 现代硬件上影响很小，主要开销在握手阶段 | Very small impact on modern hardware, main overhead is during handshake phase
  - 自签名证书在生产环境中安全吗？| Are self-signed certificates secure in production environments?
    **答案 | Answer:** 不安全，浏览器会警告且无法验证身份 | Not secure, browsers will warn and cannot verify identity

- **TLS握手详细流程 | Detailed TLS Handshake Process**
  
  **概念定义 | Concept Definition:**
  TLS握手是建立安全连接的初始化过程，包括协议版本协商、密码套件选择、证书验证和密钥交换。整个过程确保双方能够安全地建立加密通信通道。| TLS handshake is the initialization process for establishing secure connections, including protocol version negotiation, cipher suite selection, certificate verification, and key exchange. The entire process ensures both parties can securely establish an encrypted communication channel.
  
  **核心特征 | Key Characteristics:**
  - 握手过程需要多次往返通信 | Handshake process requires multiple round-trip communications
  - 客户端和服务器协商最高支持的TLS版本 | Client and server negotiate the highest supported TLS version
  - 服务器发送证书链供客户端验证 | Server sends certificate chain for client verification
  - 双方生成会话密钥用于后续加密通信 | Both parties generate session keys for subsequent encrypted communication
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. TLS握手过程中首先发送的消息是什么？| What is the first message sent during TLS handshake?
     **答案 | Answer:** Client Hello消息 | Client Hello message
  2. Server Hello消息包含哪些重要信息？| What important information does the Server Hello message contain?
     **答案 | Answer:** 选择的密码套件、服务器随机数、会话ID | Selected cipher suite, server random number, session ID
  3. 证书验证失败时握手会继续吗？| Will handshake continue when certificate verification fails?
     **答案 | Answer:** 不会，连接会被终止 | No, the connection will be terminated
  4. 预主密钥(Pre-master Secret)的作用是什么？| What is the role of the Pre-master Secret?
     **答案 | Answer:** 用于生成主密钥，进而派生会话密钥 | Used to generate master key, which then derives session keys
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // TLS握手状态监控 | TLS handshake status monitoring
  use rustls::{ServerConnection, ConnectionCommon};
  use std::sync::Arc;
  
  // 监控TLS连接状态 | Monitor TLS connection status
  fn monitor_tls_handshake(conn: &ServerConnection) -> String {
      match conn.is_handshaking() {
          true => {
              // 握手进行中 | Handshake in progress
              format!("TLS握手进行中... | TLS handshake in progress... Protocol: {:?}", 
                     conn.protocol_version())
          },
          false => {
              // 握手完成 | Handshake completed
              let suite = conn.negotiated_cipher_suite()
                  .map(|cs| format!("{:?}", cs.suite()))
                  .unwrap_or("未知 | Unknown".to_string());
              format!("TLS握手完成 | TLS handshake completed. Cipher Suite: {}", suite)
          }
      }
  }
  
  // TLS连接信息获取 | TLS connection information retrieval
  fn get_tls_connection_info(conn: &ServerConnection) {
      if let Some(certs) = conn.peer_certificates() {
          println!("客户端证书数量 | Client certificates count: {}", certs.len());
      }
      
      if let Some(protocol) = conn.protocol_version() {
          println!("协商的TLS版本 | Negotiated TLS version: {:?}", protocol);
      }
      
      if let Some(cipher_suite) = conn.negotiated_cipher_suite() {
          println!("密码套件 | Cipher suite: {:?}", cipher_suite.suite());
          println!("密钥交换 | Key exchange: {:?}", cipher_suite.kx_group());
      }
  }
  ```

### 2. 常见安全头设置 | Common Security Header Settings (1小时 | 1 hour)

- **CORS跨域资源共享 | CORS Cross-Origin Resource Sharing**
  
  **概念定义 | Concept Definition:**
  CORS是一种安全机制，允许或限制Web页面从不同域、协议或端口访问资源。它通过HTTP头控制跨域请求，防止恶意网站未经授权访问用户数据。| CORS is a security mechanism that allows or restricts web pages from accessing resources from different domains, protocols, or ports. It controls cross-origin requests through HTTP headers, preventing malicious sites from accessing user data without authorization.
  
  **核心特征 | Key Characteristics:**
  - 浏览器强制执行同源策略，CORS提供例外机制 | Browsers enforce same-origin policy, CORS provides exception mechanism
  - 简单请求直接发送，复杂请求需要预检 | Simple requests are sent directly, complex requests require preflight
  - 服务器通过响应头控制允许的跨域访问 | Server controls allowed cross-origin access through response headers
  - 支持凭证（cookies）的跨域传输控制 | Supports credential (cookies) cross-origin transmission control
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 所有跨域请求都需要预检吗？| Do all cross-origin requests require preflight?
     **答案 | Answer:** 否 | No - 简单请求不需要预检 | Simple requests do not require preflight
  2. Access-Control-Allow-Origin: * 能与凭证一起使用吗？| Can Access-Control-Allow-Origin: * be used with credentials?
     **答案 | Answer:** 不能 | No - 使用凭证时必须指定具体域名 | Must specify specific domain when using credentials
  3. 预检请求使用什么HTTP方法？| What HTTP method does preflight request use?
     **答案 | Answer:** OPTIONS方法 | OPTIONS method
  4. CORS错误会在哪里显示？| Where do CORS errors appear?
     **答案 | Answer:** 浏览器控制台 | Browser console
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // CORS中间件实现 | CORS middleware implementation
  use axum::{
      http::{HeaderValue, Method, StatusCode},
      response::Response,
      routing::get,
      Router,
  };
  use tower::ServiceBuilder;
  use tower_http::cors::{CorsLayer, Any};
  
  // 配置CORS策略 | Configure CORS policy
  fn create_cors_layer() -> CorsLayer {
      CorsLayer::new()
          // 允许的源 | Allowed origins
          .allow_origin("https://example.com".parse::<HeaderValue>().unwrap())
          // 允许的方法 | Allowed methods  
          .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
          // 允许的头部 | Allowed headers
          .allow_headers(Any)
          // 允许凭证 | Allow credentials
          .allow_credentials(true)
          // 暴露的头部 | Exposed headers
          .expose_headers([axum::http::header::CONTENT_LENGTH])
  }
  
  // 自定义CORS处理 | Custom CORS handling
  async fn handle_cors_preflight() -> StatusCode {
      StatusCode::OK
  }
  
  fn create_app_with_cors() -> Router {
      Router::new()
          .route("/api/data", get(|| async { "Hello CORS!" }))
          .route("/cors-preflight", axum::routing::options(handle_cors_preflight))
          .layer(ServiceBuilder::new().layer(create_cors_layer()))
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这个配置允许所有域名访问吗？| Does this configuration allow access from all domains?
    **答案 | Answer:** 不允许，只允许example.com | No, only allows example.com
  - 预检请求会缓存多长时间？| How long will preflight requests be cached?
    **答案 | Answer:** 使用浏览器默认值，可通过max_age配置 | Uses browser default, configurable via max_age

- **CSP内容安全策略 | CSP Content Security Policy**
  
  **概念定义 | Concept Definition:**
  CSP是一种安全机制，通过声明白名单来控制网页可以加载和执行的资源，有效防止XSS攻击、代码注入和数据泄露。| CSP is a security mechanism that controls what resources a webpage can load and execute through declaring whitelists, effectively preventing XSS attacks, code injection, and data leaks.
  
  **核心特征 | Key Characteristics:**
  - 通过HTTP头或meta标签定义安全策略 | Defines security policy through HTTP headers or meta tags
  - 支持多种指令控制不同类型资源 | Supports multiple directives to control different resource types
  - 可以阻止内联脚本和样式的执行 | Can block execution of inline scripts and styles
  - 提供报告机制记录策略违规 | Provides reporting mechanism to log policy violations
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. CSP能完全防止XSS攻击吗？| Can CSP completely prevent XSS attacks?
     **答案 | Answer:** 不能完全防止，但能显著降低风险 | Cannot completely prevent, but can significantly reduce risk
  2. 内联JavaScript在strict CSP下能执行吗？| Can inline JavaScript execute under strict CSP?
     **答案 | Answer:** 不能，除非使用nonce或hash | No, unless using nonce or hash
  3. CSP策略违规会阻止页面加载吗？| Will CSP policy violations prevent page loading?
     **答案 | Answer:** 取决于是enforce还是report-only模式 | Depends on whether it's enforce or report-only mode
  4. default-src指令的作用是什么？| What is the role of the default-src directive?
     **答案 | Answer:** 为其他资源类型提供默认策略 | Provides default policy for other resource types
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // CSP中间件实现 | CSP middleware implementation
  use axum::{
      http::{header, HeaderValue, Request, Response},
      middleware::Next,
      response::Html,
      routing::get,
      Router,
  };
  
  // CSP策略配置 | CSP policy configuration
  const CSP_POLICY: &str = concat!(
      "default-src 'self'; ",                    // 默认只允许同源资源 | Default allows only same-origin resources
      "script-src 'self' 'unsafe-inline'; ",    // 脚本源策略 | Script source policy
      "style-src 'self' 'unsafe-inline'; ",     // 样式源策略 | Style source policy  
      "img-src 'self' data: https:; ",          // 图片源策略 | Image source policy
      "font-src 'self'; ",                      // 字体源策略 | Font source policy
      "connect-src 'self'; ",                   // 连接源策略 | Connection source policy
      "media-src 'none'; ",                     // 媒体源策略 | Media source policy
      "object-src 'none'; ",                    // 对象源策略 | Object source policy
      "base-uri 'self'; ",                      // 基础URI策略 | Base URI policy
      "form-action 'self'; ",                   // 表单动作策略 | Form action policy
      "frame-ancestors 'none'; ",               // 框架祖先策略 | Frame ancestors policy
      "report-uri /csp-report"                  // 违规报告URI | Violation report URI
  );
  
  // CSP中间件 | CSP middleware
  async fn csp_middleware<B>(
      request: Request<B>,
      next: Next<B>,
  ) -> Response {
      let mut response = next.run(request).await;
      
      // 添加CSP头 | Add CSP header
      response.headers_mut().insert(
          header::CONTENT_SECURITY_POLICY,
          HeaderValue::from_static(CSP_POLICY),
      );
      
      // 添加报告模式CSP头 | Add report-only CSP header
      response.headers_mut().insert(
          header::CONTENT_SECURITY_POLICY_REPORT_ONLY,
          HeaderValue::from_static("script-src 'self'; report-uri /csp-report-only"),
      );
      
      response
  }
  
  // CSP违规报告处理 | CSP violation report handling
  async fn handle_csp_report(body: String) -> &'static str {
      println!("CSP违规报告 | CSP Violation Report: {}", body);
      // 这里可以记录到日志系统或监控系统 | Here you can log to logging system or monitoring system
      "Report received"
  }
  
  // 安全的HTML页面 | Secure HTML page
  async fn secure_page() -> Html<&'static str> {
      Html(r#"
      <!DOCTYPE html>
      <html>
      <head>
          <title>CSP保护页面 | CSP Protected Page</title>
          <style>
              body { font-family: Arial, sans-serif; }
              .safe { color: green; }
          </style>
      </head>
      <body>
          <h1>CSP安全演示 | CSP Security Demo</h1>
          <p class="safe">这个页面受CSP保护 | This page is protected by CSP</p>
          <script>
              // 这个内联脚本需要'unsafe-inline'或nonce | This inline script requires 'unsafe-inline' or nonce
              console.log('CSP允许的脚本 | CSP allowed script');
          </script>
      </body>
      </html>
      "#)
  }
  ```

### 3. HSTS与其他安全头 | HSTS and Other Security Headers (45分钟 | 45 minutes)

- **HSTS强制HTTPS传输安全 | HSTS HTTP Strict Transport Security**
  
  **概念定义 | Concept Definition:**
  HSTS是一种Web安全策略机制，强制浏览器使用HTTPS连接与网站通信，防止协议降级攻击和中间人攻击。一旦启用，浏览器会自动将HTTP请求重定向为HTTPS。| HSTS is a web security policy mechanism that forces browsers to communicate with websites using HTTPS connections, preventing protocol downgrade attacks and man-in-the-middle attacks. Once enabled, browsers automatically redirect HTTP requests to HTTPS.
  
  **核心特征 | Key Characteristics:**
  - 浏览器会记住HSTS策略并在指定时间内强制使用HTTPS | Browser remembers HSTS policy and enforces HTTPS for specified time
  - 包含子域名保护选项，防止子域名攻击 | Includes subdomain protection option to prevent subdomain attacks
  - 支持预加载列表，在首次访问前就启用保护 | Supports preload list to enable protection before first visit
  - 有效防止SSL剥离攻击和协议降级 | Effectively prevents SSL stripping and protocol downgrade attacks
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. HSTS策略在HTTP连接中也会生效吗？| Does HSTS policy take effect in HTTP connections?
     **答案 | Answer:** 不会，只在HTTPS连接中传输和生效 | No, only transmitted and effective in HTTPS connections
  2. max-age设为0会发生什么？| What happens when max-age is set to 0?
     **答案 | Answer:** 浏览器会删除已存储的HSTS策略 | Browser will remove stored HSTS policy
  3. includeSubDomains指令的作用是什么？| What is the function of the includeSubDomains directive?
     **答案 | Answer:** 将HSTS策略应用到所有子域名 | Apply HSTS policy to all subdomains
  4. 用户能够绕过HSTS策略访问HTTP版本吗？| Can users bypass HSTS policy to access HTTP version?
     **答案 | Answer:** 不能，浏览器会强制重定向到HTTPS | No, browser will force redirect to HTTPS
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 完整的安全头中间件实现 | Complete security headers middleware implementation
  use axum::{
      http::{header, HeaderValue, Request, Response},
      middleware::Next,
      routing::get,
      Router,
  };
  
  // 安全头配置 | Security headers configuration
  async fn security_headers_middleware<B>(
      request: Request<B>,
      next: Next<B>,
  ) -> Response {
      let mut response = next.run(request).await;
      let headers = response.headers_mut();
      
      // HSTS - 强制HTTPS，包含子域名，预加载 | HSTS - Force HTTPS, include subdomains, preload
      headers.insert(
          header::STRICT_TRANSPORT_SECURITY,
          HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
      );
      
      // X-Frame-Options - 防止点击劫持 | X-Frame-Options - Prevent clickjacking
      headers.insert(
          header::X_FRAME_OPTIONS,
          HeaderValue::from_static("DENY"),
      );
      
      // X-Content-Type-Options - 防止MIME类型嗅探 | X-Content-Type-Options - Prevent MIME sniffing
      headers.insert(
          header::X_CONTENT_TYPE_OPTIONS,
          HeaderValue::from_static("nosniff"),
      );
      
      // X-XSS-Protection - XSS过滤 | X-XSS-Protection - XSS filtering
      headers.insert(
          HeaderValue::from_static("x-xss-protection"),
          HeaderValue::from_static("1; mode=block"),
      );
      
      // Referrer-Policy - 控制referrer信息泄露 | Referrer-Policy - Control referrer information leakage
      headers.insert(
          header::REFERRER_POLICY,
          HeaderValue::from_static("strict-origin-when-cross-origin"),
      );
      
      // Permissions-Policy - 控制浏览器功能访问 | Permissions-Policy - Control browser feature access
      headers.insert(
          HeaderValue::from_static("permissions-policy"),
          HeaderValue::from_static(
              "camera=(), microphone=(), geolocation=(), interest-cohort=()"
          ),
      );
      
      response
  }
  
  // HTTPS重定向中间件 | HTTPS redirect middleware
  async fn https_redirect_middleware<B>(
      request: Request<B>,
      next: Next<B>,
  ) -> Response {
      // 检查是否为HTTPS连接 | Check if it's HTTPS connection
      let is_https = request.headers()
          .get("x-forwarded-proto")
          .and_then(|h| h.to_str().ok())
          .map(|proto| proto == "https")
          .unwrap_or(false);
      
      if !is_https {
          // 非HTTPS连接，重定向到HTTPS | Non-HTTPS connection, redirect to HTTPS
          let host = request.headers()
              .get(header::HOST)
              .and_then(|h| h.to_str().ok())
              .unwrap_or("localhost");
          
          let https_url = format!("https://{}{}", host, request.uri().path_and_query()
              .map(|pq| pq.as_str())
              .unwrap_or("/"));
          
          return Response::builder()
              .status(axum::http::StatusCode::MOVED_PERMANENTLY)
              .header(header::LOCATION, https_url)
              .body("Redirecting to HTTPS".into())
              .unwrap();
      }
      
      next.run(request).await
  }
  ```

### 4. 认证与授权机制 | Authentication and Authorization Mechanisms (1小时 | 1 hour)

- **JWT令牌认证 | JWT Token Authentication**
  
  **概念定义 | Concept Definition:**
  JWT (JSON Web Token) 是一种开放标准的令牌格式，用于在各方之间安全地传输信息。它包含头部、载荷和签名三部分，支持无状态认证，广泛用于微服务架构中。| JWT (JSON Web Token) is an open standard token format for securely transmitting information between parties. It contains header, payload, and signature parts, supports stateless authentication, and is widely used in microservice architectures.
  
  **核心特征 | Key Characteristics:**
  - 自包含的令牌，包含所有必要的用户信息 | Self-contained tokens containing all necessary user information
  - 无状态设计，服务器不需要存储会话信息 | Stateless design, server doesn't need to store session information
  - 支持数字签名验证令牌完整性和真实性 | Supports digital signature verification for token integrity and authenticity
  - 可以设置过期时间，支持令牌自动失效 | Can set expiration time, supports automatic token expiration
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. JWT令牌可以在客户端修改吗？| Can JWT tokens be modified on the client side?
     **答案 | Answer:** 可以修改但会导致签名验证失败 | Can be modified but will cause signature verification failure
  2. JWT令牌需要在服务器端存储吗？| Do JWT tokens need to be stored on the server side?
     **答案 | Answer:** 不需要，这是无状态认证的优势 | No, this is the advantage of stateless authentication
  3. 过期的JWT令牌还能使用吗？| Can expired JWT tokens still be used?
     **答案 | Answer:** 不能，验证时会检查过期时间 | No, expiration time is checked during verification
  4. JWT的签名算法可以是对称的吗？| Can JWT signature algorithms be symmetric?
     **答案 | Answer:** 可以，如HMAC，也支持非对称算法如RSA | Yes, like HMAC, also supports asymmetric algorithms like RSA
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // JWT认证中间件实现 | JWT authentication middleware implementation
  use axum::{
      extract::{Request, State},
      http::{header, StatusCode},
      middleware::Next,
      response::Response,
      Json,
  };
  use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
  use serde::{Deserialize, Serialize};
  use std::sync::Arc;
  use time::{Duration, OffsetDateTime};
  
  // JWT载荷结构 | JWT payload structure
  #[derive(Debug, Serialize, Deserialize)]
  struct Claims {
      sub: String,        // 用户ID | User ID
      exp: i64,          // 过期时间 | Expiration time
      iat: i64,          // 签发时间 | Issued at
      roles: Vec<String>, // 用户角色 | User roles
      permissions: Vec<String>, // 用户权限 | User permissions
  }
  
  // JWT配置 | JWT configuration
  #[derive(Clone)]
  struct JwtConfig {
      encoding_key: EncodingKey,
      decoding_key: DecodingKey,
      algorithm: Algorithm,
  }
  
  impl JwtConfig {
      fn new(secret: &[u8]) -> Self {
          Self {
              encoding_key: EncodingKey::from_secret(secret),
              decoding_key: DecodingKey::from_secret(secret),
              algorithm: Algorithm::HS256,
          }
      }
  }
  
  // 生成JWT令牌 | Generate JWT token
  fn generate_jwt(
      user_id: &str,
      roles: Vec<String>,
      permissions: Vec<String>,
      config: &JwtConfig,
  ) -> Result<String, jsonwebtoken::errors::Error> {
      let now = OffsetDateTime::now_utc().unix_timestamp();
      let expiration = (OffsetDateTime::now_utc() + Duration::hours(24)).unix_timestamp();
      
      let claims = Claims {
          sub: user_id.to_string(),
          exp: expiration,
          iat: now,
          roles,
          permissions,
      };
      
      encode(&Header::new(config.algorithm), &claims, &config.encoding_key)
  }
  
  // 验证JWT令牌 | Verify JWT token
  fn verify_jwt(token: &str, config: &JwtConfig) -> Result<Claims, jsonwebtoken::errors::Error> {
      let mut validation = Validation::new(config.algorithm);
      validation.validate_exp = true; // 验证过期时间 | Verify expiration time
      
      decode::<Claims>(token, &config.decoding_key, &validation)
          .map(|token_data| token_data.claims)
  }
  
  // JWT认证中间件 | JWT authentication middleware
  async fn jwt_auth_middleware(
      State(config): State<Arc<JwtConfig>>,
      mut request: Request,
      next: Next,
  ) -> Result<Response, StatusCode> {
      // 从Authorization头获取令牌 | Get token from Authorization header
      let auth_header = request.headers()
          .get(header::AUTHORIZATION)
          .and_then(|h| h.to_str().ok())
          .ok_or(StatusCode::UNAUTHORIZED)?;
      
      // 检查Bearer前缀 | Check Bearer prefix
      let token = auth_header
          .strip_prefix("Bearer ")
          .ok_or(StatusCode::UNAUTHORIZED)?;
      
      // 验证JWT令牌 | Verify JWT token
      let claims = verify_jwt(token, &config)
          .map_err(|_| StatusCode::UNAUTHORIZED)?;
      
      // 将用户信息添加到请求扩展中 | Add user info to request extensions
      request.extensions_mut().insert(claims);
      
      Ok(next.run(request).await)
  }
  
  // 用户登录API | User login API
  #[derive(Deserialize)]
  struct LoginRequest {
      username: String,
      password: String,
  }
  
  #[derive(Serialize)]
  struct LoginResponse {
      token: String,
      expires_in: i64,
  }
  
  async fn login(
      State(config): State<Arc<JwtConfig>>,
      Json(login_req): Json<LoginRequest>,
  ) -> Result<Json<LoginResponse>, StatusCode> {
      // 这里应该验证用户凭据 | Here should verify user credentials
      // 为演示目的，我们假设验证成功 | For demo purposes, we assume verification succeeds
      if login_req.username == "admin" && login_req.password == "password" {
          let roles = vec!["admin".to_string()];
          let permissions = vec!["read".to_string(), "write".to_string()];
          
          let token = generate_jwt(&login_req.username, roles, permissions, &config)
              .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
          
          Ok(Json(LoginResponse {
              token,
              expires_in: 24 * 3600, // 24小时 | 24 hours
          }))
      } else {
          Err(StatusCode::UNAUTHORIZED)
      }
  }
  ```

- **基于角色的访问控制(RBAC) | Role-Based Access Control (RBAC)**
  
  **概念定义 | Concept Definition:**
  RBAC是一种访问控制模型，通过角色来管理用户权限。用户被分配到角色，角色拥有特定的权限，这种层次化的权限管理简化了复杂系统的授权管理。| RBAC is an access control model that manages user permissions through roles. Users are assigned to roles, and roles have specific permissions. This hierarchical permission management simplifies authorization management in complex systems.
  
  **核心特征 | Key Characteristics:**
  - 用户、角色、权限三层架构模型 | Three-tier architecture model: users, roles, permissions
  - 支持角色继承和层次化权限管理 | Supports role inheritance and hierarchical permission management
  - 权限可以细化到具体的资源和操作 | Permissions can be refined to specific resources and operations
  - 便于权限的批量管理和审计 | Facilitates batch permission management and auditing
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 一个用户可以拥有多个角色吗？| Can a user have multiple roles?
     **答案 | Answer:** 可以，这是RBAC的常见场景 | Yes, this is a common scenario in RBAC
  2. 角色权限的变更会立即影响所有该角色的用户吗？| Do role permission changes immediately affect all users with that role?
     **答案 | Answer:** 取决于实现，通常需要重新认证或刷新 | Depends on implementation, usually requires re-authentication or refresh
  3. RBAC中的权限可以是动态的吗？| Can permissions in RBAC be dynamic?
     **答案 | Answer:** 可以，支持基于上下文的动态权限 | Yes, supports context-based dynamic permissions
  4. 角色继承中子角色会获得父角色的所有权限吗？| In role inheritance, do child roles get all parent role permissions?
     **答案 | Answer:** 是的，这是继承的基本原理 | Yes, this is the basic principle of inheritance
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // RBAC权限系统实现 | RBAC permission system implementation
  use axum::{
      extract::{Path, Request, State},
      http::StatusCode,
      middleware::Next,
      response::Response,
      Json,
  };
  use serde::{Deserialize, Serialize};
  use std::collections::{HashMap, HashSet};
  use std::sync::Arc;
  use tokio::sync::RwLock;
  
  // 权限定义 | Permission definition
  #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
  struct Permission {
      resource: String,    // 资源名称 | Resource name
      action: String,      // 操作类型 | Action type
  }
  
  // 角色定义 | Role definition
  #[derive(Debug, Clone, Serialize, Deserialize)]
  struct Role {
      name: String,
      permissions: HashSet<Permission>,
      parent_roles: Vec<String>, // 父角色 | Parent roles
  }
  
  // 用户定义 | User definition
  #[derive(Debug, Clone, Serialize, Deserialize)]
  struct User {
      id: String,
      username: String,
      roles: Vec<String>,
  }
  
  // RBAC系统 | RBAC system
  #[derive(Debug)]
  struct RbacSystem {
      users: HashMap<String, User>,
      roles: HashMap<String, Role>,
  }
  
  impl RbacSystem {
      fn new() -> Self {
          let mut system = Self {
              users: HashMap::new(),
              roles: HashMap::new(),
          };
          
          // 初始化默认角色 | Initialize default roles
          system.create_default_roles();
          system
      }
      
      // 创建默认角色 | Create default roles
      fn create_default_roles(&mut self) {
          // 管理员角色 | Admin role
          let admin_permissions = [
              Permission { resource: "users".to_string(), action: "read".to_string() },
              Permission { resource: "users".to_string(), action: "write".to_string() },
              Permission { resource: "users".to_string(), action: "delete".to_string() },
              Permission { resource: "posts".to_string(), action: "read".to_string() },
              Permission { resource: "posts".to_string(), action: "write".to_string() },
              Permission { resource: "posts".to_string(), action: "delete".to_string() },
          ].into_iter().collect();
          
          self.roles.insert("admin".to_string(), Role {
              name: "admin".to_string(),
              permissions: admin_permissions,
              parent_roles: vec![],
          });
          
          // 编辑者角色 | Editor role
          let editor_permissions = [
              Permission { resource: "posts".to_string(), action: "read".to_string() },
              Permission { resource: "posts".to_string(), action: "write".to_string() },
          ].into_iter().collect();
          
          self.roles.insert("editor".to_string(), Role {
              name: "editor".to_string(),
              permissions: editor_permissions,
              parent_roles: vec![],
          });
          
          // 读者角色 | Reader role
          let reader_permissions = [
              Permission { resource: "posts".to_string(), action: "read".to_string() },
          ].into_iter().collect();
          
          self.roles.insert("reader".to_string(), Role {
              name: "reader".to_string(),
              permissions: reader_permissions,
              parent_roles: vec![],
          });
      }
      
      // 检查用户权限 | Check user permissions
      fn check_permission(&self, user_id: &str, resource: &str, action: &str) -> bool {
          let user = match self.users.get(user_id) {
              Some(user) => user,
              None => return false,
          };
          
          let required_permission = Permission {
              resource: resource.to_string(),
              action: action.to_string(),
          };
          
          // 遍历用户的所有角色 | Iterate through all user roles
          for role_name in &user.roles {
              if let Some(role) = self.roles.get(role_name) {
                  if self.role_has_permission(role, &required_permission) {
                      return true;
                  }
              }
          }
          
          false
      }
      
      // 检查角色是否有权限（包括继承） | Check if role has permission (including inheritance)
      fn role_has_permission(&self, role: &Role, permission: &Permission) -> bool {
          // 检查直接权限 | Check direct permissions
          if role.permissions.contains(permission) {
              return true;
          }
          
          // 检查继承的权限 | Check inherited permissions
          for parent_role_name in &role.parent_roles {
              if let Some(parent_role) = self.roles.get(parent_role_name) {
                  if self.role_has_permission(parent_role, permission) {
                      return true;
                  }
              }
          }
          
          false
      }
      
      // 获取用户的所有权限 | Get all user permissions
      fn get_user_permissions(&self, user_id: &str) -> HashSet<Permission> {
          let mut permissions = HashSet::new();
          
          if let Some(user) = self.users.get(user_id) {
              for role_name in &user.roles {
                  if let Some(role) = self.roles.get(role_name) {
                      permissions.extend(self.get_role_all_permissions(role));
                  }
              }
          }
          
          permissions
      }
      
      // 获取角色的所有权限（包括继承） | Get all role permissions (including inheritance)
      fn get_role_all_permissions(&self, role: &Role) -> HashSet<Permission> {
          let mut permissions = role.permissions.clone();
          
          for parent_role_name in &role.parent_roles {
              if let Some(parent_role) = self.roles.get(parent_role_name) {
                  permissions.extend(self.get_role_all_permissions(parent_role));
              }
          }
          
          permissions
      }
  }
  
  // 权限检查中间件 | Permission check middleware
  async fn rbac_middleware(
      State(rbac_system): State<Arc<RwLock<RbacSystem>>>,
      Path((resource, action)): Path<(String, String)>,
      mut request: Request,
      next: Next,
  ) -> Result<Response, StatusCode> {
      // 从请求扩展中获取用户信息 | Get user info from request extensions
      let claims = request.extensions()
          .get::<Claims>()
          .ok_or(StatusCode::UNAUTHORIZED)?;
      
      // 检查权限 | Check permissions
      let rbac = rbac_system.read().await;
      let has_permission = rbac.check_permission(&claims.sub, &resource, &action);
      
      if !has_permission {
          return Err(StatusCode::FORBIDDEN);
      }
      
      Ok(next.run(request).await)
  }
  ```

### 5. 安全最佳实践 | Security Best Practices (30分钟 | 30 minutes)

- **输入验证与输出编码 | Input Validation and Output Encoding**
  
  **概念定义 | Concept Definition:**
  输入验证是对用户输入数据进行检查和清理的过程，输出编码是在将数据输出到不同上下文时进行适当编码的过程。这两个机制共同防止注入攻击和数据污染。| Input validation is the process of checking and sanitizing user input data, while output encoding is the process of properly encoding data when outputting to different contexts. These two mechanisms together prevent injection attacks and data contamination.
  
  **核心特征 | Key Characteristics:**
  - 在信任边界处验证所有外部输入 | Validate all external inputs at trust boundaries
  - 使用白名单验证而非黑名单过滤 | Use whitelist validation rather than blacklist filtering
  - 根据输出上下文选择合适的编码方式 | Choose appropriate encoding methods based on output context
  - 对敏感数据进行额外的验证和保护 | Provide additional validation and protection for sensitive data
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 客户端的输入验证能代替服务端验证吗？| Can client-side input validation replace server-side validation?
     **答案 | Answer:** 不能，客户端验证可以被绕过 | No, client-side validation can be bypassed
  2. 对于用户密码应该如何存储？| How should user passwords be stored?
     **答案 | Answer:** 使用加盐哈希，永不存储明文 | Use salted hashing, never store plaintext
  3. SQL注入主要通过什么方式防护？| How is SQL injection primarily prevented?
     **答案 | Answer:** 使用参数化查询或预编译语句 | Use parameterized queries or prepared statements
  4. XSS攻击的主要防护手段是什么？| What is the main defense against XSS attacks?
     **答案 | Answer:** 输出编码和CSP策略 | Output encoding and CSP policies
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 安全的输入验证和输出编码 | Secure input validation and output encoding
  use axum::{
      extract::Json,
      http::StatusCode,
      response::{Html, Json as ResponseJson},
  };
  use regex::Regex;
  use serde::{Deserialize, Serialize};
  use std::collections::HashMap;
  use validator::{Validate, ValidationError};
  
  // 用户输入数据结构 | User input data structure
  #[derive(Debug, Deserialize, Validate)]
  struct UserInput {
      #[validate(length(min = 3, max = 50, message = "用户名长度必须在3-50字符之间 | Username length must be between 3-50 characters"))]
      #[validate(regex(path = "USERNAME_REGEX", message = "用户名只能包含字母、数字和下划线 | Username can only contain letters, numbers and underscores"))]
      username: String,
      
      #[validate(email(message = "邮箱格式不正确 | Invalid email format"))]
      email: String,
      
      #[validate(length(min = 8, message = "密码长度至少8位 | Password must be at least 8 characters"))]
      #[validate(custom(function = "validate_password_strength", message = "密码强度不足 | Password strength insufficient"))]
      password: String,
      
      #[validate(range(min = 18, max = 120, message = "年龄必须在18-120之间 | Age must be between 18-120"))]
      age: u8,
  }
  
  // 正则表达式常量 | Regular expression constants
  lazy_static::lazy_static! {
      static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
      static ref SCRIPT_TAG_REGEX: Regex = Regex::new(r"<script[^>]*>.*?</script>").unwrap();
  }
  
  // 自定义密码强度验证 | Custom password strength validation
  fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
      let has_lowercase = password.chars().any(|c| c.is_lowercase());
      let has_uppercase = password.chars().any(|c| c.is_uppercase());
      let has_digit = password.chars().any(|c| c.is_numeric());
      let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
      
      if has_lowercase && has_uppercase && has_digit && has_special {
          Ok(())
      } else {
          Err(ValidationError::new("password_too_weak"))
      }
  }
  
  // HTML编码函数 | HTML encoding function
  fn html_encode(input: &str) -> String {
      input
          .replace('&', "&amp;")
          .replace('<', "&lt;")
          .replace('>', "&gt;")
          .replace('"', "&quot;")
          .replace('\'', "&#x27;")
          .replace('/', "&#x2F;")
  }
  
  // JavaScript编码函数 | JavaScript encoding function
  fn js_encode(input: &str) -> String {
      input
          .chars()
          .map(|c| match c {
              '"' => "\\\"".to_string(),
              '\'' => "\\'".to_string(),
              '\\' => "\\\\".to_string(),
              '\n' => "\\n".to_string(),
              '\r' => "\\r".to_string(),
              '\t' => "\\t".to_string(),
              c if c.is_control() => format!("\\u{:04x}", c as u32),
              c => c.to_string(),
          })
          .collect()
  }
  
  // URL编码函数 | URL encoding function
  fn url_encode(input: &str) -> String {
      urlencoding::encode(input).to_string()
  }
  
  // 输入清理函数 | Input sanitization function
  fn sanitize_input(input: &str) -> String {
      // 移除潜在的脚本标签 | Remove potential script tags
      let cleaned = SCRIPT_TAG_REGEX.replace_all(input, "");
      // 限制长度 | Limit length
      if cleaned.len() > 1000 {
          cleaned[..1000].to_string()
      } else {
          cleaned.to_string()
      }
  }
  
  // 安全的用户创建API | Secure user creation API
  async fn create_user(
      Json(mut user_input): Json<UserInput>,
  ) -> Result<ResponseJson<HashMap<String, String>>, StatusCode> {
      // 输入验证 | Input validation
      if let Err(errors) = user_input.validate() {
          let error_messages: Vec<String> = errors
              .field_errors()
              .iter()
              .flat_map(|(field, errors)| {
                  errors.iter().map(move |error| {
                      format!("{}: {}", field, error.message.as_ref().unwrap_or(&"验证失败".into()))
                  })
              })
              .collect();
          
          return Err(StatusCode::BAD_REQUEST);
      }
      
      // 输入清理 | Input sanitization
      user_input.username = sanitize_input(&user_input.username);
      user_input.email = sanitize_input(&user_input.email);
      
      // 密码哈希处理 | Password hashing
      let password_hash = hash_password(&user_input.password)
          .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
      
      // 这里应该将用户保存到数据库 | Here should save user to database
      // 为演示目的，返回成功响应 | For demo purposes, return success response
      let mut response = HashMap::new();
      response.insert("status".to_string(), "success".to_string());
      response.insert("message".to_string(), "用户创建成功 | User created successfully".to_string());
      response.insert("username".to_string(), html_encode(&user_input.username));
      
      Ok(ResponseJson(response))
  }
  
  // 密码哈希函数 | Password hashing function
  fn hash_password(password: &str) -> Result<String, argon2::Error> {
      use argon2::{Argon2, PasswordHasher};
      use argon2::password_hash::{SaltString, rand_core::OsRng};
      
      let salt = SaltString::generate(&mut OsRng);
      let argon2 = Argon2::default();
      let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
      
      Ok(password_hash.to_string())
  }
  
  // 安全的HTML响应生成 | Secure HTML response generation
  async fn safe_html_response(user_data: String) -> Html<String> {
      let safe_user_data = html_encode(&user_data);
      let html_content = format!(
          r#"
          <!DOCTYPE html>
          <html>
          <head>
              <title>安全页面 | Secure Page</title>
              <meta charset="UTF-8">
          </head>
          <body>
              <h1>用户信息 | User Information</h1>
              <p>用户数据 | User Data: {}</p>
              <script>
                  // 安全的JavaScript数据传递 | Safe JavaScript data passing
                  const userData = "{}";
                  console.log("用户数据 | User data:", userData);
              </script>
          </body>
          </html>
          "#,
          safe_user_data,
          js_encode(&user_data)
      );
      
      Html(html_content)
  }
  ```

### 6. 综合安全配置 | Comprehensive Security Configuration (15分钟 | 15 minutes)

- **生产级安全配置汇总 | Production-Grade Security Configuration Summary**
  
  **概念定义 | Concept Definition:**
  生产级安全配置是将各种安全机制综合应用的最佳实践，包括传输安全、应用安全、数据安全和运维安全等多个层面的协调配置，确保系统的整体安全性。| Production-grade security configuration is the best practice of comprehensively applying various security mechanisms, including coordinated configurations across multiple layers such as transport security, application security, data security, and operational security to ensure overall system security.
  
  **核心特征 | Key Characteristics:**
  - 多层防御策略，每层都有独立的安全控制 | Multi-layered defense strategy with independent security controls at each layer
  - 安全配置应该是默认启用和安全的 | Security configurations should be enabled and secure by default
  - 定期更新和审计安全配置 | Regular updates and audits of security configurations
  - 监控和日志记录所有安全相关事件 | Monitor and log all security-related events
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 生产环境中应该禁用哪些调试功能？| What debugging features should be disabled in production?
     **答案 | Answer:** 详细错误信息、调试日志、开发者工具 | Detailed error messages, debug logs, developer tools
  2. 安全配置的更新应该如何管理？| How should security configuration updates be managed?
     **答案 | Answer:** 版本控制、测试验证、分阶段部署 | Version control, testing validation, staged deployment
  3. 安全事件的日志应该包含哪些信息？| What information should security event logs contain?
     **答案 | Answer:** 时间戳、用户ID、操作类型、结果状态 | Timestamp, user ID, operation type, result status
  4. 如何确保安全配置在容器化环境中的一致性？| How to ensure consistency of security configurations in containerized environments?
     **答案 | Answer:** 使用配置管理工具和基础镜像 | Use configuration management tools and base images
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 生产级安全配置整合 | Production-grade security configuration integration
  use axum::{
      middleware,
      routing::{get, post},
      Router,
  };
  use std::sync::Arc;
  use tokio::sync::RwLock;
  use tower::ServiceBuilder;
  use tower_http::{
      cors::CorsLayer,
      compression::CompressionLayer,
      timeout::TimeoutLayer,
      trace::TraceLayer,
  };
  use std::time::Duration;
  
  // 应用配置结构 | Application configuration structure
  #[derive(Clone)]
  struct AppConfig {
      jwt_config: Arc<JwtConfig>,
      rbac_system: Arc<RwLock<RbacSystem>>,
      security_settings: SecuritySettings,
  }
  
  #[derive(Clone)]
  struct SecuritySettings {
      max_request_size: usize,
      rate_limit_per_minute: u32,
      session_timeout_hours: u64,
      password_policy: PasswordPolicy,
  }
  
  #[derive(Clone)]
  struct PasswordPolicy {
      min_length: usize,
      require_uppercase: bool,
      require_lowercase: bool,
      require_numbers: bool,
      require_special_chars: bool,
      max_age_days: u32,
  }
  
  // 创建安全的应用路由 | Create secure application router
  fn create_secure_app(config: AppConfig) -> Router {
      Router::new()
          // 公共路由 | Public routes
          .route("/health", get(health_check))
          .route("/login", post(login))
          
          // 需要认证的路由 | Authenticated routes
          .route("/api/profile", get(get_profile))
          .route("/api/users", get(list_users))
          .route("/api/users", post(create_user))
          
          // 需要管理员权限的路由 | Admin-only routes  
          .route("/admin/users", get(admin_list_users))
          .route("/admin/system", get(system_status))
          
          // 应用状态 | Application state
          .with_state(config.clone())
          
          // 中间件层 | Middleware layers
          .layer(
              ServiceBuilder::new()
                  // 请求追踪 | Request tracing
                  .layer(TraceLayer::new_for_http())
                  
                  // 请求超时 | Request timeout
                  .layer(TimeoutLayer::new(Duration::from_secs(30)))
                  
                  // 响应压缩 | Response compression
                  .layer(CompressionLayer::new())
                  
                  // CORS配置 | CORS configuration
                  .layer(create_cors_layer())
                  
                  // 安全头 | Security headers
                  .layer(middleware::from_fn(security_headers_middleware))
                  
                  // 速率限制 | Rate limiting
                  .layer(middleware::from_fn_with_state(
                      config.clone(),
                      rate_limiting_middleware,
                  ))
                  
                  // HTTPS重定向 | HTTPS redirect
                  .layer(middleware::from_fn(https_redirect_middleware))
          )
          
          // JWT认证中间件（仅用于API路由） | JWT auth middleware (for API routes only)
          .nest("/api", api_routes().layer(middleware::from_fn_with_state(
              config.jwt_config.clone(),
              jwt_auth_middleware,
          )))
          
          // 管理员权限中间件 | Admin permission middleware
          .nest("/admin", admin_routes().layer(middleware::from_fn_with_state(
              config.rbac_system.clone(),
              admin_permission_middleware,
          )))
  }
  
  // API路由 | API routes
  fn api_routes() -> Router<AppConfig> {
      Router::new()
          .route("/profile", get(get_profile))
          .route("/users", get(list_users))
          .route("/users", post(create_user))
  }
  
  // 管理员路由 | Admin routes
  fn admin_routes() -> Router<AppConfig> {
      Router::new()
          .route("/users", get(admin_list_users))
          .route("/system", get(system_status))
  }
  
  // 速率限制中间件 | Rate limiting middleware
  async fn rate_limiting_middleware(
      State(config): State<AppConfig>,
      request: Request,
      next: Next,
  ) -> Result<Response, StatusCode> {
      // 实现基于IP的速率限制 | Implement IP-based rate limiting
      // 这里应该使用Redis或内存存储来跟踪请求频率 | Here should use Redis or memory store to track request frequency
      
      // 获取客户端IP | Get client IP
      let client_ip = request.headers()
          .get("x-forwarded-for")
          .or_else(|| request.headers().get("x-real-ip"))
          .and_then(|h| h.to_str().ok())
          .unwrap_or("unknown");
      
      // 检查速率限制（这里简化为始终通过） | Check rate limit (simplified to always pass)
      // 在实际实现中，应该检查该IP在过去一分钟内的请求次数 | In actual implementation, should check request count for this IP in the past minute
      
      Ok(next.run(request).await)
  }
  
  // 管理员权限中间件 | Admin permission middleware
  async fn admin_permission_middleware(
      State(rbac_system): State<Arc<RwLock<RbacSystem>>>,
      request: Request,
      next: Next,
  ) -> Result<Response, StatusCode> {
      // 检查用户是否有管理员角色 | Check if user has admin role
      let claims = request.extensions()
          .get::<Claims>()
          .ok_or(StatusCode::UNAUTHORIZED)?;
      
      if !claims.roles.contains(&"admin".to_string()) {
          return Err(StatusCode::FORBIDDEN);
      }
      
      Ok(next.run(request).await)
  }
  
  // 健康检查端点 | Health check endpoint
  async fn health_check() -> &'static str {
      "OK"
  }
  
  // 系统状态端点（管理员） | System status endpoint (admin)
  async fn system_status() -> Json<serde_json::Value> {
      Json(serde_json::json!({
          "status": "healthy",
          "timestamp": chrono::Utc::now().to_rfc3339(),
          "version": env!("CARGO_PKG_VERSION")
      }))
  }
  
  // 启动安全的HTTPS服务器 | Start secure HTTPS server
  #[tokio::main]
  async fn main() -> Result<(), Box<dyn std::error::Error>> {
      // 初始化日志 | Initialize logging
      tracing_subscriber::fmt::init();
      
      // 创建应用配置 | Create application configuration
      let config = AppConfig {
          jwt_config: Arc::new(JwtConfig::new(b"your-secret-key-change-this-in-production")),
          rbac_system: Arc::new(RwLock::new(RbacSystem::new())),
          security_settings: SecuritySettings {
              max_request_size: 1024 * 1024, // 1MB
              rate_limit_per_minute: 60,
              session_timeout_hours: 24,
              password_policy: PasswordPolicy {
                  min_length: 8,
                  require_uppercase: true,
                  require_lowercase: true,
                  require_numbers: true,
                  require_special_chars: true,
                  max_age_days: 90,
              },
          },
      };
      
      // 创建应用 | Create application
      let app = create_secure_app(config);
      
      // TLS配置 | TLS configuration
      let tls_config = load_tls_config()?;
      
      // 启动HTTPS服务器 | Start HTTPS server
      let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
      println!("🔒 安全服务器启动在 https://localhost:3000 | 🔒 Secure server starting on https://localhost:3000");
      
      axum::serve(listener, app.into_make_service())
          .with_graceful_shutdown(shutdown_signal())
          .await?;
      
      Ok(())
  }
  
  // 优雅关闭信号 | Graceful shutdown signal
  async fn shutdown_signal() {
      tokio::signal::ctrl_c()
          .await
          .expect("监听Ctrl+C信号失败 | Failed to listen for Ctrl+C signal");
      println!("🛑 收到关闭信号，正在优雅关闭... | 🛑 Received shutdown signal, shutting down gracefully...");
  }
  ```

## 实践项目：配置安全的HTTPS服务 | Practical Project: Configure Secure HTTPS Service

### 目标 | Objective
构建一个生产级的安全Web服务，综合应用HTTPS、安全头、JWT认证、RBAC授权等安全机制，并实现完整的安全监控和日志记录。| Build a production-grade secure web service that comprehensively applies HTTPS, security headers, JWT authentication, RBAC authorization and other security mechanisms, and implements complete security monitoring and logging.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. TLS握手过程中，客户端验证服务器证书的主要步骤是什么？| What are the main steps for client to verify server certificate during TLS handshake?
   **答案 | Answer:** 检查证书链、验证数字签名、检查域名匹配、确认证书未过期
2. CORS预检请求在什么情况下会被触发？| Under what circumstances will CORS preflight requests be triggered?
   **答案 | Answer:** 复杂请求，如自定义头部、PUT/DELETE方法、application/json内容类型
3. JWT令牌的三个组成部分分别是什么？| What are the three components of a JWT token?
   **答案 | Answer:** Header（头部）、Payload（载荷）、Signature（签名）

### 步骤 | Steps
1. **TLS证书生成和配置** | TLS Certificate Generation and Configuration
2. **安全头中间件实现** | Security Headers Middleware Implementation  
3. **JWT认证系统开发** | JWT Authentication System Development
4. **RBAC权限控制实现** | RBAC Permission Control Implementation
5. **安全日志和监控配置** | Security Logging and Monitoring Configuration

### 示例代码 | Example Code
```rust
"""
安全HTTPS Web服务 | Secure HTTPS Web Service
项目描述：生产级安全Web服务，展示现代Web安全最佳实践 | Project description: Production-grade secure web service demonstrating modern web security best practices

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- TLS/HTTPS传输加密 | TLS/HTTPS transport encryption
- 安全HTTP头配置 | Security HTTP headers configuration
- JWT无状态认证 | JWT stateless authentication
- RBAC角色权限控制 | RBAC role-based access control
- 输入验证和输出编码 | Input validation and output encoding
- 安全日志和监控 | Security logging and monitoring
"""

use axum::{
    extract::{Json, State, Path},
    http::{StatusCode, HeaderValue},
    middleware::{self, Next},
    response::{Json as ResponseJson, Response},
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
};
use tracing::{info, warn, error};

// 应用状态 | Application state
#[derive(Clone)]
struct AppState {
    jwt_config: Arc<JwtConfig>,
    rbac_system: Arc<RwLock<RbacSystem>>,
    audit_logger: Arc<AuditLogger>,
}

// 审计日志记录器 | Audit logger
struct AuditLogger;

impl AuditLogger {
    fn log_security_event(&self, event_type: &str, user_id: Option<&str>, details: &str) {
        let user = user_id.unwrap_or("anonymous");
        info!(
            event_type = event_type,
            user_id = user,
            details = details,
            "安全事件记录 | Security event logged"
        );
    }
    
    fn log_auth_attempt(&self, username: &str, success: bool, ip: &str) {
        if success {
            info!(
                username = username,
                ip = ip,
                "认证成功 | Authentication successful"
            );
        } else {
            warn!(
                username = username,
                ip = ip,
                "认证失败 | Authentication failed"
            );
        }
    }
}

// 主应用函数 | Main application function
fn create_secure_app(state: AppState) -> Router {
    Router::new()
        // 公共路由 | Public routes
        .route("/health", get(health_check))
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/register", post(register_handler))
        
        // 认证保护的路由 | Authenticated routes
        .nest("/api", authenticated_routes())
        
        // 管理员路由 | Admin routes
        .nest("/admin", admin_routes())
        
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
                .layer(create_cors_layer())
                .layer(middleware::from_fn(security_headers_middleware))
                .layer(middleware::from_fn(audit_middleware))
        )
}

// 认证路由 | Authenticated routes
fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/profile", get(get_profile))
        .route("/profile", put(update_profile))
        .layer(middleware::from_fn_with_state(
            (), // JWT config will be extracted from state
            jwt_auth_middleware,
        ))
}

// 管理员路由 | Admin routes  
fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_all_users))
        .route("/users/:id", delete(delete_user))
        .route("/system/status", get(system_status))
        .layer(middleware::from_fn(admin_permission_middleware))
}

// 审计中间件 | Audit middleware
async fn audit_middleware(
    State(state): State<AppState>,
    request: axum::http::Request<axum::body::Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    // 记录所有API请求 | Log all API requests
    if uri.path().starts_with("/api") {
        state.audit_logger.log_security_event(
            "api_request",
            None, // 用户ID需要从JWT中提取 | User ID needs to be extracted from JWT
            &format!("{} {} {} {:?}", method, uri, status, duration),
        );
    }
    
    response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化追踪 | Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    
    // 创建应用状态 | Create application state
    let state = AppState {
        jwt_config: Arc::new(JwtConfig::new(b"your-256-bit-secret-key-change-this-in-production!!")),
        rbac_system: Arc::new(RwLock::new(RbacSystem::new())),
        audit_logger: Arc::new(AuditLogger),
    };
    
    // 创建应用 | Create application
    let app = create_secure_app(state);
    
    // 启动服务器 | Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3443").await?;
    info!("🔒 安全服务器启动在 https://localhost:3443 | 🔒 Secure server starting on https://localhost:3443");
    
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("安装Ctrl+C处理器失败 | Failed to install Ctrl+C handler");
    info!("🛑 收到关闭信号 | 🛑 Received shutdown signal");
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确配置了TLS/HTTPS？| Does the project correctly configure TLS/HTTPS?
2. 安全头设置是否符合最佳实践？| Do security header settings follow best practices?
3. JWT认证和RBAC授权是否工作正常？| Do JWT authentication and RBAC authorization work properly?
4. 输入验证和输出编码是否完整实现？| Are input validation and output encoding fully implemented?
5. 安全日志记录是否全面覆盖？| Does security logging provide comprehensive coverage?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **TLS协议深化练习 | TLS Protocol Deepening Exercise**
   - **练习描述 | Exercise Description:** 实现自定义TLS握手监控，分析不同密码套件的性能差异
   - **概念检查 | Concept Check:** TLS握手各阶段的时间消耗分布如何？
   - **学习目标 | Learning Objective:** 深入理解TLS协议细节和性能优化

2. **安全头策略练习 | Security Headers Policy Exercise**
   - **练习描述 | Exercise Description:** 设计适合不同应用场景的CSP和安全头组合策略
   - **概念检查 | Concept Check:** 如何平衡安全性和功能性需求？
   - **学习目标 | Learning Objective:** 掌握复杂环境下的安全策略制定

3. **认证机制整合练习 | Authentication Mechanism Integration Exercise**
   - **练习描述 | Exercise Description:** 实现多种认证方式的整合（JWT + OAuth2 + 生物识别）
   - **概念检查 | Concept Check:** 不同认证方式的安全强度如何评估？
   - **学习目标 | Learning Objective:** 构建灵活的认证架构

4. **安全漏洞防护练习 | Security Vulnerability Protection Exercise**
   - **练习描述 | Exercise Description:** 模拟常见Web攻击并实现对应的防护机制
   - **概念检查 | Concept Check:** 如何设计多层次的安全防护体系？
   - **学习目标 | Learning Objective:** 培养全面的安全防护思维

5. **安全监控创新练习 | Security Monitoring Innovation Exercise**
   - **练习描述 | Exercise Description:** 开发基于机器学习的异常行为检测系统
   - **概念检查 | Concept Check:** 如何区分正常行为和潜在威胁？
   - **学习目标 | Learning Objective:** 探索智能化安全监控技术

6. **安全知识教学练习 | Security Knowledge Teaching Exercise**
   - **练习描述 | Exercise Description:** 向团队成员讲解Web安全最佳实践
   - **概念检查 | Concept Check:** 如何让非安全专业人员理解安全重要性？
   - **学习目标 | Learning Objective:** 提升安全知识传播能力

7. **安全架构扩展练习 | Security Architecture Extension Exercise**
   - **练习描述 | Exercise Description:** 扩展项目为支持微服务架构的安全网关
   - **概念检查 | Concept Check:** 分布式环境下如何保证安全策略一致性？
   - **学习目标 | Learning Objective:** 掌握企业级安全架构设计

## 学习资源 | Learning Resources
- [Rust TLS文档](https://docs.rs/rustls/) - Rust TLS实现
- [OWASP安全指南](https://owasp.org/) - Web应用安全最佳实践
- [MDN Web安全](https://developer.mozilla.org/en-US/docs/Web/Security) - Web安全概念和实践
- [JWT.io](https://jwt.io/) - JWT令牌工具和文档

---

✅ **完成检查清单 | Completion Checklist**
- [ ] TLS握手过程原理理解 | TLS handshake process principles understanding
- [ ] 安全HTTP头配置掌握 | Security HTTP headers configuration mastery
- [ ] CORS策略设计能力 | CORS policy design capability
- [ ] CSP内容安全策略应用 | CSP content security policy application
- [ ] JWT认证机制实现 | JWT authentication mechanism implementation
- [ ] RBAC权限控制系统 | RBAC permission control system
- [ ] 输入验证输出编码技术 | Input validation and output encoding techniques
- [ ] 生产级安全配置部署 | Production-grade security configuration deployment
- [ ] 安全日志监控机制 | Security logging and monitoring mechanisms
- [ ] 实践项目安全服务完成 | Practical project secure service completion

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释每个核心安全概念，特别是TLS握手、安全头配置和认证授权机制的工作原理。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain each core security concept to others, especially the working principles of TLS handshake, security headers configuration, and authentication/authorization mechanisms.