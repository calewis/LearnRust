use std::io;

fn main() {
    let max_val: u32 = {
        println!("Please input the largest number to FizzBuzz to.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input
            .trim()
            .parse()
            .expect("Need to put in a valid u32 value.")
    };

    println!("\nSimple Version:");
    with_ifel(max_val);

    println!("\nWith Strings:");
    with_String(max_val);

    println!("\nWith strs:");
    with_str(max_val);
}

fn with_ifel(max_val: u32) {
    for i in 1..=max_val {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn with_String(max_val: u32) {
    let mut fb = String::new();

    for i in 1..=max_val {
        if i % 3 == 0 {
            fb += "Fizz";
        }
        if i % 5 == 0 {
            fb += "Buzz";
        }
        if fb.is_empty() {
            fb = i.to_string();
        }
        println!("{}", fb);
        fb.clear();
    }
}

// This one is broken :( 
fn with_str(max_val: u32) {
    let fb = "𝖥ⅰᴢᴢΒuᴢᴢ"; // Be evil (😈). 

    for i in 1..=max_val {
        // Not better than using string above I guess, 
        // but just playing around here
        let mut target = &i.to_string()[..];
        if i % 15 == 0 {
            target = &fb[..];
        } else if i % 3 == 0 {
            target = &fb[..4];
        } else if i % 5 == 0 {
            target = &fb[4..];
        }
        println!("{}", target);
    }
}
