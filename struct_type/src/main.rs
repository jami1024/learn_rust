
fn main() {
    println!("Hello, 结构体!");
    // 定义结构体
    // 一个结构体由几部分组成：

    // 通过关键字 struct 定义
    // 一个清晰明确的结构体 名称
    // 几个有名字的结构体 字段
    # [derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    // 创建User结构体实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 注意
    // 初始化实例时，每个字段都需要进行初始化
    // 初始化时的字段顺序不需要和结构体定义时的顺序一致

    // 访问结构体字段
    print!("{}\n", user1.username);
    // someusername123
    // 结构体更新语法，用..语法表明凡是我们没有显式声明的字段，全部从 user1 中自动获取。需要注意的是 ..user1 必须在结构体的尾部使用。
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    print!("{}\n", user2.email); // another@example.com

    // 结构体更新语法跟赋值语句 = 非常相像，因此在上面代码中，user1 的部分字段所有权被转移到 user2 中：username 字段发生了所有权转移，作为结果，user1 无法再被使用。
    // 聪明的读者肯定要发问了：明明有三个字段进行了自动赋值，为何只有 username 发生了所有权转移？
    // 仔细回想一下所有权那一节的内容，我们提到了 Copy 特征：实现了 Copy 特征的类型无需所有权转移，可以直接在赋值时进行 数据拷贝，其中 bool 和 u64 类型就实现了 Copy 特征，因此 active 和 sign_in_count 字段在赋值给 user2 时，仅仅发生了拷贝，而不是所有权转移。
    //值得注意的是：username 所有权被转移给了 user2，导致了 user1 无法再被使用，但是并不代表 user1 内部的其它字段不能被继续使用，例如：

    // 元组结构体(Tuple Struct)
    // 结构体必须要有名称，但是结构体的字段可以没有名称，这种结构体长得很像元组，因此被称为元组结构体

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // 单元结构体(Unit-like Struct)
    // 如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体：
    struct AlwaysEqual;

    let _subject = AlwaysEqual;

    // 使用 #[derive(Debug)] 来打印结构体的信息

    println!("user2 is {:?}", user2);
    // user2 is User { active: true, username: "someusername123", email: "another@example.com", sign_in_count: 1 }



    
}
