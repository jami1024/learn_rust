### [match 和 if let - Rust语言圣经(Rust Course)](https://course.rs/basic/match-pattern/match-if-let.html)

在 Rust 中，模式匹配是非常重要的一部分。常用的模式匹配有 `match` 和 `if let`。本章将详细介绍这两者的使用。

---

### 1. `match` 语法与用法

#### 示例代码
```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}
```

#### 关键点
- **必须穷尽所有可能性**： `match` 必须列出所有可能的情况，如果没有列出，可以用 `_` 作为兜底。
- **类型一致性**： `match` 的所有分支必须返回相同类型的值。
- **多个模式匹配**： 用 `|` 符号表示多个模式匹配，只要满足其中一个模式即可。

#### 匹配赋值
`match` 可以用来直接进行赋值操作：
```rust
enum IpAddr {
    Ipv4,
    Ipv6,
}

fn main() {
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);
}
```

#### 模式绑定
可以从模式中取出绑定的值：
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

#### 穷尽匹配
所有可能的情况必须列出，否则会报错：
```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
    };
}
```
这里没有列出 `West` 的情况，会报错。

### 2. `if let` 语法与用法

#### 示例代码
```rust
let v = Some(3u8);
match v {
    Some(3) => println!("three"),
    _ => (),
}

// 使用 if let 重写
if let Some(3) = v {
    println!("three");
}
```

#### 关键点
- **简化匹配**： 当只需要匹配一个条件且忽略其他条件时使用 `if let` ，可以简化代码。

### 3. `matches!` 宏
Rust 提供了 `matches!` 宏用于简化模式匹配，返回 `true` 或 `false`。

#### 示例代码
```rust
enum MyEnum {
    Foo,
    Bar,
}

let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
v.iter().filter(|x| matches!(x, MyEnum::Foo));

let foo = 'f';
assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

let bar = Some(4);
assert!(matches!(bar, Some(x) if x > 2));
```

### 4. 变量遮蔽
无论是 `match` 还是 `if let`，这里都是一个新的代码块，绑定相当于新变量，可能发生变量遮蔽：

#### 示例代码
```rust
fn main() {
    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }

    println!("在匹配后，age是{:?}", age);
}
```

#### 关键点
- **避免混淆**： 遮蔽可能导致变量值的理解混乱，最好不要使用同名变量。

### 5. 课后练习
提供了 [Rust By Practice] 的在线编辑和运行支持以及详细的习题解答。通过练习可以加深对 `match` 和 `if let` 的理解和应用。