use std::io;
use std::str;

pub fn play() {
    println!("Choose how many discs to use:");
    let mut discs = String::new();
    io::stdin()
        .read_line(&mut discs)
        .expect("Failed to read the number!");

    match discs.trim().parse() {
        Ok(num) => move_disc(num, "L", "R", "C"),

        Err(_) => {
            println!("Could not parse {:?}", discs);
        }
    };
}

fn move_disc(number_of: i32, src: &str, dest: &str, temp: &str) {
    if number_of > 0 {
        move_disc(number_of - 1, src, temp, dest);
        display_tower(number_of, src, dest);
        move_disc(number_of - 1, temp, dest, src);
    }
}

fn display_tower(number_of: i32, src: &str, dest: &str) {
    println!("Move disc {:?} from {:?} to {:?}", number_of, src, dest);
}
