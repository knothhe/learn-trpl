## 引用

### ownership

- Safety is the Absence of Undefined Behavior
- Ownership as a Discipline for Memory Safety
- Variables Live in the Stack
- Boxes Live in the Heap
- Rust Does Not Permit Manual Memory Management
- Box deallocation principle: If a variable owns a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.
- Moved heap data principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.

### reference
- Pointer Safety Principle: data should never be aliased and mutated at the same time.

## 引用勘误

- Variables Live in the Stack
  - 过于绝对。更准确是：局部变量的值通常在当前 stack frame 中，但编译器可能优化；String/Vec 这类变量本身的固定大小元数据在栈上，真正的数据缓冲区在堆上。
- Boxes Live in the Heap
  - 更准确是：Box<T> 的指针/所有权句柄本身通常在栈上，T 的值在堆上。
- Rust Does Not Permit Manual Memory Management
  - 不严谨。Rust 安全代码通常不需要手动 free；也可以显式 drop，或在 unsafe 中用 allocator、raw pointer、Box::into_raw/from_raw 等方式手动管理内存。
- Box deallocation principle
  - 释放时机应该是 owner 离开作用域或被 drop 时，不一定等到整个 stack frame 被释放。
- Moved heap data principle
  - move 不只发生在 heap data 上，而是发生在非 Copy 类型的值上。比如结构体即使字段都在栈上，也可能 move。更准确：非 Copy 值 move 后，原变量不能再被使用，除非重新赋值。

## 笔记

- Stack 存放大小固定、生命周期简单的数据，分配和释放都很快；heap 存放大小动态或需要更灵活生命周期的数据，Rust 通过所有权系统自动管理 heap 内存，不需要手动 free，也不依赖垃圾回收。
- Stack 是每次函数调用时使用的内存区域，用来存放该调用的参数、局部变量和一些调用信息。main 也是一个普通函数调用，因此 main 里的局部变量通常在 main 的 stack frame 中。对于 String、Vec 这类需要动态内存的数据，变量本身的固定大小元数据在 stack 上，真正的数据缓冲区在 heap 上。

### stack and heap

绝大多数主流语言运行时都会用到 stack 和 heap 这两个概念，因为它们对应的是两类很基础的需求：

  - stack：管理函数调用、局部状态、返回路径
  - heap：管理生命周期不完全受函数调用顺序限制的数据

### 最重要

我的理解是，rust 的 ownership 有几个比较重要的语义，move、copy、borrow、lifetime。

```rust
let s1 = String::from("hello");
// move
let s2 = s1;
// borrow
let s3 = &s1;
// borrow with mutable reference
// but cannot be used with immutable reference at the same time
let s4 = &mut s1;
// copy
let s5 = s1.clone();

let i1 : i32 = 42;
// copy for the copy type, different from non-copy type
let i2 = i1;
// borrow
let i3 = &i1;
// borrow with mutable reference
// but cannot be used with immutable reference at the same time
let i4 = &mut i1;
```

Q: rust 对于 i32 这种类型，有 move 语义吗？我知道对于 String 这种类型是有的。
A: 有。Rust 里所有值默认都有“移动语义”，包括 i32。

但 i32 实现了 Copy trait，所以它在“按值赋值 / 传参”时表现为复制，而不是让原变量失效。也就是说，语义上这个操作走的是
按值传递；因为类型是 Copy，编译器允许源变量继续使用。

```rust
let a: i32 = 1;
let b = a;

println!("{a}"); // 可以，i32 是 Copy
println!("{b}");
```

对比 String：

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{s1}"); // 编译错误：s1 已被 move
```

核心区别不是“i32 没有 move 语义”，而是：

- String：不是 Copy，赋值会 move，原变量失效
- i32：是 Copy，赋值会复制一份，原变量仍可用

可以把它理解为：Copy 类型的 move 会退化成按位复制，并且不会触发所有权转移后的失效效果。

### reference and dereference

一个完整对比例子

```rust
  fn main() {
      let mut x = 10;

      let r = &x;        // 显式不可变引用
      println!("{}", *r); // 显式解引用
      println!("{}", r);  // println 隐式处理引用

      let m = &mut x;    // 显式可变引用
      *m += 5;           // 显式解引用后修改

      println!("{}", x); // 15
  }
```

最重要的心智模型

- &T 表示“我共享借用了一个 T，不能通过这个引用直接改”。
- &mut T 表示“我独占地借用了一个 T，可以改”。
- &x 表示“创建引用”。
- *r 表示“访问引用指向的值”。

方法调用时 Rust 会帮你自动借用、自动解引用，所以：

```rust
s.len()
r.len()
```

经常都能工作。
但在普通赋值、比较、修改时，你经常需要自己写清楚：

```rust
// 当 T: Copy 且 r: &T 时，可以复制出值
let value = *r;

// 当 r: &mut T 时，可以通过解引用修改原值
*r = new_value;

foo(&x);
foo(&mut x);
```

### 更多

- &T：不可变引用，也叫 shared reference
- &mut T：可变引用，也叫 mutable/unique reference
- borrow：借用
- dereference：解引用，*
- borrow checker：借用检查器
- lifetime：引用的生命周期

### 可变引用和不可变引用

注意下面的 r2 和 r3 的区别

```rust
let mut x = 5;

// r1 这个绑定不可变，保存的是不可变引用 &i32
let r1 = &x;

// r2 这个绑定不可变，保存的是可变引用 &mut i32
let r2 = &mut x;

// r3 这个绑定可变，保存的是不可变引用 &i32
// mut r3 表示 r3 可以重新绑定到另一个 &i32，不表示可以通过 r3 修改 x
let mut r3 = &x;
```
