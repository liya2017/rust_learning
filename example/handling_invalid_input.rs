use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);//此处secret_number前没加&，usize 等数据结构，是 copy 类型的，所以如果不是要原地改本身的数据的话，一般都 不用引用宏，你要看展开代码，实际上是加了 & 的
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");//read_line(&mut guess) guess 前加了&mut，&mut 是指，我需要这个变量的可变权限去修改它本身的数据结构，所以这句话的意思可以理解成，如果我不需要改变guess，也可以直接&guess而不用加mut
    
        let guess: u32 = match guess.trim().parse() { //从 expect（）转换为match。 如果输入不能转换为字符的string，expect会另程序报错崩溃，但是match的处理方式会让程序继续运行，继续猜谜游戏
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {// 此处secret_number 前加了& 是因为cmp 方法接的参数前需要&。rust 里面，函数签名是很重要的信息，基本上看到函数签名就知道定义者想对你传入的东西做什么了。
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
