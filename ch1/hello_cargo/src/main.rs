fn main() {
    // println! calls a Rust macro. If it had called a function instead, it 
    // would be entered as println (without the !). Rust macros are a way to 
    // write code that generates code to extend Rust syntax, and we’ll discuss 
    // them in more detail in Chapter 20. 
    println!("Hello, world!");
}