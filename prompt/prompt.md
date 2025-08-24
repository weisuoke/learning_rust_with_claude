# English Prompt: Generate Programming Learning Content

## Task Overview
Generate comprehensive programming learning content following a specific bilingual template structure for educational materials. The content should be suitable for beginners learning programming concepts in any specified language.

## Content Requirements

### Format Specifications
- **Bilingual Content**: All content must be provided in both Chinese and English using the format "中文 | English"
- **File Format**: Markdown (.md) format with proper structure and formatting
- **Code Examples**: Extensive, practical code examples in the target programming language with bilingual comments
- **Progressive Difficulty**: Content should build from basic to advanced concepts within the topic

### Required Structure

#### 1. Header Section
```markdown
# [语言名称]入门 - 第X天：[Topic] | [Language] Introduction - Day X: [Topic]
```

#### 2. Learning Objectives (学习目标 | Learning Objectives)
- 4-6 bullet points describing what students will learn
- Each point in bilingual format
- Focus on practical skills and understanding

#### 3. Detailed Content (详细内容 | Detailed Content)
- **6 main sections** with time allocations (e.g., "1小时 | 1 hour")
- Each section should include:
  - **Concept Definition & Explanation**: Clear, comprehensive explanations in bilingual format
  - **Core Characteristics**: Key features and properties of the concept
  - **CCQs (Concept Checking Questions)**: 3-5 carefully designed questions to verify understanding
  - **Code Examples**: Multiple examples with detailed bilingual comments
  - **Practice Verification**: Questions to check code comprehension
  - **Common Misconceptions**: Address typical misunderstandings with targeted questions
  - **Practical applications and use cases**
  - **Best practices and optimization techniques**

#### 4. Practical Project (实践项目 | Practical Project)
- **Complete, functional project** demonstrating the day's concepts
- Include:
  - Project objectives in bilingual format
  - Step-by-step implementation
  - Full working code (200+ lines)
  - Comprehensive features covering the topic
  - Error handling and user interaction

#### 5. Extension Exercises (扩展练习 | Extension Exercises)
- 5-7 progressive challenges for further learning
- Each exercise should build on the concepts taught
- Include difficulty levels and learning objectives

#### 6. Learning Resources (学习资源 | Learning Resources)
- Official language documentation links
- Recommended tutorials and guides
- Additional reading materials

#### 7. Completion Checklist (完成检查清单 | Completion Checklist)
- 8-10 checkboxes covering all learning objectives
- Practical skills verification points

### Code Quality Standards

#### Code Examples
- **Comprehensive Comments**: Every code block should have bilingual comments explaining functionality
- **Real-world Applications**: Examples should solve actual problems, not just demonstrate syntax
- **Error Handling**: Include try-catch blocks and validation where appropriate
- **Best Practices**: Follow the target language's official style guidelines and conventions

#### Project Requirements
- **Functionality**: Project must be a complete, working application
- **Educational Value**: Should demonstrate multiple concepts from the lesson
- **User Interaction**: Include input validation and user-friendly interfaces
- **Documentation**: Detailed docstrings and comments throughout

### CCQs (Concept Checking Questions) Standards

#### CCQs Design Principles
CCQs are inspired by CELTA (Certificate in English Language Teaching to Adults) methodology and are essential for verifying conceptual understanding rather than mere syntax memorization.

#### CCQs Requirements for Programming Concepts
- **Quantity**: 3-5 CCQs per core concept
- **Format**: Closed-ended questions (Yes/No, Multiple choice, Fill-in-the-blank)
- **Language**: Bilingual format (Chinese | English) for comprehensive understanding
- **Focus**: Test concept understanding, not syntax recall

#### CCQs Categories and Examples

##### 1. Definition Verification CCQs
**Purpose**: Ensure learners understand what the concept IS
**Example Structure**:
- "变量可以存储数据吗？| Can variables store data?" (Answer: Yes)
- "函数必须有返回值吗？| Must functions have return values?" (Answer: No)

##### 2. Characteristic Identification CCQs  
**Purpose**: Verify understanding of concept properties
**Example Structure**:
- "位置参数的顺序重要吗？| Does the order of positional arguments matter?" (Answer: Yes)
- "字典的键可以重复吗？| Can dictionary keys be duplicated?" (Answer: No)

##### 3. Usage Context CCQs
**Purpose**: Check when and why to use the concept
**Example Structure**:
- "什么时候使用for循环而不是while循环？| When should we use for loops instead of while loops?"
- "哪种情况下使用列表比字典更好？| In which cases are lists better than dictionaries?"

##### 4. Differentiation CCQs
**Purpose**: Distinguish between similar concepts
**Example Structure**:
- "append()和extend()的区别是什么？| What's the difference between append() and extend()?"
- "参数和变量有什么不同？| How are parameters different from variables?"

##### 5. Application Verification CCQs
**Purpose**: Test practical understanding
**Example Structure**:
- "这段代码会输出什么？| What will this code output?"
- "如果我们改变这个参数会发生什么？| What happens if we change this parameter?"

#### CCQs Quality Standards
- **Precision**: Questions must have clear, unambiguous answers
- **Relevance**: Directly related to core concept characteristics  
- **Progression**: From basic understanding to application
- **Accessibility**: Appropriate for target learning level
- **Bilingual Consistency**: Chinese and English versions test the same concept

#### CCQs Implementation Guidelines
- **Placement**: After concept explanation, before code examples
- **Feedback**: Provide immediate answers and explanations
- **Integration**: Use CCQ results to guide subsequent explanations
- **Adaptation**: Adjust complexity based on concept difficulty

#### Programming-Specific CCQs Examples

##### Variables and Data Types
- "变量的值可以改变吗？| Can the value of a variable change?" (Answer: Yes)
- "字符串和数字是相同的数据类型吗？| Are strings and numbers the same data type?" (Answer: No)
- "变量名可以包含空格吗？| Can variable names contain spaces?" (Answer: No)

##### Functions and Parameters
- "函数每次被调用时都会执行相同的代码吗？| Does a function execute the same code each time it's called?" (Answer: Yes)
- "位置参数的顺序重要吗？| Does the order of positional arguments matter?" (Answer: Yes)  
- "函数必须有返回值吗？| Must functions have return values?" (Answer: No)
- "默认参数在调用时必须提供吗？| Must default parameters be provided when calling?" (Answer: No)

##### Control Flow (Loops/Conditionals)
- "if语句可以不包含else部分吗？| Can if statements exist without an else part?" (Answer: Yes)
- "for循环会自动停止吗？| Do for loops stop automatically?" (Answer: Yes)
- "while循环如果条件始终为True会怎样？| What happens if a while loop condition is always True?" (Answer: Infinite loop)

##### Data Structures
- "列表中的元素可以是不同类型吗？| Can list elements be of different types?" (Answer: Yes)
- "字典的键可以重复吗？| Can dictionary keys be duplicated?" (Answer: No)
- "列表的长度是固定的吗？| Is the length of a list fixed?" (Answer: No)

##### Object-Oriented Programming  
- "类和对象是同一个概念吗？| Are classes and objects the same concept?" (Answer: No)
- "一个类可以创建多个对象吗？| Can one class create multiple objects?" (Answer: Yes)
- "方法和函数有什么区别？| What's the difference between methods and functions?" (Answer: Methods belong to classes)

#### CCQs for Common Programming Misconceptions

##### Misconception: Variables are containers
**CCQ**: "变量存储数据还是指向数据？| Do variables store data or point to data?"
**Correct Understanding**: Variables are references/names that point to data in memory

##### Misconception: Assignment copies values
**CCQ**: "当我写 b = a 时，b得到a的副本吗？| When I write b = a, does b get a copy of a?"
**Correct Understanding**: Depends on data type - immutable vs mutable objects

##### Misconception: Functions always return something
**CCQ**: "没有return语句的函数返回什么？| What do functions without return statements return?"
**Correct Understanding**: They return None in Python

##### Misconception: Loops always execute at least once
**CCQ**: "如果for循环的序列为空，循环体会执行吗？| If a for loop's sequence is empty, will the loop body execute?"
**Correct Understanding**: No, the loop body won't execute if the sequence is empty

#### CCQs Evaluation and Assessment

##### Good CCQ Characteristics
- **Unambiguous**: Only one correct answer possible
- **Concept-focused**: Tests understanding, not syntax memorization  
- **Progressive**: Builds from basic to complex understanding
- **Practical**: Relates to real programming scenarios
- **Diagnostic**: Reveals specific misunderstandings

##### Poor CCQ Examples to Avoid
❌ "What does the print() function do?" (Too open-ended)
❌ "Write a for loop" (Tests syntax, not concept understanding)
❌ "Is Python good?" (Subjective, not concept-related)
❌ "列表有哪些方法？| What methods do lists have?" (Memory test, not conceptual)

##### Good CCQ Examples
✅ "变量的值可以在程序运行时改变吗？| Can a variable's value change during program execution?" 
✅ "两个变量可以指向同一个对象吗？| Can two variables point to the same object?"
✅ "函数参数和函数内部变量有什么区别？| What's the difference between function parameters and variables inside functions?"

#### CCQs Testing Pyramid

##### Level 1: Basic Concept Recognition (是什么 | What is it)
- Tests fundamental understanding of what the concept is
- Uses simple Yes/No or multiple choice questions
- Examples: "Can variables store different types of data?"

##### Level 2: Characteristic Identification (有什么特点 | What are its characteristics)  
- Tests understanding of concept properties and behaviors
- Uses scenario-based questions
- Examples: "What happens when you call a function with fewer arguments than parameters?"

##### Level 3: Usage Context (什么时候用 | When to use)
- Tests understanding of appropriate usage scenarios
- Uses comparative and situational questions  
- Examples: "When would you use a list instead of a dictionary?"

##### Level 4: Integration and Application (怎么应用 | How to apply)
- Tests ability to combine concepts and solve problems
- Uses problem-solving scenarios
- Examples: "How would you use functions and loops together to process a list of data?"

### Content Depth Guidelines

#### Beginner-Friendly Approach
- Start with basic concepts and build complexity gradually
- Provide multiple examples for each concept
- Explain not just "how" but "why" and "when" to use features
- Include common mistakes and how to avoid them

#### Advanced Concepts
- Cover edge cases and advanced use patterns
- Discuss performance considerations
- Include optimization techniques
- Provide alternative approaches

## Example Usage Prompt

```
Reference @template.md structure and generate Day [X] core learning content (bilingual Chinese/English) about [TOPIC] for [PROGRAMMING_LANGUAGE], output to @[directory_path]

Requirements:
- Follow the exact template structure from template.md
- Include comprehensive bilingual content (Chinese | English format)
- Design 3-5 CCQs (Concept Checking Questions) for each core concept using CELTA methodology
- Provide extensive code examples with practical applications in [PROGRAMMING_LANGUAGE]
- Include practice verification questions for code comprehension
- Address common misconceptions through targeted CCQs
- Create a complete functional project demonstrating the concepts
- Ensure progressive learning from basic to advanced concepts
- Include proper error handling and best practices for [PROGRAMMING_LANGUAGE]
- Target beginner to intermediate learners of [PROGRAMMING_LANGUAGE]
```

## Quality Checklist

### Content Verification
- [ ] All sections follow bilingual format consistently
- [ ] CCQs are included for every core concept (3-5 questions each)
- [ ] CCQs test understanding rather than syntax memorization
- [ ] CCQ answers are provided with explanations
- [ ] Code examples are tested and functional
- [ ] Project is complete and demonstrates all key concepts
- [ ] Learning progression is logical and well-paced
- [ ] Error handling is included where appropriate
- [ ] Best practices are highlighted throughout

### Structure Verification
- [ ] Header follows exact format requirements
- [ ] All required sections are present and properly formatted
- [ ] Time allocations are realistic and helpful
- [ ] Extension exercises provide meaningful challenges
- [ ] Completion checklist covers all learning objectives

### Educational Value
- [ ] Content is appropriate for target skill level
- [ ] CCQs effectively verify conceptual understanding
- [ ] Real-world applications are provided
- [ ] Common pitfalls are addressed through targeted questions
- [ ] Multiple learning styles are accommodated (visual, auditory, kinesthetic)
- [ ] Active learning is promoted through interactive CCQs
- [ ] Practical skills are emphasized over theoretical knowledge

## Template Customization Guidelines

### Topic-Specific Adaptations
- Adjust the 6 main sections based on topic complexity
- Modify time allocations based on concept difficulty
- Customize project complexity to match learning objectives
- Adapt extension exercises to topic-specific challenges

### Difficulty Scaling
- **Basic Topics**: More examples, simpler projects, focus on fundamentals
- **Intermediate Topics**: Complex projects, optimization techniques, multiple approaches
- **Advanced Topics**: Real-world applications, performance considerations, best practices

This prompt framework ensures consistent, high-quality educational content that follows proven pedagogical principles while maintaining the bilingual accessibility crucial for diverse learners.