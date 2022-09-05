use std::str;

pub fn move_disc(number_of: i32, src: &str, dest: &str, temp: &str) -> i32 {
    if number_of > 0 {
        move_disc(number_of - 1, src, temp, dest);
        display_tower(number_of, src, dest);
        move_disc(number_of - 1, temp, dest, src);
    }

    number_of
}

fn display_tower(number_of: i32, src: &str, dest: &str) {
    println!("Move disc {:?} from {:?} to {:?}", number_of, src, dest);
}
