use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("### 1부터 100사이의 숫자 맞추기");

    loop {
        println!("숫자를 입력하세요 : ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("숫자만 입력 가능합니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("입력하신 값 : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("작습니다."),
            Ordering::Greater => println!("큽니다."),
            Ordering::Equal => {
                println!("정답!");
                break;
            },
        }
    }


}
