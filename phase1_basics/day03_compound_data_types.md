# Rust入门 - 第3天：复合数据类型 | Rust Introduction - Day 3: Compound Data Types

## 学习目标 | Learning Objectives
- 掌握元组的定义、创建和使用 | Master tuple definition, creation, and usage
- 理解数组的特点和操作方法 | Understand array characteristics and operations
- 学习切片的基本概念和应用 | Learn basic slice concepts and applications
- 区分字符串类型 &str 和 String 的差异 | Distinguish between string types &str and String
- 能够选择合适的复合数据类型解决问题 | Be able to choose appropriate compound data types for problem solving
- 熟练使用索引和方法访问复合数据 | Proficiently use indexing and methods to access compound data

## 详细内容 | Detailed Content

### 1. 元组类型详解 | Tuple Type Deep Dive (1.5小时 | 1.5 hours)

- **元组 (Tuple) | Tuple**
  
  **概念定义 | Concept Definition:**
  元组是一个固定长度的复合类型，可以包含不同类型的值，一旦创建后长度不可改变 | A tuple is a fixed-length compound type that can contain values of different types, and its length cannot be changed once created
  
  **核心特征 | Key Characteristics:**
  - 固定长度：元组的元素个数在编译时确定 | Fixed length: the number of elements in a tuple is determined at compile time
  - 异质性：可以包含不同类型的数据 | Heterogeneity: can contain data of different types
  - 有序性：元素有固定的位置和索引 | Ordered: elements have fixed positions and indices
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 元组创建后可以改变长度吗？| Can a tuple's length be changed after creation?
     **答案 | Answer:** 否 | No - 元组长度在编译时确定，运行时不可改变 | tuple length is determined at compile time and cannot be changed at runtime
  2. 一个元组可以包含不同类型的元素吗？| Can a tuple contain elements of different types?  
     **答案 | Answer:** 是 | Yes - 元组可以混合存储不同类型的数据 | tuples can store mixed data of different types
  3. 元组的第一个元素索引是多少？| What is the index of the first element in a tuple?
     **答案 | Answer:** 0 - 元组使用从0开始的索引 | tuples use zero-based indexing
  4. 空元组 () 是有效的Rust类型吗？| Is the empty tuple () a valid Rust type?
     **答案 | Answer:** 是 | Yes - 空元组是单元类型(unit type)，常用于函数无返回值时 | empty tuple is the unit type, commonly used when functions have no return value
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 元组的创建和基本使用 | Creating and basic usage of tuples
  fn main() {
      // 创建包含不同类型的元组 | Creating tuple with different types
      let student_info: (String, i32, f64, bool) = (
          String::from("张三"),  // 姓名 | name
          20,                    // 年龄 | age  
          85.5,                  // 成绩 | grade
          true                   // 是否在校 | enrolled status
      );
      
      // 通过索引访问元组元素 | Accessing tuple elements by index
      println!("学生姓名: {}", student_info.0);    // 访问第一个元素 | access first element
      println!("学生年龄: {}", student_info.1);    // 访问第二个元素 | access second element
      println!("学生成绩: {}", student_info.2);    // 访问第三个元素 | access third element
      println!("在校状态: {}", student_info.3);    // 访问第四个元素 | access fourth element
      
      // 元组解构 | Tuple destructuring
      let (name, age, grade, enrolled) = student_info;
      println!("解构后 - 姓名: {}, 年龄: {}", name, age);
      
      // 创建嵌套元组 | Creating nested tuples
      let coordinates: ((i32, i32), (i32, i32)) = ((0, 0), (10, 20));
      println!("起点: ({}, {})", coordinates.0.0, coordinates.0.1);
      println!("终点: ({}, {})", coordinates.1.0, coordinates.1.1);
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码会输出什么？| What will this code output?
    **答案 | Answer:** 会输出学生的姓名、年龄、成绩、在校状态，以及解构后的信息和坐标点 | Will output student's name, age, grade, enrollment status, destructured info, and coordinate points
  - 如果尝试访问 student_info.4 会发生什么？| What happens if we try to access student_info.4?
    **答案 | Answer:** 编译错误，因为元组只有4个元素（索引0-3） | Compilation error, because the tuple only has 4 elements (indices 0-3)
  
  **常见误区检查 | Common Misconception Checks:**
  - 元组是否可以像数组一样用 len() 方法获取长度？| Can tuples use the len() method like arrays to get length?
    **答案 | Answer:** 不可以，元组没有 len() 方法，长度在编译时已知 | No, tuples don't have len() method, length is known at compile time
  - 元组的元素是否可以修改？| Can tuple elements be modified?
    **答案 | Answer:** 默认不可修改，除非声明为 mut，且修改时类型必须匹配 | Not modifiable by default, unless declared as mut, and types must match when modifying

### 2. 数组类型深入 | Array Type In-Depth (1小时 | 1 hour)

- **数组 (Array) | Array**
  
  **概念定义 | Concept Definition:**
  数组是一个固定长度的同质数据集合，所有元素必须是相同类型，存储在连续的内存空间中 | An array is a fixed-length homogeneous data collection where all elements must be of the same type, stored in contiguous memory space
  
  **核心特征 | Key Characteristics:**
  - 同质性：所有元素必须是相同类型 | Homogeneity: all elements must be of the same type
  - 固定长度：编译时确定大小，运行时不可改变 | Fixed length: size determined at compile time, unchangeable at runtime
  - 栈分配：通常分配在栈上，访问速度快 | Stack allocation: usually allocated on stack, fast access
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 数组中的所有元素必须是相同类型吗？| Must all elements in an array be of the same type?
     **答案 | Answer:** 是 | Yes - 数组是同质数据结构 | arrays are homogeneous data structures
  2. 数组的长度可以在运行时改变吗？| Can an array's length be changed at runtime?
     **答案 | Answer:** 否 | No - 数组长度在编译时固定 | array length is fixed at compile time
  3. 数组通常存储在栈还是堆上？| Are arrays usually stored on the stack or heap?
     **答案 | Answer:** 栈 | Stack - 因为大小在编译时已知 | because size is known at compile time
  4. 访问数组越界元素会导致什么？| What happens when accessing array elements out of bounds?
     **答案 | Answer:** 运行时panic - Rust会检查边界并终止程序 | Runtime panic - Rust checks bounds and terminates the program
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      // 数组的不同创建方式 | Different ways to create arrays
      
      // 方式1：显式指定类型和大小 | Method 1: explicitly specify type and size
      let numbers: [i32; 5] = [1, 2, 3, 4, 5];
      
      // 方式2：类型推断 | Method 2: type inference
      let fruits = ["apple", "banana", "orange"];
      
      // 方式3：初始化相同值 | Method 3: initialize with same value
      let zeros = [0; 10];  // 创建包含10个0的数组 | create array with 10 zeros
      
      // 访问数组元素 | Accessing array elements
      println!("第一个数字: {}", numbers[0]);
      println!("最后一个水果: {}", fruits[fruits.len() - 1]);
      
      // 数组遍历 | Array iteration
      println!("所有数字:");
      for (index, value) in numbers.iter().enumerate() {
          println!("索引 {}: 值 {}", index, value);
      }
      
      // 数组切片 | Array slicing
      let slice = &numbers[1..4];  // 获取索引1到3的元素 | get elements from index 1 to 3
      println!("切片内容: {:?}", slice);
      
      // 数组长度和类型信息 | Array length and type info
      println!("numbers数组长度: {}", numbers.len());
      println!("numbers数组大小(字节): {}", std::mem::size_of_val(&numbers));
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - zeros 数组包含多少个元素？| How many elements does the zeros array contain?
    **答案 | Answer:** 10个元素，每个都是0 | 10 elements, each is 0
  - slice 包含哪些元素？| What elements does slice contain?
    **答案 | Answer:** [2, 3, 4] - 索引1到3的元素 | elements from index 1 to 3
  
  **常见误区检查 | Common Misconception Checks:**
  - 数组和向量(Vec)是同一种类型吗？| Are arrays and vectors (Vec) the same type?
    **答案 | Answer:** 不是，数组长度固定且栈分配，Vec长度可变且堆分配 | No, arrays have fixed length and are stack-allocated, Vec has variable length and is heap-allocated
  - 可以向现有数组添加新元素吗？| Can you add new elements to an existing array?
    **答案 | Answer:** 不可以，数组长度固定，需要使用Vec实现动态增长 | No, array length is fixed, use Vec for dynamic growth

### 3. 切片基础概念 | Basic Slice Concepts (45分钟 | 45 minutes)

- **切片 (Slice) | Slice**
  
  **概念定义 | Concept Definition:**
  切片是对连续内存序列的引用，不拥有数据，而是借用其他数据结构的一部分 | A slice is a reference to a contiguous sequence in memory that doesn't own data but borrows part of other data structures
  
  **核心特征 | Key Characteristics:**
  - 引用类型：不拥有数据，只是借用 | Reference type: doesn't own data, only borrows
  - 动态大小：长度在运行时确定 | Dynamic size: length determined at runtime
  - 类型标记：使用 &[T] 表示类型为T的切片 | Type notation: use &[T] to represent slice of type T
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 切片拥有它引用的数据吗？| Does a slice own the data it references?
     **答案 | Answer:** 否 | No - 切片只是借用数据的引用 | slices are just references that borrow data
  2. 切片的长度在编译时确定吗？| Is a slice's length determined at compile time?
     **答案 | Answer:** 否 | No - 切片长度在运行时确定 | slice length is determined at runtime
  3. &[i32] 表示什么？| What does &[i32] represent?
     **答案 | Answer:** i32类型的切片引用 | A slice reference to i32 type elements
  4. 切片可以修改原始数据吗？| Can slices modify the original data?
     **答案 | Answer:** 取决于是否是可变切片(&mut [T]) | Depends on whether it's a mutable slice (&mut [T])
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      // 从数组创建切片 | Creating slices from arrays
      let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
      
      // 不同的切片创建方式 | Different ways to create slices
      let full_slice = &numbers[..];          // 完整切片 | full slice
      let partial_slice = &numbers[2..7];     // 部分切片 | partial slice
      let from_start = &numbers[..5];         // 从开头切片 | slice from start
      let to_end = &numbers[3..];             // 到结尾切片 | slice to end
      
      println!("原数组: {:?}", numbers);
      println!("完整切片: {:?}", full_slice);
      println!("部分切片(索引2-6): {:?}", partial_slice);
      println!("从开头切片(索引0-4): {:?}", from_start);
      println!("到结尾切片(索引3-9): {:?}", to_end);
      
      // 切片的操作方法 | Slice operation methods
      println!("切片长度: {}", partial_slice.len());
      println!("切片是否为空: {}", partial_slice.is_empty());
      println!("第一个元素: {:?}", partial_slice.first());
      println!("最后一个元素: {:?}", partial_slice.last());
      
      // 可变切片 | Mutable slices
      let mut mutable_array = [1, 2, 3, 4, 5];
      let mutable_slice = &mut mutable_array[1..4];
      
      println!("修改前的切片: {:?}", mutable_slice);
      mutable_slice[0] = 100;  // 修改切片中的元素 | modify element in slice
      println!("修改后的切片: {:?}", mutable_slice);
      println!("修改后的原数组: {:?}", mutable_array);
  }
  
  // 函数参数中使用切片 | Using slices in function parameters
  fn process_slice(data: &[i32]) -> i32 {
      // 计算切片元素的和 | calculate sum of slice elements
      data.iter().sum()
  }
  
  #[cfg(test)]
  mod tests {
      use super::*;
      
      #[test]
      fn test_slice_operations() {
          let numbers = [1, 2, 3, 4, 5];
          let slice = &numbers[1..4];  // [2, 3, 4]
          
          assert_eq!(slice.len(), 3);
          assert_eq!(process_slice(slice), 9);  // 2 + 3 + 4 = 9
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - partial_slice 包含哪些元素？| What elements does partial_slice contain?
    **答案 | Answer:** [3, 4, 5, 6, 7] - 索引2到6的元素 | elements from index 2 to 6
  - 修改 mutable_slice[0] 会影响原数组吗？| Will modifying mutable_slice[0] affect the original array?
    **答案 | Answer:** 是的，切片是引用，修改会影响原数据 | Yes, slices are references, modifications affect original data

### 4. 字符串类型区分 | String Type Differentiation (1小时 | 1 hour)

- **字符串类型 (&str vs String) | String Types (&str vs String)**
  
  **概念定义 | Concept Definition:**
  Rust中有两种主要的字符串类型：&str是字符串切片（借用），String是拥有所有权的可变字符串 | Rust has two main string types: &str is a string slice (borrowed), String is an owned, mutable string
  
  **核心特征对比 | Key Characteristics Comparison:**
  - 所有权：&str借用数据，String拥有数据 | Ownership: &str borrows data, String owns data
  - 可变性：&str不可变，String可变 | Mutability: &str is immutable, String is mutable  
  - 存储位置：&str通常在程序二进制中，String在堆上 | Storage: &str usually in program binary, String on heap
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. &str 和 String 哪个拥有字符串数据？| Which owns the string data, &str or String?
     **答案 | Answer:** String - &str只是借用引用 | String - &str is just a borrowed reference
  2. 字符串字面量 "hello" 是什么类型？| What type is the string literal "hello"?
     **答案 | Answer:** &str - 字符串字面量是字符串切片 | &str - string literals are string slices
  3. 哪种字符串类型可以修改内容？| Which string type can modify its content?
     **答案 | Answer:** String - 因为它拥有数据且可变 | String - because it owns the data and is mutable
  4. String::from() 和 .to_string() 的作用是什么？| What do String::from() and .to_string() do?
     **答案 | Answer:** 都将 &str 转换为 String，创建拥有所有权的副本 | Both convert &str to String, creating an owned copy
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  fn main() {
      // 字符串字面量 (&str) | String literals (&str)
      let greeting: &str = "Hello, World!";  // 存储在程序二进制中 | stored in program binary
      let name = "张三";                      // 支持UTF-8编码 | supports UTF-8 encoding
      
      println!("问候语: {}", greeting);
      println!("姓名: {}", name);
      
      // 创建String类型 | Creating String type
      let mut owned_string = String::new();           // 创建空String | create empty String
      let from_literal = String::from("Hello");       // 从字面量创建 | create from literal
      let to_string_method = "World".to_string();     // 使用to_string方法 | use to_string method
      
      // String的可变操作 | Mutable operations on String
      owned_string.push_str("你好");          // 追加字符串 | append string
      owned_string.push('!');                 // 追加字符 | append character
      
      println!("构建的字符串: {}", owned_string);
      
      // 字符串拼接 | String concatenation
      let combined = format!("{} {}", from_literal, to_string_method);
      let concatenated = from_literal + " " + &to_string_method; // 注意&的使用 | note the use of &
      
      println!("格式化拼接: {}", combined);
      println!("运算符拼接: {}", concatenated);
      
      // 字符串切片操作 | String slicing operations
      let text = "Hello, 世界!";
      let hello = &text[0..5];        // 英文字符切片 | English character slice
      // 注意：中文字符占用多个字节，需要小心切片 | Note: Chinese characters occupy multiple bytes, slice carefully
      
      println!("英文部分: {}", hello);
      
      // 字符串遍历 | String iteration
      println!("按字符遍历:");
      for c in text.chars() {          // 按Unicode字符遍历 | iterate by Unicode characters
          println!("字符: {}", c);
      }
      
      println!("按字节遍历:");
      for b in text.bytes() {          // 按字节遍历 | iterate by bytes
          println!("字节: {}", b);
      }
      
      // 字符串比较和搜索 | String comparison and search
      let search_text = "Rust编程语言";
      println!("包含'Rust': {}", search_text.contains("Rust"));
      println!("以'Rust'开头: {}", search_text.starts_with("Rust"));
      println!("字符串长度(字节): {}", search_text.len());
      println!("字符数量: {}", search_text.chars().count());
  }
  
  // 函数参数中的字符串类型选择 | String type choice in function parameters
  fn process_str(s: &str) -> usize {
      // 接受&str可以处理字符串字面量和String引用 | accepting &str can handle both literals and String references
      s.len()
  }
  
  fn process_string(s: String) -> String {
      // 获得String所有权，可以修改 | takes ownership of String, can modify
      s.to_uppercase()
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - search_text.len() 和 search_text.chars().count() 为什么结果不同？| Why do search_text.len() and search_text.chars().count() return different results?
    **答案 | Answer:** len()返回字节数，chars().count()返回Unicode字符数，中文字符占用多个字节 | len() returns byte count, chars().count() returns Unicode character count, Chinese characters occupy multiple bytes
  - 为什么拼接时需要写 &to_string_method？| Why do we need to write &to_string_method when concatenating?
    **答案 | Answer:** + 操作符需要右侧是&str类型，&将String转换为&str | + operator requires right side to be &str type, & converts String to &str

### 5. 数据类型选择策略 | Data Type Selection Strategy (30分钟 | 30 minutes)

- **选择合适的复合类型 | Choosing Appropriate Compound Types**
  
  **选择指导原则 | Selection Guidelines:**
  - 元组：需要组合不同类型的相关数据时 | Tuple: when need to combine related data of different types
  - 数组：固定数量的同类型数据，性能要求高时 | Array: fixed amount of same-type data with high performance requirements
  - 切片：需要处理数据的一部分，不获取所有权时 | Slice: when need to process part of data without taking ownership
  
  **实践验证问题 | Practice Verification Questions:**
  1. 存储学生姓名、年龄、成绩应该用什么类型？| What type should be used to store student name, age, and grade?
     **答案 | Answer:** 元组，因为包含不同类型的相关数据 | Tuple, because it contains related data of different types
  2. 存储一周7天的温度数据应该用什么类型？| What type should be used to store temperature data for 7 days of a week?
     **答案 | Answer:** 数组，因为是固定数量的同类型数据 | Array, because it's a fixed amount of same-type data
  3. 函数参数接收字符串时应该用什么类型？| What type should be used when a function parameter receives strings?
     **答案 | Answer:** 通常用&str，因为更灵活，可以接受字面量和String引用 | Usually &str, because it's more flexible and can accept both literals and String references

### 6. 知识巩固与综合应用 | Knowledge Consolidation and Comprehensive Application (15分钟 | 15 minutes)

- **综合概念检查 | Comprehensive Concept Check:**
  
  1. 元组、数组、切片中哪些类型的长度是编译时确定的？| Which types among tuple, array, and slice have compile-time determined length?
     **答案 | Answer:** 元组和数组，切片长度在运行时确定 | Tuple and array, slice length is determined at runtime
  2. 哪种字符串类型适合作为函数返回值返回新创建的字符串？| Which string type is suitable for returning newly created strings from functions?
     **答案 | Answer:** String，因为需要返回所有权 | String, because ownership needs to be returned
  3. 修改数组元素和修改切片元素有什么区别？| What's the difference between modifying array elements and slice elements?
     **答案 | Answer:** 修改数组直接改变原数据，修改可变切片会影响其引用的原数据 | Modifying arrays changes original data directly, modifying mutable slices affects the original data they reference
  4. 何时使用 &str 何时使用 String？| When to use &str and when to use String?
     **答案 | Answer:** 只读时用&str，需要所有权或修改时用String | Use &str for read-only, use String when ownership or modification is needed
  5. 如何安全地处理字符串切片，避免panic？| How to safely handle string slicing to avoid panic?
     **答案 | Answer:** 使用字符边界进行切片，或使用get()方法安全访问 | Use character boundaries for slicing, or use get() method for safe access

## 实践项目：学生成绩管理系统 | Practical Project: Student Grade Management System

### 目标 | Objective
创建一个学生成绩管理系统，综合运用元组、数组、切片和字符串类型来存储和处理学生信息 | Create a student grade management system that comprehensively uses tuples, arrays, slices, and string types to store and process student information

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 如何使用元组存储一个学生的多种信息？| How to use tuples to store multiple types of information for a student?
   **答案 | Answer:** 使用 (String, i32, [f64; N]) 存储姓名、年龄和成绩数组
2. 如何从成绩数组中获取特定范围的成绩？| How to get grades within a specific range from a grade array?
   **答案 | Answer:** 使用数组切片，如 &grades[1..4]
3. 函数参数应该如何接收学生姓名以保持灵活性？| How should function parameters receive student names to maintain flexibility?
   **答案 | Answer:** 使用 &str 类型，可以接受字符串字面量和String引用

### 步骤 | Steps
1. 定义学生数据结构（使用元组）| Define student data structure (using tuples)
2. 创建学生数组存储多个学生信息 | Create student array to store multiple student information
3. 实现成绩统计功能（使用切片）| Implement grade statistics functionality (using slices)
4. 实现学生搜索功能（使用字符串操作）| Implement student search functionality (using string operations)
5. 添加成绩排序和显示功能 | Add grade sorting and display functionality

### 示例代码 | Example Code
```rust
"""
学生成绩管理系统 | Student Grade Management System
综合演示复合数据类型的应用 | Comprehensive demonstration of compound data type applications

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- 元组：存储学生基本信息 | Tuples: storing basic student information
- 数组：存储固定科目成绩 | Arrays: storing grades for fixed subjects
- 切片：处理成绩数据的部分 | Slices: processing parts of grade data
- 字符串：学生姓名处理 | Strings: student name processing
"""

// 定义常量 | Define constants
const SUBJECTS: [&str; 5] = ["数学", "语文", "英语", "物理", "化学"];
const SUBJECT_COUNT: usize = 5;

// 学生信息类型定义：(姓名, 年龄, 成绩数组) | Student info type definition: (name, age, grades array)
type StudentInfo = (String, u8, [f64; SUBJECT_COUNT]);

fn main() {
    // 创建学生数据 | Create student data
    let mut students: [StudentInfo; 3] = [
        (String::from("张三"), 18, [85.5, 92.0, 78.5, 88.0, 90.5]),
        (String::from("李四"), 19, [76.0, 88.5, 95.0, 82.5, 89.0]),
        (String::from("王五"), 18, [92.5, 79.0, 87.5, 91.0, 85.5]),
    ];
    
    println!("=== 学生成绩管理系统 | Student Grade Management System ===\n");
    
    // 显示所有学生信息 | Display all student information
    display_all_students(&students);
    
    // 计算和显示统计信息 | Calculate and display statistics
    calculate_statistics(&students);
    
    // 搜索学生 | Search students
    search_student(&students, "张三");
    search_student(&students, "赵六"); // 不存在的学生 | non-existent student
    
    // 科目分析 | Subject analysis
    analyze_subject_performance(&students, 0); // 分析数学成绩 | analyze math grades
    
    // 成绩排名 | Grade ranking
    display_ranking(&students);
    
    // 演示切片操作 | Demonstrate slice operations
    demonstrate_slice_operations(&students[0].2); // 使用第一个学生的成绩 | use first student's grades
}

// 显示所有学生信息 | Display all student information  
fn display_all_students(students: &[StudentInfo]) {
    println!("📚 所有学生信息 | All Student Information:");
    println!("{:-<60}", ""); // 分隔线 | separator line
    
    for (index, student) in students.iter().enumerate() {
        let (name, age, grades) = student; // 元组解构 | tuple destructuring
        println!("{}. 学生姓名: {} | 年龄: {} 岁", index + 1, name, age);
        
        // 显示各科成绩 | Display grades for each subject
        print!("   成绩: ");
        for (subject_index, &grade) in grades.iter().enumerate() {
            print!("{}: {:.1} ", SUBJECTS[subject_index], grade);
        }
        
        // 计算平均分 | Calculate average grade
        let average = calculate_average(grades);
        println!("| 平均分: {:.2}", average);
        println!();
    }
}

// 计算平均分 (接受数组切片) | Calculate average (accepts array slice)
fn calculate_average(grades: &[f64]) -> f64 {
    let sum: f64 = grades.iter().sum(); // 使用迭代器计算总和 | use iterator to calculate sum
    sum / grades.len() as f64
}

// 计算统计信息 | Calculate statistics
fn calculate_statistics(students: &[StudentInfo]) {
    println!("📊 统计信息 | Statistics:");
    println!("{:-<40}", "");
    
    // 计算全班各科平均分 | Calculate class average for each subject
    for (subject_index, &subject_name) in SUBJECTS.iter().enumerate() {
        let subject_grades: Vec<f64> = students
            .iter()
            .map(|(_, _, grades)| grades[subject_index]) // 提取特定科目成绩 | extract specific subject grade
            .collect();
        
        let average = calculate_average(&subject_grades);
        println!("{} 平均分: {:.2}", subject_name, average);
    }
    
    // 计算全班总平均分 | Calculate overall class average
    let all_grades: Vec<f64> = students
        .iter()
        .flat_map(|(_, _, grades)| grades.iter()) // 展平所有成绩 | flatten all grades
        .cloned()
        .collect();
    
    let overall_average = calculate_average(&all_grades);
    println!("全班总平均分: {:.2}", overall_average);
    println!();
}

// 搜索学生 (使用字符串匹配) | Search student (using string matching)
fn search_student(students: &[StudentInfo], search_name: &str) {
    println!("🔍 搜索学生: \"{}\"", search_name);
    
    // 查找匹配的学生 | Find matching student
    let found_student = students
        .iter()
        .find(|(name, _, _)| name == search_name); // 字符串比较 | string comparison
    
    match found_student {
        Some((name, age, grades)) => {
            println!("✅ 找到学生:");
            println!("   姓名: {} | 年龄: {} 岁", name, age);
            
            // 显示成绩详情 | Display grade details
            print!("   成绩: ");
            for (i, &grade) in grades.iter().enumerate() {
                print!("{}: {:.1} ", SUBJECTS[i], grade);
            }
            println!("| 平均分: {:.2}", calculate_average(grades));
        }
        None => {
            println!("❌ 未找到学生: \"{}\"", search_name);
        }
    }
    println!();
}

// 分析特定科目表现 | Analyze specific subject performance
fn analyze_subject_performance(students: &[StudentInfo], subject_index: usize) {
    if subject_index >= SUBJECT_COUNT {
        println!("❌ 无效的科目索引");
        return;
    }
    
    let subject_name = SUBJECTS[subject_index];
    println!("📈 {} 科目分析:", subject_name);
    
    // 收集该科目所有成绩 | Collect all grades for this subject
    let mut subject_grades: Vec<(&str, f64)> = students
        .iter()
        .map(|(name, _, grades)| (name.as_str(), grades[subject_index])) // 转换String为&str | convert String to &str
        .collect();
    
    // 按成绩排序 | Sort by grade
    subject_grades.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // 降序排列 | descending order
    
    println!("成绩排名:");
    for (rank, (name, grade)) in subject_grades.iter().enumerate() {
        println!("{}. {} - {:.1}分", rank + 1, name, grade);
    }
    
    // 找出最高分和最低分 | Find highest and lowest scores
    let grades_only: Vec<f64> = subject_grades.iter().map(|(_, grade)| *grade).collect();
    let max_grade = grades_only.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_grade = grades_only.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    
    println!("最高分: {:.1} | 最低分: {:.1} | 平均分: {:.2}", 
             max_grade, min_grade, calculate_average(&grades_only));
    println!();
}

// 显示成绩排名 | Display grade ranking
fn display_ranking(students: &[StudentInfo]) {
    println!("🏆 学生总分排名:");
    
    // 计算每个学生的总分和平均分 | Calculate total and average score for each student
    let mut student_scores: Vec<(&str, f64, f64)> = students
        .iter()
        .map(|(name, _, grades)| {
            let total: f64 = grades.iter().sum();
            let average = total / SUBJECT_COUNT as f64;
            (name.as_str(), total, average)
        })
        .collect();
    
    // 按平均分排序 | Sort by average score
    student_scores.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    
    for (rank, (name, total, average)) in student_scores.iter().enumerate() {
        println!("{}. {} - 总分: {:.1} | 平均分: {:.2}", 
                 rank + 1, name, total, average);
    }
    println!();
}

// 演示切片操作 | Demonstrate slice operations
fn demonstrate_slice_operations(grades: &[f64; SUBJECT_COUNT]) {
    println!("🔧 切片操作演示:");
    
    // 不同的切片方式 | Different slicing methods
    let all_grades = &grades[..];           // 完整切片 | full slice
    let first_three = &grades[..3];         // 前三科 | first three subjects
    let last_two = &grades[3..];            // 后两科 | last two subjects
    let middle_subjects = &grades[1..4];    // 中间三科 | middle three subjects
    
    println!("完整成绩: {:?}", all_grades);
    println!("前三科成绩: {:?}", first_three);
    println!("后两科成绩: {:?}", last_two);
    println!("中间三科成绩: {:?}", middle_subjects);
    
    // 切片统计 | Slice statistics
    println!("前三科平均分: {:.2}", calculate_average(first_three));
    println!("后两科平均分: {:.2}", calculate_average(last_two));
    println!("中间三科平均分: {:.2}", calculate_average(middle_subjects));
    println!();
}

// 单元测试 | Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_average() {
        let grades = [80.0, 85.0, 90.0, 95.0, 100.0];
        let average = calculate_average(&grades);
        assert_eq!(average, 90.0);
    }
    
    #[test]
    fn test_slice_operations() {
        let grades = [80.0, 85.0, 90.0, 95.0, 100.0];
        let slice = &grades[1..4]; // [85.0, 90.0, 95.0]
        assert_eq!(slice.len(), 3);
        assert_eq!(calculate_average(slice), 90.0);
    }
    
    #[test]
    fn test_string_operations() {
        let student_name = String::from("测试学生");
        let name_slice: &str = &student_name;
        assert_eq!(name_slice.len(), 12); // UTF-8编码，中文字符占3字节
        assert_eq!(name_slice.chars().count(), 4); // 4个Unicode字符
    }
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确使用了元组来存储学生信息？| Does the project correctly use tuples to store student information?
2. 数组的使用是否体现了固定长度和同质性特点？| Does the array usage demonstrate fixed length and homogeneity characteristics?
3. 切片操作是否正确处理了数据的部分访问？| Do slice operations correctly handle partial data access?
4. 字符串类型的选择是否合理（&str vs String）？| Is the string type selection reasonable (&str vs String)?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **元组嵌套练习 | Tuple Nesting Exercise**
   - **练习描述 | Exercise Description:** 创建包含嵌套元组的复杂数据结构，如地址信息((省, 市), (街道, 门牌号))
   - **概念检查 | Concept Check:** 如何访问嵌套元组的深层元素？答案：使用多层索引如tuple.0.1
   - **学习目标 | Learning Objective:** 掌握复杂元组结构的创建和访问

2. **数组与向量对比练习 | Array vs Vector Comparison Exercise**
   - **练习描述 | Exercise Description:** 实现相同功能的数组版本和Vec版本，比较性能和使用场景
   - **概念检查 | Concept Check:** 何时选择数组，何时选择Vec？答案：编译时确定大小用数组，需要动态增长用Vec
   - **学习目标 | Learning Objective:** 理解固定长度与动态长度数据结构的区别

3. **字符串处理综合练习 | Comprehensive String Processing Exercise**
   - **练习描述 | Exercise Description:** 实现一个文本分析器，统计字符数、单词数、行数
   - **概念检查 | Concept Check:** 如何安全处理多字节UTF-8字符？答案：使用chars()迭代器而不是字节索引
   - **学习目标 | Learning Objective:** 深入理解Rust字符串的UTF-8特性

4. **切片边界安全练习 | Slice Boundary Safety Exercise**
   - **练习描述 | Exercise Description:** 实现安全的字符串切片函数，避免panic
   - **概念检查 | Concept Check:** 如何避免切片越界？答案：使用get()方法或检查边界条件
   - **学习目标 | Learning Objective:** 培养安全编程思维

5. **类型转换练习 | Type Conversion Exercise**
   - **练习描述 | Exercise Description:** 实现各种复合类型之间的相互转换
   - **概念检查 | Concept Check:** String和&str如何相互转换？答案：String.as_str()和String::from()
   - **学习目标 | Learning Objective:** 熟练掌握类型转换技巧

6. **内存布局理解练习 | Memory Layout Understanding Exercise**
   - **练习描述 | Exercise Description:** 使用std::mem分析不同复合类型的内存占用
   - **概念检查 | Concept Check:** 元组和数组在内存中如何布局？答案：连续存储，对齐到最大元素
   - **学习目标 | Learning Objective:** 理解Rust的内存管理机制

7. **模式匹配扩展练习 | Pattern Matching Extension Exercise**
   - **练习描述 | Exercise Description:** 使用模式匹配解构复杂的元组和数组结构
   - **概念检查 | Concept Check:** 如何在模式匹配中忽略不需要的元组元素？答案：使用_占位符
   - **学习目标 | Learning Objective:** 提高模式匹配的使用熟练度

## 学习资源 | Learning Resources
- [Rust官方文档 - 复合数据类型](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types) | [Rust Official Documentation - Compound Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)
- [Rust官方文档 - 字符串类型](https://doc.rust-lang.org/book/ch08-02-strings.html) | [Rust Official Documentation - String Types](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Rust by Example - 元组](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html) | [Rust by Example - Tuples](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html)
- [Rust by Example - 数组和切片](https://doc.rust-lang.org/rust-by-example/primitives/array.html) | [Rust by Example - Arrays and Slices](https://doc.rust-lang.org/rust-by-example/primitives/array.html)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解元组的特征和使用场景 | Understand tuple characteristics and usage scenarios
- [ ] 掌握数组的创建和访问方法 | Master array creation and access methods  
- [ ] 学会使用切片进行数据引用 | Learn to use slices for data referencing
- [ ] 区分&str和String的使用时机 | Distinguish when to use &str vs String
- [ ] 能够选择合适的复合数据类型 | Be able to choose appropriate compound data types
- [ ] 完成学生成绩管理系统项目 | Complete the student grade management system project
- [ ] 正确回答所有CCQs | Correctly answer all CCQs
- [ ] 理解字符串的UTF-8特性 | Understand UTF-8 characteristics of strings
- [ ] 掌握安全的切片操作方法 | Master safe slicing operation methods
- [ ] 完成至少3个扩展练习 | Complete at least 3 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释元组、数组、切片和字符串类型的核心概念及其使用场景。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain the core concepts and usage scenarios of tuples, arrays, slices, and string types to others.