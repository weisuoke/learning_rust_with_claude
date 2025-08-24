# Rust入门 - 第13天：字符串详解 | Rust Introduction - Day 13: String Deep Dive

## 学习目标 | Learning Objectives
- 深入理解String和&str的区别和使用场景 | Deeply understand the differences and use cases between String and &str
- 掌握字符串切片的概念和操作方法 | Master the concept and operation methods of string slicing
- 学会各种字符串方法和常用操作 | Learn various string methods and common operations
- 理解UTF-8编码和Unicode字符处理 | Understand UTF-8 encoding and Unicode character processing
- 掌握字符串的内存管理和性能优化 | Master string memory management and performance optimization
- 学会在实际项目中正确处理文本数据 | Learn to properly handle text data in real projects

## 详细内容 | Detailed Content

### 1. String vs &str 深度解析 | String vs &str Deep Analysis (1小时 | 1 hour)

- **String类型详解 | String Type Deep Dive**
  
  **概念定义 | Concept Definition:**
  String是Rust中的拥有所有权的、可增长的、UTF-8编码的字符串类型。它是一个堆分配的数据结构，包含指向堆内存的指针、长度和容量信息。 | String is an owned, growable, UTF-8 encoded string type in Rust. It's a heap-allocated data structure containing a pointer to heap memory, length, and capacity information.
  
  **核心特征 | Key Characteristics:**
  - 拥有字符串数据的所有权，可以修改 | Owns the string data and can be modified
  - 存储在堆上，支持动态增长 | Stored on heap, supports dynamic growth
  - 自动管理内存分配和释放 | Automatically manages memory allocation and deallocation
  - 实现了许多便利的字符串操作方法 | Implements many convenient string manipulation methods
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. String类型的数据存储在哪里？| Where is String type data stored?
     **答案 | Answer:** 堆上 | On the heap - String管理堆内存的动态分配 | String manages dynamic heap memory allocation
  2. String类型可以修改吗？| Can String type be modified?
     **答案 | Answer:** 是 | Yes - String是可变的，支持增长和修改 | String is mutable and supports growth and modification
  3. String会自动释放内存吗？| Does String automatically free memory?
     **答案 | Answer:** 是 | Yes - 当String超出作用域时自动释放 | Automatically freed when String goes out of scope
  4. 创建空String的方法是什么？| What's the method to create an empty String?
     **答案 | Answer:** String::new() - 或者String::from("") | Or String::from("")
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      // String创建方式 | String creation methods
      let mut s1 = String::new(); // 创建空字符串 | Create empty string
      let s2 = String::from("Hello"); // 从字面量创建 | Create from literal
      let s3 = "World".to_string(); // 从&str转换 | Convert from &str
      let s4 = format!("Hello, {}!", "Rust"); // 使用格式化宏 | Using format macro
      
      // String是可变的 | String is mutable
      s1.push_str("Rust编程 | Rust Programming"); // 追加字符串 | Append string
      s1.push('!'); // 追加单个字符 | Append single character
      
      println!("s1: {}", s1);
      println!("s2: {}", s2);
      println!("s3: {}", s3);
      println!("s4: {}", s4);
      
      // 查看String的容量和长度 | Check String capacity and length
      println!("s1长度: {}, 容量: {} | s1 length: {}, capacity: {}", 
               s1.len(), s1.capacity(), s1.len(), s1.capacity());
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码中哪些String操作会分配新内存？| Which String operations in this code allocate new memory?
    **答案 | Answer:** push_str如果超过容量会重新分配 | push_str may reallocate if exceeding capacity
  - s1的容量为什么可能大于长度？| Why might s1's capacity be greater than its length?
    **答案 | Answer:** String预分配额外空间以提高性能 | String pre-allocates extra space for performance
  
  **常见误区检查 | Common Misconception Checks:**
  - String和C++的string类似吗？| Is String similar to C++ string?
    **答案 | Answer:** 相似但有所有权语义，内存管理更安全 | Similar but with ownership semantics and safer memory management
  - String可以包含任意字节吗？| Can String contain arbitrary bytes?
    **答案 | Answer:** 否，String必须是有效的UTF-8序列 | No, String must be valid UTF-8 sequences

- **&str字符串切片详解 | &str String Slice Deep Dive**
  
  **概念定义 | Concept Definition:**
  &str是字符串切片，是对字符串数据的不可变引用。它不拥有数据，只是指向已存在的字符串数据的一个视图，可以指向String、字符串字面量或其他&str。 | &str is a string slice, an immutable reference to string data. It doesn't own the data but is merely a view into existing string data, which can point to String, string literals, or other &str.
  
  **核心特征 | Key Characteristics:**
  - 不拥有字符串数据，是借用的引用 | Doesn't own string data, is a borrowed reference
  - 不可变，不能直接修改内容 | Immutable, cannot directly modify content
  - 轻量级，只包含指针和长度信息 | Lightweight, contains only pointer and length information
  - 可以指向任何UTF-8字符串数据 | Can point to any UTF-8 string data
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. &str拥有字符串数据吗？| Does &str own string data?
     **答案 | Answer:** 否 | No - &str是借用的引用，不拥有数据 | &str is a borrowed reference, doesn't own data
  2. &str可以修改吗？| Can &str be modified?
     **答案 | Answer:** 否 | No - &str是不可变的字符串切片 | &str is an immutable string slice
  3. &str存储在哪里？| Where is &str stored?
     **答案 | Answer:** 取决于指向的数据 | Depends on the pointed data - 可能在栈、堆或程序的静态区域 | Could be on stack, heap, or program's static area
  4. 字符串字面量的类型是什么？| What's the type of string literals?
     **答案 | Answer:** &str - 更确切地说是&'static str | More precisely &'static str
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      // 不同来源的&str | Different sources of &str
      let literal: &str = "字符串字面量 | String literal"; // 静态存储区 | Static storage area
      
      let owned = String::from("拥有的字符串 | Owned string");
      let slice: &str = &owned; // String的切片 | Slice of String
      let partial: &str = &owned[0..6]; // 部分切片 | Partial slice
      
      // &str的特性演示 | Demonstration of &str characteristics
      println!("字面量: {} | Literal: {}", literal, literal);
      println!("完整切片: {} | Full slice: {}", slice, slice);
      println!("部分切片: {} | Partial slice: {}", partial, partial);
      
      // &str是Fat Pointer（包含指针和长度） | &str is Fat Pointer (contains pointer and length)
      println!("literal长度: {} | literal length: {}", literal.len(), literal.len());
      println!("slice长度: {} | slice length: {}", slice.len(), slice.len());
      
      // 函数参数通常使用&str更灵活 | Function parameters usually use &str for flexibility
      print_string_info(literal);
      print_string_info(&owned);
      print_string_info("直接传递字面量 | Direct literal");
  }
  
  // 接受&str参数的函数更通用 | Function accepting &str parameter is more general
  fn print_string_info(s: &str) {
      println!("字符串内容: '{}', 长度: {} | String content: '{}', length: {}", 
               s, s.len(), s, s.len());
  }
  ```

### 2. 字符串切片操作 | String Slicing Operations (1小时 | 1 hour)

- **切片语法和索引 | Slicing Syntax and Indexing**
  
  **概念定义 | Concept Definition:**
  字符串切片是通过指定字节索引范围来获取字符串的一部分。需要注意的是，Rust字符串切片基于字节索引，而不是字符索引，这在处理多字节Unicode字符时特别重要。 | String slicing is obtaining part of a string by specifying byte index ranges. It's important to note that Rust string slicing is based on byte indices, not character indices, which is particularly important when dealing with multi-byte Unicode characters.
  
  **核心特征 | Key Characteristics:**
  - 使用[start..end]语法进行切片 | Use [start..end] syntax for slicing
  - 索引基于字节而不是字符 | Indices are based on bytes, not characters
  - 切片必须在字符边界上进行 | Slicing must occur on character boundaries
  - 返回&str类型的引用 | Returns &str type reference
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 字符串切片的索引基于什么？| What are string slice indices based on?
     **答案 | Answer:** 字节 | Bytes - 不是字符数量 | Not character count
  2. 可以在任意字节位置进行切片吗？| Can you slice at any byte position?
     **答案 | Answer:** 否 | No - 必须在字符边界上 | Must be on character boundaries
  3. [0..5]表示什么范围？| What range does [0..5] represent?
     **答案 | Answer:** 从索引0到4的字节 | Bytes from index 0 to 4 - 不包含索引5 | Excluding index 5
  4. 切片越界会发生什么？| What happens when slicing out of bounds?
     **答案 | Answer:** 运行时panic | Runtime panic - 如果不在字符边界上 | If not on character boundary
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      let s = "Hello, 世界! 🦀"; // 包含ASCII、中文和Emoji | Contains ASCII, Chinese, and Emoji
      
      // 基本切片操作 | Basic slicing operations
      let hello = &s[0..5]; // "Hello" - ASCII字符每个1字节 | ASCII chars are 1 byte each
      println!("前5字节: {} | First 5 bytes: {}", hello, hello);
      
      // 注意UTF-8字符的字节边界 | Pay attention to UTF-8 character byte boundaries
      // "世"字占3字节，"界"字也占3字节 | "世" takes 3 bytes, "界" also takes 3 bytes
      let world = &s[7..13]; // "世界" - 从逗号后的空格之后开始 | Starting after comma and space
      println!("世界: {} | World: {}", world, world);
      
      // 使用范围语法的不同形式 | Different forms of range syntax
      let from_start = &s[..5]; // 从开头到索引5 | From start to index 5
      let to_end = &s[7..]; // 从索引7到结尾 | From index 7 to end
      let full = &s[..]; // 完整字符串 | Full string
      
      println!("从开头: {} | From start: {}", from_start, from_start);
      println!("到结尾: {} | To end: {}", to_end, to_end);
      println!("完整: {} | Full: {}", full, full);
      
      // 演示字节长度与字符数的区别 | Demonstrate difference between byte length and character count
      println!("字符串字节长度: {} | String byte length: {}", s.len(), s.len());
      println!("字符串字符数量: {} | String character count: {}", s.chars().count(), s.chars().count());
      
      // 安全的切片方法 | Safe slicing method
      if let Some(safe_slice) = s.get(0..5) {
          println!("安全切片: {} | Safe slice: {}", safe_slice, safe_slice);
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么不能使用&s[0..6]来获取"Hello,"？| Why can't we use &s[0..6] to get "Hello,"?
    **答案 | Answer:** 可以，因为逗号是ASCII字符，占1字节 | We can, because comma is ASCII character taking 1 byte
  - 如果尝试&s[7..8]会发生什么？| What happens if we try &s[7..8]?
    **答案 | Answer:** Panic，因为索引8在"世"字符中间 | Panic, because index 8 is in the middle of "世" character
  
  **常见误区检查 | Common Misconception Checks:**
  - 字符串长度等于字符数量吗？| Is string length equal to character count?
    **答案 | Answer:** 否，len()返回字节数，不是字符数 | No, len() returns byte count, not character count
  - 可以通过索引访问单个字符吗？| Can you access individual characters by index?
    **答案 | Answer:** 不能直接索引，需要使用chars()迭代器 | Cannot index directly, need to use chars() iterator

- **安全的字符串操作方法 | Safe String Manipulation Methods**
  
  **概念定义 | Concept Definition:**
  Rust提供了多种安全的字符串操作方法，避免了直接索引可能导致的panic。这些方法返回Option类型或使用迭代器，确保在处理UTF-8字符时的安全性。 | Rust provides various safe string manipulation methods that avoid potential panics from direct indexing. These methods return Option types or use iterators, ensuring safety when handling UTF-8 characters.
  
  **解决的问题 | Problems It Solves:**
  - 避免切片越界导致的panic | Avoid panic from slice out of bounds
  - 处理多字节Unicode字符的复杂性 | Handle complexity of multi-byte Unicode characters
  - 提供类型安全的字符串操作 | Provide type-safe string operations
  - 支持优雅的错误处理 | Support graceful error handling
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. get()方法与直接索引有什么区别？| What's the difference between get() method and direct indexing?
     **答案 | Answer:** get()返回Option，安全处理边界 | get() returns Option, safely handles boundaries
  2. chars()方法返回什么？| What does chars() method return?
     **答案 | Answer:** 字符迭代器 | Character iterator - 遍历Unicode标量值 | Iterates over Unicode scalar values
  3. 如何安全地获取字符串的第一个字符？| How to safely get the first character of a string?
     **答案 | Answer:** s.chars().next() - 返回Option<char> | Returns Option<char>
  4. split()方法的返回类型是什么？| What's the return type of split() method?
     **答案 | Answer:** 迭代器 | Iterator - Split<'_, &str> 类型 | Split<'_, &str> type
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      let text = "Rust编程语言🚀很棒！| Rust programming language🚀 is great!";
      
      // 安全的切片方法 | Safe slicing methods
      match text.get(0..4) {
          Some(slice) => println!("安全切片: {} | Safe slice: {}", slice, slice),
          None => println!("切片越界 | Slice out of bounds"),
      }
      
      // 字符遍历 | Character iteration
      println!("逐个字符 | Character by character:");
      for (index, ch) in text.chars().enumerate() {
          if index < 10 { // 只显示前10个字符 | Only show first 10 characters
              println!("  索引{}: '{}' | Index {}: '{}'", index, ch, index, ch);
          }
      }
      
      // 字节遍历 | Byte iteration
      println!("字节表示 | Byte representation (前20字节 | First 20 bytes):");
      for (index, byte) in text.bytes().enumerate() {
          if index < 20 {
              println!("  字节{}: {} (0x{:02x}) | Byte {}: {} (0x{:02x})", 
                       index, byte, byte, index, byte, byte);
          }
      }
      
      // 字符串分割 | String splitting
      let sentence = "Hello,世界,Rust,编程 | Hello,world,Rust,programming";
      let words: Vec<&str> = sentence.split(',').collect();
      println!("分割结果 | Split result: {:?}", words);
      
      // 字符串搜索 | String searching
      if let Some(pos) = sentence.find("Rust") {
          println!("找到Rust在位置 | Found Rust at position: {}", pos);
          if let Some(rust_part) = sentence.get(pos..) {
              println!("从Rust开始 | Starting from Rust: {}", rust_part);
          }
      }
      
      // 字符统计 | Character statistics
      let char_count = text.chars().count();
      let byte_count = text.len();
      let word_count = text.split_whitespace().count();
      
      println!("统计信息 | Statistics:");
      println!("  字符数: {} | Character count: {}", char_count, char_count);
      println!("  字节数: {} | Byte count: {}", byte_count, byte_count);
      println!("  单词数: {} | Word count: {}", word_count, word_count);
  }
  ```

### 3. 常用字符串方法 | Common String Methods (1小时 | 1 hour)

- **字符串创建和转换方法 | String Creation and Conversion Methods**
  
  **概念定义 | Concept Definition:**
  Rust提供了多种创建和转换字符串的方法，包括从各种数据类型转换为字符串、格式化字符串以及在String和&str之间转换的便捷方法。 | Rust provides various methods for creating and converting strings, including converting from various data types to strings, formatting strings, and convenient methods for converting between String and &str.
  
  **核心特征 | Key Characteristics:**
  - 多种String创建方式满足不同需求 | Multiple String creation methods for different needs
  - 类型安全的转换机制 | Type-safe conversion mechanisms
  - 高效的内存使用和分配策略 | Efficient memory usage and allocation strategies
  - 支持格式化和模板字符串 | Support for formatting and template strings
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. to_string()和String::from()有区别吗？| Is there a difference between to_string() and String::from()?
     **答案 | Answer:** 功能相同但实现可能不同 | Same functionality but potentially different implementations
  2. format!宏在运行时还是编译时执行？| Does format! macro execute at runtime or compile time?
     **答案 | Answer:** 运行时 | Runtime - 动态创建格式化字符串 | Dynamically creates formatted strings
  3. String::with_capacity的作用是什么？| What's the purpose of String::with_capacity?
     **答案 | Answer:** 预分配容量，避免重新分配 | Pre-allocate capacity to avoid reallocation
  4. parse()方法可能失败吗？| Can parse() method fail?
     **答案 | Answer:** 是 | Yes - 返回Result类型处理解析错误 | Returns Result type to handle parsing errors
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      // 不同的String创建方法 | Different String creation methods
      let s1 = String::new(); // 空字符串 | Empty string
      let s2 = String::from("Hello"); // 从&str创建 | Create from &str
      let s3 = "World".to_string(); // &str转String | &str to String
      let s4 = String::with_capacity(50); // 预分配容量 | Pre-allocate capacity
      
      println!("s4容量: {} | s4 capacity: {}", s4.capacity(), s4.capacity());
      
      // 格式化字符串创建 | Formatted string creation
      let name = "Rust";
      let version = 1.70;
      let formatted = format!("语言: {}, 版本: {} | Language: {}, Version: {}", 
                              name, version, name, version);
      println!("{}", formatted);
      
      // 数字到字符串的转换 | Number to string conversion
      let number = 42;
      let num_str1 = number.to_string(); // 使用to_string | Using to_string
      let num_str2 = format!("{}", number); // 使用format | Using format
      
      println!("数字转字符串: '{}', '{}' | Number to string: '{}', '{}'", 
               num_str1, num_str2, num_str1, num_str2);
      
      // 字符串到数字的转换 | String to number conversion
      let str_num = "123";
      match str_num.parse::<i32>() {
          Ok(num) => println!("解析成功: {} | Parsing successful: {}", num, num),
          Err(e) => println!("解析失败: {} | Parsing failed: {}", e, e),
      }
      
      // 从Vec<char>创建字符串 | Create string from Vec<char>
      let chars: Vec<char> = vec!['R', 'u', 's', 't', '🦀'];
      let from_chars: String = chars.iter().collect();
      println!("从字符数组: {} | From char array: {}", from_chars, from_chars);
      
      // 重复字符串 | Repeat string
      let repeated = "Rust ".repeat(3);
      println!("重复字符串: {} | Repeated string: {}", repeated, repeated);
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - with_capacity(50)创建的String初始长度是多少？| What's the initial length of String created with with_capacity(50)?
    **答案 | Answer:** 0 - 只是预分配了容量，长度仍为0 | Only pre-allocated capacity, length remains 0
  - parse()方法需要指定目标类型吗？| Does parse() method need to specify target type?
    **答案 | Answer:** 通常需要，通过泛型或类型推断 | Usually yes, through generics or type inference

- **字符串修改和操作方法 | String Modification and Operation Methods**
  
  **概念定义 | Concept Definition:**
  String类型提供了丰富的修改和操作方法，包括追加、插入、删除、替换等操作。这些方法直接修改String的内容，遵循Rust的可变性规则。 | The String type provides rich modification and operation methods, including append, insert, delete, replace, and other operations. These methods directly modify String content, following Rust's mutability rules.
  
  **核心特征 | Key Characteristics:**
  - 原地修改，避免不必要的内存分配 | In-place modification, avoiding unnecessary memory allocation
  - 方法链调用支持流畅的API设计 | Method chaining supports fluent API design
  - 自动处理UTF-8编码的复杂性 | Automatically handles UTF-8 encoding complexity
  - 提供高效的字符串操作算法 | Provides efficient string manipulation algorithms
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. push_str和push方法有什么区别？| What's the difference between push_str and push methods?
     **答案 | Answer:** push_str追加字符串，push追加单个字符 | push_str appends strings, push appends single character
  2. insert方法基于字节索引还是字符索引？| Does insert method use byte index or character index?
     **答案 | Answer:** 字节索引 | Byte index - 必须在字符边界上 | Must be on character boundary
  3. clear方法会释放内存吗？| Does clear method release memory?
     **答案 | Answer:** 否 | No - 只清空内容，保留容量 | Only clears content, retains capacity
  4. replace方法是否修改原字符串？| Does replace method modify the original string?
     **答案 | Answer:** 否 | No - 返回新字符串，原字符串不变 | Returns new string, original unchanged
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      let mut text = String::from("Hello");
      
      // 字符串追加操作 | String append operations
      text.push_str(", World"); // 追加字符串 | Append string
      text.push('!'); // 追加字符 | Append character
      println!("追加后: {} | After append: {}", text, text);
      
      // 字符串插入操作 | String insert operations
      text.insert(5, ','); // 在索引5处插入逗号 | Insert comma at index 5
      text.insert_str(6, " 美丽的 |"); // 插入字符串 | Insert string
      println!("插入后: {} | After insert: {}", text, text);
      
      // 字符串删除操作 | String remove operations
      text.remove(5); // 删除索引5的字符 | Remove character at index 5
      println!("删除后: {} | After remove: {}", text, text);
      
      // 字符串截断 | String truncation
      let original_len = text.len();
      text.truncate(10); // 截断到10字节 | Truncate to 10 bytes
      println!("截断后: {} (从{}字节到{}字节) | After truncate: {} (from {} bytes to {} bytes)", 
               text, original_len, text.len(), text, original_len, text.len());
      
      // 字符串替换 | String replacement
      let text2 = String::from("Rust编程很有趣 | Rust programming is fun");
      let replaced = text2.replace("有趣", "强大"); // 替换子字符串 | Replace substring
      println!("原字符串: {} | Original: {}", text2, text2);
      println!("替换后: {} | After replace: {}", replaced, replaced);
      
      // 字符串大小写转换 | String case conversion
      let mixed = String::from("Hello World 世界");
      println!("转大写: {} | To uppercase: {}", mixed.to_uppercase(), mixed.to_uppercase());
      println!("转小写: {} | To lowercase: {}", mixed.to_lowercase(), mixed.to_lowercase());
      
      // 字符串修剪 | String trimming
      let whitespace = String::from("  \t\n  Hello, Rust!  \n\t  ");
      println!("原字符串: '{}'", whitespace);
      println!("修剪空白: '{}' | Trimmed: '{}'", whitespace.trim(), whitespace.trim());
      println!("修剪开头: '{}' | Trim start: '{}'", whitespace.trim_start(), whitespace.trim_start());
      println!("修剪结尾: '{}' | Trim end: '{}'", whitespace.trim_end(), whitespace.trim_end());
      
      // 容量管理 | Capacity management
      let mut capacity_demo = String::with_capacity(10);
      println!("初始容量: {} | Initial capacity: {}", capacity_demo.capacity(), capacity_demo.capacity());
      
      capacity_demo.push_str("This is a long string");
      println!("扩容后容量: {} | Capacity after expansion: {}", capacity_demo.capacity(), capacity_demo.capacity());
      
      capacity_demo.shrink_to_fit(); // 缩减到合适大小 | Shrink to fit
      println!("缩减后容量: {} | Capacity after shrinking: {}", capacity_demo.capacity(), capacity_demo.capacity());
  }
  ```

### 4. UTF-8和Unicode处理 | UTF-8 and Unicode Handling (1小时 | 1 hour)

- **Unicode和UTF-8编码原理 | Unicode and UTF-8 Encoding Principles**
  
  **概念定义 | Concept Definition:**
  Unicode是一个国际标准，为世界上所有的字符系统提供了统一的编码方案。UTF-8是Unicode的一种变长编码方式，使用1到4个字节来表示不同的字符，兼容ASCII编码。 | Unicode is an international standard that provides a unified encoding scheme for all character systems in the world. UTF-8 is a variable-length encoding method for Unicode, using 1 to 4 bytes to represent different characters, compatible with ASCII encoding.
  
  **核心特征 | Key Characteristics:**
  - 变长编码，ASCII字符占1字节，其他字符占2-4字节 | Variable-length encoding, ASCII chars take 1 byte, others take 2-4 bytes
  - 自同步性，可以从任意字节确定字符边界 | Self-synchronizing, can determine character boundaries from any byte
  - 向后兼容ASCII编码 | Backward compatible with ASCII encoding
  - 支持世界上所有现存的文字系统 | Supports all existing writing systems in the world
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. ASCII字符在UTF-8中占几个字节？| How many bytes do ASCII characters take in UTF-8?
     **答案 | Answer:** 1字节 | 1 byte - 与原ASCII完全兼容 | Fully compatible with original ASCII
  2. 中文字符通常占几个字节？| How many bytes do Chinese characters typically take?
     **答案 | Answer:** 3字节 | 3 bytes - 大多数常用汉字 | Most commonly used Chinese characters
  3. Emoji字符占几个字节？| How many bytes do Emoji characters take?
     **答案 | Answer:** 通常4字节 | Usually 4 bytes - 有些复合emoji可能更多 | Some compound emojis may take more
  4. 如何判断一个字节序列是否为有效UTF-8？| How to determine if a byte sequence is valid UTF-8?
     **答案 | Answer:** 检查字节模式和序列规则 | Check byte patterns and sequence rules
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::str;
  
  fn main() {
      // 不同字符的UTF-8表示 | UTF-8 representation of different characters
      let ascii = "A"; // ASCII字符 | ASCII character
      let chinese = "中"; // 中文字符 | Chinese character
      let emoji = "🦀"; // Emoji字符 | Emoji character
      let mixed = "A中🦀"; // 混合字符串 | Mixed string
      
      println!("字符分析 | Character Analysis:");
      analyze_utf8(ascii, "ASCII");
      analyze_utf8(chinese, "Chinese");
      analyze_utf8(emoji, "Emoji");
      analyze_utf8(mixed, "Mixed");
      
      // 字符边界检测 | Character boundary detection
      let text = "Hello世界🌍";
      println!("\n字符边界分析 | Character Boundary Analysis:");
      for (i, byte) in text.bytes().enumerate() {
          let is_char_boundary = text.is_char_boundary(i);
          println!("字节{}: 0x{:02x} {} | Byte {}: 0x{:02x} {}", 
                   i, byte, 
                   if is_char_boundary { "(字符边界 | char boundary)" } else { "" },
                   i, byte,
                   if is_char_boundary { "(char boundary)" } else { "" });
      }
      
      // 安全的UTF-8处理 | Safe UTF-8 handling
      let bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"的字节 | Bytes for "Hello"
      match str::from_utf8(&bytes) {
          Ok(s) => println!("有效UTF-8: {} | Valid UTF-8: {}", s, s),
          Err(e) => println!("无效UTF-8: {} | Invalid UTF-8: {}", e, e),
      }
      
      // 处理无效UTF-8字节序列 | Handle invalid UTF-8 byte sequences
      let invalid_bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0xff]; // 包含无效字节 | Contains invalid byte
      match str::from_utf8(&invalid_bytes) {
          Ok(s) => println!("有效UTF-8: {} | Valid UTF-8: {}", s, s),
          Err(e) => {
              println!("检测到无效UTF-8 | Detected invalid UTF-8: {}", e);
              // 使用lossy转换 | Use lossy conversion
              let lossy = String::from_utf8_lossy(&invalid_bytes);
              println!("宽松转换结果: {} | Lossy conversion result: {}", lossy, lossy);
          }
      }
      
      // 字符统计与遍历 | Character counting and iteration
      let multilingual = "Hello नमस्ते مرحبا 你好 🌍";
      println!("\n多语言字符串分析 | Multilingual String Analysis:");
      println!("字符串: {} | String: {}", multilingual, multilingual);
      println!("字节长度: {} | Byte length: {}", multilingual.len(), multilingual.len());
      println!("字符数量: {} | Character count: {}", multilingual.chars().count(), multilingual.chars().count());
      
      println!("字符详情 | Character details:");
      for (i, ch) in multilingual.chars().enumerate() {
          let char_len = ch.len_utf8();
          println!("  字符{}: '{}' (Unicode: U+{:04X}, UTF-8字节数: {}) | Char {}: '{}' (Unicode: U+{:04X}, UTF-8 bytes: {})", 
                   i, ch, ch as u32, char_len, i, ch, ch as u32, char_len);
      }
  }
  
  // 分析字符串的UTF-8编码 | Analyze UTF-8 encoding of string
  fn analyze_utf8(s: &str, label: &str) {
      println!("{}字符 '{}': | {} character '{}':", label, s, label, s);
      println!("  字节数: {} | Byte count: {}", s.len(), s.len());
      println!("  字符数: {} | Character count: {}", s.chars().count(), s.chars().count());
      print!("  字节表示: | Byte representation: ");
      for byte in s.bytes() {
          print!("0x{:02x} ", byte);
      }
      println!();
      
      if let Some(ch) = s.chars().next() {
          println!("  Unicode码点: U+{:04X} | Unicode code point: U+{:04X}", ch as u32, ch as u32);
      }
      println!();
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么"Hello世界🌍".len()返回的不是字符数？| Why doesn't "Hello世界🌍".len() return character count?
    **答案 | Answer:** len()返回字节数，UTF-8是变长编码 | len() returns byte count, UTF-8 is variable-length encoding
  - is_char_boundary()方法的作用是什么？| What's the purpose of is_char_boundary() method?
    **答案 | Answer:** 检查指定字节索引是否在字符边界上 | Check if specified byte index is on character boundary

- **字符遍历和处理技巧 | Character Iteration and Processing Techniques**
  
  **概念定义 | Concept Definition:**
  正确处理Unicode字符需要使用专门的迭代器和方法。Rust提供了chars()、bytes()、char_indices()等迭代器，以及各种字符处理函数，确保在处理多语言文本时的正确性和效率。 | Properly handling Unicode characters requires using specialized iterators and methods. Rust provides iterators like chars(), bytes(), char_indices(), and various character processing functions to ensure correctness and efficiency when handling multilingual text.
  
  **解决的问题 | Problems It Solves:**
  - 正确遍历Unicode字符而不破坏字符边界 | Correctly iterate Unicode characters without breaking character boundaries
  - 高效处理大型文本数据 | Efficiently process large text data
  - 支持复杂的文本分析和处理需求 | Support complex text analysis and processing requirements
  - 处理不同语言文字的特殊需求 | Handle special requirements of different languages
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. chars()和bytes()迭代器有什么区别？| What's the difference between chars() and bytes() iterators?
     **答案 | Answer:** chars()遍历Unicode字符，bytes()遍历原始字节 | chars() iterates Unicode characters, bytes() iterates raw bytes
  2. char_indices()返回什么类型的数据？| What type of data does char_indices() return?
     **答案 | Answer:** (字节索引, 字符)的元组 | Tuple of (byte index, character)
  3. 如何安全地反转包含Unicode的字符串？| How to safely reverse a string containing Unicode?
     **答案 | Answer:** 收集字符后反转，不能直接反转字节 | Collect characters then reverse, cannot reverse bytes directly
  4. 字符比较是否区分大小写？| Is character comparison case-sensitive?
     **答案 | Answer:** 默认是 | By default yes - 需要显式转换大小写 | Need explicit case conversion
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      let text = "Rust🦀编程💻很有趣！| Programming is fun!";
      
      // 不同的迭代方式 | Different iteration methods
      println!("原文: {} | Original: {}", text, text);
      println!();
      
      // 字符迭代 | Character iteration
      println!("字符迭代 | Character iteration:");
      for (i, ch) in text.chars().enumerate() {
          if i < 8 {
              println!("  字符{}: '{}' | Char {}: '{}'", i, ch, i, ch);
          }
      }
      
      // 字节迭代 | Byte iteration
      println!("\n字节迭代(前20字节) | Byte iteration (first 20 bytes):");
      for (i, byte) in text.bytes().enumerate() {
          if i < 20 {
              println!("  字节{}: 0x{:02x} ({}) | Byte {}: 0x{:02x} ({})", 
                       i, byte, byte as char, i, byte, byte as char);
          }
      }
      
      // 字符索引迭代 | Character index iteration
      println!("\n字符索引迭代 | Character index iteration:");
      for (byte_pos, ch) in text.char_indices().take(8) {
          println!("  字符'{}' 在字节位置{} | Char '{}' at byte position {}", 
                   ch, byte_pos, ch, byte_pos);
      }
      
      // 字符串处理技巧 | String processing techniques
      println!("\n字符串处理技巧 | String processing techniques:");
      
      // 安全的字符串反转 | Safe string reversal
      let reversed: String = text.chars().rev().collect();
      println!("反转: {} | Reversed: {}", reversed, reversed);
      
      // 字符过滤 | Character filtering
      let ascii_only: String = text.chars()
          .filter(|c| c.is_ascii())
          .collect();
      println!("仅ASCII: {} | ASCII only: {}", ascii_only, ascii_only);
      
      // 字符转换 | Character transformation
      let uppercase: String = text.chars()
          .map(|c| c.to_uppercase().to_string())
          .collect::<Vec<String>>()
          .join("");
      println!("大写: {} | Uppercase: {}", uppercase, uppercase);
      
      // 字符统计 | Character statistics
      let char_stats = analyze_characters(&text);
      println!("\n字符统计 | Character statistics:");
      println!("  字母数: {} | Letter count: {}", char_stats.letters, char_stats.letters);
      println!("  数字数: {} | Digit count: {}", char_stats.digits, char_stats.digits);
      println!("  空白数: {} | Whitespace count: {}", char_stats.whitespace, char_stats.whitespace);
      println!("  其他字符: {} | Other chars: {}", char_stats.others, char_stats.others);
      
      // 单词边界检测 | Word boundary detection
      println!("\n单词分割 | Word segmentation:");
      let words: Vec<&str> = text.split_whitespace().collect();
      for (i, word) in words.iter().enumerate() {
          println!("  单词{}: {} | Word {}: {}", i + 1, word, i + 1, word);
      }
      
      // 自定义字符处理 | Custom character processing
      let processed = process_mixed_script(&text);
      println!("\n处理后的文本 | Processed text: {}", processed);
  }
  
  // 字符统计结构 | Character statistics structure
  #[derive(Debug, Default)]
  struct CharStats {
      letters: usize,
      digits: usize,
      whitespace: usize,
      others: usize,
  }
  
  // 分析字符组成 | Analyze character composition
  fn analyze_characters(text: &str) -> CharStats {
      let mut stats = CharStats::default();
      
      for ch in text.chars() {
          if ch.is_alphabetic() {
              stats.letters += 1;
          } else if ch.is_numeric() {
              stats.digits += 1;
          } else if ch.is_whitespace() {
              stats.whitespace += 1;
          } else {
              stats.others += 1;
          }
      }
      
      stats
  }
  
  // 处理多语言文本 | Process multilingual text
  fn process_mixed_script(text: &str) -> String {
      text.chars()
          .map(|ch| {
              match ch {
                  // 保留ASCII字母和数字 | Keep ASCII letters and digits
                  c if c.is_ascii_alphanumeric() => c,
                  // 将其他字符替换为下划线 | Replace other characters with underscore
                  ' ' => ' ', // 保留空格 | Keep spaces
                  _ if ch.is_alphabetic() => ch, // 保留其他语言字母 | Keep other language letters
                  _ => '_', // 其他字符替换 | Replace other characters
              }
          })
          .collect()
  }
  ```

### 5. 字符串性能优化 | String Performance Optimization (30分钟 | 30 minutes)

- **内存分配和容量管理 | Memory Allocation and Capacity Management**
  
  **关键原则 | Key Principles:**
  - 预分配容量避免频繁重新分配 | Pre-allocate capacity to avoid frequent reallocation
  - 使用&str参数提高API灵活性 | Use &str parameters for API flexibility
  - 选择合适的字符串操作方法 | Choose appropriate string manipulation methods
  - 理解String内部结构和成本 | Understand String internal structure and costs
  
  **实践验证问题 | Practice Verification Questions:**
  1. 什么时候应该使用String::with_capacity？| When should String::with_capacity be used?
  2. push_str和+操作符哪个更高效？| Which is more efficient, push_str or + operator?
  3. 如何避免不必要的字符串克隆？| How to avoid unnecessary string cloning?

### 6. 最佳实践和常见陷阱 | Best Practices and Common Pitfalls (30分钟 | 30 minutes)

- **字符串使用最佳实践 | String Usage Best Practices**
  
  **关键原则 | Key Principles:**
  - 函数参数优先使用&str而不是String | Prefer &str over String for function parameters
  - 避免在循环中进行字符串连接 | Avoid string concatenation in loops
  - 正确处理UTF-8字符边界 | Properly handle UTF-8 character boundaries
  - 选择合适的字符串方法避免panic | Choose appropriate string methods to avoid panic
  
  **实践验证问题 | Practice Verification Questions:**
  1. 为什么函数参数推荐使用&str？| Why are &str parameters recommended for functions?
  2. 如何高效地构建动态字符串？| How to efficiently build dynamic strings?
  3. 处理用户输入时需要注意什么？| What should be considered when handling user input?

## 实践项目：智能文本处理工具 | Practical Project: Smart Text Processing Tool

### 目标 | Objective
创建一个综合性的文本处理工具，演示字符串的各种操作，包括文本分析、格式化、搜索替换、编码处理等功能，展示Rust字符串处理的强大能力和安全性。 | Create a comprehensive text processing tool that demonstrates various string operations, including text analysis, formatting, search and replace, encoding handling, and other features, showcasing Rust's powerful and safe string processing capabilities.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. String和&str的主要区别是什么？| What are the main differences between String and &str?
   **答案 | Answer:** String拥有数据且可变，&str是不可变的借用引用 | String owns data and is mutable, &str is immutable borrowed reference
2. 如何安全地进行字符串切片？| How to safely perform string slicing?
   **答案 | Answer:** 使用get方法或确保索引在字符边界上 | Use get method or ensure indices are on character boundaries
3. UTF-8编码的特点是什么？| What are the characteristics of UTF-8 encoding?
   **答案 | Answer:** 变长编码，ASCII兼容，支持所有Unicode字符 | Variable-length encoding, ASCII compatible, supports all Unicode characters

### 步骤 | Steps
1. 设计核心数据结构和文本分析功能 | Design core data structures and text analysis features
2. 实现基础文本操作：搜索、替换、格式化 | Implement basic text operations: search, replace, formatting
3. 添加Unicode和多语言支持 | Add Unicode and multilingual support
4. 实现高级功能：统计分析、文本转换 | Implement advanced features: statistical analysis, text transformation
5. 添加用户界面和错误处理 | Add user interface and error handling

### 示例代码 | Example Code
```rust
"""
智能文本处理工具 | Smart Text Processing Tool
演示Rust字符串处理的综合应用 | Demonstrates comprehensive application of Rust string processing

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- String和&str的正确使用 | Proper usage of String and &str
- UTF-8和Unicode字符处理 | UTF-8 and Unicode character handling
- 高效的字符串操作和内存管理 | Efficient string operations and memory management
"""

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct TextStats {
    character_count: usize,
    word_count: usize,
    line_count: usize,
    byte_count: usize,
    paragraph_count: usize,
    sentence_count: usize,
}

#[derive(Debug)]
pub struct TextProcessor {
    content: String,
    history: Vec<String>, // 操作历史 | Operation history
}

impl TextProcessor {
    // 创建新的文本处理器 | Create new text processor
    pub fn new() -> Self {
        TextProcessor {
            content: String::new(),
            history: Vec::new(),
        }
    }
    
    // 从字符串创建处理器 | Create processor from string
    pub fn from_text(text: &str) -> Self {
        let mut processor = Self::new();
        processor.set_text(text);
        processor
    }
    
    // 设置文本内容 | Set text content
    pub fn set_text(&mut self, text: &str) {
        self.save_to_history(); // 保存历史状态 | Save historical state
        self.content = text.to_string();
    }
    
    // 保存当前状态到历史 | Save current state to history
    fn save_to_history(&mut self) {
        if !self.content.is_empty() {
            self.history.push(self.content.clone());
            // 限制历史记录数量 | Limit history count
            if self.history.len() > 10 {
                self.history.remove(0);
            }
        }
    }
    
    // 撤销操作 | Undo operation
    pub fn undo(&mut self) -> bool {
        if let Some(previous) = self.history.pop() {
            self.content = previous;
            true
        } else {
            false
        }
    }
    
    // 获取文本统计信息 | Get text statistics
    pub fn get_stats(&self) -> TextStats {
        let content = &self.content;
        
        TextStats {
            character_count: content.chars().count(),
            word_count: content.split_whitespace().count(),
            line_count: content.lines().count(),
            byte_count: content.len(),
            paragraph_count: content.split("\n\n").filter(|p| !p.trim().is_empty()).count(),
            sentence_count: self.count_sentences(content),
        }
    }
    
    // 计算句子数量 | Count sentences
    fn count_sentences(&self, text: &str) -> usize {
        text.chars()
            .filter(|&c| c == '.' || c == '!' || c == '?' || c == '。' || c == '！' || c == '？')
            .count()
    }
    
    // 搜索文本 | Search text
    pub fn search(&self, pattern: &str, case_sensitive: bool) -> Vec<(usize, &str)> {
        let content = &self.content;
        let search_text = if case_sensitive {
            content.as_str()
        } else {
            // 对于不区分大小写，我们需要不同的方法 | For case-insensitive, we need different approach
            return self.search_case_insensitive(pattern);
        };
        
        let mut matches = Vec::new();
        let mut start = 0;
        
        while let Some(pos) = search_text[start..].find(pattern) {
            let absolute_pos = start + pos;
            matches.push((absolute_pos, &search_text[absolute_pos..absolute_pos + pattern.len()]));
            start = absolute_pos + 1;
        }
        
        matches
    }
    
    // 不区分大小写搜索 | Case-insensitive search
    fn search_case_insensitive(&self, pattern: &str) -> Vec<(usize, &str)> {
        let content_lower = self.content.to_lowercase();
        let pattern_lower = pattern.to_lowercase();
        let mut matches = Vec::new();
        let mut start = 0;
        
        while let Some(pos) = content_lower[start..].find(&pattern_lower) {
            let absolute_pos = start + pos;
            // 获取原文中对应的文本 | Get corresponding text in original
            if let Some(original_slice) = self.content.get(absolute_pos..absolute_pos + pattern.len()) {
                matches.push((absolute_pos, original_slice));
            }
            start = absolute_pos + 1;
        }
        
        matches
    }
    
    // 替换文本 | Replace text
    pub fn replace(&mut self, from: &str, to: &str, replace_all: bool) -> usize {
        self.save_to_history();
        
        let count = if replace_all {
            let original_len = self.content.len();
            self.content = self.content.replace(from, to);
            // 计算替换次数的近似值 | Approximate replacement count
            (original_len.saturating_sub(self.content.len()) + to.len().saturating_sub(from.len())) 
                / from.len().max(1)
        } else {
            if let Some(pos) = self.content.find(from) {
                self.content.replace_range(pos..pos + from.len(), to);
                1
            } else {
                0
            }
        };
        
        count
    }
    
    // 插入文本 | Insert text
    pub fn insert(&mut self, position: usize, text: &str) -> Result<(), String> {
        if position > self.content.len() {
            return Err(format!("位置{}超出文本长度{} | Position {} exceeds text length {}", 
                              position, self.content.len(), position, self.content.len()));
        }
        
        // 确保在字符边界上插入 | Ensure insertion at character boundary
        if !self.content.is_char_boundary(position) {
            return Err(format!("位置{}不在字符边界上 | Position {} is not on character boundary", 
                              position, position));
        }
        
        self.save_to_history();
        self.content.insert_str(position, text);
        Ok(())
    }
    
    // 删除文本范围 | Delete text range
    pub fn delete_range(&mut self, start: usize, end: usize) -> Result<String, String> {
        if start >= end || end > self.content.len() {
            return Err(format!("无效范围: {}..{} | Invalid range: {}..{}", 
                              start, end, start, end));
        }
        
        if !self.content.is_char_boundary(start) || !self.content.is_char_boundary(end) {
            return Err("范围不在字符边界上 | Range not on character boundaries".to_string());
        }
        
        self.save_to_history();
        let deleted = self.content[start..end].to_string();
        self.content.replace_range(start..end, "");
        Ok(deleted)
    }
    
    // 格式化文本 | Format text
    pub fn format_text(&mut self, operation: TextFormatOperation) {
        self.save_to_history();
        
        match operation {
            TextFormatOperation::ToUpperCase => {
                self.content = self.content.to_uppercase();
            }
            TextFormatOperation::ToLowerCase => {
                self.content = self.content.to_lowercase();
            }
            TextFormatOperation::ToTitleCase => {
                self.content = self.to_title_case(&self.content.clone());
            }
            TextFormatOperation::TrimWhitespace => {
                self.content = self.content.trim().to_string();
            }
            TextFormatOperation::RemoveExtraSpaces => {
                self.content = self.remove_extra_spaces(&self.content.clone());
            }
            TextFormatOperation::AddLineNumbers => {
                self.content = self.add_line_numbers(&self.content.clone());
            }
        }
    }
    
    // 转换为标题格式 | Convert to title case
    fn to_title_case(&self, text: &str) -> String {
        text.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
    
    // 移除多余空格 | Remove extra spaces
    fn remove_extra_spaces(&self, text: &str) -> String {
        text.split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }
    
    // 添加行号 | Add line numbers
    fn add_line_numbers(&self, text: &str) -> String {
        text.lines()
            .enumerate()
            .map(|(i, line)| format!("{:3}: {}", i + 1, line))
            .collect::<Vec<String>>()
            .join("\n")
    }
    
    // 分析字符组成 | Analyze character composition
    pub fn analyze_characters(&self) -> HashMap<String, usize> {
        let mut char_types = HashMap::new();
        
        for ch in self.content.chars() {
            let category = match ch {
                c if c.is_ascii_alphabetic() => "ASCII字母 | ASCII Letters",
                c if c.is_ascii_digit() => "ASCII数字 | ASCII Digits",
                c if c.is_ascii_punctuation() => "ASCII标点 | ASCII Punctuation",
                c if c.is_ascii_whitespace() => "ASCII空白 | ASCII Whitespace",
                c if c.is_alphabetic() && !c.is_ascii() => "非ASCII字母 | Non-ASCII Letters",
                c if c.is_numeric() && !c.is_ascii() => "非ASCII数字 | Non-ASCII Numbers",
                c if c.is_whitespace() && !c.is_ascii() => "非ASCII空白 | Non-ASCII Whitespace",
                _ => "其他字符 | Other Characters",
            };
            
            *char_types.entry(category.to_string()).or_insert(0) += 1;
        }
        
        char_types
    }
    
    // 提取单词频率 | Extract word frequency
    pub fn word_frequency(&self, min_length: usize) -> HashMap<String, usize> {
        let mut frequency = HashMap::new();
        
        for word in self.content.split_whitespace() {
            // 清理标点符号 | Clean punctuation
            let clean_word: String = word.chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
                .to_lowercase();
            
            if clean_word.len() >= min_length {
                *frequency.entry(clean_word).or_insert(0) += 1;
            }
        }
        
        frequency
    }
    
    // 获取文本内容的引用 | Get reference to text content
    pub fn content(&self) -> &str {
        &self.content
    }
    
    // 检查是否为空 | Check if empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

#[derive(Debug, Clone)]
pub enum TextFormatOperation {
    ToUpperCase,
    ToLowerCase,
    ToTitleCase,
    TrimWhitespace,
    RemoveExtraSpaces,
    AddLineNumbers,
}

// 用户界面函数 | User interface functions
pub fn run_interactive_mode() {
    println!("智能文本处理工具 | Smart Text Processing Tool");
    println!("输入 'help' 查看帮助，'quit' 退出 | Enter 'help' for help, 'quit' to exit");
    
    let mut processor = TextProcessor::new();
    let stdin = io::stdin();
    
    loop {
        print!("\n> ");
        let mut input = String::new();
        
        if stdin.read_line(&mut input).is_err() {
            println!("读取输入失败 | Failed to read input");
            continue;
        }
        
        let command = input.trim();
        
        match command {
            "quit" | "exit" => break,
            "help" => show_help(),
            "stats" => show_stats(&processor),
            "content" => show_content(&processor),
            "clear" => {
                processor.set_text("");
                println!("文本已清空 | Text cleared");
            }
            "undo" => {
                if processor.undo() {
                    println!("已撤销操作 | Operation undone");
                } else {
                    println!("无操作可撤销 | No operation to undo");
                }
            }
            cmd if cmd.starts_with("load ") => {
                let text = &cmd[5..];
                processor.set_text(text);
                println!("已加载文本 | Text loaded");
            }
            cmd if cmd.starts_with("search ") => {
                let pattern = &cmd[7..];
                let matches = processor.search(pattern, true);
                println!("找到 {} 个匹配 | Found {} matches:", matches.len(), matches.len());
                for (pos, text) in matches.iter().take(5) {
                    println!("  位置 {}: '{}' | Position {}: '{}'", pos, text, pos, text);
                }
            }
            cmd if cmd.starts_with("replace ") => {
                let parts: Vec<&str> = cmd[8..].splitn(2, " with ").collect();
                if parts.len() == 2 {
                    let count = processor.replace(parts[0], parts[1], true);
                    println!("已替换 {} 处 | Replaced {} occurrences", count, count);
                } else {
                    println!("格式: replace <old> with <new> | Format: replace <old> with <new>");
                }
            }
            "uppercase" => {
                processor.format_text(TextFormatOperation::ToUpperCase);
                println!("已转换为大写 | Converted to uppercase");
            }
            "lowercase" => {
                processor.format_text(TextFormatOperation::ToLowerCase);
                println!("已转换为小写 | Converted to lowercase");
            }
            "titlecase" => {
                processor.format_text(TextFormatOperation::ToTitleCase);
                println!("已转换为标题格式 | Converted to title case");
            }
            "analyze" => show_character_analysis(&processor),
            "frequency" => show_word_frequency(&processor),
            _ => println!("未知命令，输入 'help' 查看帮助 | Unknown command, enter 'help' for help"),
        }
    }
    
    println!("再见！| Goodbye!");
}

fn show_help() {
    println!("可用命令 | Available commands:");
    println!("  load <text>     - 加载文本 | Load text");
    println!("  content         - 显示当前文本 | Show current text");
    println!("  stats           - 显示统计信息 | Show statistics");
    println!("  search <pattern> - 搜索文本 | Search text");
    println!("  replace <old> with <new> - 替换文本 | Replace text");
    println!("  uppercase       - 转换为大写 | Convert to uppercase");
    println!("  lowercase       - 转换为小写 | Convert to lowercase");
    println!("  titlecase       - 转换为标题格式 | Convert to title case");
    println!("  analyze         - 字符分析 | Character analysis");
    println!("  frequency       - 词频分析 | Word frequency analysis");
    println!("  undo            - 撤销操作 | Undo operation");
    println!("  clear           - 清空文本 | Clear text");
    println!("  help            - 显示帮助 | Show help");
    println!("  quit            - 退出程序 | Exit program");
}

fn show_stats(processor: &TextProcessor) {
    let stats = processor.get_stats();
    println!("文本统计 | Text Statistics:");
    println!("  字符数: {} | Characters: {}", stats.character_count, stats.character_count);
    println!("  单词数: {} | Words: {}", stats.word_count, stats.word_count);
    println!("  行数: {} | Lines: {}", stats.line_count, stats.line_count);
    println!("  字节数: {} | Bytes: {}", stats.byte_count, stats.byte_count);
    println!("  段落数: {} | Paragraphs: {}", stats.paragraph_count, stats.paragraph_count);
    println!("  句子数: {} | Sentences: {}", stats.sentence_count, stats.sentence_count);
}

fn show_content(processor: &TextProcessor) {
    if processor.is_empty() {
        println!("当前无文本内容 | No text content currently");
    } else {
        println!("当前文本内容 | Current text content:");
        println!("---");
        println!("{}", processor.content());
        println!("---");
    }
}

fn show_character_analysis(processor: &TextProcessor) {
    let analysis = processor.analyze_characters();
    println!("字符类型分析 | Character Type Analysis:");
    
    for (char_type, count) in analysis.iter() {
        println!("  {}: {}", char_type, count);
    }
}

fn show_word_frequency(processor: &TextProcessor) {
    let frequency = processor.word_frequency(3); // 最小长度3 | Minimum length 3
    println!("词频分析 (长度≥3) | Word Frequency Analysis (length≥3):");
    
    let mut sorted_words: Vec<_> = frequency.iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(a.1)); // 按频率降序 | Sort by frequency descending
    
    for (word, count) in sorted_words.iter().take(10) {
        println!("  '{}': {} 次 | '{}': {} times", word, count, word, count);
    }
}

fn main() {
    // 演示基本功能 | Demonstrate basic functionality
    println!("智能文本处理工具演示 | Smart Text Processing Tool Demo");
    println!("=".repeat(50));
    
    // 创建文本处理器并加载示例文本 | Create text processor and load sample text
    let sample_text = "Hello World! 你好世界！🌍 This is a sample text with multiple languages. \
                       这是一个包含多种语言的示例文本。We'll analyze this text using various methods. \
                       我们将使用各种方法分析这段文本。Programming in Rust is fun! Rust编程很有趣！";
    
    let mut processor = TextProcessor::from_text(sample_text);
    
    // 显示基本统计 | Show basic statistics
    println!("\n1. 基本统计信息 | Basic Statistics:");
    let stats = processor.get_stats();
    println!("字符数: {}, 单词数: {}, 字节数: {} | Characters: {}, Words: {}, Bytes: {}", 
             stats.character_count, stats.word_count, stats.byte_count,
             stats.character_count, stats.word_count, stats.byte_count);
    
    // 搜索演示 | Search demonstration
    println!("\n2. 搜索演示 | Search Demonstration:");
    let matches = processor.search("Rust", true);
    println!("找到 'Rust' {} 次 | Found 'Rust' {} times", matches.len(), matches.len());
    
    // 替换演示 | Replace demonstration
    println!("\n3. 替换演示 | Replace Demonstration:");
    let count = processor.replace("fun", "awesome", false);
    println!("替换了 {} 处 | Replaced {} occurrences", count, count);
    
    // 字符分析 | Character analysis
    println!("\n4. 字符类型分析 | Character Type Analysis:");
    let char_analysis = processor.analyze_characters();
    for (char_type, count) in char_analysis.iter() {
        println!("  {}: {}", char_type, count);
    }
    
    // 词频分析 | Word frequency analysis
    println!("\n5. 词频分析 (前5个) | Word Frequency Analysis (Top 5):");
    let frequency = processor.word_frequency(2);
    let mut sorted_words: Vec<_> = frequency.iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(a.1));
    
    for (word, count) in sorted_words.iter().take(5) {
        println!("  '{}': {} 次 | '{}': {} times", word, count, word, count);
    }
    
    println!("\n演示完成！输入任意键继续交互模式... | Demo complete! Press any key to continue to interactive mode...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    
    // 启动交互模式 | Start interactive mode
    run_interactive_mode();
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确区分了String和&str的使用场景？| Does the project correctly distinguish between String and &str use cases?
2. UTF-8和Unicode字符处理是否安全无误？| Is UTF-8 and Unicode character handling safe and correct?
3. 字符串操作是否考虑了性能优化？| Do string operations consider performance optimization?
4. 是否演示了字符串的各种实用操作？| Does it demonstrate various practical string operations?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **字符串内存管理练习 | String Memory Management Exercise**
   - **练习描述 | Exercise Description:** 比较不同字符串操作的内存使用和性能
   - **概念检查 | Concept Check:** 理解String的内存分配机制和优化策略？
   - **学习目标 | Learning Objective:** 掌握高效的字符串内存管理

2. **Unicode处理深入练习 | Unicode Handling Deep Dive Exercise**
   - **练习描述 | Exercise Description:** 处理复杂的多语言文本和特殊Unicode字符
   - **概念检查 | Concept Check:** 能否正确处理各种Unicode字符边界情况？
   - **学习目标 | Learning Objective:** 深入理解UTF-8编码和Unicode标准

3. **字符串解析练习 | String Parsing Exercise**
   - **练习描述 | Exercise Description:** 实现复杂的文本解析器（如CSV、JSON等）
   - **概念检查 | Concept Check:** 如何安全高效地解析结构化文本数据？
   - **学习目标 | Learning Objective:** 提高文本解析和处理能力

4. **性能对比练习 | Performance Comparison Exercise**
   - **练习描述 | Exercise Description:** 对比不同字符串操作方法的性能差异
   - **概念检查 | Concept Check:** 理解各种字符串操作的时间和空间复杂度？
   - **学习目标 | Learning Objective:** 学会选择最优的字符串处理方法

5. **正则表达式集成练习 | Regular Expression Integration Exercise**
   - **练习描述 | Exercise Description:** 结合regex库实现高级文本匹配和处理
   - **概念检查 | Concept Check:** 如何在Rust中有效使用正则表达式？
   - **学习目标 | Learning Objective:** 掌握复杂文本模式匹配

6. **文本编码转换练习 | Text Encoding Conversion Exercise**
   - **练习描述 | Exercise Description:** 实现不同字符编码间的转换（UTF-8、UTF-16、GBK等）
   - **概念检查 | Concept Check:** 理解不同字符编码的特点和转换方法？
   - **学习目标 | Learning Objective:** 掌握字符编码转换技术

7. **流式文本处理练习 | Streaming Text Processing Exercise**
   - **练习描述 | Exercise Description:** 处理大型文本文件而不将全部内容加载到内存
   - **概念检查 | Concept Check:** 如何高效处理超大文本文件？
   - **学习目标 | Learning Objective:** 学会内存高效的流式文本处理

## 学习资源 | Learning Resources
- [Rust官方文档 - Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Rust Reference - String Literals](https://doc.rust-lang.org/reference/tokens.html#string-literals)
- [Unicode Standard](https://unicode.org/standard/standard.html)
- [UTF-8 Encoding](https://en.wikipedia.org/wiki/UTF-8)
- [Rust String API Documentation](https://doc.rust-lang.org/std/string/struct.String.html)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解String和&str的区别和使用场景 | Understand differences and use cases of String and &str
- [ ] 掌握字符串切片的安全操作方法 | Master safe string slicing operations
- [ ] 熟悉各种字符串方法和常用操作 | Be familiar with various string methods and common operations
- [ ] 理解UTF-8编码和Unicode字符处理 | Understand UTF-8 encoding and Unicode character handling
- [ ] 掌握字符遍历和处理技巧 | Master character iteration and processing techniques
- [ ] 学会字符串性能优化策略 | Learn string performance optimization strategies
- [ ] 完成智能文本处理工具项目 | Complete the smart text processing tool project
- [ ] 能够处理多语言和特殊字符 | Can handle multilingual and special characters
- [ ] 理解字符串内存管理机制 | Understand string memory management mechanisms
- [ ] 至少完成3个扩展练习 | Complete at least 3 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释每个核心概念。特别重要的是理解Rust字符串系统的设计哲学：安全性、性能和正确的Unicode支持，以及如何在实际编程中正确使用String和&str类型。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain each core concept to others. It's particularly important to understand Rust's string system design philosophy: safety, performance, and proper Unicode support, as well as how to correctly use String and &str types in practical programming.