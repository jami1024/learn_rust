fn main() {
    println!("Hello, 数组!");
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // 声明一个整型数组，包含五个元素

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    print!("{:?}", arr);

    print!("{:?}", months);

    // let first = arr[0]; // 获取a数组第一个元素

    // 数组元素为非基础类型 的错误写法
    // let array = [String::from("rust is good!"); 8];

    // 正确写法
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));

    println!("{:#?}", array);

    // 编译器自动推导出one的类型
    let one             = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3]    = [1, 2, 3];
    let blank1          = [0; 3];
    let blank2: [u8; 3] = [0; 3];
    
    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];
    
    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
        print!("\t{} + 10 = {}", n, n+10);
        }
    
        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
        sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
    
}
