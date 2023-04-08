use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;

fn main()
{
    println!("\n{}","  🦀🦀 Welcome to Colour Guessing Game 🦀🦀  ".cyan().bold().on_black());
    println!("{}","  🦀🦀 Play the game by guessing the RGB values for a random colour 🦀🦀  ".cyan().bold().on_black());

    let red = rand::thread_rng().gen_range(0..=255);
    let green = rand::thread_rng().gen_range(0..=255);
    let blue = rand::thread_rng().gen_range(0..=255);

    println!("\nActual Color: {}, {}, {}", red.to_string().red(), green.to_string().green(), blue.to_string().blue());

    let mut r = false;
    let mut g = false;
    let mut b = false;

    let mut red_guess = String::new();
    let mut green_guess = String::new();
    let mut blue_guess = String::new();

    let guess = "GUESS THIS COLOUR".truecolor(red, green, blue).bold().on_truecolor(255-red, 255-green, 255-blue);

    loop{
        println!("\n{}", guess);

        if !r {
            println!("Enter the colour value for {}", "RED".red());
            io::stdin().read_line(&mut red_guess).expect("Failed to read");
        }

        if !g {
            println!("Enter the colour value for {}", "GREEN".green());
            io::stdin().read_line(&mut green_guess).expect("Failed to read");
        }

        if !b {
            println!("Enter the colour value for {}", "BLUE".blue());
            io::stdin().read_line(&mut blue_guess).expect("Failed to read");
        };

        
        let red_guess: u8 = match red_guess.to_string().trim().parse() 
        {
            Ok(num) => {
                red_guess = 0.to_string();
                num
            }    
            Err(_) =>{
                println!("Invalid Input");
                continue;
            },
        };
        let green_guess: u8 = match green_guess.to_string().trim().parse() 
        {
            Ok(num) => {
                green_guess = 0.to_string();
                num
            }  
            Err(_) =>{
                println!("Invalid Input");
                continue;
            },
        };
        let blue_guess: u8 = match blue_guess.to_string().trim().parse() 
        {
            Ok(num) => {
                blue_guess = 0.to_string();
                num
            }  
            Err(_) =>{
                println!("Invalid Input");
                continue;
            },
        };

        println!("Your Guess: {}, {}, {}", red_guess.to_string().red(), green_guess.to_string().green(), blue_guess.to_string().blue());

        if !r {
            match red_guess.cmp(&red) {
                Ordering::Less => { 
                    println!("{}","Go Higher".red());
                },
                Ordering::Greater => {
                    println!("{}","Go Lower".red());
                },
                Ordering::Equal => {
                    println!("{} Value Guessed ❤️ ❤️ !!", "RED".red());
                    r = true;
                }
            }
        }    

        if !g {
            match green_guess.cmp(&green) {
                Ordering::Less => { 
                    println!("{}","Go Higher".green());
                },
                Ordering::Greater => {
                    println!("{}","Go Lower".green());
                },
                Ordering::Equal => {
                    println!("{} Value Guessed ☘️ ☘️ !!", "GREEN".green());
                    g = true;
                }
            }
        }

        if !b {
            match blue_guess.cmp(&blue) {
                Ordering::Less => { 
                    println!("{}","Go Higher".blue());
                },
                Ordering::Greater => {
                    println!("{}","Go Lower".blue());
                },
                Ordering::Equal => {
                    println!("{} Value Guessed ❄️ ❄️ !!", "BLUE".blue());
                    b = true;
                }
            }
        }

        if r && g && b {
            println!("{}","\n  COLOUR GUESSED 🙌 🙌 !!!!".yellow().bold().on_black());
            println!("{}","  YOU WON 🥳 🥳 🎉 🎉 🍾 🍾 🥂 🥂 !!!!".yellow().bold().on_black());
            break;
        }
    }

}

