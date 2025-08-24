# Rust入门 - 第22天：并发编程入门 | Rust Introduction - Day 22: Concurrency Introduction

## 学习目标 | Learning Objectives

- 理解并发编程的基本概念和重要性 | Understand basic concepts and importance of concurrent programming
- 掌握Rust中线程的创建和管理 | Master thread creation and management in Rust
- 学会使用thread::spawn创建新线程 | Learn to use thread::spawn to create new threads
- 了解线程间的数据共享问题和挑战 | Understand data sharing problems and challenges between threads
- 实践基本的多线程程序编写 | Practice writing basic multi-threaded programs
- 理解Rust所有权系统在并发中的作用 | Understand the role of Rust's ownership system in concurrency

## 详细内容 | Detailed Content

### 1. 并发编程基础概念 | Basic Concurrency Concepts (1小时 | 1 hour)

- **并发 vs 并行 | Concurrency vs Parallelism**
  
  **概念定义 | Concept Definition:**
  并发是指程序能够处理多个任务，但不一定同时执行；并行是指多个任务真正同时执行。 | Concurrency means a program can handle multiple tasks, but not necessarily execute them simultaneously; parallelism means multiple tasks actually execute at the same time.
  
  **核心特征 | Key Characteristics:**
  - 并发关注任务的组织和调度 | Concurrency focuses on task organization and scheduling
  - 并行关注任务的同时执行 | Parallelism focuses on simultaneous task execution
  - Rust提供了安全的并发编程抽象 | Rust provides safe concurrency programming abstractions
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 并发程序必须在多核CPU上运行吗？| Must concurrent programs run on multi-core CPUs?
     **答案 | Answer:** 否 | No - 并发程序可以在单核CPU上通过时间片轮转运行 | Concurrent programs can run on single-core CPUs through time-slicing
  2. 两个任务并发执行意味着它们同时开始和结束吗？| Does concurrent execution of two tasks mean they start and end simultaneously?
     **答案 | Answer:** 否 | No - 并发任务可以在不同时间开始和结束 | Concurrent tasks can start and end at different times
  3. Rust的所有权系统能帮助避免并发问题吗？| Can Rust's ownership system help avoid concurrency problems?
     **答案 | Answer:** 是 | Yes - 所有权系统在编译时防止数据竞争 | The ownership system prevents data races at compile time
  4. 单线程程序可以实现并发吗？| Can single-threaded programs achieve concurrency?
     **答案 | Answer:** 可以 | Yes - 通过异步编程或协程可以实现单线程并发 | Through asynchronous programming or coroutines
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 顺序执行示例 | Sequential execution example
  fn sequential_example() {
      println!("任务1开始 | Task 1 starts");
      std::thread::sleep(std::time::Duration::from_millis(1000));
      println!("任务1完成 | Task 1 completes");
      
      println!("任务2开始 | Task 2 starts");
      std::thread::sleep(std::time::Duration::from_millis(1000));
      println!("任务2完成 | Task 2 completes");
  }
  
  // 基本并发概念演示 | Basic concurrency concept demonstration
  fn concurrent_example() {
      println!("主线程开始 | Main thread starts");
      
      // 创建新线程 | Create new thread
      let handle = std::thread::spawn(|| {
          println!("子线程开始工作 | Child thread starts working");
          std::thread::sleep(std::time::Duration::from_millis(1000));
          println!("子线程完成 | Child thread completes");
      });
      
      println!("主线程继续执行其他工作 | Main thread continues with other work");
      std::thread::sleep(std::time::Duration::from_millis(500));
      
      // 等待子线程完成 | Wait for child thread to complete
      handle.join().unwrap();
      println!("所有工作完成 | All work completed");
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - sequential_example()和concurrent_example()的执行时间有什么不同？| What's the difference in execution time between sequential_example() and concurrent_example()?
    **答案 | Answer:** sequential_example()需要2秒，concurrent_example()约1.5秒 | sequential_example() takes 2 seconds, concurrent_example() takes about 1.5 seconds
  - 为什么并发版本更快？| Why is the concurrent version faster?
    **答案 | Answer:** 主线程和子线程可以同时工作，减少了总等待时间 | Main thread and child thread can work simultaneously, reducing total wait time
  
  **常见误区检查 | Common Misconception Checks:**
  - 并发总是比顺序执行更快吗？| Is concurrency always faster than sequential execution?
    **答案 | Answer:** 不一定，并发有开销，对于简单任务可能更慢 | Not necessarily, concurrency has overhead and might be slower for simple tasks
  - 更多线程总是意味着更好的性能吗？| Do more threads always mean better performance?
    **答案 | Answer:** 否，线程过多会导致上下文切换开销和资源竞争 | No, too many threads can cause context switching overhead and resource contention

- **线程的概念 | Thread Concept**
  
  **概念定义 | Concept Definition:**
  线程是操作系统能够进行运算调度的最小单位，是进程中的实际运作单位。 | A thread is the smallest unit of processing that can be performed by an operating system scheduler, and is the actual operating unit within a process.
  
  **核心特征 | Key Characteristics:**
  - 每个线程有独立的栈空间 | Each thread has its own stack space
  - 线程共享进程的堆内存和代码段 | Threads share process heap memory and code segments
  - 线程切换比进程切换开销更小 | Thread switching has less overhead than process switching
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 线程是否拥有独立的内存空间？| Do threads have independent memory space?
     **答案 | Answer:** 部分独立 | Partially - 线程有独立栈但共享堆内存 | Threads have independent stacks but share heap memory
  2. 创建线程是否比创建进程更轻量？| Is creating threads lighter than creating processes?
     **答案 | Answer:** 是 | Yes - 线程创建和切换开销更小 | Thread creation and switching has lower overhead
  3. 多个线程可以同时修改同一个变量吗？| Can multiple threads modify the same variable simultaneously?
     **答案 | Answer:** 理论上可以但不安全 | Theoretically yes but unsafe - 需要同步机制保护 | Need synchronization mechanisms for protection
  4. 主线程结束时所有子线程也会结束吗？| Do all child threads end when the main thread ends?
     **答案 | Answer:** 在Rust中是的 | Yes in Rust - 主线程结束会终止整个程序 | Main thread ending terminates the entire program

### 2. thread::spawn 基础用法 | Basic Usage of thread::spawn (1小时 | 1 hour)

- **创建线程的基本语法 | Basic Thread Creation Syntax**
  
  **概念定义 | Concept Definition:**
  thread::spawn是Rust标准库提供的创建新线程的函数，接受一个闭包作为参数。 | thread::spawn is a function provided by Rust's standard library to create new threads, taking a closure as parameter.
  
  **核心特征 | Key Characteristics:**
  - 返回JoinHandle用于线程管理 | Returns JoinHandle for thread management
  - 接受闭包或函数指针作为线程代码 | Accepts closures or function pointers as thread code
  - 新线程在独立的栈上运行 | New threads run on independent stacks
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. thread::spawn返回什么类型？| What type does thread::spawn return?
     **答案 | Answer:** JoinHandle<T> - 用于管理和等待线程 | Used to manage and wait for threads
  2. 线程函数可以有返回值吗？| Can thread functions have return values?
     **答案 | Answer:** 可以 | Yes - 通过JoinHandle::join()获取返回值 | Retrieved through JoinHandle::join()
  3. 不调用join()会怎样？| What happens if join() is not called?
     **答案 | Answer:** 子线程可能被提前终止 | Child threads might be terminated early
  4. 线程创建会立即开始执行吗？| Do threads start executing immediately upon creation?
     **答案 | Answer:** 是的 | Yes - 线程创建后立即开始执行 | Threads start executing immediately after creation
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::thread;
  use std::time::Duration;
  
  // 基本线程创建 | Basic thread creation
  fn basic_thread_example() {
      println!("主线程开始 | Main thread starts");
      
      // 创建新线程 | Create new thread
      let handle = thread::spawn(|| {
          for i in 1..=5 {
              println!("子线程: {}", i);
              thread::sleep(Duration::from_millis(500));
          }
      });
      
      // 主线程工作 | Main thread work
      for i in 1..=3 {
          println!("主线程: {}", i);
          thread::sleep(Duration::from_millis(800));
      }
      
      // 等待子线程完成 | Wait for child thread to complete
      handle.join().unwrap();
      println!("所有线程完成 | All threads completed");
  }
  
  // 带返回值的线程 | Thread with return value
  fn thread_with_return_value() {
      let handle = thread::spawn(|| {
          let mut sum = 0;
          for i in 1..=100 {
              sum += i;
          }
          sum // 返回计算结果 | Return calculation result
      });
      
      // 获取线程返回值 | Get thread return value
      let result = handle.join().unwrap();
      println!("计算结果: {} | Calculation result: {}", result, result);
  }
  
  // 多个线程示例 | Multiple threads example
  fn multiple_threads_example() {
      let mut handles = vec![];
      
      // 创建多个线程 | Create multiple threads
      for i in 0..3 {
          let handle = thread::spawn(move || {
              println!("线程 {} 开始工作 | Thread {} starts working", i, i);
              thread::sleep(Duration::from_millis(1000));
              println!("线程 {} 完成工作 | Thread {} completes work", i, i);
              i * 2 // 返回值 | Return value
          });
          handles.push(handle);
      }
      
      // 等待所有线程完成 | Wait for all threads to complete
      for (index, handle) in handles.into_iter().enumerate() {
          let result = handle.join().unwrap();
          println!("线程 {} 返回值: {} | Thread {} return value: {}", index, result, index, result);
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 在basic_thread_example中，主线程和子线程的输出是否总是按固定顺序？| In basic_thread_example, are the outputs of main and child threads always in fixed order?
    **答案 | Answer:** 不是，线程调度不确定，输出顺序可能变化 | No, thread scheduling is non-deterministic, output order may vary
  - thread_with_return_value中如果不调用join()能获取返回值吗？| Can we get the return value without calling join() in thread_with_return_value?
    **答案 | Answer:** 不能，必须通过join()获取返回值 | No, must get return value through join()

### 3. 线程数据传递与move | Thread Data Passing and move (1小时 | 1 hour)

- **move闭包与数据所有权 | move Closures and Data Ownership**
  
  **概念定义 | Concept Definition:**
  move关键字强制闭包获取所有使用变量的所有权，这在线程编程中确保数据安全传递。 | The move keyword forces closures to take ownership of all used variables, ensuring safe data passing in thread programming.
  
  **核心特征 | Key Characteristics:**
  - 防止悬垂指针问题 | Prevents dangling pointer issues
  - 确保线程间数据独立性 | Ensures data independence between threads
  - 编译时检查所有权转移 | Compile-time ownership transfer checking
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 不使用move的闭包可以访问外部变量吗？| Can closures without move access external variables?
     **答案 | Answer:** 可以借用，但在线程中可能不安全 | Can borrow, but may be unsafe in threads
  2. move闭包会复制所有捕获的变量吗？| Do move closures copy all captured variables?
     **答案 | Answer:** 转移所有权，对于Copy类型是复制 | Transfer ownership, copying for Copy types
  3. 使用move后原变量还能使用吗？| Can original variables be used after move?
     **答案 | Answer:** 不能，所有权已转移 | No, ownership has been transferred
  4. 为什么线程编程中经常需要move？| Why is move often needed in thread programming?
     **答案 | Answer:** 确保数据在线程生命周期内有效 | Ensures data validity during thread lifetime
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::thread;
  
  // move闭包示例 | move closure example
  fn move_closure_example() {
      let data = vec![1, 2, 3, 4, 5];
      let name = String::from("数据处理线程 | Data processing thread");
      
      let handle = thread::spawn(move || {
          println!("{} 开始处理 | {} starts processing", name, name);
          let sum: i32 = data.iter().sum();
          println!("数据总和: {} | Data sum: {}", sum, sum);
          sum
      });
      
      // 注意：这里不能再使用data和name | Note: cannot use data and name here
      // println!("{:?}", data); // 编译错误 | Compile error
      
      let result = handle.join().unwrap();
      println!("线程返回值: {} | Thread return value: {}", result, result);
  }
  
  // 数据克隆示例 | Data cloning example
  fn cloning_data_example() {
      let original_data = vec![10, 20, 30];
      
      // 克隆数据给线程使用 | Clone data for thread use
      let data_for_thread = original_data.clone();
      
      let handle = thread::spawn(move || {
          let doubled: Vec<i32> = data_for_thread.iter().map(|x| x * 2).collect();
          doubled
      });
      
      // 原数据仍可使用 | Original data still usable
      println!("原始数据: {:?} | Original data: {:?}", original_data, original_data);
      
      let result = handle.join().unwrap();
      println!("处理结果: {:?} | Processing result: {:?}", result, result);
  }
  
  // 复合数据传递 | Complex data passing
  fn complex_data_passing() {
      #[derive(Debug)]
      struct Task {
          id: u32,
          description: String,
          priority: i32,
      }
      
      let tasks = vec![
          Task { id: 1, description: "任务1 | Task 1".to_string(), priority: 5 },
          Task { id: 2, description: "任务2 | Task 2".to_string(), priority: 3 },
          Task { id: 3, description: "任务3 | Task 3".to_string(), priority: 1 },
      ];
      
      let handle = thread::spawn(move || {
          // 按优先级排序 | Sort by priority
          let mut sorted_tasks = tasks;
          sorted_tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
          
          println!("排序后的任务 | Sorted tasks:");
          for task in &sorted_tasks {
              println!("  ID: {}, 描述: {}, 优先级: {} | ID: {}, Description: {}, Priority: {}", 
                      task.id, task.description, task.priority,
                      task.id, task.description, task.priority);
          }
          
          sorted_tasks.len()
      });
      
      let count = handle.join().unwrap();
      println!("处理了 {} 个任务 | Processed {} tasks", count, count);
  }
  ```

### 4. 线程同步与join | Thread Synchronization and join (1小时 | 1 hour)

- **JoinHandle的作用 | Role of JoinHandle**
  
  **概念定义 | Concept Definition:**
  JoinHandle是线程的句柄，用于等待线程完成并获取其返回值，确保线程同步。 | JoinHandle is a thread handle used to wait for thread completion and retrieve its return value, ensuring thread synchronization.
  
  **核心特征 | Key Characteristics:**
  - 提供join()方法等待线程完成 | Provides join() method to wait for thread completion
  - 可以获取线程的返回值 | Can retrieve thread's return value
  - 确保线程执行顺序 | Ensures thread execution order
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. join()是阻塞操作吗？| Is join() a blocking operation?
     **答案 | Answer:** 是 | Yes - 会等待线程完成才继续 | Waits for thread completion before continuing
  2. 可以多次调用同一个JoinHandle的join()吗？| Can join() be called multiple times on the same JoinHandle?
     **答案 | Answer:** 不可以 | No - join()会消费JoinHandle | join() consumes the JoinHandle
  3. 不调用join()的线程称为什么？| What are threads called when join() is not called?
     **答案 | Answer:** 分离线程或守护线程 | Detached or daemon threads
  4. join()返回什么类型？| What type does join() return?
     **答案 | Answer:** Result<T, Box<dyn Any + Send>> - T是线程返回类型 | T is the thread return type
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::thread;
  use std::time::Duration;
  
  // 基本join使用 | Basic join usage
  fn basic_join_example() {
      println!("开始创建线程 | Starting to create threads");
      
      let handle1 = thread::spawn(|| {
          for i in 1..=3 {
              println!("线程1: 步骤 {} | Thread 1: Step {}", i, i);
              thread::sleep(Duration::from_millis(500));
          }
          "线程1完成 | Thread 1 completed"
      });
      
      let handle2 = thread::spawn(|| {
          for i in 1..=5 {
              println!("线程2: 迭代 {} | Thread 2: Iteration {}", i, i);
              thread::sleep(Duration::from_millis(300));
          }
          42
      });
      
      println!("等待线程完成 | Waiting for threads to complete");
      
      // 按顺序等待线程 | Wait for threads in order
      let result1 = handle1.join().unwrap();
      let result2 = handle2.join().unwrap();
      
      println!("结果1: {} | Result 1: {}", result1, result1);
      println!("结果2: {} | Result 2: {}", result2, result2);
  }
  
  // 线程错误处理 | Thread error handling
  fn thread_error_handling() {
      let handle = thread::spawn(|| {
          thread::sleep(Duration::from_millis(100));
          // 模拟可能的错误 | Simulate possible error
          if true {
              panic!("线程发生错误 | Thread error occurred");
          }
          "正常完成 | Normal completion"
      });
      
      // 处理线程panic | Handle thread panic
      match handle.join() {
          Ok(result) => println!("线程成功: {} | Thread success: {}", result, result),
          Err(e) => println!("线程失败: {:?} | Thread failed: {:?}", e, e),
      }
  }
  
  // 条件等待示例 | Conditional waiting example
  fn conditional_waiting_example() {
      let handles: Vec<_> = (0..3).map(|i| {
          thread::spawn(move || {
              let work_time = (i + 1) * 500;
              println!("线程 {} 工作 {}ms | Thread {} working for {}ms", i, work_time, i, work_time);
              thread::sleep(Duration::from_millis(work_time as u64));
              println!("线程 {} 完成 | Thread {} completed", i, i);
              i
          })
      }).collect();
      
      println!("所有线程已启动，等待完成 | All threads started, waiting for completion");
      
      // 收集所有结果 | Collect all results
      let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
      println!("所有线程结果: {:?} | All thread results: {:?}", results, results);
  }
  ```

### 5. 线程间数据共享问题 | Data Sharing Problems Between Threads (1小时 | 1 hour)

- **数据竞争与不安全共享 | Data Races and Unsafe Sharing**
  
  **概念定义 | Concept Definition:**
  数据竞争是指多个线程同时访问同一内存位置，且至少有一个线程进行写操作，而没有适当的同步机制。 | Data race occurs when multiple threads access the same memory location simultaneously, with at least one thread performing a write operation, without proper synchronization mechanisms.
  
  **核心特征 | Key Characteristics:**
  - 导致程序行为不确定 | Causes non-deterministic program behavior
  - 难以调试和重现 | Difficult to debug and reproduce
  - Rust在编译时预防数据竞争 | Rust prevents data races at compile time
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. 两个线程同时读取同一个变量是数据竞争吗？| Is it a data race when two threads read the same variable simultaneously?
     **答案 | Answer:** 不是 | No - 只有读写或写写同时发生才是数据竞争 | Only read-write or write-write simultaneity constitutes data race
  2. Rust如何防止数据竞争？| How does Rust prevent data races?
     **答案 | Answer:** 通过所有权系统和借用检查器 | Through ownership system and borrow checker
  3. 共享数据在线程间传递需要什么特质？| What traits are needed for sharing data between threads?
     **答案 | Answer:** Send和Sync特质 | Send and Sync traits
  4. 使用Arc<Mutex<T>>是否完全消除了数据竞争？| Does using Arc<Mutex<T>> completely eliminate data races?
     **答案 | Answer:** 是的 | Yes - 提供了安全的共享访问机制 | Provides safe shared access mechanism
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  use std::thread;
  use std::sync::{Arc, Mutex};
  use std::time::Duration;
  
  // 错误的数据共享尝试 | Incorrect data sharing attempt
  // 注意：这段代码不会编译 | Note: This code won't compile
  /*
  fn unsafe_sharing_attempt() {
      let mut counter = 0;
      
      let handle = thread::spawn(|| {
          // 编译错误：无法捕获可变引用 | Compile error: cannot capture mutable reference
          for _ in 0..1000 {
              counter += 1;
          }
      });
      
      for _ in 0..1000 {
          counter += 1;
      }
      
      handle.join().unwrap();
      println!("Counter: {}", counter);
  }
  */
  
  // 演示需要同步的问题 | Demonstrate the need for synchronization
  fn demonstrate_sync_need() {
      // 使用Arc来共享数据 | Use Arc to share data
      let counter = Arc::new(Mutex::new(0));
      let mut handles = vec![];
      
      for i in 0..10 {
          let counter_clone = Arc::clone(&counter);
          let handle = thread::spawn(move || {
              let thread_id = i;
              println!("线程 {} 开始 | Thread {} starts", thread_id, thread_id);
              
              // 模拟一些工作 | Simulate some work
              thread::sleep(Duration::from_millis(100));
              
              // 安全地访问共享数据 | Safely access shared data
              let mut num = counter_clone.lock().unwrap();
              *num += 1;
              println!("线程 {} 递增计数器到 {} | Thread {} incremented counter to {}", 
                      thread_id, *num, thread_id, *num);
          });
          handles.push(handle);
      }
      
      // 等待所有线程完成 | Wait for all threads to complete
      for handle in handles {
          handle.join().unwrap();
      }
      
      println!("最终计数: {} | Final count: {}", *counter.lock().unwrap(), *counter.lock().unwrap());
  }
  
  // Send和Sync特质演示 | Send and Sync traits demonstration
  fn send_sync_demonstration() {
      // 大多数类型都实现了Send | Most types implement Send
      let data = vec![1, 2, 3, 4, 5];
      
      let handle = thread::spawn(move || {
          // Vec<i32>实现了Send，可以安全地移动到其他线程 | Vec<i32> implements Send, safe to move to other threads
          let sum: i32 = data.iter().sum();
          println!("数据总和: {} | Data sum: {}", sum, sum);
      });
      
      handle.join().unwrap();
      
      // 演示不能发送的类型 | Demonstrate non-Send types
      // Rc<T>没有实现Send | Rc<T> does not implement Send
      use std::rc::Rc;
      let rc_data = Rc::new(vec![1, 2, 3]);
      
      // 这会编译失败 | This would fail to compile
      /*
      let handle = thread::spawn(move || {
          println!("{:?}", rc_data);
      });
      */
      
      println!("Rc数据只能在单线程中使用 | Rc data can only be used in single thread: {:?}", rc_data);
  }
  ```

### 6. 实践应用与最佳实践 | Practical Applications and Best Practices (30分钟 | 30 minutes)

- **线程使用的最佳实践 | Best Practices for Thread Usage**
  
  **关键原则 | Key Principles:**
  - 避免创建过多线程 | Avoid creating too many threads
  - 使用线程池管理线程生命周期 | Use thread pools to manage thread lifecycles
  - 优先考虑数据分割而非共享 | Prefer data partitioning over sharing
  
  **实践验证问题 | Practice Verification Questions:**
  1. 什么时候应该使用多线程？| When should multi-threading be used?
     **答案 | Answer:** 当有CPU密集型任务或IO等待时 | When there are CPU-intensive tasks or IO waits
  2. 如何确定合适的线程数量？| How to determine appropriate number of threads?
     **答案 | Answer:** 通常不超过CPU核心数，需要根据任务类型调整 | Usually no more than CPU cores, adjust based on task type
  3. 线程间通信的最佳方式是什么？| What's the best way for inter-thread communication?
     **答案 | Answer:** 使用通道(channels)或共享状态加锁 | Use channels or shared state with locks

## 实践项目：多线程计算器 | Practical Project: Multi-threaded Calculator

### 目标 | Objective
创建一个能够并行处理多个数学计算任务的计算器，演示线程创建、数据传递和结果收集。 | Create a calculator that can process multiple mathematical calculation tasks in parallel, demonstrating thread creation, data passing, and result collection.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. 如何使用thread::spawn创建新线程？| How to use thread::spawn to create new threads?
   **答案 | Answer:** thread::spawn(闭包) 返回JoinHandle | thread::spawn(closure) returns JoinHandle
2. 什么时候需要在闭包前添加move关键字？| When is the move keyword needed before closures?
   **答案 | Answer:** 当闭包需要获取外部变量的所有权时 | When closures need to take ownership of external variables
3. 如何等待线程完成并获取返回值？| How to wait for thread completion and get return value?
   **答案 | Answer:** 调用JoinHandle的join()方法 | Call the join() method of JoinHandle

### 步骤 | Steps
1. 设计计算任务结构体，包含操作类型和操作数 | Design calculation task struct with operation type and operands
2. 创建任务分发函数，为每个任务创建独立线程 | Create task dispatch function, creating independent threads for each task
3. 实现结果收集和汇总功能 | Implement result collection and summary functionality
4. 添加执行时间统计和性能对比 | Add execution time statistics and performance comparison
5. 测试不同数量任务的并发处理效果 | Test concurrent processing effects with different numbers of tasks

### 示例代码 | Example Code
```rust
"""
多线程计算器 | Multi-threaded Calculator
这个项目演示了Rust中基本的并发编程概念 | This project demonstrates basic concurrency concepts in Rust

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- thread::spawn线程创建 | thread::spawn thread creation
- move闭包和数据所有权 | move closures and data ownership
- JoinHandle和线程同步 | JoinHandle and thread synchronization
"""

use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
    Power(f64, f64),
    Factorial(u32),
}

#[derive(Debug)]
struct CalculationTask {
    id: u32,
    operation: Operation,
    description: String,
}

impl CalculationTask {
    fn new(id: u32, operation: Operation, description: String) -> Self {
        Self { id, operation, description }
    }
    
    fn execute(&self) -> Result<f64, String> {
        // 模拟计算耗时 | Simulate computation time
        thread::sleep(Duration::from_millis(100));
        
        match &self.operation {
            Operation::Add(a, b) => Ok(a + b),
            Operation::Subtract(a, b) => Ok(a - b),
            Operation::Multiply(a, b) => Ok(a * b),
            Operation::Divide(a, b) => {
                if *b == 0.0 {
                    Err("除零错误 | Division by zero".to_string())
                } else {
                    Ok(a / b)
                }
            },
            Operation::Power(base, exp) => Ok(base.powf(*exp)),
            Operation::Factorial(n) => {
                if *n > 20 {
                    Err("阶乘数值过大 | Factorial number too large".to_string())
                } else {
                    let mut result = 1.0;
                    for i in 1..=*n {
                        result *= i as f64;
                    }
                    Ok(result)
                }
            },
        }
    }
}

#[derive(Debug)]
struct CalculationResult {
    task_id: u32,
    result: Result<f64, String>,
    execution_time: Duration,
}

struct MultiThreadCalculator {
    tasks: Vec<CalculationTask>,
}

impl MultiThreadCalculator {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    
    fn add_task(&mut self, task: CalculationTask) {
        self.tasks.push(task);
    }
    
    // 顺序执行所有任务 | Execute all tasks sequentially
    fn execute_sequential(&self) -> (Vec<CalculationResult>, Duration) {
        let start_time = Instant::now();
        let mut results = Vec::new();
        
        for task in &self.tasks {
            let task_start = Instant::now();
            let result = task.execute();
            let execution_time = task_start.elapsed();
            
            results.push(CalculationResult {
                task_id: task.id,
                result,
                execution_time,
            });
            
            println!("顺序执行 - 任务 {}: {:?} | Sequential - Task {}: {:?}", 
                    task.id, task.description, task.id, task.description);
        }
        
        let total_time = start_time.elapsed();
        (results, total_time)
    }
    
    // 并发执行所有任务 | Execute all tasks concurrently
    fn execute_concurrent(&self) -> (Vec<CalculationResult>, Duration) {
        let start_time = Instant::now();
        let mut handles = Vec::new();
        
        // 为每个任务创建线程 | Create thread for each task
        for task in &self.tasks {
            let task_clone = task.clone(); // 克隆任务数据 | Clone task data
            
            let handle = thread::spawn(move || {
                println!("并发执行 - 线程开始处理任务 {} | Concurrent - Thread starts processing task {}", 
                        task_clone.id, task_clone.id);
                
                let task_start = Instant::now();
                let result = task_clone.execute();
                let execution_time = task_start.elapsed();
                
                println!("并发执行 - 任务 {} 完成 | Concurrent - Task {} completed", 
                        task_clone.id, task_clone.id);
                
                CalculationResult {
                    task_id: task_clone.id,
                    result,
                    execution_time,
                }
            });
            
            handles.push(handle);
        }
        
        // 收集所有线程结果 | Collect results from all threads
        let mut results = Vec::new();
        for handle in handles {
            match handle.join() {
                Ok(result) => results.push(result),
                Err(e) => println!("线程执行失败 | Thread execution failed: {:?}", e),
            }
        }
        
        // 按任务ID排序结果 | Sort results by task ID
        results.sort_by(|a, b| a.task_id.cmp(&b.task_id));
        
        let total_time = start_time.elapsed();
        (results, total_time)
    }
    
    // 打印结果对比 | Print result comparison
    fn print_results_comparison(&self) {
        println!("\n=== 多线程计算器性能对比 | Multi-threaded Calculator Performance Comparison ===");
        
        println!("\n执行顺序计算... | Executing sequential calculation...");
        let (seq_results, seq_time) = self.execute_sequential();
        
        println!("\n执行并发计算... | Executing concurrent calculation...");
        let (conc_results, conc_time) = self.execute_concurrent();
        
        println!("\n=== 结果对比 | Results Comparison ===");
        println!("任务总数 | Total tasks: {}", self.tasks.len());
        println!("顺序执行时间 | Sequential time: {:?}", seq_time);
        println!("并发执行时间 | Concurrent time: {:?}", conc_time);
        
        if seq_time > conc_time {
            let speedup = seq_time.as_millis() as f64 / conc_time.as_millis() as f64;
            println!("并发加速比 | Concurrent speedup: {:.2}x", speedup);
        }
        
        println!("\n=== 详细结果 | Detailed Results ===");
        for (seq_result, conc_result) in seq_results.iter().zip(conc_results.iter()) {
            println!("任务 {} | Task {}: {:?}", 
                    seq_result.task_id, seq_result.task_id, seq_result.result);
        }
    }
}

fn main() {
    let mut calculator = MultiThreadCalculator::new();
    
    // 添加各种计算任务 | Add various calculation tasks
    calculator.add_task(CalculationTask::new(
        1, 
        Operation::Add(10.5, 20.3), 
        "加法运算 | Addition".to_string()
    ));
    
    calculator.add_task(CalculationTask::new(
        2, 
        Operation::Multiply(7.5, 8.0), 
        "乘法运算 | Multiplication".to_string()
    ));
    
    calculator.add_task(CalculationTask::new(
        3, 
        Operation::Power(2.0, 10.0), 
        "幂运算 | Power".to_string()
    ));
    
    calculator.add_task(CalculationTask::new(
        4, 
        Operation::Factorial(10), 
        "阶乘运算 | Factorial".to_string()
    ));
    
    calculator.add_task(CalculationTask::new(
        5, 
        Operation::Divide(100.0, 3.0), 
        "除法运算 | Division".to_string()
    ));
    
    calculator.add_task(CalculationTask::new(
        6, 
        Operation::Subtract(50.0, 25.5), 
        "减法运算 | Subtraction".to_string()
    ));
    
    // 执行性能对比 | Execute performance comparison
    calculator.print_results_comparison();
    
    println!("\n=== 程序完成 | Program Completed ===");
}

// 辅助函数：创建大量计算任务进行压力测试 | Helper function: create many tasks for stress testing
#[allow(dead_code)]
fn stress_test() {
    let mut calculator = MultiThreadCalculator::new();
    
    // 创建大量任务 | Create many tasks
    for i in 1..=20 {
        let operation = match i % 4 {
            0 => Operation::Add(i as f64, (i * 2) as f64),
            1 => Operation::Multiply(i as f64, 1.5),
            2 => Operation::Power(2.0, (i % 10) as f64),
            _ => Operation::Factorial((i % 12) as u32),
        };
        
        calculator.add_task(CalculationTask::new(
            i, 
            operation, 
            format!("批量任务 {} | Batch task {}", i, i)
        ));
    }
    
    println!("压力测试：{} 个任务 | Stress test: {} tasks", calculator.tasks.len(), calculator.tasks.len());
    calculator.print_results_comparison();
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确使用了thread::spawn创建线程？| Does the project correctly use thread::spawn to create threads?
2. move闭包是否正确处理了数据所有权转移？| Do move closures correctly handle data ownership transfer?
3. 是否通过JoinHandle正确等待所有线程完成？| Are all threads properly awaited through JoinHandle?
4. 并发版本是否比顺序版本表现更好？| Does the concurrent version perform better than sequential version?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **线程生命周期理解练习 | Thread Lifecycle Understanding Exercise**
   - **练习描述 | Exercise Description:** 创建不同生命周期的线程，观察主线程结束对子线程的影响
   - **概念检查 | Concept Check:** 主线程结束时子线程会如何处理？| What happens to child threads when main thread ends?
   - **学习目标 | Learning Objective:** 深入理解线程生命周期管理

2. **数据所有权传递练习 | Data Ownership Transfer Exercise**
   - **练习描述 | Exercise Description:** 实验不同数据类型在线程间的传递方式，对比move和clone的区别
   - **概念检查 | Concept Check:** 何时使用move，何时使用clone？| When to use move, when to use clone?
   - **学习目标 | Learning Objective:** 掌握线程间数据传递的最佳实践

3. **线程错误处理练习 | Thread Error Handling Exercise**
   - **练习描述 | Exercise Description:** 创建可能panic的线程，实现错误恢复机制
   - **概念检查 | Concept Check:** 线程panic如何影响其他线程？| How does thread panic affect other threads?
   - **学习目标 | Learning Objective:** 学会处理线程中的错误情况

4. **性能分析练习 | Performance Analysis Exercise**
   - **练习描述 | Exercise Description:** 对比不同数量线程的性能表现，找出最优配置
   - **概念检查 | Concept Check:** 线程数量和性能的关系是什么？| What's the relationship between thread count and performance?
   - **学习目标 | Learning Objective:** 理解并发编程的性能优化原理

5. **Send和Sync特质探索练习 | Send and Sync Traits Exploration Exercise**
   - **练习描述 | Exercise Description:** 实验不同类型的Send和Sync特质，理解其作用
   - **概念检查 | Concept Check:** 哪些类型实现了Send和Sync？| Which types implement Send and Sync?
   - **学习目标 | Learning Objective:** 深入理解Rust的线程安全机制

6. **线程调度观察练习 | Thread Scheduling Observation Exercise**
   - **练习描述 | Exercise Description:** 创建多个线程观察其调度顺序和执行模式
   - **概念检查 | Concept Check:** 线程执行顺序是否可预测？| Is thread execution order predictable?
   - **学习目标 | Learning Objective:** 理解操作系统的线程调度机制

7. **实际应用场景练习 | Real-world Application Scenario Exercise**
   - **练习描述 | Exercise Description:** 设计一个文件处理系统，使用多线程并发处理多个文件
   - **概念检查 | Concept Check:** 什么情况下多线程比单线程更有优势？| When is multi-threading more advantageous than single-threading?
   - **学习目标 | Learning Objective:** 将理论知识应用到实际问题解决中

## 学习资源 | Learning Resources
- [Rust官方文档 - 并发编程](https://doc.rust-lang.org/book/ch16-00-concurrency.html) | [Rust Official Documentation - Concurrent Programming]
- [std::thread模块文档](https://doc.rust-lang.org/std/thread/) | [std::thread Module Documentation]
- [Rust异步编程指南](https://rust-lang.github.io/async-book/) | [Rust Async Programming Guide]
- [并发编程最佳实践](https://github.com/rust-unofficial/patterns) | [Concurrency Best Practices]

---

✅ **完成检查清单 | Completion Checklist**
- [ ] 理解并发和并行的区别 | Understand difference between concurrency and parallelism
- [ ] 掌握thread::spawn的基本用法 | Master basic usage of thread::spawn
- [ ] 了解move闭包在线程中的作用 | Understand role of move closures in threads
- [ ] 能够正确使用JoinHandle等待线程 | Can correctly use JoinHandle to wait for threads
- [ ] 理解线程间数据共享的问题 | Understand data sharing problems between threads
- [ ] 完成多线程计算器项目 | Complete multi-threaded calculator project
- [ ] 掌握基本的线程错误处理 | Master basic thread error handling
- [ ] 理解Send和Sync特质的概念 | Understand Send and Sync traits concepts
- [ ] 能够分析多线程程序的性能 | Can analyze multi-threaded program performance
- [ ] 完成至少3个扩展练习 | Complete at least 3 extension exercises

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释线程创建、数据传递和同步的核心概念。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain core concepts of thread creation, data passing, and synchronization to others.