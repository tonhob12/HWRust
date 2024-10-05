use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivinhe um número!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("digite seu palpite.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to readline");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("você adivinhou: {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("muito pequeno"),
        Ordering::Greater => println!("muito grande"),
        Ordering::Equal => {
            println!("você acertou!");
            break;
        }
    }
    }
    
}
