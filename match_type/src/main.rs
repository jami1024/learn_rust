fn main() {
    println!("Hello, match!");

    enum Direction {
        East,
        West,
        North,
        South,
    }

    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}
