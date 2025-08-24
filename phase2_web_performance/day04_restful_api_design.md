# Rust入门 - 第4天：RESTful API设计 | Rust Introduction - Day 4: RESTful API Design

## 学习目标 | Learning Objectives
- 深入理解REST架构原则和约束 | Deeply understand REST architectural principles and constraints
- 掌握RESTful API设计的最佳实践 | Master best practices for RESTful API design
- 熟练使用HTTP状态码和方法 | Proficiently use HTTP status codes and methods
- 学会设计可扩展的API版本控制策略 | Learn to design scalable API versioning strategies
- 能够设计完整的用户管理API系统 | Able to design complete user management API systems
- 理解API安全性和性能优化考虑 | Understand API security and performance optimization considerations

## 详细内容 | Detailed Content

### 1. REST架构原则深入理解 | Deep Understanding of REST Architectural Principles (1.5小时 | 1.5 hours)

- **REST架构约束 | REST Architectural Constraints**
  
  **概念定义 | Concept Definition:**
  REST (Representational State Transfer) 是一种架构风格，定义了一组约束条件和原则，用于创建可伸缩的Web服务 | REST (Representational State Transfer) is an architectural style that defines a set of constraints and principles for creating scalable web services
  
  **核心特征 | Key Characteristics:**
  - 客户端-服务器架构：分离关注点，提高可移植性 | Client-server architecture: separates concerns and improves portability
  - 无状态性：每个请求必须包含所有必需信息 | Statelessness: each request must contain all necessary information
  - 可缓存性：响应数据必须明确标记是否可缓存 | Cacheability: response data must be explicitly marked as cacheable or non-cacheable
  - 统一接口：通过标准HTTP方法操作资源 | Uniform interface: manipulate resources through standard HTTP methods
  - 分层系统：允许通过中间层增强系统功能 | Layered system: allows system functionality to be enhanced through intermediate layers
  - 按需代码（可选）：服务器可向客户端发送可执行代码 | Code on demand (optional): server can send executable code to clients
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. REST架构要求服务器保存客户端状态信息吗？| Does REST architecture require the server to store client state information?
     **答案 | Answer:** 否 | No - REST要求无状态性，服务器不应保存客户端上下文 | REST requires statelessness, servers should not store client context
  2. RESTful API可以使用除HTTP之外的协议吗？| Can RESTful APIs use protocols other than HTTP?
     **答案 | Answer:** 是 | Yes - REST是架构风格，不限定特定协议，但HTTP是最常用的 | REST is an architectural style not tied to specific protocols, though HTTP is most common
  3. 缓存是REST架构的可选特性吗？| Is caching an optional feature in REST architecture?
     **答案 | Answer:** 否 | No - 可缓存性是REST的核心约束之一 | Cacheability is one of the core constraints of REST
  4. 分层系统约束允许客户端直接访问最终服务器吗？| Does the layered system constraint allow clients to access the final server directly?
     **答案 | Answer:** 否 | No - 客户端不应假设与最终服务器直接连接 | Clients should not assume direct connection to the final server
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // REST架构示例：无状态的用户认证处理 | REST architecture example: stateless user authentication handling
  use axum::{
      extract::{Query, Path},
      http::{StatusCode, HeaderMap},
      response::Json,
      routing::{get, post, put, delete},
      Router,
  };
  use serde::{Deserialize, Serialize};
  use std::collections::HashMap;
  
  #[derive(Serialize, Deserialize, Clone)]
  struct User {
      id: u32,
      username: String,
      email: String,
      created_at: String,
  }
  
  // 无状态的请求处理 - 每个请求包含所有必需信息
  // Stateless request handling - each request contains all necessary information
  async fn get_user_stateless(
      Path(user_id): Path<u32>,
      headers: HeaderMap, // 认证信息通过header传递 | Authentication info passed via headers
  ) -> Result<Json<User>, StatusCode> {
      // 验证认证令牌（无状态） | Validate auth token (stateless)
      if let Some(auth_header) = headers.get("authorization") {
          // 令牌验证逻辑 | Token validation logic
          validate_token(auth_header.to_str().unwrap_or(""))?;
      } else {
          return Err(StatusCode::UNAUTHORIZED);
      }
      
      // 获取用户数据 | Fetch user data
      let user = get_user_by_id(user_id).await?;
      Ok(Json(user))
  }
  
  fn validate_token(token: &str) -> Result<(), StatusCode> {
      // 令牌验证实现（JWT等） | Token validation implementation (JWT, etc.)
      if token.starts_with("Bearer ") {
          Ok(())
      } else {
          Err(StatusCode::UNAUTHORIZED)
      }
  }
  
  async fn get_user_by_id(id: u32) -> Result<User, StatusCode> {
      // 数据库查询逻辑 | Database query logic
      Ok(User {
          id,
          username: format!("user_{}", id),
          email: format!("user_{}@example.com", id),
          created_at: "2024-01-01T00:00:00Z".to_string(),
      })
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码如何体现REST的无状态原则？| How does this code demonstrate REST's statelessness principle?
    **答案 | Answer:** 每个请求通过Authorization头传递认证信息，服务器不保存会话状态 | Each request passes authentication info via Authorization header, server maintains no session state
  - 如果需要支持缓存，应该如何修改响应？| How should the response be modified to support caching?
    **答案 | Answer:** 添加Cache-Control、ETag或Last-Modified等缓存相关的HTTP头 | Add cache-related HTTP headers like Cache-Control, ETag, or Last-Modified
  
  **常见误区检查 | Common Misconception Checks:**
  - REST API必须返回JSON格式数据吗？| Must REST APIs return JSON format data?
    **答案 | Answer:** 否，REST支持多种表示格式，JSON只是最常用的一种 | No, REST supports multiple representation formats, JSON is just the most commonly used
  - 使用HTTP协议就等同于RESTful吗？| Does using HTTP protocol equal being RESTful?
    **答案 | Answer:** 否，仅使用HTTP不足够，还需遵循REST的所有架构约束 | No, merely using HTTP is not sufficient, all REST architectural constraints must be followed

### 2. HTTP方法和语义化设计 | HTTP Methods and Semantic Design (1小时 | 1 hour)

- **HTTP方法标准化使用 | Standardized HTTP Method Usage**
  
  **概念定义 | Concept Definition:**
  HTTP方法定义了对资源执行的操作类型，每个方法都有特定的语义和约束 | HTTP methods define the type of operation to be performed on resources, each method has specific semantics and constraints
  
  **核心特征 | Key Characteristics:**
  - GET：安全且幂等，仅用于数据检索 | GET: safe and idempotent, used only for data retrieval
  - POST：不安全不幂等，用于创建新资源 | POST: neither safe nor idempotent, used for creating new resources
  - PUT：不安全但幂等，用于完整更新或创建资源 | PUT: not safe but idempotent, used for complete update or resource creation
  - PATCH：不安全不幂等，用于部分更新资源 | PATCH: neither safe nor idempotent, used for partial resource updates
  - DELETE：不安全但幂等，用于删除资源 | DELETE: not safe but idempotent, used for resource deletion
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. GET请求应该修改服务器状态吗？| Should GET requests modify server state?
     **答案 | Answer:** 否 | No - GET方法是安全的，不应产生副作用 | GET method is safe and should not produce side effects
  2. PUT请求是幂等的吗？| Are PUT requests idempotent?
     **答案 | Answer:** 是 | Yes - 多次执行相同的PUT请求应该产生相同结果 | Multiple identical PUT requests should produce the same result
  3. POST请求可以用于更新现有资源吗？| Can POST requests be used to update existing resources?
     **答案 | Answer:** 可以但不推荐 | Yes but not recommended - 应该使用PUT或PATCH进行更新操作 | PUT or PATCH should be used for update operations
  4. DELETE请求删除不存在的资源应该返回什么状态码？| What status code should DELETE requests return when deleting non-existent resources?
     **答案 | Answer:** 404或204 | 404 or 204 - 取决于API设计策略 | Depends on API design strategy
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // HTTP方法语义化实现 | Semantic HTTP method implementation
  use axum::{
      extract::{Path, Query, Json as ExtractJson},
      http::StatusCode,
      response::Json,
      routing::{get, post, put, patch, delete},
      Router,
  };
  use serde::{Deserialize, Serialize};
  use std::collections::HashMap;
  
  #[derive(Serialize, Deserialize, Clone)]
  struct User {
      id: Option<u32>,
      username: String,
      email: String,
      full_name: Option<String>,
  }
  
  #[derive(Deserialize)]
  struct UserQuery {
      page: Option<u32>,
      limit: Option<u32>,
      search: Option<String>,
  }
  
  #[derive(Deserialize)]
  struct UserPatch {
      username: Option<String>,
      email: Option<String>,
      full_name: Option<String>,
  }
  
  // GET - 安全且幂等的资源检索 | GET - safe and idempotent resource retrieval
  async fn get_users(Query(params): Query<UserQuery>) -> Json<Vec<User>> {
      let page = params.page.unwrap_or(1);
      let limit = params.limit.unwrap_or(10);
      
      // 仅查询数据，不修改服务器状态 | Only query data, don't modify server state
      let users = fetch_users(page, limit, params.search).await;
      Json(users)
  }
  
  async fn get_user(Path(user_id): Path<u32>) -> Result<Json<User>, StatusCode> {
      match find_user_by_id(user_id).await {
          Some(user) => Ok(Json(user)),
          None => Err(StatusCode::NOT_FOUND),
      }
  }
  
  // POST - 创建新资源 | POST - create new resource
  async fn create_user(ExtractJson(user): ExtractJson<User>) -> Result<Json<User>, StatusCode> {
      // 验证用户数据 | Validate user data
      if user.username.is_empty() || user.email.is_empty() {
          return Err(StatusCode::BAD_REQUEST);
      }
      
      // 创建新用户 | Create new user
      let new_user = store_user(user).await
          .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
      
      Ok(Json(new_user))
  }
  
  // PUT - 完整资源更新（幂等） | PUT - complete resource update (idempotent)
  async fn update_user(
      Path(user_id): Path<u32>,
      ExtractJson(user): ExtractJson<User>
  ) -> Result<Json<User>, StatusCode> {
      // PUT应该替换整个资源 | PUT should replace the entire resource
      let mut updated_user = user;
      updated_user.id = Some(user_id);
      
      // 幂等操作：多次执行结果相同 | Idempotent operation: same result when executed multiple times
      let user = replace_user(user_id, updated_user).await
          .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
      
      Ok(Json(user))
  }
  
  // PATCH - 部分资源更新 | PATCH - partial resource update
  async fn patch_user(
      Path(user_id): Path<u32>,
      ExtractJson(patch): ExtractJson<UserPatch>
  ) -> Result<Json<User>, StatusCode> {
      // 获取现有用户 | Get existing user
      let mut user = find_user_by_id(user_id).await
          .ok_or(StatusCode::NOT_FOUND)?;
      
      // 只更新提供的字段 | Only update provided fields
      if let Some(username) = patch.username {
          user.username = username;
      }
      if let Some(email) = patch.email {
          user.email = email;
      }
      if let Some(full_name) = patch.full_name {
          user.full_name = Some(full_name);
      }
      
      let updated_user = save_user(user).await
          .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
      
      Ok(Json(updated_user))
  }
  
  // DELETE - 删除资源（幂等） | DELETE - delete resource (idempotent)
  async fn delete_user(Path(user_id): Path<u32>) -> StatusCode {
      match remove_user(user_id).await {
          Ok(true) => StatusCode::NO_CONTENT,  // 删除成功 | Deletion successful
          Ok(false) => StatusCode::NOT_FOUND,  // 资源不存在 | Resource doesn't exist
          Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
      }
  }
  
  // 辅助函数 | Helper functions
  async fn fetch_users(page: u32, limit: u32, search: Option<String>) -> Vec<User> {
      // 数据库查询逻辑 | Database query logic
      vec![]
  }
  
  async fn find_user_by_id(id: u32) -> Option<User> {
      // 查找用户逻辑 | Find user logic
      Some(User {
          id: Some(id),
          username: format!("user_{}", id),
          email: format!("user_{}@example.com", id),
          full_name: None,
      })
  }
  
  async fn store_user(user: User) -> Result<User, Box<dyn std::error::Error>> {
      // 存储用户逻辑 | Store user logic
      Ok(user)
  }
  
  async fn replace_user(id: u32, user: User) -> Result<User, Box<dyn std::error::Error>> {
      // 替换用户逻辑 | Replace user logic
      Ok(user)
  }
  
  async fn save_user(user: User) -> Result<User, Box<dyn std::error::Error>> {
      // 保存用户逻辑 | Save user logic
      Ok(user)
  }
  
  async fn remove_user(id: u32) -> Result<bool, Box<dyn std::error::Error>> {
      // 删除用户逻辑 | Delete user logic
      Ok(true)
  }
  
  // 路由配置 | Route configuration
  pub fn create_user_routes() -> Router {
      Router::new()
          .route("/users", get(get_users))
          .route("/users", post(create_user))
          .route("/users/:id", get(get_user))
          .route("/users/:id", put(update_user))
          .route("/users/:id", patch(patch_user))
          .route("/users/:id", delete(delete_user))
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - PATCH和PUT方法的主要区别是什么？| What's the main difference between PATCH and PUT methods?
    **答案 | Answer:** PUT替换整个资源，PATCH只更新指定字段 | PUT replaces the entire resource, PATCH only updates specified fields
  - 为什么DELETE方法返回204而不是200状态码？| Why does the DELETE method return 204 instead of 200 status code?
    **答案 | Answer:** 204 No Content表示操作成功但无内容返回，更符合DELETE语义 | 204 No Content indicates successful operation with no content to return, better fits DELETE semantics

### 3. HTTP状态码正确使用 | Correct Usage of HTTP Status Codes (1小时 | 1 hour)

- **状态码分类和语义 | Status Code Classification and Semantics**
  
  **概念定义 | Concept Definition:**
  HTTP状态码是服务器向客户端传达请求处理结果的标准化方式，分为五大类别 | HTTP status codes are standardized ways for servers to communicate request processing results to clients, divided into five major categories
  
  **核心特征 | Key Characteristics:**
  - 1xx信息性响应：请求已接收，继续处理 | 1xx informational responses: request received, continue processing
  - 2xx成功响应：请求已成功接收、理解并处理 | 2xx success responses: request successfully received, understood, and processed
  - 3xx重定向：需要采取进一步操作来完成请求 | 3xx redirection: further action needs to be taken to complete request
  - 4xx客户端错误：请求包含语法错误或无法完成 | 4xx client errors: request contains syntax errors or cannot be fulfilled
  - 5xx服务器错误：服务器处理请求时发生错误 | 5xx server errors: server encountered an error while processing request
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 400和422状态码的区别是什么？| What's the difference between 400 and 422 status codes?
     **答案 | Answer:** 400表示请求语法错误，422表示请求格式正确但内容验证失败 | 400 indicates request syntax error, 422 indicates correct format but content validation failure
  2. 创建资源成功应该返回200还是201？| Should successful resource creation return 200 or 201?
     **答案 | Answer:** 201 | 201 - 表示资源已创建 | Indicates resource has been created
  3. 401和403状态码有什么区别？| What's the difference between 401 and 403 status codes?
     **答案 | Answer:** 401表示未认证，403表示已认证但无权限 | 401 means unauthenticated, 403 means authenticated but unauthorized
  4. 幂等操作删除不存在资源应该返回什么状态码？| What status code should idempotent deletion of non-existent resource return?
     **答案 | Answer:** 通常返回204或404，取决于API设计哲学 | Usually 204 or 404, depending on API design philosophy
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // HTTP状态码正确使用示例 | Correct HTTP status code usage examples
  use axum::{
      extract::{Path, Json as ExtractJson},
      http::StatusCode,
      response::{Json, Response, IntoResponse},
      routing::{get, post, put, delete},
  };
  use serde::{Deserialize, Serialize};
  
  #[derive(Serialize, Deserialize)]
  struct User {
      id: Option<u32>,
      username: String,
      email: String,
  }
  
  #[derive(Serialize)]
  struct ApiError {
      error: String,
      message: String,
      code: u16,
  }
  
  #[derive(Serialize)]
  struct ValidationError {
      field: String,
      message: String,
  }
  
  #[derive(Serialize)]
  struct ValidationResponse {
      error: String,
      details: Vec<ValidationError>,
  }
  
  // 2xx 成功状态码示例 | 2xx success status code examples
  async fn get_user_success(Path(user_id): Path<u32>) -> impl IntoResponse {
      match find_user(user_id).await {
          Some(user) => {
              // 200 OK - 成功获取资源 | 200 OK - successfully retrieved resource
              (StatusCode::OK, Json(user))
          }
          None => {
              // 404 Not Found - 资源不存在 | 404 Not Found - resource doesn't exist
              (StatusCode::NOT_FOUND, Json(ApiError {
                  error: "NotFound".to_string(),
                  message: format!("User with id {} not found", user_id),
                  code: 404,
              }))
          }
      }
  }
  
  async fn create_user_success(ExtractJson(user): ExtractJson<User>) -> impl IntoResponse {
      // 输入验证 | Input validation
      let validation_errors = validate_user(&user);
      if !validation_errors.is_empty() {
          // 422 Unprocessable Entity - 验证失败 | 422 Unprocessable Entity - validation failure
          return (StatusCode::UNPROCESSABLE_ENTITY, Json(ValidationResponse {
              error: "Validation failed".to_string(),
              details: validation_errors,
          }));
      }
      
      match create_user_in_db(user).await {
          Ok(created_user) => {
              // 201 Created - 资源已创建 | 201 Created - resource created
              (StatusCode::CREATED, Json(created_user))
          }
          Err(e) if e.is_duplicate() => {
              // 409 Conflict - 资源冲突 | 409 Conflict - resource conflict
              (StatusCode::CONFLICT, Json(ApiError {
                  error: "Conflict".to_string(),
                  message: "User with this username already exists".to_string(),
                  code: 409,
              }))
          }
          Err(_) => {
              // 500 Internal Server Error - 服务器错误 | 500 Internal Server Error - server error
              (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError {
                  error: "InternalError".to_string(),
                  message: "Failed to create user".to_string(),
                  code: 500,
              }))
          }
      }
  }
  
  async fn update_user_partial(
      Path(user_id): Path<u32>,
      ExtractJson(user): ExtractJson<User>
  ) -> impl IntoResponse {
      // 检查用户是否存在 | Check if user exists
      if !user_exists(user_id).await {
          return (StatusCode::NOT_FOUND, Json(ApiError {
              error: "NotFound".to_string(),
              message: format!("User with id {} not found", user_id),
              code: 404,
          }));
      }
      
      // 验证更新数据 | Validate update data
      let validation_errors = validate_user(&user);
      if !validation_errors.is_empty() {
          return (StatusCode::UNPROCESSABLE_ENTITY, Json(ValidationResponse {
              error: "Validation failed".to_string(),
              details: validation_errors,
          }));
      }
      
      match update_user_in_db(user_id, user).await {
          Ok(updated_user) => {
              // 200 OK - 更新成功 | 200 OK - update successful
              (StatusCode::OK, Json(updated_user))
          }
          Err(_) => {
              (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError {
                  error: "InternalError".to_string(),
                  message: "Failed to update user".to_string(),
                  code: 500,
              }))
          }
      }
  }
  
  async fn delete_user_proper(Path(user_id): Path<u32>) -> impl IntoResponse {
      match delete_user_from_db(user_id).await {
          Ok(true) => {
              // 204 No Content - 删除成功，无内容返回 | 204 No Content - deletion successful, no content
              StatusCode::NO_CONTENT
          }
          Ok(false) => {
              // 404 Not Found - 用户不存在 | 404 Not Found - user doesn't exist
              StatusCode::NOT_FOUND
          }
          Err(_) => {
              // 500 Internal Server Error - 服务器错误 | 500 Internal Server Error
              StatusCode::INTERNAL_SERVER_ERROR
          }
      }
  }
  
  // 认证和授权状态码示例 | Authentication and authorization status code examples
  async fn protected_endpoint(
      headers: axum::http::HeaderMap,
      Path(user_id): Path<u32>
  ) -> impl IntoResponse {
      // 检查认证 | Check authentication
      let auth_token = match headers.get("authorization") {
          Some(token) => token.to_str().unwrap_or(""),
          None => {
              // 401 Unauthorized - 缺少认证信息 | 401 Unauthorized - missing authentication
              return (StatusCode::UNAUTHORIZED, Json(ApiError {
                  error: "Unauthorized".to_string(),
                  message: "Authentication required".to_string(),
                  code: 401,
              }));
          }
      };
      
      // 验证令牌 | Validate token
      let user_from_token = match validate_auth_token(auth_token).await {
          Ok(user) => user,
          Err(_) => {
              // 401 Unauthorized - 无效令牌 | 401 Unauthorized - invalid token
              return (StatusCode::UNAUTHORIZED, Json(ApiError {
                  error: "Unauthorized".to_string(),
                  message: "Invalid authentication token".to_string(),
                  code: 401,
              }));
          }
      };
      
      // 检查授权 | Check authorization
      if !user_can_access(user_from_token.id, user_id) {
          // 403 Forbidden - 已认证但无权限 | 403 Forbidden - authenticated but no permission
          return (StatusCode::FORBIDDEN, Json(ApiError {
              error: "Forbidden".to_string(),
              message: "Insufficient permissions to access this resource".to_string(),
              code: 403,
          }));
      }
      
      // 操作成功 | Operation successful
      (StatusCode::OK, Json(serde_json::json!({
          "message": "Access granted",
          "user_id": user_id
      })))
  }
  
  // 辅助函数 | Helper functions
  async fn find_user(id: u32) -> Option<User> {
      // 查找用户逻辑 | Find user logic
      None
  }
  
  fn validate_user(user: &User) -> Vec<ValidationError> {
      let mut errors = Vec::new();
      
      if user.username.is_empty() {
          errors.push(ValidationError {
              field: "username".to_string(),
              message: "Username cannot be empty".to_string(),
          });
      }
      
      if !user.email.contains('@') {
          errors.push(ValidationError {
              field: "email".to_string(),
              message: "Invalid email format".to_string(),
          });
      }
      
      errors
  }
  
  #[derive(Debug)]
  struct DatabaseError {
      duplicate: bool,
  }
  
  impl DatabaseError {
      fn is_duplicate(&self) -> bool {
          self.duplicate
      }
  }
  
  async fn create_user_in_db(user: User) -> Result<User, DatabaseError> {
      // 数据库创建逻辑 | Database creation logic
      Ok(user)
  }
  
  async fn user_exists(id: u32) -> bool {
      // 检查用户存在逻辑 | Check user existence logic
      true
  }
  
  async fn update_user_in_db(id: u32, user: User) -> Result<User, Box<dyn std::error::Error>> {
      // 数据库更新逻辑 | Database update logic
      Ok(user)
  }
  
  async fn delete_user_from_db(id: u32) -> Result<bool, Box<dyn std::error::Error>> {
      // 数据库删除逻辑 | Database deletion logic
      Ok(true)
  }
  
  async fn validate_auth_token(token: &str) -> Result<User, Box<dyn std::error::Error>> {
      // 令牌验证逻辑 | Token validation logic
      Ok(User {
          id: Some(1),
          username: "authenticated_user".to_string(),
          email: "user@example.com".to_string(),
      })
  }
  
  fn user_can_access(requester_id: Option<u32>, target_id: u32) -> bool {
      // 权限检查逻辑 | Permission check logic
      requester_id == Some(target_id)
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 什么时候应该使用422而不是400状态码？| When should you use 422 instead of 400 status code?
    **答案 | Answer:** 当请求格式正确但业务逻辑验证失败时使用422 | Use 422 when request format is correct but business logic validation fails
  - 为什么删除成功返回204而不是200？| Why return 204 instead of 200 for successful deletion?
    **答案 | Answer:** 204表示操作成功但无响应体，更适合DELETE操作语义 | 204 indicates successful operation with no response body, better fits DELETE operation semantics

### 4. 资源设计和URI规范 | Resource Design and URI Standards (45分钟 | 45 minutes)

- **RESTful资源建模 | RESTful Resource Modeling**
  
  **概念定义 | Concept Definition:**
  RESTful API中的资源是系统中可以被唯一标识和操作的实体，通过URI进行标识和访问 | Resources in RESTful APIs are entities in the system that can be uniquely identified and manipulated, identified and accessed through URIs
  
  **核心特征 | Key Characteristics:**
  - 资源应该是名词而不是动词 | Resources should be nouns, not verbs
  - URI应该有层次结构和逻辑关系 | URIs should have hierarchical structure and logical relationships
  - 使用复数名词表示资源集合 | Use plural nouns for resource collections
  - 避免在URI中包含动词 | Avoid including verbs in URIs
  - 保持URI的一致性和可预测性 | Maintain URI consistency and predictability
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. URI中应该使用动词还是名词？| Should URIs use verbs or nouns?
     **答案 | Answer:** 名词 | Nouns - URI标识资源，动词通过HTTP方法表达 | URIs identify resources, verbs are expressed through HTTP methods
  2. 资源集合应该使用单数还是复数形式？| Should resource collections use singular or plural forms?
     **答案 | Answer:** 复数 | Plural - 例如/users而不是/user | Example: /users instead of /user
  3. 嵌套资源的URI层级应该有限制吗？| Should nested resource URI levels be limited?
     **答案 | Answer:** 是 | Yes - 通常建议不超过2-3层以保持清晰 | Generally recommend no more than 2-3 levels for clarity
  4. URI中应该包含查询参数还是路径参数？| Should URIs contain query parameters or path parameters?
     **答案 | Answer:** 取决于用途 | Depends on usage - 路径参数标识资源，查询参数过滤和排序 | Path parameters identify resources, query parameters filter and sort
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // RESTful资源设计示例 | RESTful resource design examples
  use axum::{
      extract::{Path, Query},
      response::Json,
      routing::{get, post, put, delete},
      Router,
  };
  use serde::{Deserialize, Serialize};
  
  // 用户资源模型 | User resource model
  #[derive(Serialize, Deserialize, Clone)]
  struct User {
      id: u32,
      username: String,
      email: String,
      profile: UserProfile,
  }
  
  #[derive(Serialize, Deserialize, Clone)]
  struct UserProfile {
      first_name: String,
      last_name: String,
      bio: Option<String>,
  }
  
  // 文章资源模型 | Article resource model
  #[derive(Serialize, Deserialize, Clone)]
  struct Article {
      id: u32,
      title: String,
      content: String,
      author_id: u32,
      tags: Vec<String>,
      created_at: String,
  }
  
  // 评论资源模型 | Comment resource model
  #[derive(Serialize, Deserialize, Clone)]
  struct Comment {
      id: u32,
      article_id: u32,
      author_id: u32,
      content: String,
      created_at: String,
  }
  
  #[derive(Deserialize)]
  struct ArticleQuery {
      page: Option<u32>,
      limit: Option<u32>,
      tag: Option<String>,
      author: Option<u32>,
  }
  
  #[derive(Deserialize)]
  struct CommentQuery {
      page: Option<u32>,
      limit: Option<u32>,
  }
  
  // 用户资源端点 | User resource endpoints
  // GET /users - 获取用户列表 | Get user list
  async fn get_users() -> Json<Vec<User>> {
      Json(vec![])
  }
  
  // GET /users/{id} - 获取特定用户 | Get specific user
  async fn get_user(Path(user_id): Path<u32>) -> Json<User> {
      Json(User {
          id: user_id,
          username: format!("user_{}", user_id),
          email: format!("user_{}@example.com", user_id),
          profile: UserProfile {
              first_name: "John".to_string(),
              last_name: "Doe".to_string(),
              bio: None,
          },
      })
  }
  
  // POST /users - 创建新用户 | Create new user
  async fn create_user(Json(user): Json<User>) -> Json<User> {
      Json(user)
  }
  
  // PUT /users/{id} - 更新用户 | Update user
  async fn update_user(Path(user_id): Path<u32>, Json(user): Json<User>) -> Json<User> {
      Json(user)
  }
  
  // DELETE /users/{id} - 删除用户 | Delete user
  async fn delete_user(Path(user_id): Path<u32>) -> Json<serde_json::Value> {
      Json(serde_json::json!({"message": "User deleted"}))
  }
  
  // 文章资源端点 | Article resource endpoints
  // GET /articles - 获取文章列表（带过滤） | Get article list (with filtering)
  async fn get_articles(Query(params): Query<ArticleQuery>) -> Json<Vec<Article>> {
      // 实现分页、标签过滤等 | Implement pagination, tag filtering, etc.
      Json(vec![])
  }
  
  // GET /articles/{id} - 获取特定文章 | Get specific article
  async fn get_article(Path(article_id): Path<u32>) -> Json<Article> {
      Json(Article {
          id: article_id,
          title: "Sample Article".to_string(),
          content: "Article content...".to_string(),
          author_id: 1,
          tags: vec!["rust".to_string(), "api".to_string()],
          created_at: "2024-01-01T00:00:00Z".to_string(),
      })
  }
  
  // GET /users/{user_id}/articles - 获取用户的文章 | Get user's articles
  async fn get_user_articles(Path(user_id): Path<u32>) -> Json<Vec<Article>> {
      Json(vec![])
  }
  
  // 嵌套资源：文章评论 | Nested resource: article comments
  // GET /articles/{article_id}/comments - 获取文章评论 | Get article comments
  async fn get_article_comments(
      Path(article_id): Path<u32>,
      Query(params): Query<CommentQuery>
  ) -> Json<Vec<Comment>> {
      Json(vec![])
  }
  
  // POST /articles/{article_id}/comments - 为文章创建评论 | Create comment for article
  async fn create_article_comment(
      Path(article_id): Path<u32>,
      Json(comment): Json<Comment>
  ) -> Json<Comment> {
      Json(comment)
  }
  
  // GET /comments/{id} - 获取特定评论 | Get specific comment
  async fn get_comment(Path(comment_id): Path<u32>) -> Json<Comment> {
      Json(Comment {
          id: comment_id,
          article_id: 1,
          author_id: 1,
          content: "Great article!".to_string(),
          created_at: "2024-01-01T00:00:00Z".to_string(),
      })
  }
  
  // URI设计最佳实践示例 | URI design best practices examples
  pub fn create_restful_routes() -> Router {
      Router::new()
          // 用户资源 | User resources
          .route("/users", get(get_users))
          .route("/users", post(create_user))
          .route("/users/:id", get(get_user))
          .route("/users/:id", put(update_user))
          .route("/users/:id", delete(delete_user))
          
          // 文章资源 | Article resources
          .route("/articles", get(get_articles))
          .route("/articles/:id", get(get_article))
          
          // 用户文章关系 | User-article relationship
          .route("/users/:user_id/articles", get(get_user_articles))
          
          // 文章评论嵌套资源 | Article comments nested resource
          .route("/articles/:article_id/comments", get(get_article_comments))
          .route("/articles/:article_id/comments", post(create_article_comment))
          
          // 评论资源 | Comment resource
          .route("/comments/:id", get(get_comment))
  }
  
  // URI规范化工具函数 | URI normalization utility functions
  fn normalize_resource_uri(resource: &str, id: Option<u32>) -> String {
      match id {
          Some(id) => format!("/{}/{}", resource, id),
          None => format!("/{}", resource),
      }
  }
  
  fn build_collection_uri(resource: &str, filters: &[(&str, &str)]) -> String {
      let mut uri = format!("/{}", resource);
      if !filters.is_empty() {
          let query_params: Vec<String> = filters
              .iter()
              .map(|(key, value)| format!("{}={}", key, value))
              .collect();
          uri.push_str(&format!("?{}", query_params.join("&")));
      }
      uri
  }
  
  // 资源关系设计示例 | Resource relationship design examples
  #[derive(Serialize)]
  struct ResourceLinks {
      #[serde(rename = "self")]
      self_link: String,
      related: Vec<RelatedLink>,
  }
  
  #[derive(Serialize)]
  struct RelatedLink {
      name: String,
      href: String,
      method: String,
  }
  
  fn generate_user_links(user_id: u32) -> ResourceLinks {
      ResourceLinks {
          self_link: format!("/users/{}", user_id),
          related: vec![
              RelatedLink {
                  name: "articles".to_string(),
                  href: format!("/users/{}/articles", user_id),
                  method: "GET".to_string(),
              },
              RelatedLink {
                  name: "profile".to_string(),
                  href: format!("/users/{}/profile", user_id),
                  method: "GET".to_string(),
              },
          ],
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么使用/users而不是/user作为集合端点？| Why use /users instead of /user for collection endpoints?
    **答案 | Answer:** 复数形式更直观地表示资源集合，符合REST约定 | Plural form more intuitively represents resource collections, follows REST conventions
  - 嵌套资源URI何时合适？| When are nested resource URIs appropriate?
    **答案 | Answer:** 当子资源依赖于父资源且逻辑上属于父资源时 | When child resources depend on parent resources and logically belong to parent resources

### 5. API版本控制策略 | API Versioning Strategies (30分钟 | 30 minutes)

- **版本控制方法比较 | Versioning Method Comparison**
  
  **概念定义 | Concept Definition:**
  API版本控制是管理API演进过程中向后兼容性和功能更新的策略，确保现有客户端继续工作的同时引入新功能 | API versioning is a strategy for managing backward compatibility and feature updates during API evolution, ensuring existing clients continue to work while introducing new features
  
  **核心特征 | Key Characteristics:**
  - URI版本控制：在URL路径中包含版本号 | URI versioning: include version number in URL path
  - 头部版本控制：通过HTTP头指定版本 | Header versioning: specify version through HTTP headers
  - 查询参数版本控制：通过查询参数指定版本 | Query parameter versioning: specify version through query parameters
  - 媒体类型版本控制：通过Content-Type指定版本 | Media type versioning: specify version through Content-Type
  - 语义化版本控制：遵循语义化版本规范 | Semantic versioning: follow semantic versioning specification
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 主版本号何时应该递增？| When should the major version number be incremented?
     **答案 | Answer:** 当引入不兼容的破坏性变更时 | When introducing incompatible breaking changes
  2. URI版本控制的主要优点是什么？| What's the main advantage of URI versioning?
     **答案 | Answer:** 简单直观，版本信息清晰可见 | Simple and intuitive, version information clearly visible
  3. 应该同时维护多少个API版本？| How many API versions should be maintained simultaneously?
     **答案 | Answer:** 通常2-3个版本，平衡维护成本和兼容性 | Usually 2-3 versions, balancing maintenance cost and compatibility
  4. API弃用策略应该包含什么？| What should an API deprecation strategy include?
     **答案 | Answer:** 弃用通知、迁移指南、时间表和替代方案 | Deprecation notice, migration guide, timeline, and alternatives
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // API版本控制实现示例 | API versioning implementation examples
  use axum::{
      extract::{Path, Query, Request},
      http::{HeaderMap, StatusCode},
      middleware::{self, Next},
      response::{Json, Response, IntoResponse},
      routing::{get, post},
      Router,
  };
  use serde::{Deserialize, Serialize};
  use std::collections::HashMap;
  
  // 版本信息结构 | Version info structure
  #[derive(Debug, Clone)]
  pub enum ApiVersion {
      V1,
      V2,
      V3,
  }
  
  impl ApiVersion {
      fn from_string(version: &str) -> Option<Self> {
          match version {
              "v1" | "1" | "1.0" => Some(ApiVersion::V1),
              "v2" | "2" | "2.0" => Some(ApiVersion::V2),
              "v3" | "3" | "3.0" => Some(ApiVersion::V3),
              _ => None,
          }
      }
      
      fn to_string(&self) -> &'static str {
          match self {
              ApiVersion::V1 => "v1",
              ApiVersion::V2 => "v2",
              ApiVersion::V3 => "v3",
          }
      }
      
      fn is_supported(&self) -> bool {
          matches!(self, ApiVersion::V2 | ApiVersion::V3)
      }
      
      fn is_deprecated(&self) -> bool {
          matches!(self, ApiVersion::V1)
      }
  }
  
  // 用户数据模型 - 不同版本 | User data models - different versions
  #[derive(Serialize, Deserialize)]
  struct UserV1 {
      id: u32,
      name: String,
      email: String,
  }
  
  #[derive(Serialize, Deserialize)]
  struct UserV2 {
      id: u32,
      username: String,  // V2: name改为username | V2: name changed to username
      email: String,
      created_at: String,  // V2: 新增创建时间 | V2: added creation time
  }
  
  #[derive(Serialize, Deserialize)]
  struct UserV3 {
      id: u32,
      username: String,
      email: String,
      created_at: String,
      profile: UserProfile,  // V3: 新增用户档案 | V3: added user profile
      is_verified: bool,     // V3: 新增验证状态 | V3: added verification status
  }
  
  #[derive(Serialize, Deserialize)]
  struct UserProfile {
      first_name: String,
      last_name: String,
      bio: Option<String>,
  }
  
  // 版本控制中间件 | Versioning middleware
  async fn version_middleware(
      mut request: Request,
      next: Next,
  ) -> Result<Response, StatusCode> {
      let version = extract_version_from_request(&request)
          .unwrap_or(ApiVersion::V3); // 默认最新版本 | Default to latest version
      
      // 检查版本支持状态 | Check version support status
      if !version.is_supported() {
          return Err(StatusCode::BAD_REQUEST);
      }
      
      // 将版本信息添加到请求扩展 | Add version info to request extensions
      request.extensions_mut().insert(version.clone());
      
      let mut response = next.run(request).await;
      
      // 添加版本相关的响应头 | Add version-related response headers
      let headers = response.headers_mut();
      headers.insert("API-Version", version.to_string().parse().unwrap());
      
      if version.is_deprecated() {
          headers.insert("Deprecation", "true".parse().unwrap());
          headers.insert(
              "Sunset", 
              "2024-12-31T23:59:59Z".parse().unwrap()  // 弃用时间 | Deprecation time
          );
      }
      
      Ok(response)
  }
  
  // 1. URI版本控制 | URI versioning
  fn extract_version_from_uri(uri: &str) -> Option<ApiVersion> {
      if let Some(captures) = regex::Regex::new(r"/api/(v\d+)/")
          .unwrap()
          .captures(uri) 
      {
          if let Some(version_match) = captures.get(1) {
              return ApiVersion::from_string(version_match.as_str());
          }
      }
      None
  }
  
  // 2. 头部版本控制 | Header versioning
  fn extract_version_from_headers(headers: &HeaderMap) -> Option<ApiVersion> {
      headers
          .get("API-Version")
          .and_then(|h| h.to_str().ok())
          .and_then(ApiVersion::from_string)
  }
  
  // 3. 查询参数版本控制 | Query parameter versioning
  #[derive(Deserialize)]
  struct VersionQuery {
      version: Option<String>,
  }
  
  fn extract_version_from_query(query: &str) -> Option<ApiVersion> {
      serde_urlencoded::from_str::<VersionQuery>(query)
          .ok()
          .and_then(|q| q.version)
          .and_then(|v| ApiVersion::from_string(&v))
  }
  
  // 4. 媒体类型版本控制 | Media type versioning
  fn extract_version_from_accept_header(headers: &HeaderMap) -> Option<ApiVersion> {
      headers
          .get("Accept")
          .and_then(|h| h.to_str().ok())
          .and_then(|accept| {
              if accept.contains("application/vnd.api+json;version=1") {
                  Some(ApiVersion::V1)
              } else if accept.contains("application/vnd.api+json;version=2") {
                  Some(ApiVersion::V2)
              } else if accept.contains("application/vnd.api+json;version=3") {
                  Some(ApiVersion::V3)
              } else {
                  None
              }
          })
  }
  
  // 综合版本提取函数 | Comprehensive version extraction function
  fn extract_version_from_request(request: &Request) -> Option<ApiVersion> {
      let uri = request.uri().path();
      let headers = request.headers();
      let query = request.uri().query().unwrap_or("");
      
      // 优先级：URI > 头部 > 查询参数 > Accept头部
      // Priority: URI > Header > Query Parameter > Accept Header
      extract_version_from_uri(uri)
          .or_else(|| extract_version_from_headers(headers))
          .or_else(|| extract_version_from_query(query))
          .or_else(|| extract_version_from_accept_header(headers))
  }
  
  // 版本化的用户处理器 | Versioned user handlers
  async fn get_user_versioned(
      Path(user_id): Path<u32>,
      axum::Extension(version): axum::Extension<ApiVersion>,
  ) -> impl IntoResponse {
      match version {
          ApiVersion::V1 => {
              let user = UserV1 {
                  id: user_id,
                  name: format!("user_{}", user_id),
                  email: format!("user_{}@example.com", user_id),
              };
              Json(serde_json::to_value(user).unwrap()).into_response()
          }
          ApiVersion::V2 => {
              let user = UserV2 {
                  id: user_id,
                  username: format!("user_{}", user_id),
                  email: format!("user_{}@example.com", user_id),
                  created_at: "2024-01-01T00:00:00Z".to_string(),
              };
              Json(serde_json::to_value(user).unwrap()).into_response()
          }
          ApiVersion::V3 => {
              let user = UserV3 {
                  id: user_id,
                  username: format!("user_{}", user_id),
                  email: format!("user_{}@example.com", user_id),
                  created_at: "2024-01-01T00:00:00Z".to_string(),
                  profile: UserProfile {
                      first_name: "John".to_string(),
                      last_name: "Doe".to_string(),
                      bio: None,
                  },
                  is_verified: false,
              };
              Json(serde_json::to_value(user).unwrap()).into_response()
          }
      }
  }
  
  // API信息端点 | API info endpoint
  #[derive(Serialize)]
  struct ApiInfo {
      current_version: String,
      supported_versions: Vec<String>,
      deprecated_versions: Vec<DeprecatedVersion>,
  }
  
  #[derive(Serialize)]
  struct DeprecatedVersion {
      version: String,
      sunset_date: String,
      migration_guide: String,
  }
  
  async fn get_api_info() -> Json<ApiInfo> {
      Json(ApiInfo {
          current_version: "v3".to_string(),
          supported_versions: vec!["v2".to_string(), "v3".to_string()],
          deprecated_versions: vec![
              DeprecatedVersion {
                  version: "v1".to_string(),
                  sunset_date: "2024-12-31".to_string(),
                  migration_guide: "/docs/migration/v1-to-v2".to_string(),
              }
          ],
      })
  }
  
  // 路由配置 - 不同版本控制方法 | Route configuration - different versioning methods
  pub fn create_versioned_routes() -> Router {
      Router::new()
          // 1. URI版本控制路由 | URI versioning routes
          .route("/api/v1/users/:id", get(get_user_versioned))
          .route("/api/v2/users/:id", get(get_user_versioned))
          .route("/api/v3/users/:id", get(get_user_versioned))
          
          // 2. 通用路由配合头部版本控制 | Generic routes with header versioning
          .route("/users/:id", get(get_user_versioned))
          
          // API信息端点 | API info endpoint
          .route("/api/info", get(get_api_info))
          
          // 添加版本控制中间件 | Add versioning middleware
          .layer(middleware::from_fn(version_middleware))
  }
  
  // 版本迁移辅助工具 | Version migration helper utilities
  impl UserV1 {
      fn to_v2(self) -> UserV2 {
          UserV2 {
              id: self.id,
              username: self.name,  // name -> username
              email: self.email,
              created_at: "1970-01-01T00:00:00Z".to_string(),  // 默认值
          }
      }
  }
  
  impl UserV2 {
      fn to_v3(self) -> UserV3 {
          UserV3 {
              id: self.id,
              username: self.username,
              email: self.email,
              created_at: self.created_at,
              profile: UserProfile {
                  first_name: "".to_string(),  // 默认空值
                  last_name: "".to_string(),
                  bio: None,
              },
              is_verified: false,  // 默认未验证
          }
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 何时应该创建新的主版本？| When should a new major version be created?
    **答案 | Answer:** 当引入破坏现有客户端兼容性的变更时 | When introducing changes that break compatibility with existing clients
  - 为什么要同时支持多个API版本？| Why support multiple API versions simultaneously?
    **答案 | Answer:** 给客户端足够时间迁移，避免服务中断 | Give clients sufficient time to migrate, avoiding service interruptions

### 6. API安全性和最佳实践 | API Security and Best Practices (15分钟 | 15 minutes)

- **API安全基础原则 | Fundamental API Security Principles**
  
  **概念定义 | Concept Definition:**
  API安全是保护API免受恶意攻击和未授权访问的综合策略，涵盖认证、授权、数据保护和威胁防护 | API security is a comprehensive strategy to protect APIs from malicious attacks and unauthorized access, covering authentication, authorization, data protection, and threat prevention
  
  **关键安全实践 | Key Security Practices:**
  - 始终使用HTTPS进行数据传输 | Always use HTTPS for data transmission
  - 实施强认证和授权机制 | Implement strong authentication and authorization
  - 输入验证和数据清理 | Input validation and data sanitization
  - 限流和防滥用保护 | Rate limiting and abuse prevention
  - 安全头部配置 | Security header configuration
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. API密钥应该如何传输？| How should API keys be transmitted?
     **答案 | Answer:** 通过HTTPS头部，永不在URL中传输 | Via HTTPS headers, never in URLs
  2. 什么是CORS，为什么重要？| What is CORS and why is it important?
     **答案 | Answer:** 跨源资源共享，控制哪些域可以访问API | Cross-Origin Resource Sharing, controls which domains can access APIs
  3. 为什么需要限流？| Why is rate limiting needed?
     **答案 | Answer:** 防止滥用和DDoS攻击，保护服务可用性 | Prevent abuse and DDoS attacks, protect service availability
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // API安全实现示例 | API security implementation example
  use axum::{
      middleware::{self, Next},
      extract::Request,
      http::{StatusCode, HeaderValue},
      response::Response,
  };
  
  // CORS中间件 | CORS middleware
  async fn cors_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
      let mut response = next.run(request).await;
      let headers = response.headers_mut();
      
      headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
      headers.insert("Access-Control-Allow-Methods", HeaderValue::from_static("GET,POST,PUT,DELETE"));
      headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("Content-Type,Authorization"));
      
      Ok(response)
  }
  
  // 安全头部中间件 | Security headers middleware
  async fn security_headers_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
      let mut response = next.run(request).await;
      let headers = response.headers_mut();
      
      // 防止XSS攻击 | Prevent XSS attacks
      headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
      headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
      headers.insert("X-XSS-Protection", HeaderValue::from_static("1; mode=block"));
      
      // 强制HTTPS | Enforce HTTPS
      headers.insert("Strict-Transport-Security", 
          HeaderValue::from_static("max-age=31536000; includeSubDomains"));
      
      Ok(response)
  }
  ```

## 实践项目：用户管理API系统 | Practical Project: User Management API System

### 目标 | Objective
设计和实现一个完整的RESTful用户管理API，综合应用REST原则、HTTP方法、状态码、资源设计和版本控制策略 | Design and implement a complete RESTful user management API, comprehensively applying REST principles, HTTP methods, status codes, resource design, and versioning strategies

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. REST架构约束中的无状态性如何在API设计中体现？| How is statelessness in REST architectural constraints reflected in API design?
   **答案 | Answer:** 每个请求必须包含所有必要信息，服务器不保存客户端状态 | Each request must contain all necessary information, server maintains no client state

2. POST和PUT方法在语义上有什么根本区别？| What's the fundamental semantic difference between POST and PUT methods?
   **答案 | Answer:** POST用于创建资源且不幂等，PUT用于替换资源且幂等 | POST for resource creation and non-idempotent, PUT for resource replacement and idempotent

3. 422状态码什么时候使用比400更合适？| When is 422 status code more appropriate than 400?
   **答案 | Answer:** 当请求格式正确但业务逻辑验证失败时 | When request format is correct but business logic validation fails

### 步骤 | Steps
1. **架构设计阶段**：定义资源模型和API结构 | Architecture design phase: define resource models and API structure
2. **基础CRUD实现**：实现用户资源的基本操作 | Basic CRUD implementation: implement basic operations for user resources
3. **高级功能添加**：添加搜索、分页、关联资源 | Advanced features: add search, pagination, related resources
4. **版本控制集成**：实现API版本控制策略 | Version control integration: implement API versioning strategy
5. **安全性强化**：添加认证、授权和安全防护 | Security hardening: add authentication, authorization, and security protection

### 示例代码 | Example Code
```rust
"""
用户管理API系统 | User Management API System
完整的RESTful API实现，演示REST设计原则 | Complete RESTful API implementation demonstrating REST design principles

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- REST架构原则 | REST architectural principles
- HTTP方法语义化使用 | Semantic usage of HTTP methods
- 状态码正确应用 | Correct application of status codes
- 资源设计最佳实践 | Resource design best practices
- API版本控制策略 | API versioning strategies
- 安全性和性能考虑 | Security and performance considerations
"""

use axum::{
    extract::{Path, Query, Json as ExtractJson},
    http::{StatusCode, HeaderMap},
    response::{Json, IntoResponse},
    routing::{get, post, put, patch, delete},
    middleware,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};
use uuid::Uuid;

// 核心数据模型 | Core data models
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: Uuid,
    username: String,
    email: String,
    full_name: String,
    created_at: String,
    updated_at: String,
    is_active: bool,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    full_name: String,
}

#[derive(Deserialize)]
struct UpdateUserRequest {
    username: Option<String>,
    email: Option<String>,
    full_name: Option<String>,
    is_active: Option<bool>,
}

#[derive(Deserialize)]
struct UserQuery {
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>,
    active: Option<bool>,
}

// 响应模型 | Response models
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
    message: Option<String>,
}

#[derive(Serialize)]
struct PaginatedResponse<T> {
    items: Vec<T>,
    total: u32,
    page: u32,
    limit: u32,
    total_pages: u32,
}

#[derive(Serialize)]
struct ApiError {
    error: String,
    message: String,
    code: u16,
}

// REST端点实现 | REST endpoint implementations

// GET /api/v1/users - 获取用户列表（支持分页和过滤）
// GET /api/v1/users - Get user list (with pagination and filtering)
async fn get_users(Query(params): Query<UserQuery>) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10).min(100); // 限制最大页面大小
    
    // 模拟数据库查询 | Simulate database query
    let users = fetch_users_from_db(page, limit, params.search, params.active).await;
    let total = count_users_in_db().await;
    
    let response = PaginatedResponse {
        items: users,
        total,
        page,
        limit,
        total_pages: (total + limit - 1) / limit,
    };
    
    (StatusCode::OK, Json(ApiResponse {
        success: true,
        data: response,
        message: None,
    }))
}

// GET /api/v1/users/{id} - 获取特定用户
// GET /api/v1/users/{id} - Get specific user
async fn get_user(Path(user_id): Path<Uuid>) -> impl IntoResponse {
    match find_user_by_id(user_id).await {
        Some(user) => (StatusCode::OK, Json(ApiResponse {
            success: true,
            data: user,
            message: None,
        })),
        None => (StatusCode::NOT_FOUND, Json(ApiError {
            error: "NotFound".to_string(),
            message: "User not found".to_string(),
            code: 404,
        })),
    }
}

// POST /api/v1/users - 创建新用户
// POST /api/v1/users - Create new user
async fn create_user(ExtractJson(request): ExtractJson<CreateUserRequest>) -> impl IntoResponse {
    // 输入验证 | Input validation
    if let Err(validation_errors) = validate_create_user(&request) {
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(validation_errors));
    }
    
    // 检查用户名/邮箱唯一性 | Check username/email uniqueness
    if user_exists(&request.username, &request.email).await {
        return (StatusCode::CONFLICT, Json(ApiError {
            error: "Conflict".to_string(),
            message: "Username or email already exists".to_string(),
            code: 409,
        }));
    }
    
    // 创建用户 | Create user
    let new_user = User {
        id: Uuid::new_v4(),
        username: request.username,
        email: request.email,
        full_name: request.full_name,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        is_active: true,
    };
    
    match save_user_to_db(&new_user).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse {
            success: true,
            data: new_user,
            message: Some("User created successfully".to_string()),
        })),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError {
            error: "InternalError".to_string(),
            message: "Failed to create user".to_string(),
            code: 500,
        })),
    }
}

// PUT /api/v1/users/{id} - 完整更新用户
// PUT /api/v1/users/{id} - Complete user update
async fn update_user(
    Path(user_id): Path<Uuid>,
    ExtractJson(request): ExtractJson<CreateUserRequest>
) -> impl IntoResponse {
    // 检查用户是否存在 | Check if user exists
    let existing_user = match find_user_by_id(user_id).await {
        Some(user) => user,
        None => return (StatusCode::NOT_FOUND, Json(ApiError {
            error: "NotFound".to_string(),
            message: "User not found".to_string(),
            code: 404,
        })),
    };
    
    // 验证更新数据 | Validate update data
    if let Err(validation_errors) = validate_create_user(&request) {
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(validation_errors));
    }
    
    // PUT语义：完全替换资源 | PUT semantics: complete resource replacement
    let updated_user = User {
        id: user_id,
        username: request.username,
        email: request.email,
        full_name: request.full_name,
        created_at: existing_user.created_at,
        updated_at: chrono::Utc::now().to_rfc3339(),
        is_active: true, // PUT时重置为默认值
    };
    
    match save_user_to_db(&updated_user).await {
        Ok(_) => (StatusCode::OK, Json(ApiResponse {
            success: true,
            data: updated_user,
            message: Some("User updated successfully".to_string()),
        })),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError {
            error: "InternalError".to_string(),
            message: "Failed to update user".to_string(),
            code: 500,
        })),
    }
}

// PATCH /api/v1/users/{id} - 部分更新用户
// PATCH /api/v1/users/{id} - Partial user update
async fn patch_user(
    Path(user_id): Path<Uuid>,
    ExtractJson(request): ExtractJson<UpdateUserRequest>
) -> impl IntoResponse {
    // 获取现有用户 | Get existing user
    let mut user = match find_user_by_id(user_id).await {
        Some(user) => user,
        None => return (StatusCode::NOT_FOUND, Json(ApiError {
            error: "NotFound".to_string(),
            message: "User not found".to_string(),
            code: 404,
        })),
    };
    
    // PATCH语义：只更新提供的字段 | PATCH semantics: only update provided fields
    if let Some(username) = request.username {
        user.username = username;
    }
    if let Some(email) = request.email {
        user.email = email;
    }
    if let Some(full_name) = request.full_name {
        user.full_name = full_name;
    }
    if let Some(is_active) = request.is_active {
        user.is_active = is_active;
    }
    
    user.updated_at = chrono::Utc::now().to_rfc3339();
    
    match save_user_to_db(&user).await {
        Ok(_) => (StatusCode::OK, Json(ApiResponse {
            success: true,
            data: user,
            message: Some("User updated successfully".to_string()),
        })),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError {
            error: "InternalError".to_string(),
            message: "Failed to update user".to_string(),
            code: 500,
        })),
    }
}

// DELETE /api/v1/users/{id} - 删除用户
// DELETE /api/v1/users/{id} - Delete user
async fn delete_user(Path(user_id): Path<Uuid>) -> StatusCode {
    match delete_user_from_db(user_id).await {
        Ok(true) => StatusCode::NO_CONTENT,  // 删除成功 | Deletion successful
        Ok(false) => StatusCode::NOT_FOUND,  // 用户不存在 | User doesn't exist
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// 版本控制和安全中间件 | Versioning and security middleware
async fn api_middleware(
    request: axum::extract::Request,
    next: middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let mut response = next.run(request).await;
    
    // 添加API版本头 | Add API version header
    response.headers_mut().insert(
        "API-Version",
        "v1".parse().unwrap()
    );
    
    // 添加安全头 | Add security headers
    response.headers_mut().insert(
        "X-Content-Type-Options",
        "nosniff".parse().unwrap()
    );
    
    Ok(response)
}

// 辅助函数实现 | Helper function implementations
async fn fetch_users_from_db(
    page: u32,
    limit: u32,
    search: Option<String>,
    active: Option<bool>
) -> Vec<User> {
    // 模拟数据库查询逻辑 | Simulate database query logic
    vec![]
}

async fn count_users_in_db() -> u32 {
    // 模拟用户总数查询 | Simulate total user count query
    0
}

async fn find_user_by_id(id: Uuid) -> Option<User> {
    // 模拟数据库查询 | Simulate database query
    None
}

async fn user_exists(username: &str, email: &str) -> bool {
    // 检查用户名和邮箱是否已存在 | Check if username and email already exist
    false
}

async fn save_user_to_db(user: &User) -> Result<(), Box<dyn std::error::Error>> {
    // 模拟数据库保存 | Simulate database save
    Ok(())
}

async fn delete_user_from_db(user_id: Uuid) -> Result<bool, Box<dyn std::error::Error>> {
    // 模拟数据库删除 | Simulate database deletion
    Ok(true)
}

fn validate_create_user(request: &CreateUserRequest) -> Result<(), ApiError> {
    if request.username.is_empty() {
        return Err(ApiError {
            error: "ValidationError".to_string(),
            message: "Username cannot be empty".to_string(),
            code: 422,
        });
    }
    
    if !request.email.contains('@') {
        return Err(ApiError {
            error: "ValidationError".to_string(),
            message: "Invalid email format".to_string(),
            code: 422,
        });
    }
    
    Ok(())
}

// 主应用程序 | Main application
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建路由 | Create routes
    let app = Router::new()
        .route("/api/v1/users", get(get_users))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/:id", get(get_user))
        .route("/api/v1/users/:id", put(update_user))
        .route("/api/v1/users/:id", patch(patch_user))
        .route("/api/v1/users/:id", delete(delete_user))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::new().allow_origin(Any))
                .layer(middleware::from_fn(api_middleware))
        );
    
    // 启动服务器 | Start server
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("🚀 用户管理API服务器启动在 http://127.0.0.1:3000");
    println!("🚀 User Management API server running at http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确实现了REST架构的无状态原则？| Does the project correctly implement REST architecture's statelessness principle?
2. HTTP方法的使用是否符合语义化要求？| Does the HTTP method usage follow semantic requirements?
3. 状态码的使用是否准确反映了操作结果？| Do status codes accurately reflect operation results?
4. 资源URI设计是否遵循RESTful最佳实践？| Does resource URI design follow RESTful best practices?
5. 是否实现了合适的版本控制策略？| Is an appropriate versioning strategy implemented?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **REST约束理解强化练习 | REST Constraints Understanding Reinforcement Exercise**
   - **练习描述 | Exercise Description:** 分析现有API是否违反REST原则并提出改进方案
   - **概念检查 | Concept Check:** 能否识别API设计中违反REST约束的问题？
   - **学习目标 | Learning Objective:** 深化对REST架构约束的理解和应用

2. **HTTP语义应用练习 | HTTP Semantics Application Exercise**
   - **练习描述 | Exercise Description:** 设计一个复杂的资源操作场景，正确使用HTTP方法和状态码
   - **概念检查 | Concept Check:** 能否为不同场景选择合适的HTTP方法和状态码？
   - **学习目标 | Learning Objective:** 提高HTTP语义化使用能力

3. **API版本演进练习 | API Version Evolution Exercise**
   - **练习描述 | Exercise Description:** 设计API从v1到v3的演进过程，实现向后兼容
   - **概念检查 | Concept Check:** 理解版本控制的复杂性和迁移策略？
   - **学习目标 | Learning Objective:** 掌握API版本管理策略

4. **安全防护集成练习 | Security Protection Integration Exercise**
   - **练习描述 | Exercise Description:** 为用户管理API添加完整的安全防护机制
   - **概念检查 | Concept Check:** 能否识别和防范常见的API安全威胁？
   - **学习目标 | Learning Objective:** 提升API安全设计能力

5. **性能优化实践练习 | Performance Optimization Practice Exercise**
   - **练习描述 | Exercise Description:** 优化API响应时间和吞吐量，实现缓存策略
   - **概念检查 | Concept Check:** 理解API性能瓶颈和优化方法？
   - **学习目标 | Learning Objective:** 发展API性能优化技能

6. **API文档设计练习 | API Documentation Design Exercise**
   - **练习描述 | Exercise Description:** 为用户管理API创建完整的OpenAPI文档
   - **概念检查 | Concept Check:** 能否清晰描述API的使用方法和约束？
   - **学习目标 | Learning Objective:** 提高API文档编写质量

7. **错误处理完善练习 | Error Handling Enhancement Exercise**
   - **练习描述 | Exercise Description:** 设计统一的错误处理机制和用户友好的错误信息
   - **概念检查 | Concept Check:** 理解不同错误场景的正确处理方式？
   - **学习目标 | Learning Objective:** 完善错误处理和用户体验设计

## 学习资源 | Learning Resources
- [RESTful API设计最佳实践指南](https://restfulapi.net/)
- [HTTP状态码完整列表和使用指南](https://httpstatuses.com/)
- [API版本控制策略对比分析](https://swagger.io/blog/api-strategy/versioning/)
- [Web API安全防护最佳实践](https://owasp.org/www-project-api-security/)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] REST架构原则理解和应用
- [ ] HTTP方法语义化使用掌握
- [ ] 状态码正确选择和应用
- [ ] 资源设计最佳实践实施
- [ ] API版本控制策略实现
- [ ] 安全性考虑和防护机制
- [ ] 实践项目完成并通过功能测试
- [ ] 所有CCQs正确回答
- [ ] 代码示例理解和运行
- [ ] 扩展练习至少完成3个

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释RESTful API设计的核心原则和最佳实践。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain the core principles and best practices of RESTful API design to others.