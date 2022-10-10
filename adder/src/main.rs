use add_one;
use add_two;

fn main() {
    let num = 10;
    println!("你好，世界！{} 加一會是 {}！", num, add_one::add_one(num));
    println!("你好，世界！{} 加二會是 {}！", num, add_two::add_two(num));
}
