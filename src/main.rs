use std::io::stdin;
fn main() {
    println!("Hello, please enter your name");
    let mut name = String::new();

    // We use the _ to mark as the posible result
    let _ = stdin().read_line(&mut name);
    greeting(name);
    
}

fn greeting(name: String){
    println!("Hi, its a pleasure {}", name.trim());
}