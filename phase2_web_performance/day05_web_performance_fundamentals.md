# Rust入门 - 第5天：Web性能基础指标 | Rust Introduction - Day 5: Web Performance Fundamentals

## 学习目标 | Learning Objectives
- 深入理解Core Web Vitals核心指标体系 | Master the Core Web Vitals metrics system
- 掌握TTFB、TTI等关键性能指标的含义和优化方法 | Understand TTFB, TTI and other key performance metrics and optimization methods
- 学习网络延迟与带宽优化的实践技巧 | Learn practical techniques for network latency and bandwidth optimization
- 能够使用Rust工具进行Web性能分析和监控 | Develop ability to perform web performance analysis and monitoring with Rust tools
- 构建完整的HTTP性能分析工具项目 | Build a complete HTTP performance analysis tool project
- 建立Web性能优化的系统性思维框架 | Establish systematic thinking framework for web performance optimization

## 详细内容 | Detailed Content

### 1. Core Web Vitals核心指标深度解析 | Core Web Vitals In-Depth Analysis (2小时 | 2 hours)

- **Core Web Vitals基础概念 | Core Web Vitals Fundamentals**
  
  **概念定义 | Concept Definition:**
  Core Web Vitals是Google定义的三个核心用户体验指标，用于衡量网页加载性能、交互性和视觉稳定性。这些指标直接影响搜索排名和用户满意度。| Core Web Vitals are three essential user experience metrics defined by Google to measure loading performance, interactivity, and visual stability of web pages. These metrics directly impact search rankings and user satisfaction.
  
  **核心特征 | Key Characteristics:**
  - 用户中心化：基于真实用户体验数据而非技术指标 | User-centric: based on real user experience data rather than technical metrics
  - 量化可测：提供具体的数值标准和测量方法 | Quantifiable: provides specific numerical standards and measurement methods
  - 业务相关：与转化率、用户留存等业务指标密切相关 | Business-relevant: closely related to conversion rates, user retention and other business metrics
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. Core Web Vitals包含几个核心指标？| How many core metrics does Core Web Vitals include?
     **答案 | Answer:** 3个 | Three - LCP、FID、CLS分别测量加载、交互和视觉稳定性 | LCP, FID, CLS measure loading, interactivity and visual stability respectively
  2. 这些指标是基于用户体验还是服务器性能？| Are these metrics based on user experience or server performance?
     **答案 | Answer:** 用户体验 | User experience - Core Web Vitals专注于用户在浏览器中的实际感受 | Core Web Vitals focus on users' actual experience in browsers
  3. Core Web Vitals会影响搜索引擎排名吗？| Do Core Web Vitals affect search engine rankings?
     **答案 | Answer:** 是 | Yes - Google已将这些指标作为搜索排名因素之一 | Google has incorporated these metrics as search ranking factors
  4. 这些指标可以通过实验室数据完全评估吗？| Can these metrics be fully evaluated through lab data alone?
     **答案 | Answer:** 否 | No - 需要结合实际用户数据(RUM)才能获得完整picture | Need to combine with Real User Monitoring (RUM) data for complete picture
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // Core Web Vitals监控基础结构 | Core Web Vitals monitoring foundation
  use serde::{Deserialize, Serialize};
  use std::time::{Duration, Instant};
  
  #[derive(Debug, Serialize, Deserialize)]
  pub struct CoreWebVitals {
      pub lcp: Option<f64>,    // Largest Contentful Paint (秒 | seconds)
      pub fid: Option<f64>,    // First Input Delay (毫秒 | milliseconds)  
      pub cls: Option<f64>,    // Cumulative Layout Shift (分数 | score)
      pub measurement_time: u64, // 测量时间戳 | measurement timestamp
  }
  
  impl CoreWebVitals {
      pub fn new() -> Self {
          Self {
              lcp: None,
              fid: None, 
              cls: None,
              measurement_time: chrono::Utc::now().timestamp_millis() as u64,
          }
      }
      
      // LCP评分判断 | LCP score evaluation
      pub fn lcp_score(&self) -> &'static str {
          match self.lcp {
              Some(lcp) if lcp <= 2.5 => "Good",      // 良好 | Good
              Some(lcp) if lcp <= 4.0 => "Needs Improvement", // 需要改进 | Needs Improvement
              Some(_) => "Poor",                       // 差 | Poor
              None => "Not Measured",                  // 未测量 | Not Measured
          }
      }
      
      // FID评分判断 | FID score evaluation
      pub fn fid_score(&self) -> &'static str {
          match self.fid {
              Some(fid) if fid <= 100.0 => "Good",
              Some(fid) if fid <= 300.0 => "Needs Improvement",
              Some(_) => "Poor",
              None => "Not Measured",
          }
      }
      
      // CLS评分判断 | CLS score evaluation  
      pub fn cls_score(&self) -> &'static str {
          match self.cls {
              Some(cls) if cls <= 0.1 => "Good",
              Some(cls) if cls <= 0.25 => "Needs Improvement", 
              Some(_) => "Poor",
              None => "Not Measured",
          }
      }
  }
  ```
  
  **实践检查问题 | Practice Checking Questions:**
  - 这段代码会将LCP值3.0秒评为什么等级？| What grade would this code assign to an LCP value of 3.0 seconds?
    **答案 | Answer:** "Needs Improvement" - 因为3.0秒在2.5-4.0秒区间内 | Because 3.0 seconds falls within the 2.5-4.0 second range
  - 如果FID为50毫秒，评分结果是什么？| What would be the score result if FID is 50 milliseconds?
    **答案 | Answer:** "Good" - 50毫秒小于等于100毫秒的良好标准 | "Good" - 50ms is less than or equal to the 100ms good standard
  
  **常见误区检查 | Common Misconception Checks:**
  - Core Web Vitals只需要在开发环境测试即可？| Are Core Web Vitals only needed to be tested in development environment?
    **答案 | Answer:** 错误 | Wrong - 必须在真实用户环境中持续监控，因为网络条件、设备性能等会显著影响结果 | Must be continuously monitored in real user environments as network conditions, device performance etc. significantly affect results
  - 所有三个指标都必须达到"Good"级别吗？| Must all three metrics reach "Good" level?
    **答案 | Answer:** 理想情况是 | Ideally yes - 但应优先优化对用户体验影响最大的指标 | But should prioritize optimizing metrics with greatest impact on user experience

### 2. LCP(Largest Contentful Paint)最大内容绘制优化 | LCP Optimization (30分钟 | 30 minutes)

- **LCP深度理解 | LCP Deep Understanding**
  
  **概念定义 | Concept Definition:**
  LCP测量页面主要内容完成渲染的时间点，通常是最大的文本块、图像或视频元素。它反映用户感知的加载速度。| LCP measures when the largest content element finishes rendering, usually the largest text block, image, or video element. It reflects users' perceived loading speed.
  
  **影响因素分析 | Impact Factor Analysis:**
  - 服务器响应时间：TTFB直接影响LCP | Server response time: TTFB directly affects LCP
  - 资源加载时间：CSS、JavaScript阻塞渲染 | Resource loading time: CSS, JavaScript blocking rendering
  - 客户端渲染时间：浏览器解析和绘制耗时 | Client-side rendering time: browser parsing and painting duration
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. LCP测量的是第一个像素绘制时间吗？| Does LCP measure the first pixel paint time?
     **答案 | Answer:** 否 | No - LCP测量最大内容元素的绘制完成时间 | LCP measures when the largest content element finishes painting
  2. 页面滚动会影响LCP的计算吗？| Does page scrolling affect LCP calculation?
     **答案 | Answer:** 否 | No - LCP只考虑初始视口内的元素 | LCP only considers elements within the initial viewport
  3. LCP元素在页面加载过程中可能会改变吗？| Can the LCP element change during page loading?
     **答案 | Answer:** 是 | Yes - 随着更大元素的加载，LCP候选元素会更新 | As larger elements load, LCP candidate elements update
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // LCP监控和优化工具 | LCP monitoring and optimization tool
  use reqwest::Client;
  use std::time::Instant;
  use tokio::time::{sleep, Duration};
  
  #[derive(Debug)]
  pub struct LCPAnalyzer {
      client: Client,
  }
  
  impl LCPAnalyzer {
      pub fn new() -> Self {
          Self {
              client: Client::builder()
                  .timeout(Duration::from_secs(30))
                  .build()
                  .unwrap(),
          }
      }
      
      // 模拟LCP测量 | Simulate LCP measurement
      pub async fn measure_lcp(&self, url: &str) -> Result<f64, Box<dyn std::error::Error>> {
          let start = Instant::now();
          
          // 1. 测量TTFB | Measure TTFB
          let response = self.client.get(url).send().await?;
          let ttfb = start.elapsed().as_secs_f64();
          
          // 2. 获取HTML内容 | Get HTML content
          let html = response.text().await?;
          
          // 3. 分析关键资源 | Analyze critical resources
          let critical_resources = self.extract_critical_resources(&html);
          
          // 4. 模拟资源加载时间 | Simulate resource loading time
          let resource_load_time = self.estimate_resource_load_time(&critical_resources).await;
          
          // 5. 计算预估LCP | Calculate estimated LCP
          let estimated_lcp = ttfb + resource_load_time;
          
          println!("URL: {}", url);
          println!("TTFB: {:.3}s", ttfb);
          println!("Resource Load Time: {:.3}s", resource_load_time);
          println!("Estimated LCP: {:.3}s", estimated_lcp);
          
          Ok(estimated_lcp)
      }
      
      // 提取关键资源 | Extract critical resources
      fn extract_critical_resources(&self, html: &str) -> Vec<String> {
          let mut resources = Vec::new();
          
          // 简单的HTML解析查找图片和CSS | Simple HTML parsing to find images and CSS
          for line in html.lines() {
              if line.contains("<img") && line.contains("src=") {
                  // 提取图片URL | Extract image URL
                  if let Some(start) = line.find("src=\"") {
                      let start = start + 5;
                      if let Some(end) = line[start..].find("\"") {
                          let img_src = &line[start..start + end];
                          resources.push(img_src.to_string());
                      }
                  }
              }
              
              if line.contains("<link") && line.contains("stylesheet") {
                  // 提取CSS URL | Extract CSS URL  
                  if let Some(start) = line.find("href=\"") {
                      let start = start + 6;
                      if let Some(end) = line[start..].find("\"") {
                          let css_href = &line[start..start + end];
                          resources.push(css_href.to_string());
                      }
                  }
              }
          }
          
          resources
      }
      
      // 估算资源加载时间 | Estimate resource loading time
      async fn estimate_resource_load_time(&self, resources: &[String]) -> f64 {
          // 模拟并行加载前3个关键资源 | Simulate parallel loading of first 3 critical resources
          let mut total_time = 0.0;
          for (i, _resource) in resources.iter().take(3).enumerate() {
              // 模拟网络延迟 | Simulate network latency
              let simulated_delay = 0.1 + (i as f64 * 0.05);
              total_time = total_time.max(simulated_delay);
          }
          total_time
      }
      
      // LCP优化建议 | LCP optimization suggestions
      pub fn get_optimization_suggestions(&self, lcp_time: f64) -> Vec<&'static str> {
          let mut suggestions = Vec::new();
          
          if lcp_time > 4.0 {
              suggestions.push("Critical: 服务器响应时间过长，考虑CDN加速 | Server response time too long, consider CDN acceleration");
              suggestions.push("Critical: 优化图片格式和大小 | Optimize image format and size");
              suggestions.push("Critical: 实施关键资源预加载 | Implement critical resource preloading");
          } else if lcp_time > 2.5 {
              suggestions.push("Important: 启用图片懒加载 | Enable image lazy loading");
              suggestions.push("Important: 压缩CSS和JavaScript | Compress CSS and JavaScript");
              suggestions.push("Important: 使用现代图片格式(WebP/AVIF) | Use modern image formats (WebP/AVIF)");
          }
          
          suggestions.push("General: 监控LCP变化趋势 | Monitor LCP trend changes");
          suggestions
      }
  }
  ```

### 3. FID(First Input Delay)首次输入延迟分析 | FID Analysis (30分钟 | 30 minutes)

- **FID交互性能深入 | FID Interactivity Performance Deep Dive**
  
  **概念定义 | Concept Definition:**
  FID测量用户首次与页面交互时的响应延迟，反映页面的交互就绪状态。它只在用户实际交互时才能测量。| FID measures the response delay when users first interact with the page, reflecting the page's interaction readiness. It can only be measured during actual user interactions.
  
  **核心特征 | Key Characteristics:**
  - 真实用户指标：无法在实验室环境完全模拟 | Real user metric: cannot be fully simulated in lab environments
  - 首次交互：只测量第一次交互的延迟 | First interaction: only measures first interaction delay
  - JavaScript依赖：主要受JavaScript执行阻塞影响 | JavaScript dependent: mainly affected by JavaScript execution blocking
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. FID可以通过自动化工具完全测量吗？| Can FID be fully measured by automated tools?
     **答案 | Answer:** 否 | No - FID需要真实用户交互，只能通过RUM数据获得 | FID requires real user interaction, only obtainable through RUM data
  2. 页面上的第二次点击会影响FID指标吗？| Does the second click on a page affect the FID metric?
     **答案 | Answer:** 否 | No - FID只测量首次交互延迟 | FID only measures first interaction delay
  3. FID主要由什么因素造成？| What primarily causes FID?
     **答案 | Answer:** JavaScript执行阻塞 | JavaScript execution blocking - 主线程忙碌时无法响应用户输入 | Main thread busy unable to respond to user input
  4. 长时间的CSS解析会直接影响FID吗？| Does long CSS parsing directly affect FID?
     **答案 | Answer:** 间接影响 | Indirect effect - CSS阻塞可能延迟JavaScript执行，从而影响交互就绪时间 | CSS blocking may delay JavaScript execution, thus affecting interaction readiness
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // FID模拟和优化分析工具 | FID simulation and optimization analysis tool
  use std::collections::HashMap;
  use std::time::{Duration, Instant};
  use tokio::time::sleep;
  
  #[derive(Debug, Clone)]
  pub struct JavaScriptTask {
      pub name: String,
      pub execution_time: Duration, // JavaScript任务执行时间 | JavaScript task execution time
      pub is_blocking: bool,        // 是否阻塞主线程 | Whether blocking main thread
  }
  
  #[derive(Debug)]
  pub struct FIDSimulator {
      pub tasks: Vec<JavaScriptTask>,
      pub page_load_complete: bool,
  }
  
  impl FIDSimulator {
      pub fn new() -> Self {
          Self {
              tasks: Vec::new(),
              page_load_complete: false,
          }
      }
      
      // 添加JavaScript任务 | Add JavaScript task
      pub fn add_task(&mut self, name: &str, execution_time_ms: u64, is_blocking: bool) {
          self.tasks.push(JavaScriptTask {
              name: name.to_string(),
              execution_time: Duration::from_millis(execution_time_ms),
              is_blocking,
          });
      }
      
      // 模拟页面加载和JavaScript执行 | Simulate page loading and JavaScript execution
      pub async fn simulate_page_load(&mut self) -> Duration {
          let start = Instant::now();
          let mut total_blocking_time = Duration::from_millis(0);
          
          println!("开始页面加载模拟... | Starting page load simulation...");
          
          // 顺序执行阻塞任务 | Execute blocking tasks sequentially
          for task in &self.tasks {
              if task.is_blocking {
                  println!("执行阻塞任务: {} ({:?}) | Executing blocking task: {} ({:?})", 
                          task.name, task.execution_time);
                  sleep(task.execution_time).await;
                  total_blocking_time += task.execution_time;
              } else {
                  println!("异步任务: {} | Async task: {}", task.name);
              }
          }
          
          self.page_load_complete = true;
          let total_time = start.elapsed();
          
          println!("页面加载完成 | Page load complete");
          println!("总阻塞时间: {:?} | Total blocking time: {:?}", total_blocking_time);
          println!("总加载时间: {:?} | Total load time: {:?}", total_time);
          
          total_blocking_time
      }
      
      // 模拟用户输入延迟 | Simulate user input delay
      pub async fn simulate_user_interaction(&self, interaction_type: &str) -> Duration {
          let start = Instant::now();
          
          println!("用户尝试{} | User attempting {}", interaction_type, interaction_type);
          
          // 检查主线程是否忙碌 | Check if main thread is busy
          if !self.page_load_complete {
              println!("主线程忙碌，用户输入需要等待... | Main thread busy, user input needs to wait...");
              
              // 计算剩余的阻塞时间 | Calculate remaining blocking time
              let remaining_blocking = self.calculate_remaining_blocking_time();
              sleep(remaining_blocking).await;
              
              let fid = start.elapsed();
              println!("FID测量结果: {:?} | FID measurement result: {:?}", fid);
              return fid;
          }
          
          // 主线程空闲，立即响应 | Main thread idle, immediate response
          let fid = Duration::from_millis(5); // 最小响应时间 | Minimum response time
          println!("主线程空闲，立即响应 | Main thread idle, immediate response");
          println!("FID测量结果: {:?} | FID measurement result: {:?}", fid);
          fid
      }
      
      // 计算剩余阻塞时间 | Calculate remaining blocking time
      fn calculate_remaining_blocking_time(&self) -> Duration {
          // 简化计算：假设还有一些JavaScript需要执行 | Simplified calculation: assume some JavaScript still needs execution
          Duration::from_millis(150)
      }
      
      // 获取FID优化建议 | Get FID optimization suggestions
      pub fn get_fid_optimization_suggestions(&self, fid_ms: f64) -> Vec<&'static str> {
          let mut suggestions = Vec::new();
          
          if fid_ms > 300.0 {
              suggestions.push("Critical: 拆分长时间运行的JavaScript任务 | Break up long-running JavaScript tasks");
              suggestions.push("Critical: 移除非必要的第三方脚本 | Remove non-essential third-party scripts");
              suggestions.push("Critical: 使用Web Workers处理复杂计算 | Use Web Workers for complex calculations");
          } else if fid_ms > 100.0 {
              suggestions.push("Important: 优化JavaScript包大小 | Optimize JavaScript bundle size");
              suggestions.push("Important: 实施代码分割 | Implement code splitting");
              suggestions.push("Important: 延迟加载非关键JavaScript | Defer non-critical JavaScript");
          }
          
          suggestions.push("General: 使用TBT(Total Blocking Time)监控主线程阻塞 | Use TBT to monitor main thread blocking");
          suggestions
      }
  }
  
  // FID测试示例 | FID test example
  pub async fn run_fid_test() {
      let mut simulator = FIDSimulator::new();
      
      // 添加典型的页面JavaScript任务 | Add typical page JavaScript tasks
      simulator.add_task("DOM解析", 50, true);   // DOM parsing
      simulator.add_task("框架初始化", 200, true); // Framework initialization  
      simulator.add_task("API调用", 100, false);  // API calls
      simulator.add_task("事件绑定", 30, true);    // Event binding
      
      // 模拟页面加载 | Simulate page loading
      let _blocking_time = simulator.simulate_page_load().await;
      
      // 模拟用户交互 | Simulate user interaction
      let fid = simulator.simulate_user_interaction("点击按钮").await;
      
      // 获取优化建议 | Get optimization suggestions
      let suggestions = simulator.get_fid_optimization_suggestions(fid.as_millis() as f64);
      
      println!("\n优化建议 | Optimization Suggestions:");
      for suggestion in suggestions {
          println!("- {}", suggestion);
      }
  }
  ```

### 4. CLS(Cumulative Layout Shift)累积布局偏移控制 | CLS Control (30分钟 | 30 minutes)

- **CLS视觉稳定性优化 | CLS Visual Stability Optimization**
  
  **概念定义 | Concept Definition:**
  CLS测量页面加载过程中元素布局意外移动的累积分数，反映视觉稳定性。布局偏移会干扰用户体验，导致误点击。| CLS measures the cumulative score of unexpected element layout shifts during page loading, reflecting visual stability. Layout shifts disrupt user experience and cause accidental clicks.
  
  **核心特征 | Key Characteristics:**
  - 累积性：整个页面生命周期内所有布局偏移的总和 | Cumulative: sum of all layout shifts throughout page lifecycle
  - 影响分数 = 影响区域 × 距离分数 | Impact score = impact area × distance score
  - 排除预期移动：用户触发的移动不计入CLS | Excludes expected movement: user-triggered movements not counted in CLS
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. CLS分数为0表示什么？| What does a CLS score of 0 indicate?
     **答案 | Answer:** 完全没有布局偏移 | No layout shifts at all - 页面加载过程中元素位置完全稳定 | Element positions completely stable during page loading
  2. 用户点击按钮后的内容展开会计入CLS吗？| Does content expansion after user clicking a button count towards CLS?
     **答案 | Answer:** 不会 | No - 用户交互触发的布局变化不计入CLS | Layout changes triggered by user interaction don't count towards CLS
  3. 图片尺寸未设置会导致CLS问题吗？| Can unset image dimensions cause CLS issues?
     **答案 | Answer:** 会 | Yes - 图片加载后会推动其他元素位移 | Images loading will push other elements to shift
  4. CLS分数的计算考虑时间因素吗？| Does CLS score calculation consider time factors?
     **答案 | Answer:** 考虑 | Yes - 使用会话窗口来分组相近时间的偏移 | Uses session windows to group shifts occurring close in time
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // CLS分析和预防工具 | CLS analysis and prevention tool
  use serde::{Deserialize, Serialize};
  use std::collections::HashMap;
  
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct LayoutShift {
      pub timestamp: f64,        // 偏移发生时间 | Shift occurrence time
      pub impact_fraction: f64,  // 影响区域比例 | Impact area fraction
      pub distance_fraction: f64, // 移动距离比例 | Movement distance fraction
      pub element_selector: String, // 元素选择器 | Element selector
  }
  
  impl LayoutShift {
      pub fn new(timestamp: f64, impact_fraction: f64, distance_fraction: f64, element: &str) -> Self {
          Self {
              timestamp,
              impact_fraction,
              distance_fraction,
              element_selector: element.to_string(),
          }
      }
      
      // 计算布局偏移分数 | Calculate layout shift score
      pub fn shift_score(&self) -> f64 {
          self.impact_fraction * self.distance_fraction
      }
  }
  
  #[derive(Debug)]
  pub struct CLSAnalyzer {
      pub shifts: Vec<LayoutShift>,
      pub session_gap_ms: f64, // 会话间隔毫秒 | Session gap in milliseconds
  }
  
  impl CLSAnalyzer {
      pub fn new() -> Self {
          Self {
              shifts: Vec::new(),
              session_gap_ms: 1000.0, // 1秒会话间隔 | 1 second session gap
          }
      }
      
      // 记录布局偏移 | Record layout shift
      pub fn record_shift(&mut self, shift: LayoutShift) {
          println!("记录布局偏移 | Recording layout shift: {} at {:.3}s (score: {:.4})", 
                  shift.element_selector, shift.timestamp / 1000.0, shift.shift_score());
          self.shifts.push(shift);
      }
      
      // 计算CLS分数 | Calculate CLS score
      pub fn calculate_cls_score(&self) -> f64 {
          if self.shifts.is_empty() {
              return 0.0;
          }
          
          // 按时间排序 | Sort by time
          let mut sorted_shifts = self.shifts.clone();
          sorted_shifts.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
          
          // 分组到会话窗口 | Group into session windows
          let mut sessions = Vec::new();
          let mut current_session = Vec::new();
          let mut last_timestamp = 0.0;
          
          for shift in sorted_shifts {
              if shift.timestamp - last_timestamp > self.session_gap_ms {
                  if !current_session.is_empty() {
                      sessions.push(current_session.clone());
                      current_session.clear();
                  }
              }
              current_session.push(shift);
              last_timestamp = shift.timestamp;
          }
          
          if !current_session.is_empty() {
              sessions.push(current_session);
          }
          
          // 找出最差的会话分数 | Find worst session score
          let mut max_session_score = 0.0;
          for (i, session) in sessions.iter().enumerate() {
              let session_score: f64 = session.iter().map(|s| s.shift_score()).sum();
              println!("会话 {} 分数: {:.4} | Session {} score: {:.4}", i + 1, session_score, i + 1, session_score);
              max_session_score = max_session_score.max(session_score);
          }
          
          println!("最终CLS分数: {:.4} | Final CLS score: {:.4}", max_session_score);
          max_session_score
      }
      
      // 分析CLS问题源头 | Analyze CLS problem sources
      pub fn analyze_cls_sources(&self) -> HashMap<String, f64> {
          let mut element_scores = HashMap::new();
          
          for shift in &self.shifts {
              let score = shift.shift_score();
              *element_scores.entry(shift.element_selector.clone()).or_insert(0.0) += score;
          }
          
          element_scores
      }
      
      // 获取CLS优化建议 | Get CLS optimization suggestions  
      pub fn get_cls_optimization_suggestions(&self, cls_score: f64) -> Vec<&'static str> {
          let mut suggestions = Vec::new();
          
          if cls_score > 0.25 {
              suggestions.push("Critical: 为图片和视频设置明确的宽高属性 | Set explicit width/height for images and videos");
              suggestions.push("Critical: 预留广告和嵌入内容的空间 | Reserve space for ads and embedded content");
              suggestions.push("Critical: 避免在现有内容上方插入内容 | Avoid inserting content above existing content");
          } else if cls_score > 0.1 {
              suggestions.push("Important: 使用font-display:swap优化字体加载 | Optimize font loading with font-display:swap");
              suggestions.push("Important: 预加载关键字体文件 | Preload critical font files");
              suggestions.push("Important: 为动态内容使用骨架屏 | Use skeleton screens for dynamic content");
          }
          
          // 分析具体问题元素 | Analyze specific problem elements
          let sources = self.analyze_cls_sources();
          let mut problem_elements: Vec<_> = sources.iter().collect();
          problem_elements.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
          
          if let Some((element, score)) = problem_elements.first() {
              if **score > 0.05 {
                  suggestions.push("Specific: 重点优化主要问题元素 | Focus on optimizing main problem elements");
              }
          }
          
          suggestions.push("General: 在开发阶段使用CLS监控工具 | Use CLS monitoring tools during development");
          suggestions
      }
  }
  
  // CLS测试示例 | CLS test example
  pub fn run_cls_test() {
      let mut analyzer = CLSAnalyzer::new();
      
      // 模拟典型的布局偏移场景 | Simulate typical layout shift scenarios
      
      // 1. 图片加载导致的偏移 | Image loading shift
      analyzer.record_shift(LayoutShift::new(
          500.0,  // 500ms后图片加载完成 | Image loads after 500ms
          0.3,    // 影响30%的视口区域 | Affects 30% of viewport
          0.25,   // 元素向下移动25%的距离 | Elements move down 25% distance
          "img.hero-image"
      ));
      
      // 2. 广告加载导致的偏移 | Ad loading shift
      analyzer.record_shift(LayoutShift::new(
          1200.0, // 1.2s后广告加载 | Ad loads after 1.2s
          0.15,   // 影响15%的区域 | Affects 15% of area
          0.4,    // 较大的移动距离 | Larger movement distance
          ".ad-banner"
      ));
      
      // 3. 字体加载导致的偏移 | Font loading shift
      analyzer.record_shift(LayoutShift::new(
          300.0,  // 300ms后字体加载 | Font loads after 300ms
          0.8,    // 影响大部分文本 | Affects most text
          0.05,   // 微小的高度变化 | Small height change
          "h1, p, .text-content"
      ));
      
      // 计算CLS分数 | Calculate CLS score
      let cls_score = analyzer.calculate_cls_score();
      
      // 获取优化建议 | Get optimization suggestions
      let suggestions = analyzer.get_cls_optimization_suggestions(cls_score);
      
      println!("\nCLS优化建议 | CLS Optimization Suggestions:");
      for suggestion in suggestions {
          println!("- {}", suggestion);
      }
      
      // 显示问题元素分析 | Show problem element analysis
      let sources = analyzer.analyze_cls_sources();
      println!("\n问题元素分析 | Problem Element Analysis:");
      for (element, score) in sources {
          println!("  {}: {:.4}", element, score);
      }
  }
  ```

### 5. TTFB与TTI关键性能指标优化 | TTFB and TTI Optimization (45分钟 | 45 minutes)

- **TTFB(Time to First Byte)首字节时间优化 | TTFB Optimization**
  
  **概念定义 | Concept Definition:**
  TTFB测量从用户发起请求到接收到服务器响应第一个字节的时间，反映服务器响应性能和网络延迟。| TTFB measures the time from user initiating a request to receiving the first byte of server response, reflecting server response performance and network latency.
  
  **影响因素分解 | Impact Factor Breakdown:**
  - DNS查询时间：域名解析耗时 | DNS lookup time: domain name resolution duration
  - TCP连接建立：三次握手时间 | TCP connection establishment: three-way handshake time
  - TLS握手：HTTPS加密协商 | TLS handshake: HTTPS encryption negotiation
  - 服务器处理：后端逻辑执行时间 | Server processing: backend logic execution time
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. TTFB包含HTML内容下载时间吗？| Does TTFB include HTML content download time?
     **答案 | Answer:** 否 | No - TTFB只测量到接收第一个字节的时间 | TTFB only measures time to receive first byte
  2. CDN会影响TTFB吗？| Does CDN affect TTFB?
     **答案 | Answer:** 会 | Yes - CDN可以显著减少网络延迟和服务器响应时间 | CDN can significantly reduce network latency and server response time
  3. TTFB超过多少毫秒被认为是慢的？| What TTFB is considered slow?
     **答案 | Answer:** 800毫秒以上 | Above 800ms - Google建议TTFB应在800ms以内 | Google recommends TTFB should be within 800ms
  4. 静态文件的TTFB应该比动态页面快吗？| Should static files have faster TTFB than dynamic pages?
     **答案 | Answer:** 是 | Yes - 静态文件无需服务器处理，理论上TTFB更快 | Static files require no server processing, theoretically faster TTFB
  
- **TTI(Time to Interactive)可交互时间分析 | TTI Analysis**
  
  **概念定义 | Concept Definition:**
  TTI测量页面变为完全可交互的时间点，此时主线程空闲且能够可靠地响应用户输入。| TTI measures when the page becomes fully interactive, with the main thread idle and able to reliably respond to user input.
  
  **TTI判定条件 | TTI Criteria:**
  - FCP已完成：首次内容绘制 | FCP completed: First Contentful Paint
  - 主线程空闲：连续5秒无长任务 | Main thread idle: no long tasks for 5 consecutive seconds  
  - 网络稳定：HTTP请求数量稳定 | Network stable: HTTP request count stable
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. TTI一定晚于FCP吗？| Does TTI always occur after FCP?
     **答案 | Answer:** 是 | Yes - TTI的前提条件包括FCP完成 | TTI prerequisites include FCP completion
  2. JavaScript执行时间长会影响TTI吗？| Does long JavaScript execution affect TTI?
     **答案 | Answer:** 会 | Yes - 长时间JavaScript任务会延迟TTI | Long JavaScript tasks delay TTI
  3. TTI和FID有什么关系？| What's the relationship between TTI and FID?
     **答案 | Answer:** TTI越早，FID越小 | Earlier TTI leads to smaller FID - TTI反映页面交互就绪状态 | TTI reflects page interaction readiness
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // TTFB和TTI综合性能分析工具 | Comprehensive TTFB and TTI performance analysis tool
  use reqwest::Client;
  use std::time::{Duration, Instant};
  use tokio::time::sleep;
  use serde::{Deserialize, Serialize};
  
  #[derive(Debug, Serialize, Deserialize)]
  pub struct PerformanceMetrics {
      pub url: String,
      pub ttfb: f64,           // Time to First Byte (秒 | seconds)
      pub tti: f64,            // Time to Interactive (秒 | seconds)
      pub fcp: f64,            // First Contentful Paint (秒 | seconds)
      pub dns_time: f64,       // DNS查询时间 | DNS lookup time
      pub tcp_time: f64,       // TCP连接时间 | TCP connection time
      pub tls_time: f64,       // TLS握手时间 | TLS handshake time
      pub server_time: f64,    // 服务器处理时间 | Server processing time
      pub main_thread_blocking: f64, // 主线程阻塞时间 | Main thread blocking time
  }
  
  #[derive(Debug)]
  pub struct WebPerformanceAnalyzer {
      client: Client,
  }
  
  impl WebPerformanceAnalyzer {
      pub fn new() -> Self {
          Self {
              client: Client::builder()
                  .timeout(Duration::from_secs(30))
                  .user_agent("WebPerformanceAnalyzer/1.0")
                  .build()
                  .unwrap(),
          }
      }
      
      // 综合性能分析 | Comprehensive performance analysis
      pub async fn analyze_performance(&self, url: &str) -> Result<PerformanceMetrics, Box<dyn std::error::Error>> {
          println!("开始分析网站性能: {} | Starting website performance analysis: {}", url, url);
          
          let start = Instant::now();
          
          // 1. 测量TTFB组成部分 | Measure TTFB components
          let ttfb_breakdown = self.measure_ttfb_breakdown(url).await?;
          
          // 2. 分析页面资源和JavaScript | Analyze page resources and JavaScript
          let response = self.client.get(url).send().await?;
          let html = response.text().await?;
          let js_analysis = self.analyze_javascript_impact(&html).await;
          
          // 3. 计算TTI | Calculate TTI
          let tti = self.estimate_tti(&ttfb_breakdown, &js_analysis).await;
          
          // 4. 估算FCP | Estimate FCP
          let fcp = self.estimate_fcp(&ttfb_breakdown, &html);
          
          let total_time = start.elapsed();
          println!("性能分析完成，耗时: {:?} | Performance analysis completed, duration: {:?}", total_time, total_time);
          
          Ok(PerformanceMetrics {
              url: url.to_string(),
              ttfb: ttfb_breakdown.total_ttfb,
              tti: tti,
              fcp: fcp,
              dns_time: ttfb_breakdown.dns_time,
              tcp_time: ttfb_breakdown.tcp_time,
              tls_time: ttfb_breakdown.tls_time,
              server_time: ttfb_breakdown.server_time,
              main_thread_blocking: js_analysis.total_blocking_time,
          })
      }
      
      // 测量TTFB分解 | Measure TTFB breakdown
      async fn measure_ttfb_breakdown(&self, url: &str) -> Result<TTFBBreakdown, Box<dyn std::error::Error>> {
          let start = Instant::now();
          
          // 模拟DNS查询 | Simulate DNS lookup
          let dns_start = Instant::now();
          sleep(Duration::from_millis(50)).await; // 模拟DNS查询 | Simulate DNS lookup
          let dns_time = dns_start.elapsed().as_secs_f64();
          
          // 模拟TCP连接 | Simulate TCP connection  
          let tcp_start = Instant::now();
          sleep(Duration::from_millis(30)).await; // 模拟TCP连接 | Simulate TCP connection
          let tcp_time = tcp_start.elapsed().as_secs_f64();
          
          // 模拟TLS握手 | Simulate TLS handshake
          let tls_start = Instant::now();
          if url.starts_with("https") {
              sleep(Duration::from_millis(100)).await; // HTTPS握手 | HTTPS handshake
          }
          let tls_time = tls_start.elapsed().as_secs_f64();
          
          // 实际HTTP请求 | Actual HTTP request
          let request_start = Instant::now();
          let _response = self.client.get(url).send().await?;
          let request_time = request_start.elapsed().as_secs_f64();
          
          let total_ttfb = start.elapsed().as_secs_f64();
          let server_time = request_time - tls_time - tcp_time - dns_time;
          
          println!("TTFB分解分析 | TTFB Breakdown Analysis:");
          println!("  DNS查询: {:.3}s | DNS Lookup: {:.3}s", dns_time);
          println!("  TCP连接: {:.3}s | TCP Connection: {:.3}s", tcp_time);
          println!("  TLS握手: {:.3}s | TLS Handshake: {:.3}s", tls_time);
          println!("  服务器处理: {:.3}s | Server Processing: {:.3}s", server_time);
          println!("  总TTFB: {:.3}s | Total TTFB: {:.3}s", total_ttfb);
          
          Ok(TTFBBreakdown {
              dns_time,
              tcp_time,
              tls_time,
              server_time,
              total_ttfb,
          })
      }
      
      // 分析JavaScript影响 | Analyze JavaScript impact
      async fn analyze_javascript_impact(&self, html: &str) -> JavaScriptAnalysis {
          let mut js_files = Vec::new();
          let mut total_blocking_time = 0.0;
          
          // 简单解析JavaScript文件 | Simple JavaScript file parsing
          for line in html.lines() {
              if line.contains("<script") && line.contains("src=") {
                  // 提取JavaScript URL | Extract JavaScript URL
                  if let Some(start) = line.find("src=\"") {
                      let start = start + 5;
                      if let Some(end) = line[start..].find("\"") {
                          let js_src = &line[start..start + end];
                          js_files.push(js_src.to_string());
                      }
                  }
              }
              
              // 内联脚本 | Inline scripts
              if line.contains("<script") && !line.contains("src=") {
                  total_blocking_time += 0.05; // 估算内联脚本执行时间 | Estimate inline script execution time
              }
          }
          
          // 估算外部JavaScript加载和执行时间 | Estimate external JavaScript loading and execution time
          for js_file in &js_files {
              // 模拟不同类型JavaScript的影响 | Simulate impact of different JavaScript types
              if js_file.contains("framework") || js_file.contains("library") {
                  total_blocking_time += 0.2; // 框架加载时间 | Framework loading time
              } else if js_file.contains("analytics") {
                  total_blocking_time += 0.05; // 分析脚本时间 | Analytics script time
              } else {
                  total_blocking_time += 0.1; // 普通脚本时间 | Regular script time
              }
          }
          
          println!("JavaScript影响分析 | JavaScript Impact Analysis:");
          println!("  JavaScript文件数量: {} | JavaScript file count: {}", js_files.len());
          println!("  预估总阻塞时间: {:.3}s | Estimated total blocking time: {:.3}s", total_blocking_time);
          
          JavaScriptAnalysis {
              js_files,
              total_blocking_time,
          }
      }
      
      // 估算TTI | Estimate TTI
      async fn estimate_tti(&self, ttfb: &TTFBBreakdown, js_analysis: &JavaScriptAnalysis) -> f64 {
          // TTI = TTFB + 资源加载时间 + JavaScript执行时间 + 稳定期等待 | TTI = TTFB + resource loading time + JavaScript execution time + stability wait
          let resource_load_time = 0.5; // 估算关键资源加载时间 | Estimate critical resource loading time
          let stability_wait = 0.3;     // 5秒稳定期的简化模拟 | Simplified simulation of 5-second stability period
          
          let tti = ttfb.total_ttfb + resource_load_time + js_analysis.total_blocking_time + stability_wait;
          
          println!("TTI计算 | TTI Calculation:");
          println!("  TTFB: {:.3}s", ttfb.total_ttfb);
          println!("  资源加载: {:.3}s | Resource Loading: {:.3}s", resource_load_time);
          println!("  JavaScript阻塞: {:.3}s | JavaScript Blocking: {:.3}s", js_analysis.total_blocking_time);
          println!("  稳定期等待: {:.3}s | Stability Wait: {:.3}s", stability_wait);
          println!("  预估TTI: {:.3}s | Estimated TTI: {:.3}s", tti);
          
          tti
      }
      
      // 估算FCP | Estimate FCP
      fn estimate_fcp(&self, ttfb: &TTFBBreakdown, html: &str) -> f64 {
          // FCP = TTFB + 关键CSS下载 + 首次内容渲染 | FCP = TTFB + critical CSS download + first content rendering
          let css_count = html.matches("stylesheet").count();
          let css_load_time = (css_count as f64) * 0.1; // 估算CSS加载时间 | Estimate CSS loading time
          let render_time = 0.05; // 估算渲染时间 | Estimate rendering time
          
          ttfb.total_ttfb + css_load_time + render_time
      }
      
      // 生成性能优化建议 | Generate performance optimization suggestions
      pub fn get_performance_suggestions(&self, metrics: &PerformanceMetrics) -> Vec<String> {
          let mut suggestions = Vec::new();
          
          // TTFB优化建议 | TTFB optimization suggestions
          if metrics.ttfb > 0.8 {
              suggestions.push("Critical: TTFB过长，优先优化服务器响应时间 | TTFB too long, prioritize server response time optimization".to_string());
              
              if metrics.dns_time > 0.1 {
                  suggestions.push("DNS: 使用更快的DNS服务或DNS预解析 | Use faster DNS service or DNS prefetch".to_string());
              }
              
              if metrics.server_time > 0.5 {
                  suggestions.push("Server: 优化后端代码和数据库查询 | Optimize backend code and database queries".to_string());
              }
              
              if metrics.tls_time > 0.15 {
                  suggestions.push("TLS: 优化TLS配置，启用OCSP装订 | Optimize TLS configuration, enable OCSP stapling".to_string());
              }
          }
          
          // TTI优化建议 | TTI optimization suggestions
          if metrics.tti > 5.0 {
              suggestions.push("Critical: TTI过长，影响用户体验 | TTI too long, impacts user experience".to_string());
              suggestions.push("JavaScript: 减少JavaScript包大小，实施代码分割 | Reduce JavaScript bundle size, implement code splitting".to_string());
          }
          
          if metrics.main_thread_blocking > 0.3 {
              suggestions.push("Blocking: 拆分长时间运行的JavaScript任务 | Break up long-running JavaScript tasks".to_string());
          }
          
          // FCP优化建议 | FCP optimization suggestions
          if metrics.fcp > 2.5 {
              suggestions.push("FCP: 优化关键渲染路径，预加载重要资源 | Optimize critical rendering path, preload important resources".to_string());
          }
          
          suggestions.push("Monitor: 建议设置持续的性能监控 | Recommend setting up continuous performance monitoring".to_string());
          
          suggestions
      }
  }
  
  // 辅助结构体 | Helper structs
  #[derive(Debug)]
  struct TTFBBreakdown {
      dns_time: f64,
      tcp_time: f64,
      tls_time: f64,
      server_time: f64,
      total_ttfb: f64,
  }
  
  #[derive(Debug)]
  struct JavaScriptAnalysis {
      js_files: Vec<String>,
      total_blocking_time: f64,
  }
  
  // 性能分析示例 | Performance analysis example
  pub async fn run_performance_analysis() {
      let analyzer = WebPerformanceAnalyzer::new();
      
      let test_urls = vec![
          "https://example.com",
          "https://google.com",
          "https://github.com",
      ];
      
      for url in test_urls {
          match analyzer.analyze_performance(url).await {
              Ok(metrics) => {
                  println!("\n=== 性能分析结果 | Performance Analysis Results ===");
                  println!("URL: {}", metrics.url);
                  println!("TTFB: {:.3}s", metrics.ttfb);
                  println!("FCP: {:.3}s", metrics.fcp);
                  println!("TTI: {:.3}s", metrics.tti);
                  
                  let suggestions = analyzer.get_performance_suggestions(&metrics);
                  println!("\n优化建议 | Optimization Suggestions:");
                  for (i, suggestion) in suggestions.iter().enumerate() {
                      println!("{}. {}", i + 1, suggestion);
                  }
              }
              Err(e) => {
                  println!("分析失败 | Analysis failed: {}", e);
              }
          }
          
          println!("\n" + &"=".repeat(50) + "\n");
      }
  }
  ```

### 6. 网络性能优化实践 | Network Performance Optimization Practice (30分钟 | 30 minutes)

- **网络延迟与带宽优化策略 | Network Latency and Bandwidth Optimization Strategies**
  
  **概念定义 | Concept Definition:**
  网络性能优化通过减少延迟、提高带宽利用率和优化传输协议来改善Web应用的加载速度和用户体验。| Network performance optimization improves web application loading speed and user experience by reducing latency, improving bandwidth utilization, and optimizing transmission protocols.
  
  **核心优化原则 | Core Optimization Principles:**
  - 减少往返次数：合并请求，减少HTTP连接数 | Reduce round trips: merge requests, reduce HTTP connections
  - 优化传输内容：压缩、缓存、CDN加速 | Optimize transmitted content: compression, caching, CDN acceleration  
  - 改进协议效率：HTTP/2、HTTP/3的多路复用 | Improve protocol efficiency: HTTP/2, HTTP/3 multiplexing
  
  **概念检查问题 | Concept Checking Questions (CCQs):**
  1. HTTP/2的多路复用解决了什么问题？| What problem does HTTP/2 multiplexing solve?
     **答案 | Answer:** 队头阻塞 | Head-of-line blocking - 允许在单个连接上并行发送多个请求 | Allows parallel sending of multiple requests on single connection
  2. CDN主要优化什么性能指标？| What performance metric does CDN primarily optimize?
     **答案 | Answer:** 网络延迟 | Network latency - 通过地理位置就近提供内容 | By providing content from geographically closer locations
  3. Gzip压缩会增加服务器CPU负载吗？| Does Gzip compression increase server CPU load?
     **答案 | Answer:** 会 | Yes - 但通常带宽节省的收益大于CPU开销 | But bandwidth savings usually outweigh CPU overhead
  4. 浏览器并发连接数限制影响加载速度吗？| Do browser concurrent connection limits affect loading speed?
     **答案 | Answer:** 会 | Yes - HTTP/1.1通常限制每域名6个连接 | HTTP/1.1 typically limits 6 connections per domain
  
  **代码示例与验证 | Code Examples and Verification:**
  ```rust
  // 网络性能优化工具集 | Network performance optimization toolkit
  use reqwest::{Client, header};
  use std::collections::HashMap;
  use std::time::{Duration, Instant};
  use tokio::time::sleep;
  use serde::{Deserialize, Serialize};
  
  #[derive(Debug, Serialize, Deserialize)]
  pub struct NetworkOptimizationReport {
      pub url: String,
      pub http_version: String,
      pub compression_enabled: bool,
      pub cache_headers: HashMap<String, String>,
      pub connection_reuse: bool,
      pub resource_count: usize,
      pub total_size: u64,
      pub compressed_size: u64,
      pub load_time: f64,
      pub suggestions: Vec<String>,
  }
  
  #[derive(Debug)]
  pub struct NetworkOptimizer {
      client: Client,
  }
  
  impl NetworkOptimizer {
      pub fn new() -> Self {
          Self {
              client: Client::builder()
                  .timeout(Duration::from_secs(30))
                  .gzip(true)  // 启用Gzip解压 | Enable Gzip decompression
                  .build()
                  .unwrap(),
          }
      }
      
      // 网络性能综合分析 | Comprehensive network performance analysis
      pub async fn analyze_network_performance(&self, url: &str) -> Result<NetworkOptimizationReport, Box<dyn std::error::Error>> {
          println!("开始网络性能分析: {} | Starting network performance analysis: {}", url, url);
          
          let start = Instant::now();
          
          // 1. 分析主页面 | Analyze main page
          let response = self.client.get(url).send().await?;
          let load_time = start.elapsed().as_secs_f64();
          
          // 2. 提取响应头信息 | Extract response headers
          let headers = response.headers();
          let mut cache_headers = HashMap::new();
          
          // 检查缓存相关头部 | Check cache-related headers
          for cache_header in ["cache-control", "expires", "etag", "last-modified"] {
              if let Some(value) = headers.get(cache_header) {
                  cache_headers.insert(
                      cache_header.to_string(), 
                      value.to_str().unwrap_or("").to_string()
                  );
              }
          }
          
          // 3. 检查压缩 | Check compression
          let compression_enabled = headers.get("content-encoding")
              .map(|v| v.to_str().unwrap_or("").contains("gzip") || v.to_str().unwrap_or("").contains("br"))
              .unwrap_or(false);
          
          // 4. 获取HTTP版本 | Get HTTP version
          let http_version = format!("{:?}", response.version());
          
          // 5. 分析页面内容 | Analyze page content
          let html = response.text().await?;
          let resource_analysis = self.analyze_page_resources(&html).await;
          
          // 6. 生成优化建议 | Generate optimization suggestions
          let suggestions = self.generate_network_suggestions(
              &cache_headers, 
              compression_enabled, 
              &http_version, 
              &resource_analysis
          );
          
          println!("网络性能分析完成 | Network performance analysis completed");
          
          Ok(NetworkOptimizationReport {
              url: url.to_string(),
              http_version,
              compression_enabled,
              cache_headers,
              connection_reuse: resource_analysis.connection_reuse,
              resource_count: resource_analysis.total_resources,
              total_size: resource_analysis.total_size,
              compressed_size: resource_analysis.compressed_size,
              load_time,
              suggestions,
          })
      }
      
      // 分析页面资源 | Analyze page resources
      async fn analyze_page_resources(&self, html: &str) -> ResourceAnalysis {
          let mut total_resources = 0;
          let mut total_size = 0u64;
          let mut compressed_size = 0u64;
          
          // 分析图片资源 | Analyze image resources
          let image_count = html.matches("<img").count();
          total_resources += image_count;
          total_size += (image_count as u64) * 50000; // 假设平均50KB每张图片 | Assume average 50KB per image
          compressed_size += (image_count as u64) * 35000; // Gzip压缩后 | After Gzip compression
          
          // 分析CSS资源 | Analyze CSS resources  
          let css_count = html.matches("stylesheet").count();
          total_resources += css_count;
          total_size += (css_count as u64) * 30000; // 假设平均30KB每个CSS | Assume average 30KB per CSS
          compressed_size += (css_count as u64) * 8000; // CSS压缩效果好 | CSS compresses well
          
          // 分析JavaScript资源 | Analyze JavaScript resources
          let js_count = html.matches("<script").count();
          total_resources += js_count;
          total_size += (js_count as u64) * 80000; // 假设平均80KB每个JS | Assume average 80KB per JS
          compressed_size += (js_count as u64) * 25000; // JavaScript压缩效果 | JavaScript compression effect
          
          println!("资源分析结果 | Resource Analysis Results:");
          println!("  图片资源: {} | Image resources: {}", image_count);
          println!("  CSS资源: {} | CSS resources: {}", css_count);
          println!("  JavaScript资源: {} | JavaScript resources: {}", js_count);
          println!("  总资源数: {} | Total resources: {}", total_resources);
          println!("  预估原始大小: {:.2} MB | Estimated original size: {:.2} MB", total_size as f64 / 1024.0 / 1024.0);
          println!("  预估压缩后大小: {:.2} MB | Estimated compressed size: {:.2} MB", compressed_size as f64 / 1024.0 / 1024.0);
          
          ResourceAnalysis {
              total_resources,
              total_size,
              compressed_size,
              connection_reuse: total_resources < 10, // 简化判断 | Simplified judgment
          }
      }
      
      // 生成网络优化建议 | Generate network optimization suggestions
      fn generate_network_suggestions(
          &self,
          cache_headers: &HashMap<String, String>,
          compression_enabled: bool,
          http_version: &str,
          resource_analysis: &ResourceAnalysis
      ) -> Vec<String> {
          let mut suggestions = Vec::new();
          
          // HTTP版本建议 | HTTP version suggestions
          if !http_version.contains("HTTP/2") && !http_version.contains("HTTP/3") {
              suggestions.push("Critical: 升级到HTTP/2或HTTP/3以启用多路复用 | Upgrade to HTTP/2 or HTTP/3 to enable multiplexing".to_string());
          }
          
          // 压缩建议 | Compression suggestions
          if !compression_enabled {
              suggestions.push("Critical: 启用Gzip或Brotli压缩减少传输大小 | Enable Gzip or Brotli compression to reduce transfer size".to_string());
          }
          
          // 缓存建议 | Caching suggestions
          if !cache_headers.contains_key("cache-control") {
              suggestions.push("Important: 设置适当的Cache-Control头提高缓存效率 | Set appropriate Cache-Control headers to improve caching efficiency".to_string());
          }
          
          if !cache_headers.contains_key("etag") && !cache_headers.contains_key("last-modified") {
              suggestions.push("Important: 添加ETag或Last-Modified头启用条件请求 | Add ETag or Last-Modified headers to enable conditional requests".to_string());
          }
          
          // 资源优化建议 | Resource optimization suggestions
          if resource_analysis.total_resources > 30 {
              suggestions.push("Performance: 资源数量过多，考虑合并CSS和JavaScript文件 | Too many resources, consider combining CSS and JavaScript files".to_string());
          }
          
          if resource_analysis.total_size > 2_000_000 { // 2MB
              suggestions.push("Performance: 页面资源过大，考虑图片压缩和代码分割 | Page resources too large, consider image compression and code splitting".to_string());
          }
          
          let compression_ratio = resource_analysis.compressed_size as f64 / resource_analysis.total_size as f64;
          if compression_ratio > 0.7 {
              suggestions.push("Optimization: 压缩效率不理想，检查是否已压缩过的资源被重复压缩 | Compression efficiency suboptimal, check if pre-compressed resources are being re-compressed".to_string());
          }
          
          // CDN建议 | CDN suggestions
          suggestions.push("Infrastructure: 考虑使用CDN加速静态资源分发 | Consider using CDN to accelerate static resource distribution".to_string());
          
          // 资源预加载建议 | Resource preloading suggestions
          suggestions.push("Loading: 使用<link rel='preload'>预加载关键资源 | Use <link rel='preload'> to preload critical resources".to_string());
          
          suggestions
      }
      
      // 网络连接测试 | Network connection testing
      pub async fn test_connection_performance(&self, url: &str, concurrent_requests: usize) -> Result<ConnectionTestResult, Box<dyn std::error::Error>> {
          println!("开始连接性能测试: {} ({} 并发请求) | Starting connection performance test: {} ({} concurrent requests)", 
                  url, concurrent_requests, url, concurrent_requests);
          
          let start = Instant::now();
          let mut handles = Vec::new();
          
          // 并发发起多个请求 | Initiate multiple concurrent requests
          for i in 0..concurrent_requests {
              let client = self.client.clone();
              let test_url = url.to_string();
              
              let handle = tokio::spawn(async move {
                  let request_start = Instant::now();
                  let result = client.get(&test_url).send().await;
                  let request_time = request_start.elapsed();
                  
                  match result {
                      Ok(response) => {
                          println!("请求 {} 完成: {:?} (状态: {}) | Request {} completed: {:?} (status: {})", 
                                  i + 1, request_time, response.status(), i + 1, request_time, response.status());
                          Ok(request_time)
                      }
                      Err(e) => {
                          println!("请求 {} 失败: {} | Request {} failed: {}", i + 1, e, i + 1, e);
                          Err(e)
                      }
                  }
              });
              
              handles.push(handle);
          }
          
          // 等待所有请求完成 | Wait for all requests to complete
          let mut successful_requests = Vec::new();
          let mut failed_requests = 0;
          
          for handle in handles {
              match handle.await {
                  Ok(Ok(duration)) => successful_requests.push(duration),
                  Ok(Err(_)) => failed_requests += 1,
                  Err(_) => failed_requests += 1,
              }
          }
          
          let total_time = start.elapsed();
          
          // 计算统计数据 | Calculate statistics
          let success_count = successful_requests.len();
          let avg_time = if success_count > 0 {
              successful_requests.iter().sum::<Duration>().as_secs_f64() / success_count as f64
          } else {
              0.0
          };
          
          let min_time = successful_requests.iter().min().copied().unwrap_or(Duration::from_secs(0)).as_secs_f64();
          let max_time = successful_requests.iter().max().copied().unwrap_or(Duration::from_secs(0)).as_secs_f64();
          
          println!("\n连接性能测试结果 | Connection Performance Test Results:");
          println!("  总请求数: {} | Total requests: {}", concurrent_requests);
          println!("  成功请求数: {} | Successful requests: {}", success_count);
          println!("  失败请求数: {} | Failed requests: {}", failed_requests);
          println!("  平均响应时间: {:.3}s | Average response time: {:.3}s", avg_time);
          println!("  最快响应时间: {:.3}s | Fastest response time: {:.3}s", min_time);
          println!("  最慢响应时间: {:.3}s | Slowest response time: {:.3}s", max_time);
          println!("  总测试时间: {:.3}s | Total test time: {:.3}s", total_time.as_secs_f64());
          
          Ok(ConnectionTestResult {
              total_requests: concurrent_requests,
              successful_requests: success_count,
              failed_requests,
              avg_response_time: avg_time,
              min_response_time: min_time,
              max_response_time: max_time,
              total_test_time: total_time.as_secs_f64(),
          })
      }
  }
  
  // 辅助结构体 | Helper structs
  #[derive(Debug)]
  struct ResourceAnalysis {
      total_resources: usize,
      total_size: u64,
      compressed_size: u64,
      connection_reuse: bool,
  }
  
  #[derive(Debug, Serialize, Deserialize)]
  pub struct ConnectionTestResult {
      pub total_requests: usize,
      pub successful_requests: usize,
      pub failed_requests: usize,
      pub avg_response_time: f64,
      pub min_response_time: f64,
      pub max_response_time: f64,
      pub total_test_time: f64,
  }
  ```

## 实践项目：HTTP性能分析工具 | Practical Project: HTTP Performance Analysis Tool

### 目标 | Objective
构建一个综合性的HTTP性能分析工具，集成Core Web Vitals监控、TTFB分析、网络优化建议等功能，帮助开发者系统性地分析和优化Web应用性能。| Build a comprehensive HTTP performance analysis tool that integrates Core Web Vitals monitoring, TTFB analysis, network optimization suggestions, and other features to help developers systematically analyze and optimize web application performance.

### 概念应用检查 | Concept Application Check
在开始项目前，请确认对以下概念的理解 | Before starting the project, please confirm understanding of the following concepts:

1. Core Web Vitals的三个核心指标是什么？| What are the three core metrics of Core Web Vitals?
   **答案 | Answer:** LCP (Largest Contentful Paint)、FID (First Input Delay)、CLS (Cumulative Layout Shift) - 分别测量加载性能、交互性能和视觉稳定性 | Respectively measure loading performance, interactivity performance, and visual stability
2. TTFB包含哪些时间组成部分？| What time components does TTFB include?
   **答案 | Answer:** DNS查询时间、TCP连接时间、TLS握手时间、服务器处理时间 | DNS lookup time, TCP connection time, TLS handshake time, server processing time
3. 如何判断一个网站的性能是否达到"Good"标准？| How to determine if a website's performance meets "Good" standards?
   **答案 | Answer:** LCP ≤ 2.5s, FID ≤ 100ms, CLS ≤ 0.1, TTFB ≤ 800ms | LCP ≤ 2.5s, FID ≤ 100ms, CLS ≤ 0.1, TTFB ≤ 800ms

### 步骤 | Steps
1. 创建项目结构和基础配置 | Create project structure and basic configuration
2. 实现Core Web Vitals监控模块 | Implement Core Web Vitals monitoring module  
3. 开发TTFB和TTI分析功能 | Develop TTFB and TTI analysis functionality
4. 集成网络性能优化建议系统 | Integrate network performance optimization suggestion system
5. 构建CLI界面和报告生成器 | Build CLI interface and report generator

### 示例代码 | Example Code
```rust
"""
HTTP性能分析工具 | HTTP Performance Analysis Tool
综合性能分析工具，帮助开发者优化Web应用性能 | Comprehensive performance analysis tool to help developers optimize web application performance

本项目演示以下概念的综合应用：| This project demonstrates comprehensive application of:
- Core Web Vitals监控 | Core Web Vitals monitoring
- TTFB和TTI性能分析 | TTFB and TTI performance analysis
- 网络优化建议生成 | Network optimization suggestion generation
"""

// 主程序入口 | Main program entry
use clap::{App, Arg, SubCommand};
use tokio;
use std::fs;
use serde_json;

// 引入我们之前开发的模块 | Import previously developed modules
mod core_web_vitals;
mod performance_analyzer;
mod network_optimizer;

use core_web_vitals::{CoreWebVitals, LCPAnalyzer, FIDSimulator, CLSAnalyzer};
use performance_analyzer::WebPerformanceAnalyzer;
use network_optimizer::NetworkOptimizer;

#[derive(Debug, serde::Serialize)]
struct ComprehensiveReport {
    pub url: String,
    pub timestamp: u64,
    pub core_web_vitals: CoreWebVitals,
    pub performance_metrics: performance_analyzer::PerformanceMetrics,
    pub network_report: network_optimizer::NetworkOptimizationReport,
    pub overall_score: f64,
    pub priority_suggestions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("HTTP性能分析工具 | HTTP Performance Analysis Tool")
        .version("1.0")
        .author("Your Name")
        .about("全面分析Web应用性能并提供优化建议 | Comprehensive web application performance analysis with optimization suggestions")
        .subcommand(SubCommand::with_name("analyze")
            .about("分析单个网站性能 | Analyze single website performance")
            .arg(Arg::with_name("url")
                .help("要分析的网站URL | Website URL to analyze")
                .required(true)
                .index(1))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("报告输出文件 | Report output file")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("batch")
            .about("批量分析多个网站 | Batch analyze multiple websites")
            .arg(Arg::with_name("urls-file")
                .help("包含URL列表的文件 | File containing URL list")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("monitor")
            .about("持续监控网站性能 | Continuously monitor website performance")
            .arg(Arg::with_name("url")
                .help("要监控的网站URL | Website URL to monitor")
                .required(true)
                .index(1))
            .arg(Arg::with_name("interval")
                .short("i")
                .long("interval")
                .value_name("SECONDS")
                .help("监控间隔(秒) | Monitoring interval (seconds)")
                .takes_value(true)
                .default_value("300")))
        .get_matches();

    match matches.subcommand() {
        ("analyze", Some(analyze_matches)) => {
            let url = analyze_matches.value_of("url").unwrap();
            let output_file = analyze_matches.value_of("output");
            
            println!("🚀 开始分析网站性能: {} | Starting website performance analysis: {}", url, url);
            
            let report = analyze_website_comprehensive(url).await?;
            
            if let Some(output_path) = output_file {
                save_report_to_file(&report, output_path)?;
                println!("📊 报告已保存到: {} | Report saved to: {}", output_path, output_path);
            }
            
            display_comprehensive_report(&report);
        }
        
        ("batch", Some(batch_matches)) => {
            let urls_file = batch_matches.value_of("urls-file").unwrap();
            run_batch_analysis(urls_file).await?;
        }
        
        ("monitor", Some(monitor_matches)) => {
            let url = monitor_matches.value_of("url").unwrap();
            let interval = monitor_matches.value_of("interval").unwrap().parse::<u64>()?;
            run_continuous_monitoring(url, interval).await?;
        }
        
        _ => {
            println!("使用 --help 查看使用说明 | Use --help for usage information");
        }
    }

    Ok(())
}

// 综合网站性能分析 | Comprehensive website performance analysis
async fn analyze_website_comprehensive(url: &str) -> Result<ComprehensiveReport, Box<dyn std::error::Error>> {
    println!("📈 开始综合性能分析... | Starting comprehensive performance analysis...");
    
    // 1. Core Web Vitals分析 | Core Web Vitals analysis
    println!("🎯 分析Core Web Vitals... | Analyzing Core Web Vitals...");
    let mut cwv = CoreWebVitals::new();
    
    // 使用LCP分析器 | Use LCP analyzer
    let lcp_analyzer = LCPAnalyzer::new();
    let estimated_lcp = lcp_analyzer.measure_lcp(url).await?;
    cwv.lcp = Some(estimated_lcp);
    
    // 模拟FID测量 | Simulate FID measurement
    let mut fid_simulator = FIDSimulator::new();
    fid_simulator.add_task("Framework Init", 200, true);
    fid_simulator.add_task("Event Binding", 50, true);
    let _blocking_time = fid_simulator.simulate_page_load().await;
    let simulated_fid = fid_simulator.simulate_user_interaction("click").await;
    cwv.fid = Some(simulated_fid.as_millis() as f64);
    
    // 模拟CLS测量 | Simulate CLS measurement
    let mut cls_analyzer = CLSAnalyzer::new();
    cls_analyzer.record_shift(core_web_vitals::LayoutShift::new(500.0, 0.2, 0.3, "img"));
    cls_analyzer.record_shift(core_web_vitals::LayoutShift::new(800.0, 0.1, 0.2, "ad"));
    let cls_score = cls_analyzer.calculate_cls_score();
    cwv.cls = Some(cls_score);
    
    // 2. 详细性能指标分析 | Detailed performance metrics analysis
    println!("⚡ 分析详细性能指标... | Analyzing detailed performance metrics...");
    let perf_analyzer = WebPerformanceAnalyzer::new();
    let performance_metrics = perf_analyzer.analyze_performance(url).await?;
    
    // 3. 网络优化分析 | Network optimization analysis
    println!("🌐 分析网络优化机会... | Analyzing network optimization opportunities...");
    let network_optimizer = NetworkOptimizer::new();
    let network_report = network_optimizer.analyze_network_performance(url).await?;
    
    // 4. 计算综合评分 | Calculate overall score
    let overall_score = calculate_overall_score(&cwv, &performance_metrics, &network_report);
    
    // 5. 生成优先级建议 | Generate priority suggestions
    let priority_suggestions = generate_priority_suggestions(&cwv, &performance_metrics, &network_report);
    
    println!("✅ 综合性能分析完成 | Comprehensive performance analysis completed");
    
    Ok(ComprehensiveReport {
        url: url.to_string(),
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
        core_web_vitals: cwv,
        performance_metrics,
        network_report,
        overall_score,
        priority_suggestions,
    })
}

// 计算综合评分 | Calculate overall score
fn calculate_overall_score(
    cwv: &CoreWebVitals,
    perf: &performance_analyzer::PerformanceMetrics,
    network: &network_optimizer::NetworkOptimizationReport,
) -> f64 {
    let mut score = 100.0;
    
    // Core Web Vitals评分 (40分权重) | Core Web Vitals scoring (40 points weight)
    if let Some(lcp) = cwv.lcp {
        score -= if lcp > 4.0 { 20.0 } else if lcp > 2.5 { 10.0 } else { 0.0 };
    }
    
    if let Some(fid) = cwv.fid {
        score -= if fid > 300.0 { 15.0 } else if fid > 100.0 { 5.0 } else { 0.0 };
    }
    
    if let Some(cls) = cwv.cls {
        score -= if cls > 0.25 { 10.0 } else if cls > 0.1 { 5.0 } else { 0.0 };
    }
    
    // TTFB评分 (30分权重) | TTFB scoring (30 points weight)
    score -= if perf.ttfb > 1.5 { 20.0 } else if perf.ttfb > 0.8 { 10.0 } else { 0.0 };
    
    // TTI评分 (20分权重) | TTI scoring (20 points weight)  
    score -= if perf.tti > 7.0 { 15.0 } else if perf.tti > 5.0 { 5.0 } else { 0.0 };
    
    // 网络优化评分 (10分权重) | Network optimization scoring (10 points weight)
    if !network.compression_enabled { score -= 5.0; }
    if network.cache_headers.is_empty() { score -= 5.0; }
    
    score.max(0.0)
}

// 生成优先级建议 | Generate priority suggestions
fn generate_priority_suggestions(
    cwv: &CoreWebVitals,
    perf: &performance_analyzer::PerformanceMetrics,
    network: &network_optimizer::NetworkOptimizationReport,
) -> Vec<String> {
    let mut suggestions = Vec::new();
    
    // 根据影响程度排序建议 | Sort suggestions by impact level
    let mut critical_issues = Vec::new();
    let mut important_issues = Vec::new();
    let mut minor_issues = Vec::new();
    
    // 检查关键问题 | Check critical issues
    if let Some(lcp) = cwv.lcp {
        if lcp > 4.0 {
            critical_issues.push("🔴 LCP过长严重影响用户体验，需立即优化 | LCP too long severely impacts user experience, requires immediate optimization".to_string());
        }
    }
    
    if perf.ttfb > 1.5 {
        critical_issues.push("🔴 TTFB过长，优先优化服务器响应速度 | TTFB too long, prioritize server response speed optimization".to_string());
    }
    
    if let Some(cls) = cwv.cls {
        if cls > 0.25 {
            critical_issues.push("🔴 布局偏移严重，影响用户交互体验 | Severe layout shifts impacting user interaction experience".to_string());
        }
    }
    
    // 检查重要问题 | Check important issues
    if !network.compression_enabled {
        important_issues.push("🟡 未启用压缩，可显著减少传输大小 | Compression not enabled, can significantly reduce transfer size".to_string());
    }
    
    if perf.tti > 5.0 {
        important_issues.push("🟡 页面交互就绪时间过长，优化JavaScript执行 | Page interaction ready time too long, optimize JavaScript execution".to_string());
    }
    
    // 检查次要问题 | Check minor issues
    if network.cache_headers.is_empty() {
        minor_issues.push("🟢 添加缓存头可提高重复访问性能 | Adding cache headers can improve repeat visit performance".to_string());
    }
    
    if network.resource_count > 30 {
        minor_issues.push("🟢 考虑合并资源文件减少HTTP请求数 | Consider merging resource files to reduce HTTP request count".to_string());
    }
    
    // 合并所有建议 | Merge all suggestions
    suggestions.extend(critical_issues);
    suggestions.extend(important_issues);
    suggestions.extend(minor_issues);
    
    // 限制建议数量 | Limit suggestion count
    suggestions.into_iter().take(8).collect()
}

// 显示综合报告 | Display comprehensive report
fn display_comprehensive_report(report: &ComprehensiveReport) {
    println!("\n🎯 ==================== 性能分析报告 | Performance Analysis Report ====================");
    println!("📝 网站: {} | Website: {}", report.url);
    println!("⏰ 分析时间: {} | Analysis time: {}", 
             chrono::DateTime::<chrono::Utc>::from_timestamp_millis(report.timestamp as i64).unwrap().format("%Y-%m-%d %H:%M:%S UTC"));
    println!("📊 综合评分: {:.1}/100 | Overall score: {:.1}/100", report.overall_score);
    
    // 显示评分等级 | Display score grade
    let grade = if report.overall_score >= 90.0 { "🟢 优秀 | Excellent" }
    else if report.overall_score >= 75.0 { "🟡 良好 | Good" }
    else if report.overall_score >= 60.0 { "🟠 需要改进 | Needs Improvement" }
    else { "🔴 差 | Poor" };
    println!("📈 性能等级: {} | Performance grade: {}", grade, grade);
    
    // Core Web Vitals
    println!("\n📋 Core Web Vitals:");
    if let Some(lcp) = report.core_web_vitals.lcp {
        println!("  📏 LCP: {:.3}s ({})", lcp, report.core_web_vitals.lcp_score());
    }
    if let Some(fid) = report.core_web_vitals.fid {
        println!("  ⚡ FID: {:.0}ms ({})", fid, report.core_web_vitals.fid_score());
    }
    if let Some(cls) = report.core_web_vitals.cls {
        println!("  📐 CLS: {:.3} ({})", cls, report.core_web_vitals.cls_score());
    }
    
    // 详细性能指标 | Detailed performance metrics
    println!("\n⚡ 详细性能指标 | Detailed Performance Metrics:");
    println!("  ⏱️  TTFB: {:.3}s", report.performance_metrics.ttfb);
    println!("  🎯 TTI: {:.3}s", report.performance_metrics.tti);
    println!("  🎨 FCP: {:.3}s", report.performance_metrics.fcp);
    println!("  🌐 DNS: {:.3}s", report.performance_metrics.dns_time);
    println!("  🔗 TCP: {:.3}s", report.performance_metrics.tcp_time);
    println!("  🔐 TLS: {:.3}s", report.performance_metrics.tls_time);
    println!("  🖥️  服务器: {:.3}s | Server: {:.3}s", report.performance_metrics.server_time);
    
    // 网络优化信息 | Network optimization information
    println!("\n🌐 网络优化信息 | Network Optimization Information:");
    println!("  📦 HTTP版本: {} | HTTP Version: {}", report.network_report.http_version);
    println!("  🗜️  压缩启用: {} | Compression enabled: {}", 
             if report.network_report.compression_enabled { "✅ 是 | Yes" } else { "❌ 否 | No" });
    println!("  📁 资源数量: {} | Resource count: {}", report.network_report.resource_count);
    println!("  💾 预估大小: {:.2} MB | Estimated size: {:.2} MB", 
             report.network_report.total_size as f64 / 1024.0 / 1024.0);
    println!("  ⏱️  加载时间: {:.3}s | Load time: {:.3}s", report.network_report.load_time);
    
    // 优先级建议 | Priority suggestions
    println!("\n🎯 优化建议 (按优先级排序) | Optimization Suggestions (Prioritized):");
    for (i, suggestion) in report.priority_suggestions.iter().enumerate() {
        println!("  {}. {}", i + 1, suggestion);
    }
    
    println!("\n" + &"=".repeat(100));
}

// 保存报告到文件 | Save report to file
fn save_report_to_file(report: &ComprehensiveReport, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(report)?;
    fs::write(file_path, json)?;
    Ok(())
}

// 批量分析 | Batch analysis
async fn run_batch_analysis(urls_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let urls_content = fs::read_to_string(urls_file)?;
    let urls: Vec<&str> = urls_content.lines().collect();
    
    println!("🚀 开始批量分析 {} 个网站 | Starting batch analysis of {} websites", urls.len(), urls.len());
    
    let mut reports = Vec::new();
    
    for (i, url) in urls.iter().enumerate() {
        println!("\n📊 ({}/{}) 分析: {} | Analyzing: {}", i + 1, urls.len(), url, url);
        
        match analyze_website_comprehensive(url).await {
            Ok(report) => {
                reports.push(report);
            }
            Err(e) => {
                println!("❌ 分析失败: {} | Analysis failed: {}", e, e);
            }
        }
        
        // 避免过于频繁的请求 | Avoid too frequent requests
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
    // 生成汇总报告 | Generate summary report
    generate_batch_summary(&reports);
    
    Ok(())
}

// 生成批量分析汇总 | Generate batch analysis summary
fn generate_batch_summary(reports: &[ComprehensiveReport]) {
    if reports.is_empty() {
        return;
    }
    
    println!("\n📈 ==================== 批量分析汇总 | Batch Analysis Summary ====================");
    
    let avg_score: f64 = reports.iter().map(|r| r.overall_score).sum::<f64>() / reports.len() as f64;
    let avg_lcp: f64 = reports.iter()
        .filter_map(|r| r.core_web_vitals.lcp)
        .sum::<f64>() / reports.len() as f64;
    let avg_ttfb: f64 = reports.iter()
        .map(|r| r.performance_metrics.ttfb)
        .sum::<f64>() / reports.len() as f64;
    
    println!("📊 分析网站数量: {} | Websites analyzed: {}", reports.len());
    println!("📈 平均综合评分: {:.1} | Average overall score: {:.1}", avg_score);
    println!("📏 平均LCP: {:.3}s | Average LCP: {:.3}s", avg_lcp);
    println!("⏱️  平均TTFB: {:.3}s | Average TTFB: {:.3}s", avg_ttfb);
    
    // 找出最好和最差的网站 | Find best and worst performing websites
    let best_site = reports.iter().max_by(|a, b| a.overall_score.partial_cmp(&b.overall_score).unwrap());
    let worst_site = reports.iter().min_by(|a, b| a.overall_score.partial_cmp(&b.overall_score).unwrap());
    
    if let Some(best) = best_site {
        println!("🏆 最佳性能: {} ({:.1}分) | Best performance: {} ({:.1} points)", best.url, best.overall_score);
    }
    
    if let Some(worst) = worst_site {
        println!("⚠️  需优化: {} ({:.1}分) | Needs optimization: {} ({:.1} points)", worst.url, worst.overall_score);
    }
}

// 持续监控 | Continuous monitoring
async fn run_continuous_monitoring(url: &str, interval_seconds: u64) -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 开始持续监控: {} (间隔: {}秒) | Starting continuous monitoring: {} (interval: {}s)", 
             url, interval_seconds, url, interval_seconds);
    
    loop {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
        println!("\n⏰ {} - 执行性能检查 | Performance check", timestamp);
        
        match analyze_website_comprehensive(url).await {
            Ok(report) => {
                // 简化的监控输出 | Simplified monitoring output
                println!("📊 综合评分: {:.1} | Overall score: {:.1}", report.overall_score);
                
                if let Some(lcp) = report.core_web_vitals.lcp {
                    println!("📏 LCP: {:.3}s", lcp);
                }
                
                println!("⏱️  TTFB: {:.3}s", report.performance_metrics.ttfb);
                
                // 检查是否有性能警报 | Check for performance alerts
                if report.overall_score < 70.0 {
                    println!("🚨 性能警报：评分低于70分！| Performance alert: Score below 70!");
                }
                
                // 保存监控数据 | Save monitoring data
                let monitor_file = format!("monitoring_{}.jsonl", 
                                           url.replace("://", "_").replace("/", "_"));
                let monitor_entry = serde_json::to_string(&report)?;
                fs::write(&monitor_file, format!("{}\n", monitor_entry))?;
            }
            Err(e) => {
                println!("❌ 监控检查失败: {} | Monitoring check failed: {}", e, e);
            }
        }
        
        println!("😴 等待下次检查... | Waiting for next check...");
        tokio::time::sleep(tokio::time::Duration::from_secs(interval_seconds)).await;
    }
}
```

### 项目完成检查 | Project Completion Check
1. 项目是否正确应用了Core Web Vitals监控？| Does the project correctly apply Core Web Vitals monitoring?
2. TTFB和TTI分析的使用是否符合最佳实践？| Does the usage of TTFB and TTI analysis follow best practices?
3. 代码是否体现了性能优化建议生成的正确逻辑？| Does the code reflect correct logic for performance optimization suggestion generation?

## 扩展练习 | Extension Exercises

### 概念深化练习 | Concept Deepening Exercises

1. **Core Web Vitals深度理解练习 | Core Web Vitals Deep Understanding Exercise**
   - **练习描述 | Exercise Description:** 分析不同类型网站(电商、新闻、SaaS)的Core Web Vitals特点和优化策略差异
   - **概念检查 | Concept Check:** 理解不同业务场景下的性能指标优先级
   - **学习目标 | Learning Objective:** 深化对性能指标业务价值的理解

2. **TTFB组成分析练习 | TTFB Component Analysis Exercise**
   - **练习描述 | Exercise Description:** 构建TTFB各组成部分的详细测量工具，分析每部分对总体性能的影响
   - **概念检查 | Concept Check:** 掌握DNS、TCP、TLS、服务器处理时间的测量方法
   - **学习目标 | Learning Objective:** 提高网络性能分析的精确度

3. **布局偏移检测练习 | Layout Shift Detection Exercise**
   - **练习描述 | Exercise Description:** 开发实时CLS监控工具，识别造成布局偏移的具体元素
   - **概念检查 | Concept Check:** 理解布局偏移的计算公式和影响因素
   - **学习目标 | Learning Objective:** 发展视觉稳定性优化技能

4. **性能预算管理练习 | Performance Budget Management Exercise**
   - **练习描述 | Exercise Description:** 为不同类型项目制定性能预算和监控告警机制
   - **概念检查 | Concept Check:** 理解性能指标与业务目标的关系
   - **学习目标 | Learning Objective:** 培养性能管理的战略思维

5. **性能回归检测练习 | Performance Regression Detection Exercise**
   - **练习描述 | Exercise Description:** 构建自动化性能回归检测系统，集成到CI/CD流程
   - **概念检查 | Concept Check:** 理解性能基线建立和变化检测方法
   - **学习目标 | Learning Objective:** 发展自动化性能监控能力

6. **用户体验指标关联分析练习 | User Experience Metrics Correlation Exercise**
   - **练习描述 | Exercise Description:** 分析性能指标与用户行为数据的关联性，如跳出率、转化率等
   - **概念检查 | Concept Check:** 理解性能对用户行为的影响机制
   - **学习目标 | Learning Objective:** 通过数据分析巩固性能优化价值理解

7. **多设备性能对比练习 | Multi-device Performance Comparison Exercise**
   - **练习描述 | Exercise Description:** 开发跨设备性能测试工具，分析桌面、移动设备性能差异
   - **概念检查 | Concept Check:** 理解不同设备环境对性能指标的影响
   - **学习目标 | Learning Objective:** 提高响应式性能优化能力

## 学习资源 | Learning Resources
- [Google Web Vitals官方文档 | Google Web Vitals Official Documentation](https://web.dev/vitals/)
- [MDN Performance API参考 | MDN Performance API Reference](https://developer.mozilla.org/en-US/docs/Web/API/Performance)
- [Chrome DevTools性能分析指南 | Chrome DevTools Performance Analysis Guide](https://developers.google.com/web/tools/chrome-devtools/evaluate-performance)
- [Lighthouse性能审计工具 | Lighthouse Performance Audit Tool](https://developers.google.com/web/tools/lighthouse)

---

✅ **完成检查清单 | Completion Checklist**
- [ ] Core Web Vitals三个指标的理解和应用 | Understanding and application of Core Web Vitals three metrics
- [ ] LCP优化策略和实现方法掌握 | LCP optimization strategies and implementation methods mastered
- [ ] FID测量原理和JavaScript性能优化理解 | FID measurement principles and JavaScript performance optimization understood
- [ ] CLS布局稳定性控制方法应用 | CLS layout stability control methods applied
- [ ] TTFB分解分析和服务器优化策略掌握 | TTFB breakdown analysis and server optimization strategies mastered
- [ ] TTI可交互时间优化技巧理解 | TTI interactivity time optimization techniques understood
- [ ] 网络性能优化最佳实践应用 | Network performance optimization best practices applied
- [ ] HTTP性能分析工具项目完成 | HTTP performance analysis tool project completed
- [ ] 性能监控和报告生成系统构建 | Performance monitoring and reporting system built
- [ ] 扩展练习至少完成3个 | At least 3 extension exercises completed

**概念掌握验证 | Concept Mastery Verification:**
在标记完成前，请确保能够正确回答本日所有CCQs，并能够向他人清晰解释Core Web Vitals、TTFB、TTI等核心概念的含义、测量方法和优化策略。
Before marking as complete, ensure you can correctly answer all CCQs from today and clearly explain to others the meaning, measurement methods, and optimization strategies of core concepts like Core Web Vitals, TTFB, TTI.