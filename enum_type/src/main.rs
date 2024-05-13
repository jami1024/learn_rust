fn main() {
    println!("Hello, 枚举!");

    // 枚举(enum 或 enumeration)允许你通过列举可能的成员来定义一个枚举类型，例如扑克牌花色：

    // enum PokerSuit {
    //     Clubs,
    //     Spades,
    //     Diamopnds,
    //     Hearts,
    // }
    //  枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例。

    // 枚举值
    // 现在来创建 PokerSuit 枚举类型的两个成员实例：

    #[derive(Debug)]
    enum PokerCard {
        Clubs(u8),
        Spades(u8),
        Diamonds(u8),
        Hearts(u8),
    }
    
    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds(13);
    

    fn print_suit(card: PokerCard) {
        // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
        println!("{:?}",card);
    }
    print_suit(c1);
    print_suit(c2);

    // 从这些例子可以看出，任何类型的数据都可以放入枚举成员中: 例如字符串、数值、结构体甚至另一个枚举。

    


}
