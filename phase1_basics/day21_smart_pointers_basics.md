# Rust入门 - 第21天：智能指针基础 | Rust Introduction - Day 21: Smart Pointers Basics

## 学习目标 | Learning Objectives
- 理解智能指针的概念和作用 | Understand the concept and purpose of smart pointers
- 掌握Box<T>用于堆内存分配 | Master Box<T> for heap memory allocation
- 学习Rc<T>引用计数智能指针 | Learn Rc<T> reference counting smart pointer
- 了解RefCell<T>内部可变性模式 | Understand RefCell<T> interior mutability pattern
- 掌握智能指针的适用场景 | Master appropriate use cases for smart pointers
- 能够实现基本的递归数据结构 | Be able to implement basic recursive data structures

## 详细内容 | Detailed Content

### 1. 智能指针概念理解 | Smart Pointer Concepts (1.5小时 | 1.5 hours)

- **智能指针基本概念 | Smart Pointer Basic Concepts**
  
  **概念定义 | Concept Definition:**
  智能指针是拥有数据所有权并提供额外功能的数据结构，它们实现了Deref和Drop trait，能够自动管理内存并提供类似指针的行为 | Smart pointers are data structures that own data and provide additional functionality, implementing Deref and Drop traits to automatically manage memory while providing pointer-like behavior
  
  **核心特征 | Key Characteristics:**
  - 拥有数据的所有权，在超出作用域时自动清理 | Own the data and automatically clean up when going out of scope
  - 实现Deref trait允许像引用一样使用 | Implement Deref trait allowing usage like references
  - 提供额外的元数据和功能 | Provide additional metadata and functionality
  - 在编译时保证内存安全 | Guarantee memory safety at compile time
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 智能指针和普通引用的主要区别是什么？| What's the main difference between smart pointers and regular references?
     **答案 | Answer:** 智能指针拥有数据的所有权 | Smart pointers own the data - 智能指针拥有数据，可以移动和修改，而引用只是借用数据
  2. 智能指针在超出作用域时会自动清理内存吗？| Do smart pointers automatically clean up memory when going out of scope?
     **答案 | Answer:** 是 | Yes - 它们实现Drop trait，在超出作用域时自动调用清理代码
  3. 智能指针可以像普通指针一样解引用吗？| Can smart pointers be dereferenced like regular pointers?
     **答案 | Answer:** 是 | Yes - 通过实现Deref trait，可以使用*操作符解引用
  4. 所有智能指针都存储在堆上吗？| Are all smart pointers stored on the heap?
     **答案 | Answer:** 否 | No - 智能指针本身可能在栈上，但它们通常管理堆上的数据
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 智能指针基本概念演示 | Smart pointer basic concept demonstration
  use std::ops::Deref;
  
  // 简单智能指针实现 | Simple smart pointer implementation
  struct MyBox<T> {
      data: T,
  }
  
  impl<T> MyBox<T> {
      fn new(data: T) -> MyBox<T> {
          MyBox { data }
      }
  }
  
  impl<T> Deref for MyBox<T> {
      type Target = T;
      
      fn deref(&self) -> &Self::Target {
          &self.data
      }
  }
  
  fn main() {
      let smart_ptr = MyBox::new(42);
      // 通过Deref trait可以像引用一样使用 | Can be used like a reference through Deref trait
      println!("Value: {}", *smart_ptr); // 输出: Value: 42 | Output: Value: 42
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码中MyBox如何实现解引用？| How does MyBox implement dereferencing in this code?
    **答案 | Answer:** 通过实现Deref trait的deref方法 | By implementing the deref method of the Deref trait
  - 如果不实现Deref trait会发生什么？| What happens if we don't implement the Deref trait?
    **答案 | Answer:** 无法使用*操作符解引用 | Cannot use the * operator for dereferencing
  
  **常见误区检查 | Common Misconception Checks:**
  - 智能指针就是普通的结构体吗？| Are smart pointers just regular structs?
    **答案 | Answer:** 不完全是 | Not exactly - 它们是实现了特定trait（如Deref、Drop）的结构体，提供指针语义
  - 智能指针比引用更安全吗？| Are smart pointers safer than references?
    **答案 | Answer:** 不同的安全保障 | Different safety guarantees - 智能指针提供所有权管理，引用提供借用检查

### 2. Box<T>堆分配指针 | Box<T> Heap Allocation Pointer (1小时 | 1 hour)

- **Box<T>智能指针 | Box<T> Smart Pointer**
  
  **概念定义 | Concept Definition:**
  Box<T>是最简单的智能指针，用于在堆上分配数据。它拥有数据的所有权，当Box超出作用域时会自动释放堆内存 | Box<T> is the simplest smart pointer used to allocate data on the heap. It owns the data and automatically frees heap memory when the Box goes out of scope
  
  **核心特征 | Key Characteristics:**
  - 将数据存储在堆上而不是栈上 | Stores data on the heap instead of the stack
  - 提供单一所有权语义 | Provides single ownership semantics
  - 零成本抽象，没有运行时开销 | Zero-cost abstraction with no runtime overhead
  - 支持递归类型定义 | Enables recursive type definitions
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Box<T>将数据存储在哪里？| Where does Box<T> store data?
     **答案 | Answer:** 堆上 | On the heap - Box将数据分配在堆内存中
  2. Box<T>可以有多个所有者吗？| Can Box<T> have multiple owners?
     **答案 | Answer:** 否 | No - Box提供单一所有权，遵循Rust的所有权规则
  3. Box<T>适合存储大小固定的类型吗？| Is Box<T> suitable for storing fixed-size types?
     **答案 | Answer:** 是 | Yes - 但主要用于递归类型或大型数据
  4. Box<T>在超出作用域时需要手动释放内存吗？| Does Box<T> require manual memory deallocation when going out of scope?
     **答案 | Answer:** 否 | No - 自动通过Drop trait释放内存
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // Box<T>使用示例 | Box<T> usage examples
  
  // 1. 基本堆分配 | Basic heap allocation
  fn basic_box_usage() {
      let boxed_int = Box::new(42);
      println!("Boxed value: {}", boxed_int); // 自动解引用 | Auto-dereferencing
      println!("Explicit deref: {}", *boxed_int); // 显式解引用 | Explicit dereferencing
  }
  
  // 2. 递归数据结构 | Recursive data structure
  #[derive(Debug)]
  enum List {
      Cons(i32, Box<List>),
      Nil,
  }
  
  use List::{Cons, Nil};
  
  fn recursive_list() {
      let list = Cons(1, 
          Box::new(Cons(2, 
              Box::new(Cons(3, 
                  Box::new(Nil))))));
      println!("Recursive list: {:?}", list);
  }
  
  // 3. 大型数据移动优化 | Large data movement optimization
  struct LargeData {
      data: [u8; 10000], // 大型数组 | Large array
  }
  
  fn move_large_data() {
      let large_data = Box::new(LargeData { data: [0; 10000] });
      let moved_data = large_data; // 只移动指针，不复制数据 | Only moves pointer, doesn't copy data
      // println!("{:?}", large_data); // 错误：使用了移动后的值 | Error: use after move
  }
  
  fn main() {
      basic_box_usage();
      recursive_list();
      move_large_data();
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么递归类型需要使用Box？| Why do recursive types need to use Box?
    **答案 | Answer:** 因为编译器需要知道类型的大小，Box提供固定大小 | Because the compiler needs to know the type size, Box provides fixed size
  - Box的移动操作复制数据吗？| Does moving a Box copy the data?
    **答案 | Answer:** 否 | No - 只移动堆上的指针，数据保持不变
  
  **常见误区检查 | Common Misconception Checks:**
  - Box比栈分配更快吗？| Is Box faster than stack allocation?
    **答案 | Answer:** 否 | No - 堆分配通常比栈分配慢，但适用于不同场景
  - Box可以解决所有所有权问题吗？| Can Box solve all ownership problems?
    **答案 | Answer:** 否 | No - Box只提供单一所有权，不解决共享所有权问题

### 3. Rc<T>引用计数指针 | Rc<T> Reference Counting Pointer (1小时 | 1 hour)

- **Rc<T>引用计数智能指针 | Rc<T> Reference Counting Smart Pointer**
  
  **概念定义 | Concept Definition:**
  Rc<T>（Reference Counted）是一种允许多个所有者共享同一数据的智能指针，通过引用计数追踪有多少个Rc指向相同数据，当引用计数为零时自动释放内存 | Rc<T> (Reference Counted) is a smart pointer that allows multiple owners to share the same data, tracking how many Rc instances point to the same data through reference counting, automatically freeing memory when the count reaches zero
  
  **核心特征 | Key Characteristics:**
  - 启用多重所有权，多个Rc可以指向同一数据 | Enables multiple ownership, multiple Rc can point to the same data
  - 使用引用计数跟踪活跃的引用数量 | Uses reference counting to track the number of active references
  - 只能用于单线程场景 | Can only be used in single-threaded scenarios
  - 数据是不可变的，除非配合RefCell使用 | Data is immutable unless used with RefCell
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Rc<T>允许多个所有者吗？| Does Rc<T> allow multiple owners?
     **答案 | Answer:** 是 | Yes - 这是Rc的主要特性，实现共享所有权
  2. Rc<T>适用于多线程环境吗？| Is Rc<T> suitable for multi-threaded environments?
     **答案 | Answer:** 否 | No - Rc只能用于单线程，多线程需要使用Arc
  3. Rc<T>中的数据可以修改吗？| Can data in Rc<T> be modified?
     **答案 | Answer:** 默认不可以 | Not by default - 需要配合RefCell等实现内部可变性
  4. 何时释放Rc<T>管理的内存？| When is memory managed by Rc<T> freed?
     **答案 | Answer:** 引用计数为零时 | When reference count reaches zero
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // Rc<T>使用示例 | Rc<T> usage examples
  use std::rc::Rc;
  
  #[derive(Debug)]
  struct Node {
      value: i32,
      children: Vec<Rc<Node>>,
  }
  
  fn rc_basic_usage() {
      // 创建共享数据 | Create shared data
      let data = Rc::new(vec![1, 2, 3, 4, 5]);
      println!("Reference count: {}", Rc::strong_count(&data)); // 1
      
      // 克隆Rc，增加引用计数 | Clone Rc, increase reference count
      let data_ref1 = Rc::clone(&data);
      let data_ref2 = Rc::clone(&data);
      println!("Reference count: {}", Rc::strong_count(&data)); // 3
      
      // 数据共享 | Data sharing
      println!("data: {:?}", data);
      println!("data_ref1: {:?}", data_ref1);
      println!("data_ref2: {:?}", data_ref2);
      
      // 当引用超出作用域时，引用计数递减 | Reference count decreases when references go out of scope
      drop(data_ref1);
      println!("After drop data_ref1: {}", Rc::strong_count(&data)); // 2
  }
  
  fn rc_tree_structure() {
      // 使用Rc构建共享节点的树结构 | Build tree structure with shared nodes using Rc
      let leaf = Rc::new(Node {
          value: 10,
          children: vec![],
      });
      
      let branch1 = Rc::new(Node {
          value: 5,
          children: vec![Rc::clone(&leaf)],
      });
      
      let branch2 = Rc::new(Node {
          value: 15,
          children: vec![Rc::clone(&leaf)],
      });
      
      let root = Rc::new(Node {
          value: 1,
          children: vec![branch1, branch2],
      });
      
      println!("Leaf reference count: {}", Rc::strong_count(&leaf)); // 3 (leaf, branch1, branch2)
      println!("Tree: {:?}", root);
  }
  
  fn main() {
      rc_basic_usage();
      rc_tree_structure();
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - Rc::clone和直接clone的区别是什么？| What's the difference between Rc::clone and direct clone?
    **答案 | Answer:** Rc::clone只增加引用计数，不复制数据；直接clone会深拷贝数据 | Rc::clone only increases reference count without copying data; direct clone performs deep copy
  - 为什么需要查看strong_count？| Why do we need to check strong_count?
    **答案 | Answer:** 用于调试和了解内存使用情况，确保没有内存泄漏 | For debugging and understanding memory usage, ensuring no memory leaks
  
  **常见误区检查 | Common Misconception Checks:**
  - Rc::clone很昂贵吗？| Is Rc::clone expensive?
    **答案 | Answer:** 否 | No - Rc::clone只是增加引用计数，不复制实际数据
  - Rc能解决所有共享问题吗？| Can Rc solve all sharing problems?
    **答案 | Answer:** 否 | No - Rc只提供不可变共享，可变共享需要配合RefCell

### 4. RefCell<T>内部可变性 | RefCell<T> Interior Mutability (1小时 | 1 hour)

- **RefCell<T>内部可变性模式 | RefCell<T> Interior Mutability Pattern**
  
  **概念定义 | Concept Definition:**
  RefCell<T>提供内部可变性模式，允许在拥有不可变引用的情况下修改数据，通过运行时借用检查而不是编译时检查来确保借用规则 | RefCell<T> provides interior mutability pattern, allowing data modification even when you have immutable references, ensuring borrowing rules through runtime borrow checking instead of compile-time checking
  
  **核心特征 | Key Characteristics:**
  - 将借用检查从编译时延迟到运行时 | Defers borrow checking from compile time to runtime
  - 在不可变引用情况下提供可变访问 | Provides mutable access even with immutable references
  - 只能用于单线程场景 | Can only be used in single-threaded scenarios
  - 违反借用规则时会panic而不是编译错误 | Panics instead of compile error when borrowing rules are violated
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. RefCell何时检查借用规则？| When does RefCell check borrowing rules?
     **答案 | Answer:** 运行时 | At runtime - 而不是编译时
  2. RefCell允许在不可变引用下修改数据吗？| Does RefCell allow modifying data through immutable references?
     **答案 | Answer:** 是 | Yes - 这是内部可变性的核心特性
  3. RefCell违反借用规则时会发生什么？| What happens when RefCell violates borrowing rules?
     **答案 | Answer:** 运行时panic | Runtime panic - 程序会崩溃
  4. RefCell适合多线程使用吗？| Is RefCell suitable for multi-threaded use?
     **答案 | Answer:** 否 | No - 只能用于单线程场景
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // RefCell<T>使用示例 | RefCell<T> usage examples
  use std::cell::RefCell;
  use std::rc::Rc;
  
  // 内部可变性基本使用 | Basic interior mutability usage
  fn basic_refcell() {
      let data = RefCell::new(vec![1, 2, 3]);
      
      // 不可变借用 | Immutable borrow
      {
          let borrowed = data.borrow();
          println!("Data: {:?}", *borrowed);
      } // 借用在此处结束 | Borrow ends here
      
      // 可变借用 | Mutable borrow
      {
          let mut borrowed_mut = data.borrow_mut();
          borrowed_mut.push(4);
          println!("Modified data: {:?}", *borrowed_mut);
      } // 可变借用在此处结束 | Mutable borrow ends here
      
      println!("Final data: {:?}", data.borrow());
  }
  
  // Rc与RefCell组合使用 | Combining Rc with RefCell
  #[derive(Debug)]
  struct SharedCounter {
      count: RefCell<i32>,
  }
  
  impl SharedCounter {
      fn new(initial: i32) -> Self {
          SharedCounter {
              count: RefCell::new(initial),
          }
      }
      
      fn increment(&self) {
          let mut count = self.count.borrow_mut();
          *count += 1;
      }
      
      fn get(&self) -> i32 {
          *self.count.borrow()
      }
  }
  
  fn rc_refcell_combination() {
      let counter = Rc::new(SharedCounter::new(0));
      
      let counter1 = Rc::clone(&counter);
      let counter2 = Rc::clone(&counter);
      let counter3 = Rc::clone(&counter);
      
      // 多个所有者可以修改同一数据 | Multiple owners can modify the same data
      counter1.increment();
      counter2.increment();
      counter3.increment();
      
      println!("Counter value: {}", counter.get()); // 3
      println!("Reference count: {}", Rc::strong_count(&counter)); // 4
  }
  
  // 演示运行时借用检查 | Demonstrate runtime borrow checking
  fn runtime_borrow_check() {
      let data = RefCell::new(42);
      
      let _immutable1 = data.borrow();
      let _immutable2 = data.borrow(); // 多个不可变借用OK | Multiple immutable borrows OK
      
      // 下面这行会panic，因为已有不可变借用存在 | This would panic because immutable borrows exist
      // let _mutable = data.borrow_mut();
      
      println!("Runtime borrow check passed");
  }
  
  fn main() {
      basic_refcell();
      rc_refcell_combination();
      runtime_borrow_check();
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 为什么需要在不同作用域中使用RefCell的借用？| Why do we need to use RefCell borrows in different scopes?
    **答案 | Answer:** 避免同时持有多个可变借用或可变借用与不可变借用冲突 | To avoid holding multiple mutable borrows or conflicts between mutable and immutable borrows simultaneously
  - Rc<RefCell<T>>解决了什么问题？| What problem does Rc<RefCell<T>> solve?
    **答案 | Answer:** 多重所有权的可变共享数据问题 | The problem of mutably sharing data with multiple ownership
  
  **常见误区检查 | Common Misconception Checks:**
  - RefCell消除了借用检查吗？| Does RefCell eliminate borrow checking?
    **答案 | Answer:** 否 | No - 只是将检查推迟到运行时，规则依然存在
  - RefCell比普通引用更安全吗？| Is RefCell safer than regular references?
    **答案 | Answer:** 不完全是 | Not exactly - 提供了灵活性但增加了运行时panic的风险

### 5. 智能指针选择与最佳实践 | Smart Pointer Selection and Best Practices (30分钟 | 30 minutes)

- **智能指针选择指南 | Smart Pointer Selection Guide**
  
  **关键原则 | Key Principles:**
  - 单一所有权且需要堆分配时使用Box<T> | Use Box<T> for single ownership with heap allocation
  - 多重所有权的不可变数据使用Rc<T> | Use Rc<T> for multiple ownership of immutable data
  - 需要内部可变性时使用RefCell<T> | Use RefCell<T> when interior mutability is needed
  
  **实践验证问题 | Practice Verification Questions:**
  1. 什么时候选择Box而不是直接在栈上分配？| When to choose Box instead of direct stack allocation?
  2. 如何决定使用Rc<T>还是Box<T>？| How to decide between using Rc<T> or Box<T>?
  3. RefCell的运行时开销值得吗？| Is RefCell's runtime overhead worthwhile?

### 6. 知识巩固与检查 | Knowledge Consolidation and Review (30分钟 | 30 minutes)

- **综合概念检查 | Comprehensive Concept Check**
  
  **综合概念检查 | Comprehensive Concept Check:**
  1. 三种智能指针的主要区别和适用场景是什么？| What are the main differences and use cases for the three smart pointers?
  2. 如何组合使用Rc和RefCell实现可变的共享数据？| How to combine Rc and RefCell to implement mutable shared data?
  3. 智能指针相比普通引用有哪些优势和劣势？| What are the advantages and disadvantages of smart pointers compared to regular references?
  4. 在什么情况下应该避免使用智能指针？| In what situations should smart pointers be avoided?
  5. 智能指针如何影响程序的性能和内存使用？| How do smart pointers affect program performance and memory usage?

## 实践项目：简单链表实现 | Practical Project: Simple Linked List Implementation

### 目标 | Objective
使用Box<T>智能指针实现一个单向链表，展示递归数据结构的创建和智能指针的实际应用 | Implement a singly linked list using Box<T> smart pointers, demonstrating recursive data structure creation and practical smart pointer usage

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. Box<T>适合实现递归数据结构吗？| Is Box<T> suitable for implementing recursive data structures?
   **答案 | Answer:** 是，因为它提供固定大小的指针 | Yes, because it provides fixed-size pointers
2. 链表节点的所有权应该如何管理？| How should ownership of linked list nodes be managed?
   **答案 | Answer:** 每个节点拥有下一个节点的所有权 | Each node owns the next node
3. 如何安全地遍历链表而不违反所有权规则？| How to safely traverse the linked list without violating ownership rules?
   **答案 | Answer:** 使用引用进行遍历，而不是移动所有权 | Use references for traversal instead of moving ownership

### 步骤 | Steps
1. 定义链表节点结构体，使用Box指向下一个节点 | Define linked list node struct using Box to point to next node
2. 实现链表的基本操作：push、pop、显示 | Implement basic operations: push, pop, display
3. 添加迭代器支持，安全遍历链表元素 | Add iterator support for safe traversal of list elements
4. 实现内存管理，确保正确清理链表 | Implement memory management ensuring proper cleanup
5. 添加单元测试验证功能正确性 | Add unit tests to verify functionality correctness

### 示例代码 | Example Code
```rust
"""
简单链表实现 | Simple Linked List Implementation
使用Box<T>智能指针构建单向链表数据结构 | Build singly linked list data structure using Box<T> smart pointers

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- Box<T>递归数据结构 | Box<T> recursive data structures
- 所有权管理 | Ownership management
- 内存安全 | Memory safety
"""

use std::fmt::Display;

// 链表节点定义 | Linked list node definition
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

// 链表结构 | Linked list structure
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    // 创建新链表 | Create new linked list
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }
    
    // 在头部插入元素 | Insert element at head
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(), // 获取所有权并设为None | Take ownership and set to None
        });
        self.head = Some(new_node);
        self.size += 1;
    }
    
    // 移除头部元素 | Remove head element
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }
    
    // 获取长度 | Get length
    pub fn len(&self) -> usize {
        self.size
    }
    
    // 检查是否为空 | Check if empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    // 查看头部元素而不移除 | Peek at head element without removing
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Display> LinkedList<T> {
    // 显示链表内容 | Display list contents
    pub fn display(&self) {
        let mut current = &self.head;
        print!("LinkedList: [");
        let mut first = true;
        
        while let Some(node) = current {
            if !first {
                print!(", ");
            }
            print!("{}", node.data);
            current = &node.next;
            first = false;
        }
        println!("]");
    }
}

// 实现Iterator trait进行安全遍历 | Implement Iterator trait for safe traversal
pub struct IntoIter<T>(LinkedList<T>);

impl<T> LinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// 实现Drop trait确保正确清理 | Implement Drop trait for proper cleanup
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

fn main() {
    // 创建和使用链表 | Create and use linked list
    let mut list = LinkedList::new();
    
    // 添加元素 | Add elements
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    
    list.display(); // LinkedList: [4, 3, 2, 1]
    println!("Length: {}", list.len()); // Length: 4
    
    // 查看头部元素 | Peek at head
    if let Some(head) = list.peek() {
        println!("Head element: {}", head); // Head element: 4
    }
    
    // 移除元素 | Remove elements
    while let Some(data) = list.pop() {
        println!("Popped: {}", data);
    }
    
    println!("List is empty: {}", list.is_empty()); // List is empty: true
    
    // 演示迭代器 | Demonstrate iterator
    let mut iter_list = LinkedList::new();
    iter_list.push(10);
    iter_list.push(20);
    iter_list.push(30);
    
    println!("Using iterator:");
    for item in iter_list.into_iter() {
        println!("Item: {}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_push_pop() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());
        
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert!(list.is_empty());
    }
    
    #[test]
    fn test_peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        
        list.push(42);
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.len(), 1); // peek不移除元素 | peek doesn't remove element
    }
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确使用了Box<T>实现递归结构？| Does the project correctly use Box<T> for recursive structures?
2. 所有权管理是否遵循了Rust的安全规则？| Does ownership management follow Rust's safety rules?
3. 内存是否能够正确自动释放？| Is memory properly automatically freed?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **智能指针性能分析练习 | Smart Pointer Performance Analysis Exercise**
   - **练习描述 | Exercise Description:** 比较Box、Rc、RefCell在不同场景下的性能开销
   - **概念检查 | Concept Check:** 理解每种智能指针的运行时成本
   - **学习目标 | Learning Objective:** 深入理解智能指针的性能影响

2. **循环引用问题练习 | Circular Reference Problem Exercise**
   - **练习描述 | Exercise Description:** 创建并解决使用Rc时的循环引用问题
   - **概念检查 | Concept Check:** 理解Weak<T>和强引用的区别
   - **学习目标 | Learning Objective:** 学习避免内存泄漏的技巧

3. **RefCell边界案例练习 | RefCell Edge Case Exercise**
   - **练习描述 | Exercise Description:** 探索RefCell在复杂借用场景下的行为
   - **概念检查 | Concept Check:** 理解运行时借用检查的限制
   - **学习目标 | Learning Objective:** 掌握RefCell的安全使用模式

4. **智能指针组合应用练习 | Smart Pointer Combination Exercise**
   - **练习描述 | Exercise Description:** 实现需要多种智能指针组合的数据结构
   - **概念检查 | Concept Check:** 理解不同智能指针的协作方式
   - **学习目标 | Learning Objective:** 发展复杂数据结构设计能力

5. **自定义智能指针练习 | Custom Smart Pointer Exercise**
   - **练习描述 | Exercise Description:** 实现一个自定义的智能指针类型
   - **概念检查 | Concept Check:** 深入理解Deref和Drop trait
   - **学习目标 | Learning Objective:** 掌握智能指针的实现原理

6. **内存布局分析练习 | Memory Layout Analysis Exercise**
   - **练习描述 | Exercise Description:** 分析不同智能指针的内存布局和占用
   - **概念检查 | Concept Check:** 理解堆栈内存的使用模式
   - **学习目标 | Learning Objective:** 深入理解内存管理概念

7. **二叉树实现练习 | Binary Tree Implementation Exercise**
   - **练习描述 | Exercise Description:** 使用智能指针实现完整的二叉搜索树
   - **概念检查 | Concept Check:** 综合应用多种智能指针
   - **学习目标 | Learning Objective:** 提高复杂数据结构实现能力

## 学习资源 | Learning Resources
- [Rust官方文档 - 智能指针](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) | [Rust Official Documentation - Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rust Reference - 智能指针类型](https://doc.rust-lang.org/reference/types/pointer.html) | [Rust Reference - Pointer Types](https://doc.rust-lang.org/reference/types/pointer.html)
- [std::boxed::Box文档](https://doc.rust-lang.org/std/boxed/struct.Box.html) | [std::boxed::Box Documentation](https://doc.rust-lang.org/std/boxed/struct.Box.html)
- [std::rc::Rc文档](https://doc.rust-lang.org/std/rc/struct.Rc.html) | [std::rc::Rc Documentation](https://doc.rust-lang.org/std/rc/struct.Rc.html)
- [std::cell::RefCell文档](https://doc.rust-lang.org/std/cell/struct.RefCell.html) | [std::cell::RefCell Documentation](https://doc.rust-lang.org/std/cell/struct.RefCell.html)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解智能指针的基本概念和作用 | Understand basic smart pointer concepts and purposes
- [ ] 掌握Box<T>的使用场景和方法 | Master Box<T> use cases and methods
- [ ] 了解Rc<T>引用计数机制 | Understand Rc<T> reference counting mechanism
- [ ] 掌握RefCell<T>内部可变性模式 | Master RefCell<T> interior mutability pattern
- [ ] 能够选择合适的智能指针类型 | Able to choose appropriate smart pointer types
- [ ] 完成链表实践项目 | Complete linked list practical project
- [ ] 正确回答所有概念检查问题 | Correctly answer all concept checking questions
- [ ] 理解智能指针的性能影响 | Understand performance implications of smart pointers
- [ ] 避免常见的智能指针使用错误 | Avoid common smart pointer usage mistakes
- [ ] 至少完成3个扩展练习 | Complete at least 3 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释Box、Rc、RefCell的区别和适用场景。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain the differences and use cases of Box, Rc, and RefCell to others.