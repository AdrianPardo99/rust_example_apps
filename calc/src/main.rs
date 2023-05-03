use std::io;

fn main() {
    /*
     * Mutable because in runtime we need to change the current value and work with that variable later
     **/
    let mut input = String::new();
    /*
     * Rust variables can be mutable or immutable you only need to add keyword `mut` for identify that
     * You need to know that variable can be use it only one time in runtime you cannot use it like
     * let var_a = &input;
     * let mut var_b = &input;
     * This part of code can be trigger an error
     **/
    println!("Enter your weight (kg):");
    /*
     * Use unwrap for not handling more pointers that need function read_line (More info in doc)
     **/
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    /*
     * Part for show in console the value/type of variable
     **/
    dbg!(weight);

    print!("Input a number: {}", input);
    let mars_weight = calculate_weight_on_mars(weight);
    //mars_weight *=1000.0;
    println!("Weight on mars: {}g", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    return (weight / 9.81) * 3.711;
}
