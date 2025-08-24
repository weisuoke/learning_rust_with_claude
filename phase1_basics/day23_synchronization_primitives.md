# Rust入门 - 第23天：同步原语 | Rust Introduction - Day 23: Synchronization Primitives

## 学习目标 | Learning Objectives

- 理解线程同步的必要性和基本概念 | Understand the necessity and basic concepts of thread synchronization
- 掌握Mutex<T>互斥锁的使用方法 | Master the usage of Mutex<T> mutex locks
- 学会Arc<T>原子引用计数的工作原理 | Learn how Arc<T> atomic reference counting works
- 了解通道(mpsc)的消息传递机制 | Understand the message passing mechanism of channels (mpsc)
- 实践生产者-消费者模式 | Practice producer-consumer pattern
- 能够选择合适的同步原语解决并发问题 | Be able to choose appropriate synchronization primitives to solve concurrency problems

## 详细内容 | Detailed Content

### 1. 互斥锁Mutex<T>基础 | Mutex<T> Basics (1小时 | 1 hour)

- **Mutex<T>的概念与作用 | Concept and Purpose of Mutex<T>**
  
  **概念定义 | Concept Definition:**
  Mutex（Mutual Exclusion）是一种同步原语，确保在任何时刻只有一个线程可以访问某些数据。 | Mutex (Mutual Exclusion) is a synchronization primitive that ensures only one thread can access certain data at any given time.
  
  **核心特征 | Key Characteristics:**
  - 提供数据的独占访问 | Provides exclusive access to data
  - 线程必须获取锁才能访问数据 | Threads must acquire lock to access data
  - 自动处理锁的释放 | Automatically handles lock release
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Mutex可以被多个线程同时持有吗？| Can a Mutex be held by multiple threads simultaneously?
     **答案 | Answer:** 否 | No - Mutex确保独占访问，同时只能有一个线程持有 | Mutex ensures exclusive access, only one thread can hold it at a time
  2. 如果线程忘记释放Mutex锁会发生什么？| What happens if a thread forgets to release a Mutex lock?
     **答案 | Answer:** Rust自动释放 | Rust automatically releases - 锁的生命周期由RAII管理 | Lock lifetime is managed by RAII
  3. lock()方法返回什么类型？| What type does the lock() method return?
     **答案 | Answer:** Result<MutexGuard<T>, PoisonError> - 包装了数据的守卫 | A guard wrapping the data
  4. 可以在不同线程间直接共享Mutex<T>吗？| Can Mutex<T> be directly shared between different threads?
     **答案 | Answer:** 不能 | No - 需要Arc<Mutex<T>>来实现线程间共享 | Need Arc<Mutex<T>> for inter-thread sharing
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::sync::Mutex;
  use std::thread;
  
  // 基本Mutex使用 | Basic Mutex usage
  fn basic_mutex_example() {
      let mutex = Mutex::new(0);
      
      // 获取锁并修改数据 | Acquire lock and modify data
      {
          let mut data = mutex.lock().unwrap();
          *data += 1;
          println!("数据值: {} | Data value: {}", *data, *data);
      } // 锁在此处自动释放 | Lock automatically released here
      
      // 验证数据已修改 | Verify data has been modified
      let data = mutex.lock().unwrap();
      println!("最终数据值: {} | Final data value: {}", *data, *data);
  }
  
  // Mutex守卫的生命周期 | Mutex guard lifetime
  fn mutex_guard_lifetime() {
      let mutex = Mutex::new(vec![1, 2, 3]);
      
      {
          let mut guard = mutex.lock().unwrap();
          guard.push(4);
          println!("向量长度: {} | Vector length: {}", guard.len(), guard.len());
          // guard在此作用域结束时被销毁，锁被释放 | guard destroyed here, lock released
      }
      
      // 可以再次获取锁 | Can acquire lock again
      let guard = mutex.lock().unwrap();
      println!("最终向量: {:?} | Final vector: {:?}", *guard, *guard);
  }
  
  // Mutex的错误处理 | Mutex error handling
  fn mutex_error_handling() {
      let mutex = Mutex::new(42);
      
      match mutex.lock() {
          Ok(guard) => {
              println!("成功获取锁，数据: {} | Successfully acquired lock, data: {}", *guard, *guard);
          },
          Err(poisoned) => {
              // 处理中毒锁 | Handle poisoned lock
              let guard = poisoned.into_inner();
              println!("锁已中毒，但仍可访问数据: {} | Lock poisoned but data still accessible: {}", *guard, *guard);
          }
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - basic_mutex_example中锁何时被释放？| When is the lock released in basic_mutex_example?
    **答案 | Answer:** 在内层作用域结束时，MutexGuard被销毁时释放 | When the inner scope ends and MutexGuard is destroyed
  - 如果在持有锁期间线程panic会怎样？| What happens if a thread panics while holding a lock?
    **答案 | Answer:** 锁会被标记为中毒(poisoned)，但仍可通过into_inner()访问数据 | Lock becomes poisoned but data still accessible via into_inner()
  
  **常见误区检查 | Common Misconception Checks:**
  - Mutex只能保护数字类型的数据吗？| Can Mutex only protect numeric data?
    **答案 | Answer:** 否，Mutex<T>可以保护任何类型T的数据 | No, Mutex<T> can protect data of any type T
  - 使用Mutex会影响程序性能吗？| Does using Mutex affect program performance?
    **答案 | Answer:** 有一定开销，但对于需要同步的场景是必要的 | Has overhead, but necessary for scenarios requiring synchronization

- **Arc<Mutex<T>>模式 | Arc<Mutex<T>> Pattern**
  
  **概念定义 | Concept Definition:**
  Arc<Mutex<T>>结合了原子引用计数和互斥锁，允许多个线程安全地共享和修改数据。 | Arc<Mutex<T>> combines atomic reference counting with mutex locks, allowing multiple threads to safely share and modify data.
  
  **核心特征 | Key Characteristics:**
  - Arc提供线程安全的共享所有权 | Arc provides thread-safe shared ownership
  - Mutex确保数据访问的同步 | Mutex ensures synchronized data access
  - 是Rust中最常用的共享状态模式 | Most commonly used shared state pattern in Rust
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 为什么需要Arc包装Mutex？| Why do we need Arc to wrap Mutex?
     **答案 | Answer:** Arc允许多个线程拥有Mutex的所有权 | Arc allows multiple threads to own the Mutex
  2. Rc<Mutex<T>>可以替代Arc<Mutex<T>>吗？| Can Rc<Mutex<T>> replace Arc<Mutex<T>>?
     **答案 | Answer:** 不能 | No - Rc不是线程安全的 | Rc is not thread-safe
  3. Arc::clone()是深拷贝吗？| Is Arc::clone() a deep copy?
     **答案 | Answer:** 不是 | No - 只是增加引用计数，数据本身不被复制 | Only increments reference count, data itself is not copied
  4. 当最后一个Arc被销毁时会发生什么？| What happens when the last Arc is destroyed?
     **答案 | Answer:** 包装的数据被自动清理 | Wrapped data is automatically cleaned up

  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::sync::{Arc, Mutex};
  use std::thread;
  use std::time::Duration;
  
  // Arc<Mutex<T>>基本用法 | Basic Arc<Mutex<T>> usage
  fn arc_mutex_basic() {
      let counter = Arc::new(Mutex::new(0));
      let mut handles = vec![];
      
      // 创建多个线程共享计数器 | Create multiple threads sharing counter
      for i in 0..5 {
          let counter_clone = Arc::clone(&counter);
          let handle = thread::spawn(move || {
              let thread_id = i;
              
              // 每个线程增加计数器10次 | Each thread increments counter 10 times
              for _ in 0..10 {
                  let mut num = counter_clone.lock().unwrap();
                  *num += 1;
                  println!("线程 {} 增加计数器到 {} | Thread {} incremented counter to {}", 
                          thread_id, *num, thread_id, *num);
                  
                  // 模拟一些工作 | Simulate some work
                  thread::sleep(Duration::from_millis(10));
              }
          });
          handles.push(handle);
      }
      
      // 等待所有线程完成 | Wait for all threads to complete
      for handle in handles {
          handle.join().unwrap();
      }
      
      println!("最终计数器值: {} | Final counter value: {}", 
              *counter.lock().unwrap(), *counter.lock().unwrap());
  }
  
  // 复杂数据结构的共享 | Sharing complex data structures
  fn arc_mutex_complex_data() {
      use std::collections::HashMap;
      
      let shared_data = Arc::new(Mutex::new(HashMap::new()));
      let mut handles = vec![];
      
      // 创建写线程 | Create writer threads
      for i in 0..3 {
          let data_clone = Arc::clone(&shared_data);
          let handle = thread::spawn(move || {
              let mut map = data_clone.lock().unwrap();
              map.insert(format!("线程{} | thread{}", i, i), i * 10);
              println!("线程 {} 插入数据 | Thread {} inserted data", i, i);
          });
          handles.push(handle);
      }
      
      // 等待写操作完成 | Wait for write operations to complete
      for handle in handles {
          handle.join().unwrap();
      }
      
      // 读取并打印所有数据 | Read and print all data
      let map = shared_data.lock().unwrap();
      println!("共享数据内容 | Shared data contents:");
      for (key, value) in map.iter() {
          println!("  {}: {}", key, value);
      }
  }
  ```

### 2. 原子引用计数Arc<T>详解 | Atomic Reference Counting Arc<T> Details (1小时 | 1 hour)

- **Arc<T>的工作原理 | How Arc<T> Works**
  
  **概念定义 | Concept Definition:**
  Arc（Atomically Reference Counted）是一个线程安全的引用计数智能指针，允许多个所有者共享同一数据。 | Arc (Atomically Reference Counted) is a thread-safe reference counting smart pointer that allows multiple owners to share the same data.
  
  **核心特征 | Key Characteristics:**
  - 使用原子操作进行引用计数 | Uses atomic operations for reference counting
  - 允许多线程间安全共享数据 | Allows safe data sharing between multiple threads
  - 数据是不可变的，除非使用内部可变性 | Data is immutable unless using interior mutability
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Arc和Rc的主要区别是什么？| What's the main difference between Arc and Rc?
     **答案 | Answer:** Arc是线程安全的，Rc不是 | Arc is thread-safe, Rc is not
  2. Arc<T>中的数据可以直接修改吗？| Can data in Arc<T> be directly modified?
     **答案 | Answer:** 不能 | No - 需要配合Mutex或其他内部可变性类型 | Need to combine with Mutex or other interior mutability types
  3. 多个Arc指向同一数据时，数据存储在哪里？| Where is data stored when multiple Arcs point to the same data?
     **答案 | Answer:** 在堆上，被所有Arc共享 | On the heap, shared by all Arcs
  4. Arc的引用计数何时减少？| When does Arc's reference count decrease?
     **答案 | Answer:** 当Arc被销毁时 | When Arc is destroyed
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::sync::Arc;
  use std::thread;
  
  // Arc基本使用 | Basic Arc usage
  fn arc_basic_example() {
      let data = Arc::new(vec![1, 2, 3, 4, 5]);
      
      println!("原始引用计数 | Original reference count: {}", Arc::strong_count(&data));
      
      let data_clone1 = Arc::clone(&data);
      let data_clone2 = Arc::clone(&data);
      
      println!("克隆后引用计数 | Reference count after cloning: {}", Arc::strong_count(&data));
      
      // 在不同作用域中使用 | Use in different scopes
      {
          let data_clone3 = Arc::clone(&data);
          println!("内部作用域引用计数 | Inner scope reference count: {}", Arc::strong_count(&data));
      } // data_clone3在此处被销毁 | data_clone3 destroyed here
      
      println!("作用域后引用计数 | Reference count after scope: {}", Arc::strong_count(&data));
  }
  
  // 多线程Arc共享 | Multi-threaded Arc sharing
  fn arc_multithreaded_sharing() {
      let numbers = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
      let mut handles = vec![];
      
      // 创建多个线程读取共享数据 | Create multiple threads to read shared data
      for i in 0..3 {
          let numbers_clone = Arc::clone(&numbers);
          let handle = thread::spawn(move || {
              let thread_id = i;
              let sum: i32 = numbers_clone.iter().sum();
              println!("线程 {} 计算总和: {} | Thread {} calculated sum: {}", 
                      thread_id, sum, thread_id, sum);
              
              // 访问特定元素 | Access specific elements
              if let Some(value) = numbers_clone.get(thread_id) {
                  println!("线程 {} 访问元素[{}]: {} | Thread {} accessed element[{}]: {}", 
                          thread_id, thread_id, value, thread_id, thread_id, value);
              }
          });
          handles.push(handle);
      }
      
      // 等待所有线程完成 | Wait for all threads to complete
      for handle in handles {
          handle.join().unwrap();
      }
      
      println!("最终引用计数 | Final reference count: {}", Arc::strong_count(&numbers));
  }
  
  // Arc与弱引用 | Arc with weak references
  fn arc_weak_references() {
      let strong_arc = Arc::new("Hello, Arc!");
      
      println!("强引用计数 | Strong reference count: {}", Arc::strong_count(&strong_arc));
      println!("弱引用计数 | Weak reference count: {}", Arc::weak_count(&strong_arc));
      
      // 创建弱引用 | Create weak reference
      let weak_ref1 = Arc::downgrade(&strong_arc);
      let weak_ref2 = Arc::downgrade(&strong_arc);
      
      println!("创建弱引用后 | After creating weak references:");
      println!("强引用计数 | Strong reference count: {}", Arc::strong_count(&strong_arc));
      println!("弱引用计数 | Weak reference count: {}", Arc::weak_count(&strong_arc));
      
      // 尝试升级弱引用 | Try to upgrade weak reference
      if let Some(upgraded) = weak_ref1.upgrade() {
          println!("成功升级弱引用: {} | Successfully upgraded weak reference: {}", *upgraded, *upgraded);
      }
      
      // 销毁强引用 | Drop strong reference
      drop(strong_arc);
      
      // 尝试再次升级弱引用 | Try to upgrade weak reference again
      if let Some(_upgraded) = weak_ref2.upgrade() {
          println!("升级成功 | Upgrade successful");
      } else {
          println!("弱引用升级失败，数据已被销毁 | Weak reference upgrade failed, data destroyed");
      }
  }
  ```

### 3. 消息传递与通道mpsc | Message Passing and Channels mpsc (1小时 | 1 hour)

- **通道的基本概念 | Basic Channel Concepts**
  
  **概念定义 | Concept Definition:**
  通道(channel)是一种消息传递机制，允许线程之间安全地发送数据，遵循"不要通过共享内存来通信，而要通过通信来共享内存"的原则。 | Channels are a message passing mechanism that allows threads to send data safely, following the principle "Don't communicate by sharing memory; share memory by communicating."
  
  **核心特征 | Key Characteristics:**
  - 发送者(Sender)和接收者(Receiver)分离 | Separation of Sender and Receiver
  - 支持多生产者单消费者模式 | Supports multiple producer single consumer pattern
  - 提供同步和异步两种模式 | Provides both synchronous and asynchronous modes
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. mpsc代表什么？| What does mpsc stand for?
     **答案 | Answer:** Multiple Producer Single Consumer - 多生产者单消费者 | Multiple Producer Single Consumer
  2. 发送者可以有多个，接收者呢？| Can there be multiple senders and receivers?
     **答案 | Answer:** 可以有多个发送者，但只能有一个接收者 | Can have multiple senders, but only one receiver
  3. 当所有发送者都被销毁时会发生什么？| What happens when all senders are dropped?
     **答案 | Answer:** 接收者的recv()会返回错误 | Receiver's recv() will return an error
  4. 通道是否有容量限制？| Do channels have capacity limits?
     **答案 | Answer:** 无界通道无限制，有界通道有限制 | Unbounded channels have no limit, bounded channels do
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::sync::mpsc;
  use std::thread;
  use std::time::Duration;
  
  // 基本通道使用 | Basic channel usage
  fn basic_channel_example() {
      let (sender, receiver) = mpsc::channel();
      
      // 在新线程中发送数据 | Send data in new thread
      thread::spawn(move || {
          let message = "Hello from thread!";
          println!("发送消息: {} | Sending message: {}", message, message);
          sender.send(message).unwrap();
      });
      
      // 在主线程中接收数据 | Receive data in main thread
      let received = receiver.recv().unwrap();
      println!("接收到消息: {} | Received message: {}", received, received);
  }
  
  // 多个发送者示例 | Multiple senders example
  fn multiple_senders_example() {
      let (sender, receiver) = mpsc::channel();
      
      // 创建多个发送者线程 | Create multiple sender threads
      for i in 0..3 {
          let sender_clone = sender.clone();
          thread::spawn(move || {
              let message = format!("来自线程 {} 的消息 | Message from thread {}", i, i);
              thread::sleep(Duration::from_millis(i * 100));
              sender_clone.send(message).unwrap();
          });
      }
      
      // 销毁原始发送者 | Drop original sender
      drop(sender);
      
      // 接收所有消息 | Receive all messages
      for received in receiver {
          println!("收到: {} | Received: {}", received, received);
      }
      
      println!("所有发送者已关闭，接收完成 | All senders closed, receiving complete");
  }
  
  // 发送复杂数据类型 | Sending complex data types
  fn complex_data_channel() {
      #[derive(Debug)]
      struct Task {
          id: u32,
          description: String,
          priority: u8,
      }
      
      let (sender, receiver) = mpsc::channel();
      
      // 发送任务 | Send tasks
      thread::spawn(move || {
          let tasks = vec![
              Task { id: 1, description: "处理用户请求 | Process user request".to_string(), priority: 1 },
              Task { id: 2, description: "更新数据库 | Update database".to_string(), priority: 2 },
              Task { id: 3, description: "发送邮件 | Send email".to_string(), priority: 1 },
          ];
          
          for task in tasks {
              println!("发送任务: {:?} | Sending task: {:?}", task, task);
              sender.send(task).unwrap();
              thread::sleep(Duration::from_millis(100));
          }
      });
      
      // 接收并处理任务 | Receive and process tasks
      while let Ok(task) = receiver.recv() {
          println!("处理任务: ID={}, 描述={}, 优先级={} | Processing task: ID={}, Description={}, Priority={}", 
                  task.id, task.description, task.priority, task.id, task.description, task.priority);
          
          // 模拟任务处理时间 | Simulate task processing time
          thread::sleep(Duration::from_millis(task.priority as u64 * 50));
          println!("任务 {} 完成 | Task {} completed", task.id, task.id);
      }
  }
  ```

### 4. 生产者-消费者模式实现 | Producer-Consumer Pattern Implementation (1小时 | 1 hour)

- **生产者-消费者模式概念 | Producer-Consumer Pattern Concepts**
  
  **概念定义 | Concept Definition:**
  生产者-消费者模式是一种经典的并发设计模式，生产者线程生成数据，消费者线程处理数据，通过缓冲区解耦两者。 | The producer-consumer pattern is a classic concurrent design pattern where producer threads generate data and consumer threads process data, decoupled through a buffer.
  
  **解决的问题 | Problems It Solves:**
  - 生产和消费速度不匹配的问题 | Mismatch between production and consumption rates
  - 线程间的协调和同步问题 | Thread coordination and synchronization issues
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 生产者和消费者必须同时运行吗？| Must producers and consumers run simultaneously?
     **答案 | Answer:** 不必须 | Not necessary - 缓冲区允许异步运行 | Buffer allows asynchronous operation
  2. 多个生产者可以同时工作吗？| Can multiple producers work simultaneously?
     **答案 | Answer:** 可以 | Yes - 这是模式的常见扩展 | This is a common extension of the pattern
  3. 缓冲区满了时生产者应该怎么办？| What should producers do when buffer is full?
     **答案 | Answer:** 等待或丢弃数据，取决于策略 | Wait or drop data, depending on strategy
  4. 消费者如何知道没有更多数据？| How do consumers know there's no more data?
     **答案 | Answer:** 通过通道关闭信号或特殊标记 | Through channel closing signal or special markers
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::sync::mpsc;
  use std::thread;
  use std::time::Duration;
  use std::sync::{Arc, Mutex};
  
  // 简单的生产者-消费者模式 | Simple producer-consumer pattern
  fn simple_producer_consumer() {
      let (sender, receiver) = mpsc::channel();
      
      // 生产者线程 | Producer thread
      let producer = thread::spawn(move || {
          for i in 0..10 {
              let item = format!("商品 {} | Item {}", i, i);
              println!("生产: {} | Produced: {}", item, item);
              sender.send(item).unwrap();
              thread::sleep(Duration::from_millis(100)); // 生产时间 | Production time
          }
          println!("生产者完成 | Producer finished");
      });
      
      // 消费者线程 | Consumer thread
      let consumer = thread::spawn(move || {
          for received in receiver {
              println!("消费: {} | Consumed: {}", received, received);
              thread::sleep(Duration::from_millis(150)); // 消费时间 | Consumption time
          }
          println!("消费者完成 | Consumer finished");
      });
      
      // 等待所有线程完成 | Wait for all threads to complete
      producer.join().unwrap();
      consumer.join().unwrap();
  }
  
  // 多生产者单消费者模式 | Multiple producers single consumer pattern
  fn multiple_producers_single_consumer() {
      let (sender, receiver) = mpsc::channel();
      let mut producers = vec![];
      
      // 创建多个生产者 | Create multiple producers
      for producer_id in 0..3 {
          let sender_clone = sender.clone();
          let producer = thread::spawn(move || {
              for i in 0..5 {
                  let item = format!("P{}-商品{} | P{}-Item{}", producer_id, i, producer_id, i);
                  println!("生产者 {} 生产: {} | Producer {} produced: {}", 
                          producer_id, item, producer_id, item);
                  sender_clone.send(item).unwrap();
                  thread::sleep(Duration::from_millis(200));
              }
              println!("生产者 {} 完成 | Producer {} finished", producer_id, producer_id);
          });
          producers.push(producer);
      }
      
      // 销毁原始发送者，确保通道会关闭 | Drop original sender to ensure channel closes
      drop(sender);
      
      // 消费者线程 | Consumer thread
      let consumer = thread::spawn(move || {
          let mut item_count = 0;
          for received in receiver {
              item_count += 1;
              println!("消费第 {} 个商品: {} | Consumed item {}: {}", 
                      item_count, received, item_count, received);
              thread::sleep(Duration::from_millis(100));
          }
          println!("消费者完成，共处理 {} 个商品 | Consumer finished, processed {} items", 
                  item_count, item_count);
      });
      
      // 等待所有线程完成 | Wait for all threads to complete
      for producer in producers {
          producer.join().unwrap();
      }
      consumer.join().unwrap();
  }
  
  // 带统计信息的生产者-消费者 | Producer-consumer with statistics
  fn producer_consumer_with_stats() {
      let (sender, receiver) = mpsc::channel();
      let stats = Arc::new(Mutex::new((0u32, 0u32))); // (produced, consumed)
      
      // 生产者线程 | Producer thread
      let stats_producer = Arc::clone(&stats);
      let producer = thread::spawn(move || {
          for i in 0..20 {
              let item = i * i; // 生产平方数 | Produce square numbers
              sender.send(item).unwrap();
              
              // 更新生产统计 | Update production statistics
              let mut stat = stats_producer.lock().unwrap();
              stat.0 += 1;
              
              if stat.0 % 5 == 0 {
                  println!("已生产 {} 个商品 | Produced {} items", stat.0, stat.0);
              }
              
              thread::sleep(Duration::from_millis(50));
          }
      });
      
      // 消费者线程 | Consumer thread
      let stats_consumer = Arc::clone(&stats);
      let consumer = thread::spawn(move || {
          for item in receiver {
              // 处理商品（计算平方根） | Process item (calculate square root)
              let processed = (item as f64).sqrt();
              
              // 更新消费统计 | Update consumption statistics
              let mut stat = stats_consumer.lock().unwrap();
              stat.1 += 1;
              
              println!("处理商品 {}: 平方根 = {:.2} | Processed item {}: square root = {:.2}", 
                      item, processed, item, processed);
              
              if stat.1 % 5 == 0 {
                  println!("已消费 {} 个商品 | Consumed {} items", stat.1, stat.1);
              }
              
              thread::sleep(Duration::from_millis(80));
          }
      });
      
      // 等待完成并显示最终统计 | Wait for completion and show final statistics
      producer.join().unwrap();
      consumer.join().unwrap();
      
      let final_stats = stats.lock().unwrap();
      println!("最终统计 | Final statistics:");
      println!("总生产数量: {} | Total produced: {}", final_stats.0, final_stats.0);
      println!("总消费数量: {} | Total consumed: {}", final_stats.1, final_stats.1);
  }
  ```

### 5. 高级同步模式 | Advanced Synchronization Patterns (1小时 | 1 hour)

- **条件变量与复杂同步 | Condition Variables and Complex Synchronization**
  
  **概念定义 | Concept Definition:**
  条件变量是一种同步原语，允许线程等待特定条件成立，常与互斥锁配合使用实现复杂的同步逻辑。 | Condition variables are synchronization primitives that allow threads to wait for specific conditions to be met, often used with mutexes to implement complex synchronization logic.
  
  **核心特征 | Key Characteristics:**
  - 提供wait和notify机制 | Provides wait and notify mechanisms
  - 避免忙等待，提高效率 | Avoids busy waiting, improves efficiency
  - 支持多线程协调 | Supports multi-thread coordination
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 条件变量可以单独使用吗？| Can condition variables be used alone?
     **答案 | Answer:** 不可以 | No - 必须与Mutex一起使用 | Must be used with Mutex
  2. wait()方法会释放锁吗？| Does the wait() method release the lock?
     **答案 | Answer:** 是 | Yes - 等待时释放，被唤醒时重新获取 | Releases while waiting, reacquires when woken up
  3. notify_one和notify_all的区别是什么？| What's the difference between notify_one and notify_all?
     **答案 | Answer:** notify_one唤醒一个线程，notify_all唤醒所有等待线程 | notify_one wakes one thread, notify_all wakes all waiting threads
  4. 虚假唤醒是什么？| What is spurious wakeup?
     **答案 | Answer:** 线程在条件未满足时被意外唤醒 | Thread is unexpectedly woken up when condition is not met
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::sync::{Arc, Mutex, Condvar};
  use std::thread;
  use std::time::Duration;
  
  // 条件变量基本使用 | Basic condition variable usage
  fn basic_condvar_example() {
      let pair = Arc::new((Mutex::new(false), Condvar::new()));
      let pair_clone = Arc::clone(&pair);
      
      // 等待线程 | Waiting thread
      let waiter = thread::spawn(move || {
          let (lock, cvar) = &*pair_clone;
          let mut ready = lock.lock().unwrap();
          
          println!("等待线程开始等待... | Waiting thread starts waiting...");
          while !*ready {
              ready = cvar.wait(ready).unwrap();
          }
          println!("等待线程被唤醒！| Waiting thread woken up!");
      });
      
      // 通知线程 | Notifying thread
      let notifier = thread::spawn(move || {
          thread::sleep(Duration::from_millis(1000));
          
          let (lock, cvar) = &*pair;
          let mut ready = lock.lock().unwrap();
          *ready = true;
          
          println!("通知线程设置条件并唤醒 | Notifying thread sets condition and wakes up");
          cvar.notify_one();
      });
      
      waiter.join().unwrap();
      notifier.join().unwrap();
  }
  
  // 生产者消费者与条件变量 | Producer-consumer with condition variables
  fn producer_consumer_condvar() {
      let buffer_data = Arc::new((Mutex::new(Vec::new()), Condvar::new(), Condvar::new()));
      const BUFFER_SIZE: usize = 5;
      
      // 生产者线程 | Producer thread
      let buffer_producer = Arc::clone(&buffer_data);
      let producer = thread::spawn(move || {
          let (lock, not_full, not_empty) = &*buffer_producer;
          
          for i in 0..10 {
              let mut buffer = lock.lock().unwrap();
              
              // 等待缓冲区不满 | Wait for buffer not full
              while buffer.len() >= BUFFER_SIZE {
                  println!("缓冲区已满，生产者等待... | Buffer full, producer waiting...");
                  buffer = not_full.wait(buffer).unwrap();
              }
              
              // 生产物品 | Produce item
              buffer.push(format!("物品{} | Item{}", i, i));
              println!("生产了物品{}，缓冲区大小: {} | Produced item{}, buffer size: {}", 
                      i, buffer.len(), i, buffer.len());
              
              // 通知消费者 | Notify consumer
              not_empty.notify_one();
              
              thread::sleep(Duration::from_millis(200));
          }
      });
      
      // 消费者线程 | Consumer thread
      let buffer_consumer = Arc::clone(&buffer_data);
      let consumer = thread::spawn(move || {
          let (lock, not_full, not_empty) = &*buffer_consumer;
          
          for _ in 0..10 {
              let mut buffer = lock.lock().unwrap();
              
              // 等待缓冲区不为空 | Wait for buffer not empty
              while buffer.is_empty() {
                  println!("缓冲区为空，消费者等待... | Buffer empty, consumer waiting...");
                  buffer = not_empty.wait(buffer).unwrap();
              }
              
              // 消费物品 | Consume item
              let item = buffer.remove(0);
              println!("消费了{}，缓冲区大小: {} | Consumed {}, buffer size: {}", 
                      item, buffer.len(), item, buffer.len());
              
              // 通知生产者 | Notify producer
              not_full.notify_one();
              
              thread::sleep(Duration::from_millis(300));
          }
      });
      
      producer.join().unwrap();
      consumer.join().unwrap();
  }
  ```

### 6. 同步原语选择指南 | Synchronization Primitives Selection Guide (30分钟 | 30 minutes)

- **选择合适的同步机制 | Choosing the Right Synchronization Mechanism**
  
  **关键原则 | Key Principles:**
  - 优先选择消息传递而非共享状态 | Prefer message passing over shared state
  - 根据使用场景选择合适的原语 | Choose appropriate primitives based on use cases
  - 考虑性能和复杂性的平衡 | Balance performance and complexity
  
  **实践验证问题 | Practice Verification Questions:**
  1. 何时使用Arc<Mutex<T>>而非通道？| When to use Arc<Mutex<T>> instead of channels?
     **答案 | Answer:** 需要多个线程读写共享状态时 | When multiple threads need to read/write shared state
  2. 通道适合什么样的场景？| What scenarios are channels suitable for?
     **答案 | Answer:** 任务分发、事件处理、流水线处理 | Task distribution, event handling, pipeline processing
  3. 如何避免死锁？| How to avoid deadlocks?
     **答案 | Answer:** 统一锁获取顺序、使用超时、避免嵌套锁 | Consistent lock ordering, use timeouts, avoid nested locks

## 实践项目：生产者-消费者任务处理系统 | Practical Project: Producer-Consumer Task Processing System

### 目标 | Objective
创建一个多线程任务处理系统，演示生产者-消费者模式、互斥锁、原子引用计数和通道的综合应用。 | Create a multi-threaded task processing system demonstrating comprehensive application of producer-consumer pattern, mutex locks, atomic reference counting, and channels.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. Arc<Mutex<T>>如何实现线程间数据共享？| How does Arc<Mutex<T>> enable data sharing between threads?
   **答案 | Answer:** Arc提供共享所有权，Mutex确保独占访问 | Arc provides shared ownership, Mutex ensures exclusive access
2. 通道的发送者和接收者有什么关系？| What's the relationship between channel senders and receivers?
   **答案 | Answer:** 多个发送者可以向一个接收者发送数据 | Multiple senders can send data to one receiver
3. 生产者-消费者模式解决什么问题？| What problems does the producer-consumer pattern solve?
   **答案 | Answer:** 解耦生产和消费，处理速度差异和缓冲 | Decouples production and consumption, handles speed differences and buffering

### 步骤 | Steps
1. 设计任务结构和优先级系统 | Design task structure and priority system
2. 实现任务生产者，支持不同类型的任务 | Implement task producers supporting different task types
3. 创建任务消费者线程池，并行处理任务 | Create task consumer thread pool for parallel processing
4. 添加系统监控和统计功能 | Add system monitoring and statistics functionality
5. 实现优雅关闭和资源清理 | Implement graceful shutdown and resource cleanup

### 示例代码 | Example Code
```rust
"""
生产者-消费者任务处理系统 | Producer-Consumer Task Processing System
这个项目演示了Rust中同步原语的综合应用 | This project demonstrates comprehensive application of synchronization primitives in Rust

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- Arc<Mutex<T>>共享状态管理 | Arc<Mutex<T>> shared state management
- mpsc通道消息传递 | mpsc channel message passing
- 生产者-消费者模式 | Producer-consumer pattern
"""

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum TaskType {
    DataProcessing(String),
    FileOperation(String),
    NetworkRequest(String),
    Calculation(i32, i32),
}

#[derive(Debug, Clone)]
struct Task {
    id: u64,
    task_type: TaskType,
    priority: u8, // 1-5, 5 is highest priority
    created_at: Instant,
}

impl Task {
    fn new(id: u64, task_type: TaskType, priority: u8) -> Self {
        Self {
            id,
            task_type,
            priority,
            created_at: Instant::now(),
        }
    }
    
    fn execute(&self) -> String {
        // 模拟任务执行时间 | Simulate task execution time
        let execution_time = match self.priority {
            1 => Duration::from_millis(1000),
            2 => Duration::from_millis(800),
            3 => Duration::from_millis(600),
            4 => Duration::from_millis(400),
            5 => Duration::from_millis(200),
            _ => Duration::from_millis(500),
        };
        
        thread::sleep(execution_time);
        
        match &self.task_type {
            TaskType::DataProcessing(data) => {
                format!("处理数据: {} | Processed data: {}", data, data)
            },
            TaskType::FileOperation(filename) => {
                format!("文件操作: {} | File operation: {}", filename, filename)
            },
            TaskType::NetworkRequest(url) => {
                format!("网络请求: {} | Network request: {}", url, url)
            },
            TaskType::Calculation(a, b) => {
                let result = a + b;
                format!("计算结果: {} + {} = {} | Calculation result: {} + {} = {}", 
                       a, b, result, a, b, result)
            },
        }
    }
}

#[derive(Debug)]
struct TaskStats {
    total_produced: u64,
    total_consumed: u64,
    tasks_by_type: HashMap<String, u64>,
    tasks_by_priority: HashMap<u8, u64>,
    total_processing_time: Duration,
}

impl TaskStats {
    fn new() -> Self {
        Self {
            total_produced: 0,
            total_consumed: 0,
            tasks_by_type: HashMap::new(),
            tasks_by_priority: HashMap::new(),
            total_processing_time: Duration::new(0, 0),
        }
    }
    
    fn record_produced(&mut self, task: &Task) {
        self.total_produced += 1;
        
        let type_name = match &task.task_type {
            TaskType::DataProcessing(_) => "DataProcessing",
            TaskType::FileOperation(_) => "FileOperation",
            TaskType::NetworkRequest(_) => "NetworkRequest",
            TaskType::Calculation(_, _) => "Calculation",
        };
        
        *self.tasks_by_type.entry(type_name.to_string()).or_insert(0) += 1;
        *self.tasks_by_priority.entry(task.priority).or_insert(0) += 1;
    }
    
    fn record_consumed(&mut self, task: &Task, processing_time: Duration) {
        self.total_consumed += 1;
        self.total_processing_time += processing_time;
    }
    
    fn print_summary(&self) {
        println!("\n=== 任务处理系统统计报告 | Task Processing System Statistics Report ===");
        println!("总生产任务数 | Total produced tasks: {}", self.total_produced);
        println!("总消费任务数 | Total consumed tasks: {}", self.total_consumed);
        println!("总处理时间 | Total processing time: {:.2}s", self.total_processing_time.as_secs_f64());
        
        if self.total_consumed > 0 {
            let avg_time = self.total_processing_time.as_millis() as f64 / self.total_consumed as f64;
            println!("平均处理时间 | Average processing time: {:.2}ms", avg_time);
        }
        
        println!("\n按类型分布 | Distribution by type:");
        for (task_type, count) in &self.tasks_by_type {
            println!("  {}: {}", task_type, count);
        }
        
        println!("\n按优先级分布 | Distribution by priority:");
        for priority in 1..=5 {
            let count = self.tasks_by_priority.get(&priority).unwrap_or(&0);
            println!("  优先级 {} | Priority {}: {}", priority, priority, count);
        }
    }
}

struct TaskProcessingSystem {
    task_sender: mpsc::Sender<Task>,
    stats: Arc<Mutex<TaskStats>>,
    worker_count: usize,
}

impl TaskProcessingSystem {
    fn new(worker_count: usize) -> Self {
        let (task_sender, task_receiver) = mpsc::channel();
        let stats = Arc::new(Mutex::new(TaskStats::new()));
        let task_receiver = Arc::new(Mutex::new(task_receiver));
        
        // 创建工作线程 | Create worker threads
        for worker_id in 0..worker_count {
            let receiver = Arc::clone(&task_receiver);
            let stats_clone = Arc::clone(&stats);
            
            thread::spawn(move || {
                loop {
                    let task = {
                        let receiver = receiver.lock().unwrap();
                        receiver.recv()
                    };
                    
                    match task {
                        Ok(task) => {
                            println!("工作线程 {} 开始处理任务 {} (优先级: {}) | Worker {} starts processing task {} (priority: {})", 
                                    worker_id, task.id, task.priority, worker_id, task.id, task.priority);
                            
                            let start_time = Instant::now();
                            let result = task.execute();
                            let processing_time = start_time.elapsed();
                            
                            println!("工作线程 {} 完成任务 {}: {} | Worker {} completed task {}: {}", 
                                    worker_id, task.id, result, worker_id, task.id, result);
                            
                            // 更新统计信息 | Update statistics
                            let mut stats = stats_clone.lock().unwrap();
                            stats.record_consumed(&task, processing_time);
                        },
                        Err(_) => {
                            println!("工作线程 {} 结束 | Worker {} shutting down", worker_id, worker_id);
                            break;
                        }
                    }
                }
            });
        }
        
        Self {
            task_sender,
            stats,
            worker_count,
        }
    }
    
    fn submit_task(&self, task: Task) -> Result<(), mpsc::SendError<Task>> {
        // 记录生产统计 | Record production statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.record_produced(&task);
        }
        
        self.task_sender.send(task)
    }
    
    fn get_stats(&self) -> TaskStats {
        let stats = self.stats.lock().unwrap();
        TaskStats {
            total_produced: stats.total_produced,
            total_consumed: stats.total_consumed,
            tasks_by_type: stats.tasks_by_type.clone(),
            tasks_by_priority: stats.tasks_by_priority.clone(),
            total_processing_time: stats.total_processing_time,
        }
    }
    
    fn shutdown(self) {
        println!("关闭任务处理系统... | Shutting down task processing system...");
        drop(self.task_sender); // 关闭发送端，使工作线程退出 | Drop sender to make workers exit
        
        // 等待一段时间让工作线程完成当前任务 | Wait for workers to finish current tasks
        thread::sleep(Duration::from_millis(1000));
        
        let stats = self.stats.lock().unwrap();
        stats.print_summary();
    }
}

// 任务生产者 | Task producer
struct TaskProducer {
    producer_id: u32,
    next_task_id: u64,
}

impl TaskProducer {
    fn new(producer_id: u32) -> Self {
        Self {
            producer_id,
            next_task_id: producer_id as u64 * 1000, // 避免ID冲突 | Avoid ID conflicts
        }
    }
    
    fn produce_tasks(&mut self, system: &TaskProcessingSystem, count: usize) {
        for _ in 0..count {
            let task = self.create_random_task();
            
            println!("生产者 {} 创建任务 {} | Producer {} created task {}", 
                    self.producer_id, task.id, self.producer_id, task.id);
            
            if let Err(e) = system.submit_task(task) {
                println!("生产者 {} 提交任务失败: {:?} | Producer {} failed to submit task: {:?}", 
                        self.producer_id, e, self.producer_id, e);
                break;
            }
            
            // 随机生产间隔 | Random production interval
            let delay = Duration::from_millis(50 + (self.producer_id as u64 * 50));
            thread::sleep(delay);
        }
        
        println!("生产者 {} 完成任务生产 | Producer {} finished task production", 
                self.producer_id, self.producer_id);
    }
    
    fn create_random_task(&mut self) -> Task {
        let task_id = self.next_task_id;
        self.next_task_id += 1;
        
        let task_type = match task_id % 4 {
            0 => TaskType::DataProcessing(format!("数据集_{} | dataset_{}", task_id, task_id)),
            1 => TaskType::FileOperation(format!("文件_{}.txt | file_{}.txt", task_id, task_id)),
            2 => TaskType::NetworkRequest(format!("https://api.example.com/{}", task_id)),
            _ => TaskType::Calculation((task_id % 100) as i32, ((task_id + 1) % 100) as i32),
        };
        
        let priority = ((task_id % 5) + 1) as u8; // 优先级 1-5 | Priority 1-5
        
        Task::new(task_id, task_type, priority)
    }
}

fn main() {
    println!("启动任务处理系统 | Starting task processing system");
    
    // 创建任务处理系统 | Create task processing system
    let system = TaskProcessingSystem::new(4); // 4个工作线程 | 4 worker threads
    
    // 创建生产者线程 | Create producer threads
    let mut producer_handles = vec![];
    
    for producer_id in 0..3 {
        let system_clone = TaskProcessingSystem {
            task_sender: system.task_sender.clone(),
            stats: Arc::clone(&system.stats),
            worker_count: system.worker_count,
        };
        
        let handle = thread::spawn(move || {
            let mut producer = TaskProducer::new(producer_id);
            producer.produce_tasks(&system_clone, 8); // 每个生产者生产8个任务 | Each producer creates 8 tasks
        });
        
        producer_handles.push(handle);
    }
    
    // 定期打印统计信息 | Periodically print statistics
    let stats_monitor = Arc::clone(&system.stats);
    let monitor_handle = thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_secs(2));
            let stats = stats_monitor.lock().unwrap();
            println!("\n--- 第 {} 次统计 | Statistics #{} ---", i + 1, i + 1);
            println!("已生产: {}, 已消费: {} | Produced: {}, Consumed: {}", 
                    stats.total_produced, stats.total_consumed,
                    stats.total_produced, stats.total_consumed);
        }
    });
    
    // 等待所有生产者完成 | Wait for all producers to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }
    
    println!("\n所有生产者已完成，等待消费者处理剩余任务... | All producers finished, waiting for consumers to process remaining tasks...");
    
    // 等待一段时间让消费者处理完剩余任务 | Wait for consumers to process remaining tasks
    thread::sleep(Duration::from_secs(5));
    
    // 停止监控线程 | Stop monitoring thread
    // 注意：在实际应用中应该使用更优雅的方式停止监控线程
    // Note: In real applications, should use more graceful way to stop monitoring thread
    
    // 关闭系统并显示最终统计 | Shutdown system and show final statistics
    system.shutdown();
    
    println!("\n系统关闭完成 | System shutdown complete");
}

// 辅助函数：压力测试 | Helper function: stress test
#[allow(dead_code)]
fn stress_test() {
    println!("开始压力测试 | Starting stress test");
    
    let system = TaskProcessingSystem::new(8); // 8个工作线程 | 8 worker threads
    let start_time = Instant::now();
    
    // 创建大量生产者 | Create many producers
    let mut handles = vec![];
    
    for producer_id in 0..10 {
        let system_clone = TaskProcessingSystem {
            task_sender: system.task_sender.clone(),
            stats: Arc::clone(&system.stats),
            worker_count: system.worker_count,
        };
        
        let handle = thread::spawn(move || {
            let mut producer = TaskProducer::new(producer_id);
            producer.produce_tasks(&system_clone, 50); // 每个生产者生产50个任务 | Each producer creates 50 tasks
        });
        
        handles.push(handle);
    }
    
    // 等待所有生产者完成 | Wait for all producers to finish
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 等待处理完成 | Wait for processing to complete
    thread::sleep(Duration::from_secs(10));
    
    let total_time = start_time.elapsed();
    println!("压力测试完成，总用时: {:.2}s | Stress test completed, total time: {:.2}s", 
            total_time.as_secs_f64());
    
    system.shutdown();
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确使用了Arc<Mutex<T>>来管理共享状态？| Does the project correctly use Arc<Mutex<T>> to manage shared state?
2. 通道是否有效实现了生产者-消费者通信？| Do channels effectively implement producer-consumer communication?
3. 多线程工作是否能正确协调和同步？| Can multi-threaded work be properly coordinated and synchronized?
4. 系统是否能优雅地关闭并清理资源？| Can the system shut down gracefully and clean up resources?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **死锁预防练习 | Deadlock Prevention Exercise**
   - **练习描述 | Exercise Description:** 创建可能产生死锁的场景，然后实现预防机制
   - **概念检查 | Concept Check:** 什么情况下会发生死锁？| Under what circumstances does deadlock occur?
   - **学习目标 | Learning Objective:** 深入理解死锁成因和预防方法

2. **通道类型对比练习 | Channel Types Comparison Exercise**
   - **练习描述 | Exercise Description:** 实现和对比同步通道与异步通道的性能差异
   - **概念检查 | Concept Check:** 同步通道和异步通道的区别是什么？| What's the difference between sync and async channels?
   - **学习目标 | Learning Objective:** 理解不同通道类型的适用场景

3. **性能优化练习 | Performance Optimization Exercise**
   - **练习描述 | Exercise Description:** 分析和优化多线程程序的性能瓶颈
   - **概念检查 | Concept Check:** 如何识别并发程序的性能瓶颈？| How to identify performance bottlenecks in concurrent programs?
   - **学习目标 | Learning Objective:** 掌握并发程序的性能调优技巧

4. **复杂同步场景练习 | Complex Synchronization Scenario Exercise**
   - **练习描述 | Exercise Description:** 实现读者-写者模式或哲学家就餐问题
   - **概念检查 | Concept Check:** 如何处理多种不同的访问模式？| How to handle multiple different access patterns?
   - **学习目标 | Learning Objective:** 应用同步原语解决经典并发问题

5. **错误处理与恢复练习 | Error Handling and Recovery Exercise**
   - **练习描述 | Exercise Description:** 在多线程环境中实现健壮的错误处理机制
   - **概念检查 | Concept Check:** 线程中的错误如何传播和处理？| How are errors propagated and handled in threads?
   - **学习目标 | Learning Objective:** 提高多线程程序的可靠性

6. **监控和调试练习 | Monitoring and Debugging Exercise**
   - **练习描述 | Exercise Description:** 为多线程程序添加详细的监控和调试功能
   - **概念检查 | Concept Check:** 如何有效调试多线程程序？| How to effectively debug multi-threaded programs?
   - **学习目标 | Learning Objective:** 掌握多线程程序的调试技巧

7. **实际应用场景练习 | Real-world Application Scenario Exercise**
   - **练习描述 | Exercise Description:** 设计一个Web服务器的请求处理系统，使用多线程处理并发请求
   - **概念检查 | Concept Check:** 在实际应用中如何合理使用同步原语？| How to reasonably use synchronization primitives in real applications?
   - **学习目标 | Learning Objective:** 将理论知识应用到实际项目开发中

## 学习资源 | Learning Resources
- [Rust官方文档 - 同步原语](https://doc.rust-lang.org/std/sync/) | [Rust Official Documentation - Synchronization Primitives]
- [std::sync::Mutex文档](https://doc.rust-lang.org/std/sync/struct.Mutex.html) | [std::sync::Mutex Documentation]
- [std::sync::Arc文档](https://doc.rust-lang.org/std/sync/struct.Arc.html) | [std::sync::Arc Documentation]
- [std::sync::mpsc文档](https://doc.rust-lang.org/std/sync/mpsc/) | [std::sync::mpsc Documentation]
- [Rust并发编程模式](https://github.com/rust-unofficial/patterns) | [Rust Concurrency Patterns]

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解Mutex<T>互斥锁的工作原理 | Understand how Mutex<T> mutex locks work
- [ ] 掌握Arc<T>原子引用计数的使用 | Master the usage of Arc<T> atomic reference counting
- [ ] 熟练使用Arc<Mutex<T>>模式 | Proficiently use Arc<Mutex<T>> pattern
- [ ] 理解通道的消息传递机制 | Understand channel message passing mechanisms
- [ ] 能够实现生产者-消费者模式 | Can implement producer-consumer pattern
- [ ] 了解条件变量的高级同步 | Understand advanced synchronization with condition variables
- [ ] 完成任务处理系统项目 | Complete task processing system project
- [ ] 能够选择合适的同步原语 | Can choose appropriate synchronization primitives
- [ ] 理解死锁预防和性能优化 | Understand deadlock prevention and performance optimization
- [ ] 完成至少3个扩展练习 | Complete at least 3 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释互斥锁、原子引用计数、通道和生产者-消费者模式的核心概念。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain core concepts of mutex locks, atomic reference counting, channels, and producer-consumer patterns to others.