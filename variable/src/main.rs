fn main() {
    println!("Hello, 变量绑定和解构!");
    // 变量绑定
    // 其他语言叫赋值，而rust叫变量绑定

    // 变量可变性
    // rust的变量在默认情况下不可变、可以通过mut关键字让变量可变
    
    // 当没有mut是ide会提示，当然编译的时候也会提示cannot assign twice to immutable variable
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 使用_下划线开头来忽略未使用的变量，不然会提示warning: unused variable:

    let _y = 7;

    // 变量结构
    // let 表达式不仅仅用于变量的绑定、还能可以进行变量的解构

    let (a, mut b):(bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    // 变量和常量之间的差异
    // 常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
    // 常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。

    const _MAX_POINTS: u32 = 100000;

    // 变量遮蔽(shadowing)
    // Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的
    let z = 10;
    // 在main函数的作用域内对之前的x进行遮蔽。
    // 这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，
    // 而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好
    let z = z+1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let z = z * 2;


        // 这个程序首先将数值 10 绑定到 z，
        // 然后通过重复使用 let z = 来遮蔽之前的 z，并取原来的值加上 1，所以 z 的值变成了 11。
        // 第三个 let 语句同样遮蔽前面的 z，取之前的值并乘上 2，得到的 z 最终值为 22。
        // The value of x in the inner scope is: 22
        println!("The value of x in the inner scope is: {}", z);
    }

    // The value of x is: 11
    println!("The value of x is: {}", z);



}
