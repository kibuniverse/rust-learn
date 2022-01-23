## rust 模块系统

### mod

定义模块

### pub

由于 rust 默认的值为 private，所以对外的函数或者方法都需要进行 pub 声明

```rust
pub fn foo() {
    println!("Hello, world!");
}
```

### use

use 提供了将模块路径简短化表示的方式

假设有以下的模块结构

```
├── config.rs
├── main.rs
└── modulea
  ├── add.rs
  ├── mod.rs
  └── print.rs
```

```rust
// main.rs

fn main() {
    let add = modulea::add::add(1, 2);
    println!("add: {}", add);
}


// 等价于
use modulea::add::add as add_as;

fn main() {
    let add = add_as(1, 2);
    println!("add: {}", add);
}
```

### super

super 关键词指向父级作用域

```
├── config.rs
├── main.rs
└── modulea
  ├── add.rs
  ├── mod.rs
  └── print.rs
```

如果我们需要在 print 中调用 add 模块 中的方法

```rust
// print.rs
println!(" add 1, 2 = {}", crate::modulea::add::add(1, 2));

// 等价于
println!(" add 1, 2 = {}", super::add::add(1, 2));

// 等价于
use super::add::add as add_as;
println!(" add 1, 2 = {}", add_as(1, 2));

```
