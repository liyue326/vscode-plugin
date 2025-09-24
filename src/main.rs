// use std::io;

// use rand::Rng;
// use std::cmp::Ordering;
// fn main() {
    // println!("Guess the number!");
    // let secrect_number = rand::thread_rng().gen_range(1..=99);
    // loop {
    //     println!("secrect number is{secrect_number}");
    //     println!("Please input your guess");
    //     let mut guess = String::new();
    //     io::stdin().read_line(&mut guess).expect("failed"); //和下面的类型转换顺序不能反，先读取才行

    //     let guess: i32 = match guess.trim().parse() { //parse 可能失败，match 就是 Rust 的 try/catch：成功拿数字，失败就 continue 重新来。”
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     println!("{guess}");

    //     match secrect_number.cmp(&guess) {
    //         Ordering::Equal => {
    //             println!("win");
    //             break;
    //         }
    //         Ordering::Less => println!("Too Big"),
    //         Ordering::Greater => println!("too Small"),
    //     }
    // }
// }


#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}