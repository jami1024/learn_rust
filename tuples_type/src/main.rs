fn main() {
    println!("Hello, 元组!");
    // 元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的。
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    dbg!(tup);

    // 用模式匹配解构元组

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    // The value of x is: 500
    // The value of y is: 6.4
    // The value of z is: 1
    
    // 用 . 来访问元组
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    println!("The value of five_hundred is: {}", five_hundred);
    // The value of five_hundred is: 500


    



}
