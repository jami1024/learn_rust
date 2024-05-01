fn main() {
    println!("Hello, 函数!");
    // 语句和表达式
    // Rust 的函数体是由一系列语句组成，最后由一个表达式来返回值

    fn _add_with_extra(x: i32, y: i32) -> i32 {
        let x = x + 1; // 语句
        let y = y + 5; // 语句
        x + y // 表达式
    }

    // 语句会执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值，因此在上述函数体的三行代码中，
    // 前两行是语句，最后一行是表达式。

    // 对于 Rust 语言而言，这种基于语句（statement）和表达式（expression）的方式是非常重要的，你
    // 需要能明确的区分这两个概念, 但是对于很多其它语言而言，这两个往往无需区分。基于表达式是函数式语言的重要特征，表达式总要返回值。

    let y = {
        let x = 3; // 语句
        x + 1 //表达式，表达式不能带;
    };

    // 
    println!("The value of y is: {}", y);

    // 表达式如果不返回任何值，会隐式地返回一个 ()
    assert_eq!(ret_unit_type(), ());

    // 函数要点
    // 函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
    // 函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
    // 每个函数参数都需要标注类型
    another_function(5, 6.1);

    // 函数返回
    // 函数的返回值就是函数体最后一条表达式的返回值，当然我们也可以使用 return 提前返回，下面的函数使用最后一条表达式来返回一个值：
    let x = plus_five(4);

    println!("The value of xxx is: {}", x);

    // 无返回值()
    // 函数没有返回值，那么返回一个 ()
    // 通过 ; 结尾的语句返回一个 ()

    // 永不返回的发散函数 !
    fn dead_end() -> ! {
        panic!("你已经到了穷途末路，崩溃吧！");
      }

    dead_end()



}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或者写成一行
    let z = if x % 2 == 1 { "odd" } else { "even" };

    println!("z:{}", z);
    println!("y:{}", y);
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_five(x:i32) -> i32 {
    if x > 5 {
        // 如果不用return、这里会报错，return后可以带;
        // x -5 
        return x - 5
    }

    return x + 5 // return 带不带;都是个表达式
}