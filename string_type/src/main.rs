fn main() {
    println!("Hello, 字符、布尔值!");
    // 字符类型(char), 字符类型占用 4 个字节：

    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    println!("c:{} z:{} g:{} heart_eyed_cat:{}", c, z, g, heart_eyed_cat);

    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));

    // 布尔(bool)
    // Rust 中的布尔类型有两个可能的值：true 和 false，布尔值占用内存的大小为 1 个字节：
    let _t = true;

    let _f: bool = false; // 使用类型标注,显式指定f的类型
}
