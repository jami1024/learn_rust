fn main() {
    println!("Hello, 字符串!");

    // 切片
    // 在 Rust 中，切片提供了一种高效的方式来引用集合中的部分数据，而无需复制它们。这种机制不仅适用于字符串，也适用于其它集合类型，如数组。

    
    // 切片通过指定一个元素范围来创建，这个范围是半开区间（包含起始索引，不包含终止索引）。例如，你可以从一个 String 或数组中创建切片：
    let s = String::from("hello world");

    // 完整的切片
    let _slice_full = &s[..]; // 等同于 &s[0..s.len()]

    // 从起始到中间的切片
    let _slice_start = &s[..2]; // 等同于 &s[0..2]

    // 从中间到结束的切片
    let _slice_end = &s[3..]; // 等同于 &s[3..s.len()]

    //hello 没有引用整个 String s，而是引用了 s 的一部分内容，通过 [0..5] 的方式来指定。
    //这就是创建切片的语法，使用方括号包括的一个序列：[开始索引..终止索引]，
    //其中开始索引是切片中第一个元素的索引位置，而终止索引是最后一个元素后面的索引位置，也就是这是一个 右半开区间。在切片数据结构内部会保存开始的位置和切片的长度，其中长度是通过 终止索引 - 开始索引 的方式计算得来的

    // 字符串字面量是切片 之前提到过字符串字面量,但是没有提到它的类型：
    let _s1 = "Hello, world!";
    // 实际上，s1 的类型是 &str，因此你也可以这样声明：
    let _s1: &str = "Hello, world!";
    // 该切片指向了程序可执行文件中的某个点，这也是为什么字符串字面量是不可变的，因为 &str 是一个不可变引用。


    // 什么是字符串
    // 顾名思义，字符串是由字符组成的连续集合，
    // 但是在上一节中我们提到过，Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，
    // 但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)，这样有助于大幅降低字符串所占用的内存空间。
    // Rust 在语言级别，只有一种字符串类型： str，它通常是以引用类型出现 &str，也就是上文提到的字符串切片。虽然语言级别只有上述的 str 类型，
    // 但是在标准库里，还有多种不同用途的字符串类型，其中使用最广的即是 String 类型。
    // str 类型是硬编码进可执行文件，也无法被修改，但是 String 则是一个可增长、可改变且具有所有权的 UTF-8 编码字符串，
    // 当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。

    // String 与 &str 的转换
    // 从 &str 类型生成 String 类型的操作
    let _name = "hello,world".to_string();

    let _name = String::from("hello,world");

    // 将 String 类型转为 &str 类型

    let s = String::from("hello,world!");
    println!("{}",&s);
    println!("{}",s.as_str());

    // rust中字符串不支持索引

    // 由于 String 是可变字符串，下面介绍 Rust 字符串的修改，添加，删除等常用方法：

    //追加 (Push)
    // 在字符串尾部可以使用 push() 方法追加字符 char，
    // 也可以使用 push_str() 方法追加字符串字面量。这
    // 两个方法都是在原有的字符串上追加，并不会返回新的字符串。由于字符串追加操作要修改原来的字符串，
    // 则该字符串必须是可变的，即字符串变量必须由 mut 关键字修饰。

    let mut s = String::from("Hello ");
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);
    s.push('!');
    println!("追加字符 push() -> {}", s);
    // 追加字符串 push_str() -> Hello rust
    // 追加字符 push() -> Hello rust!

    // 插入 (Insert)
    // 可以使用 insert() 方法插入单个字符 char，也可以使用 insert_str() 方法插入字符串字面量，
    // 与 push() 方法不同，这俩方法需要传入两个参数，第一个参数是字符（串）插入位置的索引，第二个参数是要插入的字符（串），索引从 0 开始计数，
    // 如果越界则会发生错误。由于字符串插入操作要修改原来的字符串，则该字符串必须是可变的，即字符串变量必须由 mut 关键字修饰。

    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    // 插入字符 insert() -> Hello, rust!
    // 插入字符串 insert_str() -> Hello, I like rust!

    // 替换 (Replace)
    // replace 该方法可适用于 String 和 &str 类型。
    // replace() 方法接收两个参数，第一个参数是要被替换的字符串，第二个参数是新的字符串。
    // 该方法会替换所有匹配到的字符串。该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);

    // [src/main.rs:90:5] new_string_replace = "I like RUST. Learning RUST is my favorite!"

    // replacen
    // 该方法可适用于 String 和 &str 类型。r
    // eplacen() 方法接收三个参数，前两个参数与 replace() 方法一样，第三个参数则表示替换的个数。
    // 该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
    // [src/main.rs:100:5] new_string_replacen = "I like RUST. Learning rust is my favorite!"

    // replace_range
    // 该方法仅适用于 String 类型。
    // replace_range 接收两个参数，第一个参数是要替换字符串的范围（Range），第二个参数是新的字符串。
    // 该方法是直接操作原来的字符串，不会返回新的字符串。该方法需要使用 mut 关键字修饰。
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    // [src/main.rs:109:5] string_replace_range = "I like Rust!"

    // 与字符串删除相关的方法有 4 个，它们分别是 pop()，remove()，truncate()，clear()。这四个方法仅适用于 String 类型。

    // pop —— 删除并返回字符串的最后一个字符
    // 该方法是直接操作原来的字符串。但是存在返回值，其返回值是一个 Option 类型，如果字符串为空，则返回 None
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
    /*
    [src/main.rs:120:5] p1 = Some(
        '!',
    )
    [src/main.rs:121:5] p2 = Some(
        '文',
    )
    [src/main.rs:122:5] string_pop = "rust pop 中"
    
     */


    // remove —— 删除并返回字符串中指定位置的字符
    // 该方法是直接操作原来的字符串。但是存在返回值，其返回值是删除位置的字符串，只接收一个参数，表示该字符起始索引位置。
    // remove() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);
    // string_remove 占 18 个字节
    // [src/main.rs:149:5] string_remove = "试remove方法"

    // truncate —— 删除字符串中从指定位置开始到结尾的全部字符
    // 该方法是直接操作原来的字符串。无返回值。
    // truncate() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
    // [src/main.rs:158:5] string_truncate = "测"

    // clear —— 清空字符串
    // 该方法是直接操作原来的字符串。调用后，删除字符串中的所有字符，相当于 truncate() 方法参数为 0 的时候。

    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    // 使用 + 或者 += 连接字符串
    // 使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型。
    // 其实当调用 + 的操作符时，相当于调用了 std::string 标准库中的 add() 方法，这里 add() 方法的第二个参数是一个引用的类型。
    // 因此我们在使用 + 时， 必须传递切片引用类型。不能直接传递 String 类型。+ 是返回一个新的字符串，所以变量声明可以不需要 mut 关键字修饰。

    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    result += "!!!";

    println!("连接字符串 + -> {}", result);
    // 连接字符串 + -> hello rust!!!!
    
    // 2、使用 format! 连接字符串
    // format! 这种方式适用于 String 和 &str 。format! 的用法与 print! 的用法类似，
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
    // hello rust!

    // 操作UTF-8字符串
    // 如果你想要以 Unicode 字符的方式遍历字符串，最好的办法是使用 chars 方法，例如：
    for c in "中国人".chars() {
        println!("{}", c);
    }
    // 这种方式是返回字符串的底层字节数组表现形式：
    for b in "中国人".bytes() {
        println!("{}", b);
    }














    
































}
