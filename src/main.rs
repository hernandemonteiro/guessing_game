use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nGuess the number!");

    let mut chances = 7;
    let secret_number = rand::thread_rng().gen_range(1..=20);

    println!("The secret number is: {}", secret_number);

    print!("The secret number is between 1 and 20\n");
    println!("You have {chances} chances to try.");
    separator(40);

    while chances > 0 {
        chances -= 1;

        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_parsed: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                println!("You have {} chances left", chances);
                separator(40);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess_parsed.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                separator(40);
            }
            Ordering::Greater => {
                println!("Too big!");
                separator(40);
            }
            Ordering::Equal => {
                end_game();
                break;
            }
        }
        println!("You have {} chances left", chances);
    }
}

fn separator(n: usize) {
    println!("\n{}\n", "*".repeat(n));
}

fn end_game() {
    let largura = 32;

    println!("{}", "=".repeat(largura));
    println!("You win!");
    println!("{}", "=".repeat(largura));

    let arte_ascii = r#"
            
    //   ) )     //   / /     //   / /     //   ) )     //   ) )
//           //   / /     //____       ((           ((
//  ____     //   / /     / ____          \\           \\
//    / /    //   / /     //                 ) )          ) )
((____/ /    ((___/ /     //____/ /    ((___ / /    ((___ / /

     "#;

    println!("{}", arte_ascii);
}
