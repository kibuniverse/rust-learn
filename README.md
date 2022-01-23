# Rust

> 记录学习 rust

## 数据类型

### 整形

| 长度    | 有符号 | 无符号 |
| ------- | ------ | ------ |
| 8-bit   | i8     | u8     |
| 16-bit  | I16    | u16    |
| 32-bit  | I32    | u32    |
| 64-bit  | I64    | u64    |
| 128-bit | I128   | U128   |
| arch    | Isize  | size   |

🦀rust 关于整形溢出的处理办法

> ##### [整型溢出](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#整型溢出)
>
> 比方说有一个 `u8` ，它可以存放从零到 `255` 的值。那么当你将其修改为 `256` 时会发生什么呢？这被称为 “整型溢出”（“integer overflow” ），关于这一行为 Rust 有一些有趣的规则。当在 debug 模式编译时，Rust 检查这类问题并使程序 _panic_，这个术语被 Rust 用来表明程序因错误而退出。第九章 [“`panic!` 与不可恢复的错误”](https://kaisery.github.io/trpl-zh-cn/ch09-01-unrecoverable-errors-with-panic.html) 部分会详细介绍 panic。
>
> 在 release 构建中，Rust 不检测溢出，相反会进行一种被称为二进制补码包装（_two’s complement wrapping_）的操作。简而言之，值 `256` 变成 `0`，值 `257` 变成 `1`，依此类推。依赖整型溢出被认为是一种错误，即便可能出现这种行为。如果你确实需要这种行为，标准库中有一个类型显式提供此功能，[`Wrapping`](https://doc.rust-lang.org/std/num/struct.Wrapping.html)。 为了显式地处理溢出的可能性，你可以使用标准库在原生数值类型上提供的以下方法:
>
> - 所有模式下都可以使用 `wrapping_*` 方法进行包装，如 `wrapping_add`
> - 如果 `check_*` 方法出现溢出，则返回 `None`值
> - 用 `overflowing_*` 方法返回值和一个布尔值，表示是否出现溢出
> - 用 `saturating_*` 方法在值的最小值或最大值处进行饱和处理

### 浮点型

- f32
- f64

### 元祖

```rust
 let tup: (i32, f64, u8) = (500, 6.4, 1);

tup.0 // 500
tup.1 // 6.4
tup.2 // 1
```

### 数组

```rust
// 声明
let arr = [1, 2, 3, 4, 5];

let arr = [3; 5];
// 等价于
let arr = [3, 3, 3, 3, 3];

// 指定类型
let arr: [u32; 6] = [1, 1, 1, 1, 1, 1];

// 取值
let b = arr[1]
// 较为高级的用法

```

### 结构体

```rust
struct Student {
    username: String,
    email: String,
    grade: u64,
    active: bool,
};

let liming = Student {
  username: String::from("liming"),
  email: String::from("liming@xupt.com"),
  grade: 2018,
  active: true,
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        grade: 2018,
    }
}

```

## 声明变量

rust 默认变量为不可变的，即声明后无法主动修改

```rust
// 声明不可变变量
let foo = 0;
foo = 1; // ❌  会引起编译错误


// 声明可变变量
let mut foo = 0;
foo = 1; // ✅
```

## 声明函数

rust 对于函数声明和 typescript 有一定的相似性.

```bash
函数声明关键词 函数名() 返回类型 {}
```

不过 rust 在声明函数的时候需要去确定当前函数是否为存在返回值的函数

```rust
fn someFn() {
  return 1; // ❌ 函数并没有声明为存在返回值的函数
}

fn someFn() -> u32 {
  return 1; // ✅
}

// 更有意思的写法
fn someFn() -> u32 {
 1 // ✅
}
```

## 控制流

### 条件

```rust
if true {
  // ...
} else {
  // ...
}
```

rust 相较于 typescript 特殊的一点是没有隐式转换：），`隐式转换` 就不多说了，个人认为这个特性导致代码可读性变差了，回到正题，所以 rust 在 if 判断的时候只允许放返回 bool 类型的`表达式`。

```rust
let num = 5

if num == 6 {
  // ...
} else {
  // ...
}
```

还有个有意思的写法

```rust
let condition = true;
let number = if condition {
  5
} else {
  6
}; // ✅
// 感觉是三目表达式的变体?🤔
// 注意这里两个值要类型相同
let condition = true;
let number = if condition {
  5
} else {
 	"six"
}; // ❌
```

### 循环写法

#### loop

```rust
loop {
  // do something
  if condition {
    break; // 退出循环
  }
  continue; // 跳过该代码块下剩余的代码，直接执行下一轮循环
}
```

#### While

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}

```

#### for

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

```

各个语言中循环的控制都大差不差，像 break, continue 这种概念感觉都是默认通用的，特殊的是 rust 可以增加 **循环标签**，然后将标签与 `break` 或 `continue` 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环，即你可以在最内层的循环中使用 break 直接跳出最外层的大循环。

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

```

同时还有一点，rust 的 loop 循环可以有返回值，跟在 break 后即可

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

## 栈和堆 以及 gc

个人感觉 rust 在内存管理方面很有意思，rust 引入了一个`所有权`的概念，rust 通过这个所有权来管理系统的内存，rust 中不需要像在 c 语言中手动的 free 去释放申请的堆内存，也不像我们在 js or ts 中任意的去写代码而不考虑 gc 的问题。

### 所有权

> 规则

1. rust 中的每一个值都有一个被称为其 **所有者**（_owner_）的变量。

1. 值在任一时刻有且只有一个所有者。
1. 当所有者（变量）离开作用域，这个值将被丢弃。

这三条规则解释了为什么 rust 的内存安全。

## rust 方向

### neon

neon 提供了将 rust 编译为二进制的能力，可以直接在 node 中调用

```rust

```
