use std::fmt;
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    Orange,
}

#[derive(Debug)]
struct ColorCombination {
    color1: Color,
    color2: Color,
    color3: Color,
    color4: Color,
}

struct Codebreaker {
    color1: Color,
    color2: Color,
    color3: Color,
    color4: Color,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Red => write!(f, "Red"),
            Color::Blue => write!(f, "Blue"),
            Color::Green => write!(f, "Green"),
            Color::Yellow => write!(f, "Yellow"),
            Color::Purple => write!(f, "Purple"),
            Color::Orange => write!(f, "Orange"),
        }
    }
}

impl fmt::Display for ColorCombination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.color1, self.color2, self.color3, self.color4
        )
    }
}

impl Codebreaker {
    fn new(color1: Color, color2: Color, color3: Color, color4: Color) -> Codebreaker {
        Codebreaker {
            color1,
            color2,
            color3,
            color4,
        }
    }
}

impl ColorCombination {
    fn new(color1: Color, color2: Color, color3: Color, color4: Color) -> ColorCombination {
        ColorCombination {
            color1,
            color2,
            color3,
            color4,
        }
    }

    fn get_codemaker_input(&self) -> String {
        format!(
            "({}, {}, {}, {})",
            self.color1, self.color2, self.color3, self.color4
        )
    }

    fn compare_codebreaker_codemaker(&self, codebreaker: &Codebreaker) -> (u32, u32) {
        let codemaker_colors = [&self.color1, &self.color2, &self.color3, &self.color4];
        let codebreaker_colors = [
            &codebreaker.color1,
            &codebreaker.color2,
            &codebreaker.color3,
            &codebreaker.color4,
        ];

        let mut black_pegs = 0;
        let mut white_pegs = 0;

        let mut codemaker_checked = vec![false; 4];
        let mut codebreaker_checked = vec![false; 4];

        // Check for black pegs (correct color and position)
        for i in 0..4 {
            if codemaker_colors[i] == codebreaker_colors[i] {
                black_pegs += 1;
                codemaker_checked[i] = true;
                codebreaker_checked[i] = true;
            }
        }

        // Check for white pegs (correct color, wrong position)
        for i in 0..4 {
            if !codebreaker_checked[i] {
                for j in 0..4 {
                    if !codemaker_checked[j] && codemaker_colors[j] == codebreaker_colors[i] {
                        white_pegs += 1;
                        codemaker_checked[j] = true;
                        break;
                    }
                }
            }
        }

        (black_pegs, white_pegs)
    }
}

fn get_color_from_input(input: &str) -> Result<Color, &'static str> {
    match input.trim().to_lowercase().as_str() {
        "red" => Ok(Color::Red),
        "blue" => Ok(Color::Blue),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "purple" => Ok(Color::Purple),
        "orange" => Ok(Color::Orange),
        _ => Err("Invalid color, please try again."),
    }
}

fn prompt(message: &str) -> String {
    loop {
        println!("{}", message);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match get_color_from_input(input.trim()) {
            Ok(color) => return color.to_string(),
            Err(_) => println!("Invalid color, please try again. colors available: red, blue, green, yellow, purple, orange"),
        }
    }
}

fn get_valid_color(message: &str, seleted_colors: &mut std::collections::HashSet<Color>) -> Color {
    loop {
        let input = prompt(message);
        match get_color_from_input(&input) {
            Ok(color) => {
                if seleted_colors.insert(color.clone()) {
                    return color;
                } else {
                    println!("Color already chosen, please select a different color.");
                }
            }
            Err(_) => println!("Invalid color, please try again."),
        }
    }
}

fn main() {
    println!("Welcome to the mastermind game!");
    println!("Please choose your secret color combination");

    let mut colors = std::collections::HashSet::new();

    let color1 = get_valid_color("Choose Color 1", &mut colors);
    let color2 = get_valid_color("Choose Color 2", &mut colors);
    let color3 = get_valid_color("Choose Color 3", &mut colors);
    let color4 = get_valid_color("Choose Color 4", &mut colors);

    let codemaker_combination = ColorCombination::new(color1, color2, color3, color4);
    println!("Your secret combination is: {:?}", codemaker_combination);

    let mut times_played = 0;
    let max_no_of_plays = 2;

    loop {
        times_played += 1;

        println!("Codebreaker, please choose your guess combination.");

        let guess_color1 = prompt("Choose Color 1");
        let guess_color1 = match get_color_from_input(&guess_color1) {
            Ok(color) => color,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        let guess_color2 = prompt("Choose Color 2");
        let guess_color2 = match get_color_from_input(&guess_color2) {
            Ok(color) => color,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        let guess_color3 = prompt("Choose Color 3");
        let guess_color3 = match get_color_from_input(&guess_color3) {
            Ok(color) => color,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        let guess_color4 = prompt("Choose Color 4");
        let guess_color4 = match get_color_from_input(&guess_color4) {
            Ok(color) => color,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        let codebreaker_guess =
            Codebreaker::new(guess_color1, guess_color2, guess_color3, guess_color4);

        let (black_pegs, white_pegs) =
            codemaker_combination.compare_codebreaker_codemaker(&codebreaker_guess);

        println!("{} black pegs and {} white pegs", black_pegs, white_pegs);
        if black_pegs == 4 {
            println!("Congratulations! You guessed the secret code.");
            println!("You win!");
            break;
        } else if times_played == max_no_of_plays {
            println!("You have played {} times. Game over!", max_no_of_plays);
            println!(
                "The Codemaker's combination is: {:?}",
                codemaker_combination.get_codemaker_input()
            );

            break;
        }
    }
}
