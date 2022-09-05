use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::rc::Weak;

mod tower;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    loop {
        println!("Choose game!");
        println!("============");
        println!("1. Binary Tree.");
        println!("2. Towers of Hanoi!");
        println!("3. Testing Node struct!");
        println!("4. End the program!");

        let mut choise = String::new();

        io::stdin()
            .read_line(&mut choise)
            .expect("Failed to read the line!");

        let choise: i32 = match choise.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choise {
            1 => bin_menu(),
            2 => hanoi_menu(),
            3 => {
                let leaf = Rc::new(Node {
                    value: 3,
                    parent: RefCell::new(Weak::new()),
                    children: RefCell::new(vec![]),
                });
                println!(
                    "leaf strong: {:?}, weak: {:?}",
                    Rc::strong_count(&leaf),
                    Rc::weak_count(&leaf)
                );

                {
                    let branch = Rc::new(Node {
                        value: 5,
                        parent: RefCell::new(Weak::new()),
                        children: RefCell::new(vec![Rc::clone(&leaf)]),
                    });

                    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
                    println!(
                        "branch strong: {:?}, weak: {:?}",
                        Rc::strong_count(&branch),
                        Rc::weak_count(&branch)
                    );
                    println!(
                        "leaf strong: {:?}, weak: {:?}",
                        Rc::strong_count(&leaf),
                        Rc::weak_count(&leaf)
                    );
                }

                println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
                println!(
                    "leaf strong: {:?}, weak: {:?}",
                    Rc::strong_count(&leaf),
                    Rc::weak_count(&leaf)
                );
            }
            4 => {
                println!("Bye bye!");
                break;
            }
            other => println!("Choose a number between 1 and 3! Choise: {:?}", other),
        }
    }
}

fn hanoi_menu() {
    loop {
        println!("1. Play.");
        println!("2. Go back.");
        println!("Choose: ");

        let mut choise = String::new();

        io::stdin()
            .read_line(&mut choise)
            .expect("Failed to read the line!");

        let choise: u32 = match choise.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choise {
            1 => {
                println!("Choose how many discs to use:");
                let mut discs = String::new();
                io::stdin()
                    .read_line(&mut discs)
                    .expect("Failed to read the number!");

                let discs: i32 = match discs.trim().parse() {
                    Ok(num) => tower::move_disc(num, "L", "R", "C"),

                    Err(_) => {
                        println!("Could not parse {:?}", discs);
                        break;
                    }
                };
            }
            2 => break,
            other => println!("Please pick a number between 1-5: {:?}", other),
        }
    }
}

fn bin_menu() {
    loop {
        println!("MENU");
        println!("====");
        println!("1. Insert 10000 random numbers.");
        println!("2. Search the tree for all the numbers.");
        println!("3. Search for a given number.");
        println!("4. Erase the tree and destroy the data.");
        println!("5. Go back.");
        println!("Choose: ");

        let mut choise = String::new();

        io::stdin()
            .read_line(&mut choise)
            .expect("Failed to read the line!");

        let choise: u32 = match choise.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choise {
            1 => println!("You have chosen: {:?}", 1),
            2 => println!("You have chosen: {:?}", 2),
            3 => println!("You have chosen: {:?}", 3),
            4 => println!("You have chosen: {:?}", 4),
            5 => break,
            other => println!("Please pick a number between 1-5: {:?}", other),
        }
    }
}
