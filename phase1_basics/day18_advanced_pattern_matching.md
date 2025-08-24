# Rust入门 - 第18天：高级模式匹配 | Rust Introduction - Day 18: Advanced Pattern Matching

## 学习目标 | Learning Objectives
- 掌握复杂的模式匹配语法和技巧 | Master complex pattern matching syntax and techniques
- 理解守卫条件的使用场景和语法 | Understand guard conditions usage scenarios and syntax
- 学会使用@绑定捕获匹配值 | Learn to use @ bindings to capture matched values
- 能够解构嵌套的复杂数据结构 | Be able to destructure nested complex data structures
- 掌握模式匹配在错误处理中的应用 | Master pattern matching applications in error handling
- 具备设计优雅模式匹配代码的能力 | Develop ability to design elegant pattern matching code

## 详细内容 | Detailed Content

### 1. 复杂模式匹配基础 | Complex Pattern Matching Fundamentals (1小时 | 1 hour)

- **复杂模式语法 | Complex Pattern Syntax**
  
  **概念定义 | Concept Definition:**
  复杂模式匹配是指在match表达式中使用多层嵌套、多种类型组合、以及特殊语法来匹配复杂数据结构的技术。它允许我们在单个模式中同时匹配数据结构的形状和内容，并提取所需的值。 | Complex pattern matching refers to the technique of using multi-level nesting, multiple type combinations, and special syntax in match expressions to match complex data structures. It allows us to simultaneously match both the shape and content of data structures in a single pattern while extracting desired values.
  
  **核心特征 | Key Characteristics:**
  - 支持多层嵌套结构的匹配和解构 | Supports matching and destructuring of multi-level nested structures
  - 可以在单个模式中结合多种匹配条件 | Can combine multiple matching conditions in a single pattern
  - 提供值绑定和忽略机制(_通配符) | Provides value binding and ignore mechanisms (_ wildcard)
  - 允许在匹配过程中进行类型检查和转换 | Allows type checking and conversion during matching process
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 复杂模式可以同时匹配数据的结构和值吗？| Can complex patterns match both structure and values of data simultaneously?
     **答案 | Answer:** 是 | Yes - 这是复杂模式匹配的核心特性 | This is the core feature of complex pattern matching
  2. 在模式中使用_通配符会绑定值吗？| Does using _ wildcard in patterns bind values?
     **答案 | Answer:** 否 | No - _用于忽略值，不进行绑定 | _ is used to ignore values without binding
  3. 一个模式可以同时匹配元组、结构体和枚举吗？| Can one pattern simultaneously match tuples, structs, and enums?
     **答案 | Answer:** 是 | Yes - 当它们嵌套在一起时可以组合匹配 | When they are nested together, they can be matched in combination
  4. 复杂模式匹配是否必须处理所有可能的情况？| Must complex pattern matching handle all possible cases?
     **答案 | Answer:** 是 | Yes - Rust要求模式匹配必须是穷尽的 | Rust requires pattern matching to be exhaustive
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 复杂数据结构定义 | Complex data structure definitions
  #[derive(Debug)]
  struct Point {
      x: i32,
      y: i32,
  }
  
  #[derive(Debug)]
  enum Shape {
      Circle(Point, f64),           // 圆：中心点和半径 | Circle: center point and radius
      Rectangle(Point, Point),      // 矩形：两个对角点 | Rectangle: two diagonal points
      Triangle(Point, Point, Point), // 三角形：三个顶点 | Triangle: three vertices
  }
  
  #[derive(Debug)]
  struct Drawing {
      name: String,
      shapes: Vec<Shape>,
      origin: Point,
  }
  
  // 复杂模式匹配示例 | Complex pattern matching examples
  fn analyze_drawing(drawing: &Drawing) {
      println!("分析图形：{} | Analyzing drawing: {}", drawing.name);
      
      for (index, shape) in drawing.shapes.iter().enumerate() {
          match shape {
              // 匹配原点处的圆形 | Match circles at origin
              Shape::Circle(Point { x: 0, y: 0 }, radius) => {
                  println!("  图形{}：原点处的圆，半径{} | Shape {}: Circle at origin, radius {}", 
                          index, radius);
              }
              // 匹配任意位置的大圆 | Match large circles at any position
              Shape::Circle(center, radius) if *radius > 10.0 => {
                  println!("  图形{}：大圆，中心({}, {})，半径{} | Shape {}: Large circle, center({}, {}), radius {}", 
                          index, center.x, center.y, radius);
              }
              // 匹配轴对齐的矩形 | Match axis-aligned rectangles
              Shape::Rectangle(
                  Point { x: x1, y: y1 }, 
                  Point { x: x2, y: y2 }
              ) if x1 == x2 || y1 == y2 => {
                  println!("  图形{}：轴对齐矩形，从({}, {})到({}, {}) | Shape {}: Axis-aligned rectangle, from ({}, {}) to ({}, {})", 
                          index, x1, y1, x2, y2);
              }
              // 匹配等边三角形（简化判断）| Match equilateral triangles (simplified)
              Shape::Triangle(p1, p2, p3) if is_equilateral(p1, p2, p3) => {
                  println!("  图形{}：等边三角形 | Shape {}: Equilateral triangle", index);
              }
              // 其他所有形状 | All other shapes
              other => {
                  println!("  图形{}：其他形状 {:?} | Shape {}: Other shape {:?}", index, other);
              }
          }
      }
  }
  
  // 辅助函数：判断是否为等边三角形 | Helper function: check if equilateral triangle
  fn is_equilateral(p1: &Point, p2: &Point, p3: &Point) -> bool {
      let d1 = distance(p1, p2);
      let d2 = distance(p2, p3);
      let d3 = distance(p3, p1);
      (d1 - d2).abs() < 0.001 && (d2 - d3).abs() < 0.001
  }
  
  fn distance(p1: &Point, p2: &Point) -> f64 {
      (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f64).sqrt()
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这个复杂匹配能正确识别原点处的圆形吗？| Can this complex matching correctly identify circles at the origin?
    **答案 | Answer:** 能，通过匹配Point { x: 0, y: 0 }模式 | Yes, by matching the Point { x: 0, y: 0 } pattern
  - 如果添加一个新的Shape变体会发生什么？| What happens if we add a new Shape variant?
    **答案 | Answer:** 需要更新match表达式以保持穷尽性 | Need to update the match expression to maintain exhaustiveness
  
  **常见误区检查 | Common Misconception Checks:**
  - 复杂模式匹配会影响运行时性能吗？| Do complex pattern matches affect runtime performance?
    **答案 | Answer:** 不会，模式匹配在编译时优化，运行时高效 | No, pattern matching is optimized at compile time and efficient at runtime
  - 可以在模式中直接调用函数吗？| Can functions be called directly in patterns?
    **答案 | Answer:** 不可以，但可以在守卫条件中调用函数 | No, but functions can be called in guard conditions

- **嵌套解构技巧 | Nested Destructuring Techniques**
  
  **概念定义 | Concept Definition:**
  嵌套解构是指在模式匹配中深入到数据结构的多个层级，同时提取和绑定内部的值。这种技术允许我们用简洁的语法处理复杂的数据结构，避免多层的手动访问。 | Nested destructuring refers to diving into multiple levels of data structures in pattern matching while extracting and binding internal values. This technique allows us to handle complex data structures with concise syntax, avoiding multiple layers of manual access.
  
  **核心特征 | Key Characteristics:**
  - 可以同时解构多层嵌套的数据结构 | Can simultaneously destructure multi-level nested data structures
  - 支持混合不同类型的解构（元组、结构体、枚举等）| Supports mixed destructuring of different types (tuples, structs, enums, etc.)
  - 允许选择性提取需要的值，忽略不需要的部分 | Allows selective extraction of needed values while ignoring unnecessary parts
  - 提供一次性访问深层数据的能力 | Provides ability to access deep data in one operation
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 嵌套解构可以跳过中间层直接访问深层数据吗？| Can nested destructuring skip intermediate layers to access deep data directly?
     **答案 | Answer:** 不可以 | No - 必须按照数据结构的层次进行解构 | Must destructure according to the hierarchy of the data structure
  2. 在嵌套解构中可以部分忽略某些字段吗？| Can some fields be partially ignored in nested destructuring?
     **答案 | Answer:** 可以 | Yes - 使用..语法可以忽略剩余字段 | Using .. syntax can ignore remaining fields
  3. 嵌套解构是否受到数据结构深度的限制？| Is nested destructuring limited by the depth of data structures?
     **答案 | Answer:** 理论上不受限制 | Theoretically unlimited - 但过深的嵌套会影响可读性 | but overly deep nesting affects readability
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 复杂嵌套数据结构 | Complex nested data structures
  #[derive(Debug, Clone)]
  struct User {
      id: u64,
      profile: UserProfile,
      settings: UserSettings,
  }
  
  #[derive(Debug, Clone)]
  struct UserProfile {
      name: String,
      email: String,
      address: Option<Address>,
  }
  
  #[derive(Debug, Clone)]
  struct Address {
      country: String,
      city: String,
      street: String,
      postal_code: String,
  }
  
  #[derive(Debug, Clone)]
  struct UserSettings {
      theme: Theme,
      notifications: NotificationSettings,
      privacy: PrivacyLevel,
  }
  
  #[derive(Debug, Clone)]
  enum Theme {
      Light,
      Dark,
      Custom { background: String, text: String },
  }
  
  #[derive(Debug, Clone)]
  struct NotificationSettings {
      email_enabled: bool,
      push_enabled: bool,
      frequency: NotificationFrequency,
  }
  
  #[derive(Debug, Clone)]
  enum NotificationFrequency {
      Immediate,
      Daily,
      Weekly,
      Custom(u32), // 自定义小时数 | Custom hours
  }
  
  #[derive(Debug, Clone)]
  enum PrivacyLevel {
      Public,
      Friends,
      Private,
  }
  
  // 嵌套解构示例 | Nested destructuring examples
  fn analyze_user(user: &User) {
      match user {
          // 深层嵌套解构：直接访问多层结构 | Deep nested destructuring: direct access to multi-level structures
          User {
              id,
              profile: UserProfile {
                  name,
                  email,
                  address: Some(Address {
                      country,
                      city,
                      postal_code,
                      ..  // 忽略street字段 | Ignore street field
                  })
              },
              settings: UserSettings {
                  theme: Theme::Custom { background, text },
                  notifications: NotificationSettings {
                      email_enabled: true,
                      frequency: NotificationFrequency::Custom(hours),
                      ..
                  },
                  privacy: PrivacyLevel::Public,
              }
          } => {
              println!("高级用户分析 | Advanced user analysis:");
              println!("  ID: {}, 姓名: {} | ID: {}, Name: {}", id, name);
              println!("  邮箱: {} | Email: {}", email);
              println!("  位置: {}, {} {} | Location: {}, {} {}", country, city, postal_code);
              println!("  自定义主题: 背景{}, 文字{} | Custom theme: background {}, text {}", background, text);
              println!("  邮件通知: 开启, {}小时频率 | Email notifications: enabled, {} hours frequency", hours);
              println!("  隐私级别: 公开 | Privacy level: Public");
          }
          
          // 匹配没有地址的用户 | Match users without address
          User {
              id,
              profile: UserProfile { name, address: None, .. },
              settings: UserSettings { theme: Theme::Dark, .. }
          } => {
              println!("简单用户：ID {}, 姓名 {}, 无地址, 暗色主题 | Simple user: ID {}, Name {}, No address, Dark theme", id, name);
          }
          
          // 匹配特定通知设置 | Match specific notification settings
          User {
              profile: UserProfile { name, .. },
              settings: UserSettings {
                  notifications: NotificationSettings {
                      email_enabled: false,
                      push_enabled: false,
                      ..
                  },
                  ..
              },
              ..
          } => {
              println!("无通知用户：{} | No-notification user: {}", name);
          }
          
          // 处理其他所有情况 | Handle all other cases
          User { 
              id, 
              profile: UserProfile { name, .. }, 
              .. 
          } => {
              println!("普通用户：ID {}, 姓名 {} | Regular user: ID {}, Name {}", id, name);
          }
      }
  }
  
  // 元组和枚举的混合嵌套解构 | Mixed nested destructuring with tuples and enums
  enum DatabaseResult<T> {
      Success(T),
      Error(String, u32), // 错误信息和错误代码 | Error message and error code
      Timeout,
  }
  
  type UserQueryResult = DatabaseResult<(User, Vec<String>)>; // 用户和标签列表 | User and tag list
  
  fn handle_query_result(result: UserQueryResult) {
      match result {
          // 成功且用户有地址的情况 | Success case with user having address
          DatabaseResult::Success((
              User {
                  profile: UserProfile {
                      name,
                      address: Some(Address { city, country, .. }),
                      ..
                  },
                  ..
              },
              tags
          )) if !tags.is_empty() => {
              println!("成功查询到用户 {} (来自 {}, {})，标签：{:?} | Successfully queried user {} (from {}, {}), tags: {:?}", 
                      name, city, country, tags);
          }
          
          // 数据库错误处理 | Database error handling
          DatabaseResult::Error(msg, code) if code >= 500 => {
              println!("严重数据库错误 {}: {} | Severe database error {}: {}", code, msg);
          }
          
          // 其他情况 | Other cases
          _ => {
              println!("查询失败或无结果 | Query failed or no results");
          }
      }
  }
  ```

### 2. 守卫条件详解 | Guard Conditions in Detail (45分钟 | 45 minutes)

- **守卫语法与应用 | Guard Syntax and Applications**
  
  **概念定义 | Concept Definition:**
  守卫条件（match guards）是在模式匹配的基础上添加的额外布尔表达式，用于进一步限制匹配的条件。只有当模式匹配成功且守卫条件为真时，该分支才会被执行。守卫提供了超出基本模式匹配能力的灵活性。 | Guard conditions (match guards) are additional boolean expressions added on top of pattern matching to further restrict matching conditions. A branch is only executed when both the pattern matches successfully and the guard condition is true. Guards provide flexibility beyond basic pattern matching capabilities.
  
  **核心特征 | Key Characteristics:**
  - 使用if关键字在模式后添加条件表达式 | Use if keyword to add conditional expressions after patterns
  - 可以访问模式中绑定的变量进行计算 | Can access variables bound in patterns for calculations
  - 支持复杂的逻辑表达式和函数调用 | Support complex logical expressions and function calls
  - 与模式匹配的穷尽性检查独立进行 | Exhaustiveness checking is independent of pattern matching
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 守卫条件会影响模式匹配的穷尽性检查吗？| Do guard conditions affect exhaustiveness checking of pattern matching?
     **答案 | Answer:** 不会 | No - 编译器只检查模式的穷尽性，不考虑守卫条件 | The compiler only checks pattern exhaustiveness, not guard conditions
  2. 守卫条件可以访问模式中绑定的变量吗？| Can guard conditions access variables bound in patterns?
     **答案 | Answer:** 可以 | Yes - 这是守卫条件的主要用途之一 | This is one of the main purposes of guard conditions
  3. 一个模式可以有多个守卫条件吗？| Can a pattern have multiple guard conditions?
     **答案 | Answer:** 不可以 | No - 一个分支只能有一个守卫条件，但可以使用逻辑运算符组合多个条件 | One branch can only have one guard condition, but multiple conditions can be combined using logical operators
  4. 守卫条件失败时会尝试下一个模式分支吗？| When a guard condition fails, will the next pattern branch be tried?
     **答案 | Answer:** 会 | Yes - 守卫失败会继续匹配后续分支 | Guard failure will continue matching subsequent branches
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 数值范围和条件判断 | Numeric ranges and conditional judgments
  fn classify_number(n: i32) -> &'static str {
      match n {
          // 使用守卫检查特定条件 | Use guards to check specific conditions
          x if x < 0 => "负数 | Negative number",
          0 => "零 | Zero",
          x if x > 0 && x <= 10 => "小正数 | Small positive number",
          x if x > 10 && x <= 100 => "中等正数 | Medium positive number",
          x if x > 100 && x % 2 == 0 => "大偶数 | Large even number",
          x if x > 100 && x % 2 == 1 => "大奇数 | Large odd number",
          _ => "未分类 | Unclassified"
      }
  }
  
  // 复杂数据结构的守卫应用 | Guard applications with complex data structures
  #[derive(Debug)]
  enum Temperature {
      Celsius(f64),
      Fahrenheit(f64),
      Kelvin(f64),
  }
  
  #[derive(Debug)]
  struct WeatherReport {
      location: String,
      temperature: Temperature,
      humidity: u8,
      wind_speed: f64,
  }
  
  fn analyze_weather(report: &WeatherReport) -> String {
      match report {
          // 极端温度警告 | Extreme temperature warnings
          WeatherReport {
              location,
              temperature: Temperature::Celsius(temp),
              ..
          } if *temp > 40.0 => {
              format!("⚠️ {}高温警告：{}°C | ⚠️ {} High temperature warning: {}°C", location, temp)
          }
          
          WeatherReport {
              location,
              temperature: Temperature::Celsius(temp),
              ..
          } if *temp < -20.0 => {
              format!("❄️ {}极寒警告：{}°C | ❄️ {} Extreme cold warning: {}°C", location, temp)
          }
          
          // 舒适天气条件 | Comfortable weather conditions
          WeatherReport {
              location,
              temperature: Temperature::Celsius(temp),
              humidity,
              wind_speed,
              ..
          } if *temp >= 20.0 && *temp <= 26.0 && *humidity <= 60 && *wind_speed <= 15.0 => {
              format!("☀️ {}天气舒适：{}°C, 湿度{}%, 风速{}km/h | ☀️ {} Comfortable weather: {}°C, humidity {}%, wind speed {}km/h", 
                     location, temp, humidity, wind_speed)
          }
          
          // 高湿度条件 | High humidity conditions
          WeatherReport {
              location,
              temperature: Temperature::Celsius(temp),
              humidity,
              ..
          } if *humidity > 80 => {
              format!("💧 {}高湿度：{}°C, {}% | 💧 {} High humidity: {}°C, {}%", location, temp, humidity)
          }
          
          // 大风条件 | Windy conditions
          WeatherReport {
              location,
              wind_speed,
              ..
          } if *wind_speed > 50.0 => {
              format!("💨 {}大风警告：{}km/h | 💨 {} Wind warning: {}km/h", location, wind_speed)
          }
          
          // 华氏温度转换处理 | Fahrenheit temperature conversion handling
          WeatherReport {
              location,
              temperature: Temperature::Fahrenheit(temp_f),
              ..
          } => {
              let temp_c = (*temp_f - 32.0) * 5.0 / 9.0;
              // 递归调用处理摄氏度 | Recursive call to handle Celsius
              let celsius_report = WeatherReport {
                  location: location.clone(),
                  temperature: Temperature::Celsius(temp_c),
                  humidity: report.humidity,
                  wind_speed: report.wind_speed,
              };
              analyze_weather(&celsius_report)
          }
          
          // 开尔文温度处理 | Kelvin temperature handling
          WeatherReport {
              location,
              temperature: Temperature::Kelvin(temp_k),
              ..
          } => {
              let temp_c = temp_k - 273.15;
              let celsius_report = WeatherReport {
                  location: location.clone(),
                  temperature: Temperature::Celsius(temp_c),
                  humidity: report.humidity,
                  wind_speed: report.wind_speed,
              };
              analyze_weather(&celsius_report)
          }
          
          // 默认情况 | Default case
          WeatherReport { location, .. } => {
              format!("📊 {}天气数据正常 | 📊 {} Weather data normal", location)
          }
      }
  }
  
  // 函数调用在守卫中的应用 | Function calls in guards
  fn is_prime(n: u32) -> bool {
      if n < 2 { return false; }
      for i in 2..=(n as f64).sqrt() as u32 {
          if n % i == 0 { return false; }
      }
      true
  }
  
  fn categorize_integer(n: u32) -> String {
      match n {
          x if x == 0 => "零 | Zero".to_string(),
          x if x == 1 => "单位 | Unit".to_string(),
          x if is_prime(x) => format!("质数：{} | Prime number: {}", x),
          x if x % 2 == 0 => format!("合数偶数：{} | Composite even: {}", x),
          x => format!("合数奇数：{} | Composite odd: {}", x),
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 如果一个守卫条件很复杂，是否会影响匹配性能？| If a guard condition is very complex, will it affect matching performance?
    **答案 | Answer:** 会，复杂的守卫会在运行时执行，应考虑性能影响 | Yes, complex guards execute at runtime and performance impact should be considered
  - 守卫条件可以修改被匹配的值吗？| Can guard conditions modify the matched value?
    **答案 | Answer:** 不能，守卫中只能进行只读访问 | No, only read-only access is allowed in guards

### 3. @绑定语法应用 | @ Binding Syntax Applications (45分钟 | 45 minutes)

- **@绑定基础概念 | @ Binding Fundamentals**
  
  **概念定义 | Concept Definition:**
  @绑定（@ binding）是Rust模式匹配中的一种特殊语法，允许我们在匹配子模式的同时，将整个匹配的值绑定到一个变量上。这样可以同时访问整体值和其内部结构，提供更大的灵活性。 | @ binding is a special syntax in Rust pattern matching that allows us to bind the entire matched value to a variable while simultaneously matching sub-patterns. This provides access to both the whole value and its internal structure, offering greater flexibility.
  
  **核心特征 | Key Characteristics:**
  - 使用variable @ pattern语法绑定整个匹配值 | Use variable @ pattern syntax to bind the entire matched value
  - 可以在守卫条件中使用绑定的变量 | Can use bound variables in guard conditions
  - 支持嵌套的@绑定用于复杂结构 | Support nested @ bindings for complex structures
  - 绑定的变量包含完整的原始数据 | Bound variables contain complete original data
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. @绑定会复制数据还是创建引用？| Does @ binding copy data or create references?
     **答案 | Answer:** 取决于类型 | Depends on the type - 对于Copy类型会复制，对于非Copy类型会移动 | Copy types are copied, non-Copy types are moved
  2. 可以在一个模式中使用多个@绑定吗？| Can multiple @ bindings be used in one pattern?
     **答案 | Answer:** 可以 | Yes - 可以为不同的子模式创建多个绑定 | Multiple bindings can be created for different sub-patterns
  3. @绑定的变量可以在模式外部使用吗？| Can variables from @ bindings be used outside the pattern?
     **答案 | Answer:** 可以 | Yes - 在对应的match分支中可以使用 | They can be used in the corresponding match branch
  4. @绑定可以与守卫条件结合使用吗？| Can @ bindings be combined with guard conditions?
     **答案 | Answer:** 可以 | Yes - 这是@绑定的常见用途 | This is a common use of @ bindings
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // @绑定基础示例 | Basic @ binding examples
  #[derive(Debug, Clone)]
  enum Message {
      Text(String),
      Image { url: String, alt: String },
      Video { url: String, duration: u32 },
      File { name: String, size: u64, mime_type: String },
  }
  
  fn process_message(msg: Message) {
      match msg {
          // 绑定整个文本消息同时访问内容 | Bind entire text message while accessing content
          text_msg @ Message::Text(content) if content.len() > 100 => {
              println!("长文本消息 | Long text message:");
              println!("  完整消息：{:?} | Complete message: {:?}", text_msg);
              println!("  内容长度：{} 字符 | Content length: {} characters", content.len());
              println!("  预览：{}... | Preview: {}...", &content[..50]);
          }
          
          // 绑定文件消息并检查大小 | Bind file message and check size
          file_msg @ Message::File { name, size, .. } if *size > 1024 * 1024 => {
              println!("大文件消息 | Large file message:");
              println!("  完整消息：{:?} | Complete message: {:?}", file_msg);
              println!("  文件名：{} | Filename: {}", name);
              println!("  大小：{} MB | Size: {} MB", size / (1024 * 1024));
              
              // 可以将整个消息传递给其他函数 | Can pass entire message to other functions
              archive_large_file(file_msg);
          }
          
          // 嵌套@绑定示例 | Nested @ binding example
          media_msg @ Message::Image { url: image_url @ _, alt } |
          media_msg @ Message::Video { url: image_url @ _, .. } => {
              println!("媒体消息处理 | Media message processing:");
              println!("  完整消息：{:?} | Complete message: {:?}", media_msg);
              println!("  媒体URL：{} | Media URL: {}", image_url);
              
              if let Message::Image { alt, .. } = &media_msg {
                  println!("  图片描述：{} | Image alt: {}", alt);
              }
          }
          
          // 简单消息处理 | Simple message processing
          simple_msg => {
              println!("简单消息：{:?} | Simple message: {:?}", simple_msg);
          }
      }
  }
  
  fn archive_large_file(file_msg: Message) {
      if let Message::File { name, size, mime_type } = file_msg {
          println!("归档文件：{} ({} bytes, {}) | Archiving file: {} ({} bytes, {})", 
                  name, size, mime_type);
      }
  }
  
  // 范围匹配中的@绑定 | @ bindings in range matching
  fn classify_http_status(code: u16) -> String {
      match code {
          // 绑定具体的成功状态码 | Bind specific success status codes
          success @ 200..=299 => {
              format!("成功响应：{} | Success response: {}", success)
          }
          
          // 绑定重定向状态码 | Bind redirection status codes
          redirect @ 300..=399 if redirect == 301 || redirect == 302 => {
              format!("永久/临时重定向：{} | Permanent/Temporary redirect: {}", redirect)
          }
          
          redirect @ 300..=399 => {
              format!("其他重定向：{} | Other redirect: {}", redirect)
          }
          
          // 绑定客户端错误 | Bind client errors
          client_error @ 400..=499 => {
              let error_type = match client_error {
                  400 => "错误请求 | Bad Request",
                  401 => "未授权 | Unauthorized", 
                  403 => "禁止访问 | Forbidden",
                  404 => "未找到 | Not Found",
                  _ => "其他客户端错误 | Other client error"
              };
              format!("客户端错误 {}: {} | Client error {}: {}", client_error, error_type)
          }
          
          // 绑定服务器错误 | Bind server errors
          server_error @ 500..=599 => {
              format!("服务器错误：{} | Server error: {}", server_error)
          }
          
          // 其他状态码 | Other status codes
          other => {
              format!("未知状态码：{} | Unknown status code: {}", other)
          }
      }
  }
  
  // 复杂结构的@绑定 | @ bindings with complex structures
  #[derive(Debug)]
  struct Config {
      database: DatabaseConfig,
      server: ServerConfig,
      logging: LoggingConfig,
  }
  
  #[derive(Debug)]
  struct DatabaseConfig {
      host: String,
      port: u16,
      name: String,
      pool_size: u32,
  }
  
  #[derive(Debug)]
  struct ServerConfig {
      host: String,
      port: u16,
      workers: u32,
  }
  
  #[derive(Debug)]
  struct LoggingConfig {
      level: String,
      file: Option<String>,
  }
  
  fn validate_config(config: &Config) -> Result<(), String> {
      match config {
          // 验证数据库配置 | Validate database configuration
          Config {
              database: db_config @ DatabaseConfig { 
                  host, 
                  port, 
                  pool_size,
                  .. 
              },
              ..
          } if host.is_empty() || *port == 0 || *pool_size == 0 => {
              Err(format!("无效的数据库配置：{:?} | Invalid database config: {:?}", db_config))
          }
          
          // 验证服务器配置 | Validate server configuration
          Config {
              server: server_config @ ServerConfig {
                  host,
                  port,
                  workers,
              },
              ..
          } if host.is_empty() || *port == 0 || *workers == 0 => {
              Err(format!("无效的服务器配置：{:?} | Invalid server config: {:?}", server_config))
          }
          
          // 特殊的生产环境配置检查 | Special production environment config check
          production_config @ Config {
              database: DatabaseConfig { pool_size, .. },
              server: ServerConfig { workers, .. },
              logging: LoggingConfig { level, file: Some(_) },
              ..
          } if level == "debug" && *pool_size > 50 && *workers > 10 => {
              println!("⚠️ 生产配置警告：调试级别日志可能影响性能 | ⚠️ Production config warning: debug level logging may impact performance");
              println!("配置详情：{:?} | Config details: {:?}", production_config);
              Ok(())
          }
          
          // 有效配置 | Valid configuration
          valid_config => {
              println!("✅ 配置验证通过：{:?} | ✅ Configuration validated: {:?}", valid_config);
              Ok(())
          }
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - @绑定的变量何时被创建？| When are variables from @ bindings created?
    **答案 | Answer:** 在模式匹配成功时创建 | Created when pattern matching succeeds
  - 可以对@绑定的值进行修改吗？| Can values from @ bindings be modified?
    **答案 | Answer:** 取决于绑定的方式，通常是不可变的 | Depends on the binding method, usually immutable

### 4. 解构嵌套数据结构 | Destructuring Nested Data Structures (45分钟 | 45 minutes)

- **深层嵌套解构策略 | Deep Nested Destructuring Strategies**
  
  **概念定义 | Concept Definition:**
  深层嵌套解构是指在复杂的多层数据结构中，通过模式匹配一次性访问和提取深层嵌套的值。这种技术可以避免多次手动访问，使代码更简洁和表达力更强。 | Deep nested destructuring refers to accessing and extracting deeply nested values in complex multi-level data structures through pattern matching in one operation. This technique avoids multiple manual accesses, making code more concise and expressive.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 深层解构是否必须指定所有中间层的结构？| Must deep destructuring specify the structure of all intermediate layers?
     **答案 | Answer:** 是的 | Yes - 必须按照数据结构的层次进行匹配 | Must match according to the hierarchy of the data structure
  2. 可以在深层解构中跳过某些字段吗？| Can some fields be skipped in deep destructuring?
     **答案 | Answer:** 可以 | Yes - 使用..或_来跳过不需要的字段 | Use .. or _ to skip unneeded fields
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 复杂嵌套结构示例 | Complex nested structure example
  #[derive(Debug, Clone)]
  struct Company {
      name: String,
      departments: Vec<Department>,
      headquarters: Location,
  }
  
  #[derive(Debug, Clone)]
  struct Department {
      name: String,
      manager: Employee,
      teams: Vec<Team>,
      budget: Budget,
  }
  
  #[derive(Debug, Clone)]
  struct Team {
      name: String,
      lead: Employee,
      members: Vec<Employee>,
      projects: Vec<Project>,
  }
  
  #[derive(Debug, Clone)]
  struct Employee {
      id: u32,
      name: String,
      position: Position,
      contact: ContactInfo,
  }
  
  #[derive(Debug, Clone)]
  struct Position {
      title: String,
      level: u8,
      salary: u64,
  }
  
  #[derive(Debug, Clone)]
  struct ContactInfo {
      email: String,
      phone: Option<String>,
      address: Option<Address>,
  }
  
  #[derive(Debug, Clone)]
  struct Address {
      street: String,
      city: String,
      country: String,
      postal_code: String,
  }
  
  #[derive(Debug, Clone)]
  struct Location {
      name: String,
      address: Address,
      coordinates: (f64, f64),
  }
  
  #[derive(Debug, Clone)]
  struct Budget {
      allocated: u64,
      spent: u64,
      quarter: u8,
  }
  
  #[derive(Debug, Clone)]
  struct Project {
      name: String,
      status: ProjectStatus,
      budget: u64,
      deadline: String,
  }
  
  #[derive(Debug, Clone)]
  enum ProjectStatus {
      Planning,
      InProgress { completion: u8 },
      Completed,
      OnHold,
  }
  
  // 深层嵌套解构分析函数 | Deep nested destructuring analysis functions
  fn analyze_company_structure(company: &Company) {
      println!("🏢 分析公司结构：{} | 🏢 Analyzing company structure: {}", company.name);
      
      for dept in &company.departments {
          match dept {
              // 解构到团队和项目级别 | Destructure to team and project level
              Department {
                  name: dept_name,
                  manager: Employee {
                      name: manager_name,
                      position: Position { title: manager_title, level: manager_level, .. },
                      contact: ContactInfo {
                          email: manager_email,
                          address: Some(Address { city: manager_city, .. }),
                          ..
                      },
                      ..
                  },
                  teams,
                  budget: Budget { allocated, spent, quarter },
                  ..
              } if *manager_level >= 8 && teams.len() > 2 => {
                  println!("  📊 高级部门：{} | 📊 Senior department: {}", dept_name);
                  println!("    管理者：{} ({}, Level {}) | Manager: {} ({}, Level {})", 
                          manager_name, manager_title, manager_level);
                  println!("    联系方式：{}, 位于{} | Contact: {}, Located in {}", 
                          manager_email, manager_city);
                  println!("    预算：{}/{} (Q{}) | Budget: {}/{} (Q{})", spent, allocated, quarter);
                  
                  // 分析团队结构 | Analyze team structure
                  for team in teams {
                      analyze_team_deep(team);
                  }
              }
              
              // 小型部门的简化处理 | Simplified handling for small departments
              Department {
                  name: dept_name,
                  manager: Employee { name: manager_name, .. },
                  teams,
                  ..
              } if teams.len() <= 2 => {
                  println!("  🏠 小型部门：{}, 管理者：{} | 🏠 Small department: {}, Manager: {}", 
                          dept_name, manager_name);
              }
              
              _ => {
                  println!("  📋 普通部门：{} | 📋 Regular department: {}", dept.name);
              }
          }
      }
  }
  
  fn analyze_team_deep(team: &Team) {
      match team {
          // 活跃项目团队的深层分析 | Deep analysis of active project teams
          Team {
              name: team_name,
              lead: Employee {
                  name: lead_name,
                  position: Position { salary: lead_salary, .. },
                  ..
              },
              members,
              projects,
              ..
          } if projects.iter().any(|p| matches!(p.status, ProjectStatus::InProgress { .. })) => {
              println!("    🚀 活跃团队：{}, 负责人：{} (薪资：{}) | 🚀 Active team: {}, Lead: {} (Salary: {})", 
                      team_name, lead_name, lead_salary);
              
              // 分析进行中的项目 | Analyze in-progress projects
              for project in projects {
                  if let Project {
                      name: project_name,
                      status: ProjectStatus::InProgress { completion },
                      budget: project_budget,
                      deadline,
                      ..
                  } = project {
                      println!("      📅 项目：{}, 完成度：{}%, 预算：{}, 截止：{} | 📅 Project: {}, Progress: {}%, Budget: {}, Deadline: {}", 
                              project_name, completion, project_budget, deadline);
                  }
              }
              
              // 分析团队成员结构 | Analyze team member structure
              let senior_members: Vec<_> = members.iter()
                  .filter_map(|member| {
                      match member {
                          Employee {
                              name,
                              position: Position { level, title, .. },
                              contact: ContactInfo { email, .. },
                              ..
                          } if *level >= 6 => Some((name, title, level, email)),
                          _ => None
                      }
                  })
                  .collect();
              
              if !senior_members.is_empty() {
                  println!("      👥 高级成员：| 👥 Senior members:");
                  for (name, title, level, email) in senior_members {
                      println!("        - {} ({}, L{}) - {} | - {} ({}, L{}) - {}", 
                              name, title, level, email);
                  }
              }
          }
          
          // 其他团队的简化分析 | Simplified analysis for other teams
          Team { name: team_name, members, .. } => {
              println!("    👥 团队：{}, 成员数：{} | 👥 Team: {}, Members: {}", 
                      team_name, members.len());
          }
      }
  }
  
  // 复杂条件的嵌套匹配 | Complex conditional nested matching
  fn find_critical_issues(company: &Company) -> Vec<String> {
      let mut issues = Vec::new();
      
      for dept in &company.departments {
          match dept {
              // 预算超支的部门 | Departments with budget overrun
              Department {
                  name: dept_name,
                  budget: Budget { allocated, spent, .. },
                  teams,
                  ..
              } if *spent > *allocated => {
                  issues.push(format!("部门 {} 预算超支：{}/{} | Department {} budget overrun: {}/{}", 
                                    dept_name, spent, allocated));
                  
                  // 检查是否有高风险项目 | Check for high-risk projects
                  for team in teams {
                      for project in &team.projects {
                          if let Project {
                              name: project_name,
                              status: ProjectStatus::InProgress { completion },
                              budget: project_budget,
                              ..
                          } = project {
                              if *completion < 50 && *project_budget > 100000 {
                                  issues.push(format!("高风险项目：{} (完成度：{}%, 预算：{}) | High-risk project: {} (Progress: {}%, Budget: {})", 
                                                    project_name, completion, project_budget));
                              }
                          }
                      }
                  }
              }
              
              // 管理层联系信息不完整 | Incomplete management contact information
              Department {
                  name: dept_name,
                  manager: Employee {
                      name: manager_name,
                      contact: ContactInfo {
                          phone: None,
                          address: None,
                          ..
                      },
                      ..
                  },
                  ..
              } => {
                  issues.push(format!("部门 {} 管理者 {} 联系信息不完整 | Department {} manager {} has incomplete contact info", 
                                    dept_name, manager_name));
              }
              
              _ => {} // 无问题的部门 | Departments without issues
          }
      }
      
      issues
  }
  ```

### 5. 错误处理中的模式匹配 | Pattern Matching in Error Handling (30分钟 | 30 minutes)

- **Result和Option的高级匹配 | Advanced Matching with Result and Option**
  
  **概念定义 | Concept Definition:**
  在错误处理中使用模式匹配是Rust的核心特性之一。通过对Result和Option类型的精细匹配，我们可以优雅地处理各种错误情况，提供详细的错误信息，并实现恰当的错误恢复策略。 | Using pattern matching in error handling is one of Rust's core features. Through fine-grained matching of Result and Option types, we can elegantly handle various error conditions, provide detailed error information, and implement appropriate error recovery strategies.
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 可以在match中同时处理Ok和Err的不同变体吗？| Can different variants of Ok and Err be handled simultaneously in match?
     **答案 | Answer:** 可以 | Yes - 可以为不同的错误类型创建不同的分支 | Different branches can be created for different error types
  2. 模式匹配是否可以链式处理多个Result？| Can pattern matching handle multiple Results in a chain?
     **答案 | Answer:** 可以 | Yes - 通过嵌套匹配或组合模式 | Through nested matching or combined patterns
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::fs::File;
  use std::io::{self, Read, Write, ErrorKind};
  use std::num::ParseIntError;
  
  // 自定义错误类型 | Custom error types
  #[derive(Debug)]
  enum ProcessingError {
      IoError(io::Error),
      ParseError(ParseIntError),
      ValidationError(String),
      NetworkError { code: u16, message: String },
      DatabaseError { table: String, operation: String },
  }
  
  // 复杂的错误处理匹配 | Complex error handling matching
  fn process_data_file(filename: &str) -> Result<Vec<i32>, ProcessingError> {
      // 尝试读取文件 | Try to read file
      let mut file = match File::open(filename) {
          Ok(file) => file,
          Err(error) => match error.kind() {
              ErrorKind::NotFound => {
                  println!("文件未找到，尝试创建默认文件 | File not found, trying to create default file");
                  return create_default_file(filename);
              }
              ErrorKind::PermissionDenied => {
                  println!("权限被拒绝，尝试只读模式 | Permission denied, trying read-only mode");
                  return Err(ProcessingError::IoError(error));
              }
              _ => return Err(ProcessingError::IoError(error)),
          }
      };
      
      let mut contents = String::new();
      file.read_to_string(&mut contents)
          .map_err(ProcessingError::IoError)?;
      
      // 解析数据 | Parse data
      let numbers: Result<Vec<i32>, _> = contents
          .lines()
          .filter(|line| !line.trim().is_empty())
          .map(|line| line.trim().parse::<i32>())
          .collect();
      
      match numbers {
          Ok(nums) => {
              // 验证数据 | Validate data
              match validate_numbers(&nums) {
                  Ok(()) => Ok(nums),
                  Err(validation_error) => Err(ProcessingError::ValidationError(validation_error)),
              }
          }
          Err(parse_error) => Err(ProcessingError::ParseError(parse_error)),
      }
  }
  
  fn create_default_file(filename: &str) -> Result<Vec<i32>, ProcessingError> {
      let default_data = "1\n2\n3\n4\n5\n";
      
      match File::create(filename) {
          Ok(mut file) => {
              match file.write_all(default_data.as_bytes()) {
                  Ok(()) => {
                      println!("默认文件创建成功 | Default file created successfully");
                      Ok(vec![1, 2, 3, 4, 5])
                  }
                  Err(write_error) => Err(ProcessingError::IoError(write_error)),
              }
          }
          Err(create_error) => Err(ProcessingError::IoError(create_error)),
      }
  }
  
  fn validate_numbers(numbers: &[i32]) -> Result<(), String> {
      match numbers {
          [] => Err("数据为空 | Data is empty".to_string()),
          nums if nums.len() < 3 => Err("数据太少 | Too little data".to_string()),
          nums if nums.iter().any(|&n| n < 0) => Err("包含负数 | Contains negative numbers".to_string()),
          nums if nums.iter().any(|&n| n > 1000) => Err("包含过大的数值 | Contains oversized values".to_string()),
          _ => Ok(()),
      }
  }
  
  // 综合错误处理和恢复策略 | Comprehensive error handling and recovery strategies
  fn robust_data_processor(filenames: &[&str]) -> (Vec<i32>, Vec<String>) {
      let mut all_data = Vec::new();
      let mut errors = Vec::new();
      
      for &filename in filenames {
          match process_data_file(filename) {
              Ok(mut data) => {
                  println!("✅ 成功处理文件：{} | ✅ Successfully processed file: {}", filename);
                  all_data.append(&mut data);
              }
              
              Err(error) => {
                  let error_message = match error {
                      ProcessingError::IoError(io_err) => match io_err.kind() {
                          ErrorKind::NotFound => {
                              format!("文件 {} 未找到 | File {} not found", filename)
                          }
                          ErrorKind::PermissionDenied => {
                              format!("文件 {} 权限不足 | Insufficient permissions for file {}", filename)
                          }
                          _ => {
                              format!("文件 {} IO错误：{} | IO error for file {}: {}", filename, io_err)
                          }
                      },
                      
                      ProcessingError::ParseError(parse_err) => {
                          format!("文件 {} 解析错误：{} | Parse error for file {}: {}", filename, parse_err)
                      }
                      
                      ProcessingError::ValidationError(msg) => {
                          format!("文件 {} 验证失败：{} | Validation failed for file {}: {}", filename, msg)
                      }
                      
                      ProcessingError::NetworkError { code, message } => {
                          format!("网络错误 {}：{} | Network error {}: {}", code, message)
                      }
                      
                      ProcessingError::DatabaseError { table, operation } => {
                          format!("数据库错误：表 {} 操作 {} 失败 | Database error: table {} operation {} failed", table, operation)
                      }
                  };
                  
                  println!("❌ {}", error_message);
                  errors.push(error_message);
              }
          }
      }
      
      (all_data, errors)
  }
  
  // Option类型的高级匹配 | Advanced matching with Option type
  #[derive(Debug)]
  struct UserProfile {
      username: String,
      email: Option<String>,
      phone: Option<String>,
      address: Option<UserAddress>,
      preferences: UserPreferences,
  }
  
  #[derive(Debug)]
  struct UserAddress {
      street: String,
      city: String,
      country: String,
      postal_code: Option<String>,
  }
  
  #[derive(Debug)]
  struct UserPreferences {
      newsletter: bool,
      notifications: NotificationLevel,
      theme: Option<String>,
  }
  
  #[derive(Debug)]
  enum NotificationLevel {
      None,
      Important,
      All,
  }
  
  fn analyze_user_profile(profile: &UserProfile) -> String {
      match profile {
          // 完整信息的用户 | Users with complete information
          UserProfile {
              username,
              email: Some(email),
              phone: Some(phone),
              address: Some(UserAddress {
                  city,
                  country,
                  postal_code: Some(postal),
                  ..
              }),
              preferences: UserPreferences {
                  newsletter: true,
                  notifications: NotificationLevel::All,
                  theme: Some(theme),
                  ..
              },
              ..
          } => {
              format!("🌟 VIP用户：{} | 🌟 VIP user: {} - 邮箱：{}, 电话：{}, 位置：{}, {}, 邮编：{}, 主题：{} | Email: {}, Phone: {}, Location: {}, {}, Postal: {}, Theme: {}", 
                     username, username, email, phone, city, country, postal, theme)
          }
          
          // 有联系方式但缺少地址 | Has contact but missing address
          UserProfile {
              username,
              email: Some(email),
              phone: phone_opt,
              address: None,
              ..
          } => {
              let phone_info = match phone_opt {
                  Some(phone) => format!("电话：{} | Phone: {}", phone),
                  None => "无电话 | No phone".to_string(),
              };
              format!("📧 联系用户：{} - 邮箱：{}, {} | 📧 Contact user: {} - Email: {}, {}", 
                     username, email, phone_info)
          }
          
          // 最小信息用户 | Users with minimal information
          UserProfile {
              username,
              email: None,
              phone: None,
              address: None,
              preferences: UserPreferences {
                  newsletter: false,
                  notifications: NotificationLevel::None,
                  ..
              },
              ..
          } => {
              format!("👤 基础用户：{} - 无联系方式，不接受通知 | 👤 Basic user: {} - No contact info, no notifications", username)
          }
          
          // 其他用户 | Other users
          UserProfile { username, .. } => {
              format!("👥 普通用户：{} | 👥 Regular user: {}", username)
          }
      }
  }
  ```

### 6. 模式匹配最佳实践 | Pattern Matching Best Practices (15分钟 | 15 minutes)

- **性能与可读性优化 | Performance and Readability Optimization**
  
  **概念定义 | Concept Definition:**
  模式匹配最佳实践涉及如何编写高效、可读和可维护的模式匹配代码。这包括合理的分支顺序、适当的复杂度控制、以及与Rust类型系统的良好配合。 | Pattern matching best practices involve writing efficient, readable, and maintainable pattern matching code. This includes reasonable branch ordering, appropriate complexity control, and good coordination with Rust's type system.
  
  **关键原则 | Key Principles:**
  - 按匹配概率排序分支以提高性能 | Order branches by matching probability to improve performance
  - 避免过度复杂的嵌套模式 | Avoid overly complex nested patterns
  - 使用守卫条件分离逻辑判断 | Use guard conditions to separate logical judgments
  - 充分利用编译器的穷尽性检查 | Fully utilize compiler's exhaustiveness checking
  
  **实践验证问题 | Practice Verification Questions:**
  1. 应该如何安排match分支的顺序？| How should match branches be ordered?
     **答案 | Answer:** 按照匹配概率从高到低排列 | Arrange from high to low matching probability
  2. 何时应该拆分复杂的match表达式？| When should complex match expressions be split?
     **答案 | Answer:** 当单个分支超过10行或整个match超过50行时考虑拆分 | Consider splitting when a single branch exceeds 10 lines or the entire match exceeds 50 lines
  3. 守卫条件与嵌套if的权衡是什么？| What's the tradeoff between guard conditions and nested ifs?
     **答案 | Answer:** 守卫条件更简洁但可能影响穷尽性检查 | Guard conditions are more concise but may affect exhaustiveness checking

## 实践项目：配置文件解析器 | Practical Project: Configuration File Parser

### 目标 | Objective
创建一个高级配置文件解析器，综合应用复杂模式匹配、守卫条件、@绑定和嵌套解构技术。该项目将处理多种配置格式，进行数据验证，并提供详细的错误报告。 | Create an advanced configuration file parser that comprehensively applies complex pattern matching, guard conditions, @ bindings, and nested destructuring techniques. This project will handle multiple configuration formats, perform data validation, and provide detailed error reporting.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 复杂模式匹配能否同时检查数据结构和值的条件？| Can complex pattern matching simultaneously check data structure and value conditions?
   **答案 | Answer:** 能，这是高级模式匹配的核心优势 | Yes, this is the core advantage of advanced pattern matching

2. 守卫条件是否会影响模式匹配的穷尽性？| Do guard conditions affect the exhaustiveness of pattern matching?
   **答案 | Answer:** 不会，编译器只检查模式的穷尽性 | No, the compiler only checks pattern exhaustiveness

3. @绑定可以在守卫条件中使用绑定的值吗？| Can @ bindings use bound values in guard conditions?
   **答案 | Answer:** 可以，这是@绑定的常见用法 | Yes, this is a common use of @ bindings

### 步骤 | Steps
1. 设计配置文件数据结构和错误类型 | Design configuration file data structures and error types
2. 实现基础的配置解析和验证逻辑 | Implement basic configuration parsing and validation logic  
3. 应用高级模式匹配处理复杂配置场景 | Apply advanced pattern matching to handle complex configuration scenarios
4. 集成@绑定和守卫条件优化错误处理 | Integrate @ bindings and guard conditions to optimize error handling
5. 添加配置文件格式转换和输出功能 | Add configuration file format conversion and output functionality

### 示例代码 | Example Code
```rust
"""
高级配置文件解析器 | Advanced Configuration File Parser
演示高级模式匹配技术的综合应用 | Demonstrates comprehensive application of advanced pattern matching techniques

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 复杂模式匹配和嵌套解构 | Complex pattern matching and nested destructuring
- 守卫条件在数据验证中的应用 | Guard conditions in data validation
- @绑定在错误处理中的使用 | @ bindings in error handling
"""

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug, Clone)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
    logging: LoggingConfig,
    features: HashMap<String, FeatureConfig>,
    environments: Vec<Environment>,
}

#[derive(Debug, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: u32,
    ssl: Option<SslConfig>,
    middleware: Vec<String>,
}

#[derive(Debug, Clone)]  
struct SslConfig {
    cert_path: String,
    key_path: String,
    protocols: Vec<String>,
}

#[derive(Debug, Clone)]
struct DatabaseConfig {
    driver: DatabaseDriver,
    connection: ConnectionConfig,
    pool: PoolConfig,
}

#[derive(Debug, Clone)]
enum DatabaseDriver {
    PostgreSQL { version: String },
    MySQL { version: String },
    SQLite { file_path: String },
    Redis { cluster: bool },
}

#[derive(Debug, Clone)]
struct ConnectionConfig {
    host: String,
    port: u16,
    username: String,
    password: Option<String>,
    database: String,
    timeout: u32,
}

#[derive(Debug, Clone)]
struct PoolConfig {
    min_connections: u32,
    max_connections: u32,
    idle_timeout: u32,
}

#[derive(Debug, Clone)]
struct LoggingConfig {
    level: LogLevel,
    outputs: Vec<LogOutput>,
    format: LogFormat,
    rotation: Option<LogRotation>,
}

#[derive(Debug, Clone)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone)]
enum LogOutput {
    Console,
    File { path: String },
    Network { host: String, port: u16 },
}

#[derive(Debug, Clone)]
struct LogFormat {
    timestamp: bool,
    level: bool,
    module: bool,
    custom_fields: Vec<String>,
}

#[derive(Debug, Clone)]
struct LogRotation {
    max_size_mb: u32,
    max_files: u32,
    compress: bool,
}

#[derive(Debug, Clone)]
struct FeatureConfig {
    enabled: bool,
    parameters: HashMap<String, ConfigValue>,
}

#[derive(Debug, Clone)]
enum ConfigValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<ConfigValue>),
    Object(HashMap<String, ConfigValue>),
}

#[derive(Debug, Clone)]
struct Environment {
    name: String,
    overrides: HashMap<String, ConfigValue>,
    active: bool,
}

#[derive(Debug)]
enum ConfigError {
    IoError(io::Error),
    ParseError { line: usize, column: usize, message: String },
    ValidationError { field: String, reason: String },
    SecurityError { issue: String, severity: SecuritySeverity },
    CompatibilityError { feature: String, reason: String },
}

#[derive(Debug)]
enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

// 高级模式匹配验证函数 | Advanced pattern matching validation functions
fn validate_config(config: &Config) -> Result<(), Vec<ConfigError>> {
    let mut errors = Vec::new();
    
    // 服务器配置验证 | Server configuration validation
    match &config.server {
        // 生产环境SSL配置检查 | Production environment SSL configuration check
        server_config @ ServerConfig {
            host,
            port,
            workers,
            ssl: Some(SslConfig {
                cert_path,
                key_path,
                protocols,
                ..
            }),
            middleware,
            ..
        } if host != "localhost" && *port == 443 => {
            println!("🔒 验证生产HTTPS配置 | 🔒 Validating production HTTPS config");
            
            // 验证SSL协议 | Validate SSL protocols
            match protocols.as_slice() {
                protocols if protocols.contains(&"TLSv1.0".to_string()) => {
                    errors.push(ConfigError::SecurityError {
                        issue: "TLSv1.0协议存在安全风险 | TLSv1.0 protocol has security risks".to_string(),
                        severity: SecuritySeverity::High,
                    });
                }
                protocols if !protocols.contains(&"TLSv1.3".to_string()) => {
                    errors.push(ConfigError::SecurityError {
                        issue: "建议启用TLSv1.3协议 | Recommend enabling TLSv1.3 protocol".to_string(),
                        severity: SecuritySeverity::Medium,
                    });
                }
                _ => {}
            }
            
            // 工作进程数量检查 | Worker process count check
            if *workers > 100 {
                errors.push(ConfigError::ValidationError {
                    field: "server.workers".to_string(),
                    reason: format!("工作进程过多：{} | Too many workers: {}", workers),
                });
            }
            
            // 中间件安全检查 | Middleware security check
            for middleware_name in middleware {
                if middleware_name == "debug" && host != "localhost" {
                    errors.push(ConfigError::SecurityError {
                        issue: format!("生产环境启用调试中间件：{} | Debug middleware enabled in production: {}", middleware_name),
                        severity: SecuritySeverity::Critical,
                    });
                }
            }
        }
        
        // 开发环境配置 | Development environment configuration
        ServerConfig {
            host,
            port,
            ssl: None,
            ..
        } if host == "localhost" || host == "127.0.0.1" => {
            if *port < 1024 {
                errors.push(ConfigError::ValidationError {
                    field: "server.port".to_string(),
                    reason: "开发环境建议使用高端口号 | Recommend high port numbers for development".to_string(),
                });
            }
        }
        
        // 不安全的生产配置 | Insecure production configuration
        insecure_config @ ServerConfig {
            host,
            port,
            ssl: None,
            ..
        } if host != "localhost" && host != "127.0.0.1" => {
            errors.push(ConfigError::SecurityError {
                issue: format!("生产服务器 {}:{} 未启用SSL | Production server {}:{} without SSL", host, port),
                severity: SecuritySeverity::Critical,
            });
            println!("⚠️ 不安全配置：{:?} | ⚠️ Insecure config: {:?}", insecure_config);
        }
        
        _ => {}
    }
    
    // 数据库配置验证 | Database configuration validation
    match &config.database {
        // 生产数据库配置 | Production database configuration
        DatabaseConfig {
            driver: db_driver @ DatabaseDriver::PostgreSQL { version },
            connection: ConnectionConfig {
                host: db_host,
                port: db_port,
                username,
                password: Some(password),
                database: db_name,
                timeout,
                ..
            },
            pool: PoolConfig {
                min_connections,
                max_connections,
                idle_timeout,
                ..
            },
            ..
        } if db_host != "localhost" && db_host != "127.0.0.1" => {
            println!("🗄️ 验证生产数据库配置 | 🗄️ Validating production database config");
            
            // 版本兼容性检查 | Version compatibility check
            if let Ok(ver) = version.parse::<f32>() {
                if ver < 12.0 {
                    errors.push(ConfigError::CompatibilityError {
                        feature: format!("PostgreSQL {}", version),
                        reason: "建议使用PostgreSQL 12+版本 | Recommend PostgreSQL 12+ version".to_string(),
                    });
                }
            }
            
            // 连接池配置检查 | Connection pool configuration check
            match (min_connections, max_connections) {
                (min, max) if min > max => {
                    errors.push(ConfigError::ValidationError {
                        field: "database.pool".to_string(),
                        reason: format!("最小连接数不能大于最大连接数：{} > {} | Min connections cannot exceed max: {} > {}", min, max),
                    });
                }
                (min, max) if *max > 1000 => {
                    errors.push(ConfigError::ValidationError {
                        field: "database.pool.max_connections".to_string(),
                        reason: format!("连接池过大：{} | Connection pool too large: {}", max),
                    });
                }
                _ => {}
            }
            
            // 密码安全检查 | Password security check
            if password.len() < 12 {
                errors.push(ConfigError::SecurityError {
                    issue: "数据库密码长度不足 | Database password too short".to_string(),
                    severity: SecuritySeverity::High,
                });
            }
            
            // 超时配置检查 | Timeout configuration check
            if *timeout > 30000 {
                println!("⏱️ 数据库超时较长：{}ms | ⏱️ Long database timeout: {}ms", timeout);
            }
        }
        
        // SQLite配置检查 | SQLite configuration check
        DatabaseConfig {
            driver: DatabaseDriver::SQLite { file_path },
            connection: conn_config,
            ..
        } if !file_path.ends_with(".db") && !file_path.ends_with(".sqlite") => {
            errors.push(ConfigError::ValidationError {
                field: "database.driver.file_path".to_string(),
                reason: format!("SQLite文件扩展名不规范：{} | Non-standard SQLite file extension: {}", file_path),
            });
        }
        
        _ => {}
    }
    
    // 日志配置验证 | Logging configuration validation
    match &config.logging {
        // 生产环境日志配置 | Production environment logging configuration
        LoggingConfig {
            level: LogLevel::Debug | LogLevel::Trace,
            outputs,
            rotation: None,
            ..
        } if outputs.iter().any(|output| matches!(output, LogOutput::File { .. })) => {
            errors.push(ConfigError::SecurityError {
                issue: "生产环境使用调试级别日志且无轮转 | Production using debug-level logging without rotation".to_string(),
                severity: SecuritySeverity::Medium,
            });
        }
        
        // 网络日志安全检查 | Network logging security check
        LoggingConfig {
            outputs,
            ..
        } => {
            for output in outputs {
                if let LogOutput::Network { host: log_host, port: log_port } = output {
                    if log_host.starts_with("http://") {
                        errors.push(ConfigError::SecurityError {
                            issue: format!("不安全的网络日志传输：{}:{} | Insecure network logging: {}:{}", log_host, log_port),
                            severity: SecuritySeverity::High,
                        });
                    }
                }
            }
        }
    }
    
    // 功能配置验证 | Feature configuration validation
    for (feature_name, feature_config) in &config.features {
        match (feature_name.as_str(), feature_config) {
            // 安全功能配置检查 | Security feature configuration check
            ("auth" | "security", FeatureConfig { enabled: true, parameters }) => {
                match parameters.get("encryption") {
                    Some(ConfigValue::String(algo)) if algo == "md5" || algo == "sha1" => {
                        errors.push(ConfigError::SecurityError {
                            issue: format!("功能 {} 使用不安全的加密算法：{} | Feature {} uses insecure encryption: {}", feature_name, algo),
                            severity: SecuritySeverity::High,
                        });
                    }
                    None => {
                        errors.push(ConfigError::ValidationError {
                            field: format!("features.{}.encryption", feature_name),
                            reason: "安全功能必须指定加密算法 | Security features must specify encryption algorithm".to_string(),
                        });
                    }
                    _ => {}
                }
            }
            
            // 实验性功能检查 | Experimental feature check
            (name, FeatureConfig { enabled: true, .. }) if name.starts_with("experimental_") => {
                println!("🧪 启用实验性功能：{} | 🧪 Experimental feature enabled: {}", name);
            }
            
            _ => {}
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// 环境特定配置处理 | Environment-specific configuration handling
fn apply_environment_overrides(config: &mut Config, env_name: &str) -> Result<(), ConfigError> {
    // 查找指定环境 | Find specified environment
    let environment = config.environments.iter()
        .find(|env| env.name == env_name && env.active);
    
    match environment {
        Some(Environment { name, overrides, active: true }) => {
            println!("🔧 应用环境配置：{} | 🔧 Applying environment config: {}", name);
            
            for (key, value) in overrides {
                match (key.as_str(), value) {
                    // 服务器配置覆盖 | Server configuration overrides
                    ("server.host", ConfigValue::String(host)) => {
                        config.server.host = host.clone();
                    }
                    ("server.port", ConfigValue::Number(port)) => {
                        config.server.port = *port as u16;
                    }
                    
                    // 数据库配置覆盖 | Database configuration overrides
                    ("database.host", ConfigValue::String(db_host)) => {
                        config.database.connection.host = db_host.clone();
                    }
                    ("database.pool.max_connections", ConfigValue::Number(max_conn)) => {
                        config.database.pool.max_connections = *max_conn as u32;
                    }
                    
                    // 日志级别覆盖 | Log level overrides
                    ("logging.level", ConfigValue::String(level)) => {
                        config.logging.level = match level.as_str() {
                            "error" => LogLevel::Error,
                            "warn" => LogLevel::Warn,
                            "info" => LogLevel::Info,
                            "debug" => LogLevel::Debug,
                            "trace" => LogLevel::Trace,
                            _ => return Err(ConfigError::ValidationError {
                                field: "logging.level".to_string(),
                                reason: format!("未知日志级别：{} | Unknown log level: {}", level),
                            }),
                        };
                    }
                    
                    // 功能开关覆盖 | Feature toggle overrides
                    (key, ConfigValue::Boolean(enabled)) if key.starts_with("features.") => {
                        let feature_name = key.strip_prefix("features.").unwrap().to_string();
                        if let Some(feature_config) = config.features.get_mut(&feature_name) {
                            feature_config.enabled = *enabled;
                        }
                    }
                    
                    _ => {
                        println!("⚠️ 未识别的配置覆盖：{} = {:?} | ⚠️ Unrecognized config override: {} = {:?}", key, value);
                    }
                }
            }
            
            Ok(())
        }
        
        None => Err(ConfigError::ValidationError {
            field: "environment".to_string(),
            reason: format!("环境配置未找到或未激活：{} | Environment config not found or inactive: {}", env_name),
        }),
    }
}

// 配置诊断和报告 | Configuration diagnostics and reporting
fn generate_config_report(config: &Config) -> String {
    let mut report = String::new();
    report.push_str("📋 配置分析报告 | 📋 Configuration Analysis Report\n");
    report.push_str("=" .repeat(50).as_str());
    report.push('\n');
    
    // 服务器配置分析 | Server configuration analysis
    match &config.server {
        ServerConfig {
            host,
            port,
            workers,
            ssl: Some(ssl_config),
            middleware,
            ..
        } => {
            report.push_str(&format!("🖥️ 服务器：{} 工作进程，SSL已启用 | 🖥️ Server: {} workers, SSL enabled\n", workers));
            report.push_str(&format!("   地址：{}:{} | Address: {}:{}\n", host, port));
            report.push_str(&format!("   中间件：{:?} | Middleware: {:?}\n", middleware));
            report.push_str(&format!("   SSL协议：{:?} | SSL Protocols: {:?}\n", ssl_config.protocols));
        }
        
        ServerConfig { host, port, workers, ssl: None, .. } => {
            report.push_str(&format!("🖥️ 服务器：{}:{}, {} 工作进程，无SSL | 🖥️ Server: {}:{}, {} workers, no SSL\n", 
                                    host, port, workers));
        }
    }
    
    // 数据库配置分析 | Database configuration analysis
    match &config.database.driver {
        DatabaseDriver::PostgreSQL { version } => {
            report.push_str(&format!("🗄️ 数据库：PostgreSQL {} | 🗄️ Database: PostgreSQL {}\n", version));
        }
        DatabaseDriver::MySQL { version } => {
            report.push_str(&format!("🗄️ 数据库：MySQL {} | 🗄️ Database: MySQL {}\n", version));
        }
        DatabaseDriver::SQLite { file_path } => {
            report.push_str(&format!("🗄️ 数据库：SQLite ({}) | 🗄️ Database: SQLite ({})\n", file_path));
        }
        DatabaseDriver::Redis { cluster } => {
            let cluster_info = if *cluster { "集群模式 | Cluster mode" } else { "单实例 | Single instance" };
            report.push_str(&format!("🗄️ 数据库：Redis ({}) | 🗄️ Database: Redis ({})\n", cluster_info));
        }
    }
    
    report.push_str(&format!("   连接池：{}-{} | Connection pool: {}-{}\n", 
                            config.database.pool.min_connections,
                            config.database.pool.max_connections));
    
    // 功能配置统计 | Feature configuration statistics
    let enabled_features: Vec<_> = config.features.iter()
        .filter(|(_, config)| config.enabled)
        .map(|(name, _)| name)
        .collect();
    
    report.push_str(&format!("🔧 启用功能：{:?} | 🔧 Enabled features: {:?}\n", enabled_features));
    
    // 环境配置信息 | Environment configuration info
    let active_envs: Vec<_> = config.environments.iter()
        .filter(|env| env.active)
        .map(|env| &env.name)
        .collect();
    
    if !active_envs.is_empty() {
        report.push_str(&format!("🌍 活跃环境：{:?} | 🌍 Active environments: {:?}\n", active_envs));
    }
    
    report
}

// 主要的配置处理函数 | Main configuration processing function
fn main() {
    // 创建示例配置用于演示 | Create example configuration for demonstration
    let mut config = create_example_config();
    
    println!("🚀 开始配置验证 | 🚀 Starting configuration validation");
    
    // 验证配置 | Validate configuration
    match validate_config(&config) {
        Ok(()) => println!("✅ 配置验证通过 | ✅ Configuration validation passed"),
        Err(errors) => {
            println!("❌ 发现 {} 个配置问题：| ❌ Found {} configuration issues:", errors.len());
            for error in errors {
                println!("  - {:?}", error);
            }
        }
    }
    
    // 应用环境覆盖 | Apply environment overrides
    if let Err(env_error) = apply_environment_overrides(&mut config, "production") {
        println!("⚠️ 环境配置应用失败：{:?} | ⚠️ Environment config application failed: {:?}", env_error);
    }
    
    // 生成配置报告 | Generate configuration report
    let report = generate_config_report(&config);
    println!("\n{}", report);
}

fn create_example_config() -> Config {
    // 这里会创建一个示例配置用于演示 | This would create an example configuration for demonstration
    // 实际实现会从文件或其他源加载配置 | Actual implementation would load configuration from files or other sources
    
    let mut features = HashMap::new();
    features.insert("auth".to_string(), FeatureConfig {
        enabled: true,
        parameters: {
            let mut params = HashMap::new();
            params.insert("encryption".to_string(), ConfigValue::String("aes256".to_string()));
            params
        },
    });
    
    Config {
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 443,
            workers: 4,
            ssl: Some(SslConfig {
                cert_path: "/etc/ssl/cert.pem".to_string(),
                key_path: "/etc/ssl/key.pem".to_string(),
                protocols: vec!["TLSv1.2".to_string(), "TLSv1.3".to_string()],
            }),
            middleware: vec!["cors".to_string(), "auth".to_string()],
        },
        database: DatabaseConfig {
            driver: DatabaseDriver::PostgreSQL { 
                version: "13.4".to_string() 
            },
            connection: ConnectionConfig {
                host: "db.example.com".to_string(),
                port: 5432,
                username: "app_user".to_string(),
                password: Some("secure_password_123456".to_string()),
                database: "app_db".to_string(),
                timeout: 5000,
            },
            pool: PoolConfig {
                min_connections: 5,
                max_connections: 20,
                idle_timeout: 300,
            },
        },
        logging: LoggingConfig {
            level: LogLevel::Info,
            outputs: vec![
                LogOutput::Console,
                LogOutput::File { 
                    path: "/var/log/app.log".to_string() 
                },
            ],
            format: LogFormat {
                timestamp: true,
                level: true,
                module: true,
                custom_fields: vec![],
            },
            rotation: Some(LogRotation {
                max_size_mb: 100,
                max_files: 10,
                compress: true,
            }),
        },
        features,
        environments: vec![
            Environment {
                name: "production".to_string(),
                overrides: HashMap::new(),
                active: true,
            }
        ],
    }
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确应用了复杂模式匹配来处理嵌套数据结构？| Does the project correctly apply complex pattern matching to handle nested data structures?
2. 守卫条件的使用是否有效地分离了逻辑判断？| Does the use of guard conditions effectively separate logical judgments?
3. @绑定是否正确地在错误处理中提供了完整的上下文信息？| Do @ bindings correctly provide complete context information in error handling?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **复杂模式识别练习 | Complex Pattern Recognition Exercise**
   - **练习描述 | Exercise Description:** 设计一个日志分析器，使用复杂模式匹配识别不同类型的日志条目和异常模式
   - **概念检查 | Concept Check:** 复杂模式能否同时匹配日志的格式、级别和内容特征？
   - **学习目标 | Learning Objective:** 深化对多维度模式匹配的理解

2. **守卫条件优化练习 | Guard Condition Optimization Exercise**
   - **练习描述 | Exercise Description:** 重构现有的嵌套if-else代码，使用守卫条件提高可读性和性能
   - **概念检查 | Concept Check:** 守卫条件如何与模式匹配协作提供更清晰的逻辑表达？
   - **学习目标 | Learning Objective:** 掌握守卫条件的最佳使用场景

3. **@绑定应用练习 | @ Binding Application Exercise**
   - **练习描述 | Exercise Description:** 创建一个表达式求值器，使用@绑定在递归处理中保留完整的表达式信息
   - **概念检查 | Concept Check:** @绑定如何在递归场景中提供上下文保存能力？
   - **学习目标 | Learning Objective:** 理解@绑定在复杂数据处理中的价值

4. **嵌套解构练习 | Nested Destructuring Exercise**
   - **练习描述 | Exercise Description:** 实现一个JSON转换器，通过深层嵌套解构处理复杂的JSON结构转换
   - **概念检查 | Concept Check:** 深层解构是否能有效简化复杂数据结构的访问？
   - **学习目标 | Learning Objective:** 培养处理复杂嵌套结构的能力

5. **错误处理集成练习 | Error Handling Integration Exercise**
   - **练习描述 | Exercise Description:** 设计一个网络请求库，综合使用模式匹配处理各种错误情况和响应类型
   - **概念检查 | Concept Check:** 模式匹配如何优雅地处理多种错误类型和恢复策略？
   - **学习目标 | Learning Objective:** 掌握高级错误处理模式

6. **性能优化练习 | Performance Optimization Exercise**
   - **练习描述 | Exercise Description:** 分析和优化模式匹配的分支顺序，测量性能改进效果
   - **概念检查 | Concept Check:** 模式匹配分支顺序对性能的影响如何量化？
   - **学习目标 | Learning Objective:** 理解模式匹配的性能特性

7. **架构设计练习 | Architecture Design Exercise**
   - **练习描述 | Exercise Description:** 使用高级模式匹配技术设计一个状态机或解释器
   - **概念检查 | Concept Check:** 高级模式匹配如何支持复杂的架构设计模式？
   - **学习目标 | Learning Objective:** 将模式匹配应用于系统架构设计

## 学习资源 | Learning Resources
- [Rust官方文档 - 模式匹配](https://doc.rust-lang.org/book/ch18-00-patterns.html) | [Rust Official Documentation - Pattern Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust Reference - 模式语法](https://doc.rust-lang.org/reference/patterns.html) | [Rust Reference - Pattern Syntax](https://doc.rust-lang.org/reference/patterns.html)
- [高级模式匹配技巧和最佳实践](https://rust-lang.github.io/rfcs/2535-or-patterns.html) | [Advanced pattern matching techniques and best practices](https://rust-lang.github.io/rfcs/2535-or-patterns.html)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解复杂模式匹配的语法和应用场景 | Understand complex pattern matching syntax and use cases
- [ ] 掌握守卫条件的编写和优化技巧 | Master writing and optimizing guard conditions
- [ ] 熟练使用@绑定捕获和访问匹配值 | Proficiently use @ bindings to capture and access matched values
- [ ] 能够解构深层嵌套的数据结构 | Able to destructure deeply nested data structures
- [ ] 在错误处理中有效应用模式匹配 | Effectively apply pattern matching in error handling
- [ ] 理解模式匹配的性能特性和最佳实践 | Understand performance characteristics and best practices of pattern matching
- [ ] 完成配置文件解析器项目 | Complete configuration file parser project
- [ ] 正确回答所有CCQs | Correctly answer all CCQs
- [ ] 运行并理解所有代码示例 | Run and understand all code examples
- [ ] 完成至少5个扩展练习 | Complete at least 5 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释每个高级模式匹配概念的应用场景和实现技巧。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain the application scenarios and implementation techniques of each advanced pattern matching concept to others.