use std::io;

fn main() {

    println!("input number > ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("not inputted number");

    println!("your number : {}", guess);


}
