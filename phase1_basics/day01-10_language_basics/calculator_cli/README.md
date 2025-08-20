# 命令行计算器项目

## 项目概述
这是第一阶段的核心项目，通过构建一个功能完整的命令行计算器来学习Rust基础语法。

## 学习目标
- 掌握Rust基本语法和数据类型
- 理解所有权、借用和生命周期
- 学会错误处理和模式匹配
- 实践结构体和枚举的使用

## 功能规划

### 第一周：基础计算器
- [x] 项目初始化
- [ ] 基本四则运算 (+, -, *, /)
- [ ] 整数和浮点数支持
- [ ] 错误处理 (除零、无效输入)

### 第二周：扩展功能
- [ ] 括号支持
- [ ] 幂运算和根运算
- [ ] 三角函数 (sin, cos, tan)
- [ ] 历史记录功能

### 第三周：高级特性
- [ ] 变量存储
- [ ] 自定义函数
- [ ] 表达式解析器
- [ ] 交互式REPL模式

## 技术栈
- 核心Rust标准库
- 可选依赖：
  - `clap` - 命令行参数解析
  - `rustyline` - 交互式输入
  - `serde` - 配置文件支持

## 运行方式
```bash
# 开发模式
cargo run

# 发布模式
cargo build --release
./target/release/calculator_cli

# 运行测试
cargo test
```

## 项目结构
```
calculator_cli/
├── src/
│   ├── main.rs          # 程序入口
│   ├── parser.rs        # 表达式解析
│   ├── calculator.rs    # 计算逻辑
│   ├── history.rs       # 历史记录
│   └── errors.rs        # 错误定义
├── tests/
│   ├── integration_tests.rs
│   └── unit_tests.rs
└── examples/
    └── usage_examples.rs
```

## 学习检查点
完成此项目后，你应该理解：
1. Rust的所有权系统如何防止内存泄漏
2. 如何使用`Result`和`Option`进行错误处理
3. 结构体和枚举的实际应用场景
4. 模式匹配的强大功能
5. Rust的模块系统和可见性规则