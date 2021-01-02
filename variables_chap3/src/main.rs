use rand::Rng;

fn main() {
    // Mutability 
    let mut x = 5;
    println!("x is: {}", x);
    x = 6;
    println!("x is: {}", x);

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // If conditionals are expressions 😮
    let r_val: i32 = rand::thread_rng().gen();
    let if_var = if r_val % 2 == 0 {2} else {3};
    println!("If var has a value of: {}.", if_var);

    // Functions and loops
    println!("loop_loop : {}.", loop_loop());
    println!("while_loop: {}.", while_loop());
    println!("for_loop  : {}.", for_loop());
}

fn loop_loop() -> i32 {
    let mut out = 0;
    return loop {
        out += 1;
        if out == 10 {
            break out;
        } 
    }
}

fn for_loop() -> i32 {
    let mut out = 0;
    for _ in 0..10 {
        out += 1;
    }
    return out;
}

fn while_loop() -> i32 {
    let mut out = 0;
    while out < 10 {
        out += 1;
    }
    return out;
}
