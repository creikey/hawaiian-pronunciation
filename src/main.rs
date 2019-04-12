use std::io;
use std::io::prelude::*;

const ENTER_HAWAIIAN_WORD: &str = "Enter a hawaiian word to pronounce ==> ";
const INVALID_CHARACTER: &str = " is not a valid hawaiian character";
const PRONUNCIATION: &str = " is pronounced ";
const ENTER_ANOTHER_WORD: &str = "Do you want to enter another word? Y/YES/N/NO ==> ";

fn flush_stdout() {
    io::stdout().flush().ok().expect("Could not flush stdout");
}

fn get_line(to_read_into: &mut String) {
    io::stdin()
        .read_line(to_read_into)
        .expect("Failed to read line");
}

fn main() {
    loop {
        print!("{}", ENTER_HAWAIIAN_WORD);
        flush_stdout();

        let mut hawaiian_word = String::new();

        get_line(&mut hawaiian_word);

        let valid = false;

        if valid {
            println!("{}{}{}", hawaiian_word, PRONUNCIATION, hawaiian_word); // TODO find pronunciation
        } else {
            println!(
                "{}{}",
                hawaiian_word.chars().nth(0).unwrap(),
                INVALID_CHARACTER
            );
        }

        let mut exit = false;
        let mut decision_made = false;

        while !decision_made {
            print!("{}", ENTER_ANOTHER_WORD);
            flush_stdout();

            let mut enter_another_word = String::new();

            get_line(&mut enter_another_word);
            enter_another_word.pop(); // remove newline

            match enter_another_word.as_ref() {
                "Y" | "YES" => {
                    println!();
                    decision_made = true;
                }
                "N" | "NO" => {
                    exit = true;
                    decision_made = true;
                }
                _ => continue,
            }
        }
        if exit {
            break;
        }
    }
}

#[cfg(tests)]
mod tests {
    use super::*;
}
