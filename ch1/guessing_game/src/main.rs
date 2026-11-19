// The io library comes from the standard library, known as std
use std::io;
use std::cmp::Ordering;
// The Rng trait defines methods that random number generators implement, and 
// this trait must be in scope for us to use those methods.
use rand::Rng;

// NB: By default, Rust has a set of items defined in the standard library that 
// it brings into the scope of every program (no need to import them). This set 
// is called the prelude.
// 
// If a type you want to use isn’t in the prelude, you have to bring that type 
// into scope explicitly with a use statement like done above

fn main() {
    println!("Guess the number!");

    // - thread_rng returns a random number generator that implements Rng, on that
    // we call gen_range.
    // - 1..=100 is a range expression. This particular kind is inclusive on 
    // both the lower and upper bound
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        // - le variabili si dichiarano con let e sempbra che venga applicata type
        //   inference
        // - le variabili sono immutabili di default
        // - String is a type that is provided by the stdlib that is a growable
        //   UTF-8 encoded bit of text 
        // - the :: syntax indicates that new is an ASSOCIATED FUNCTION of the 
        //   String type. An associated function is a function that’s implemented on 
        //   a type, in this case String.
        let mut guess = String::new();
        // - The stdin function returns an instance of 'std::io::Stdin', which is a 
        // type that represents a handle to the standard input for your terminal.
        // - read_line appends into the passed string
        // - the string argument needs to be mutable reference 
        //   - a reference lets multiple parts of our code access a piece of data
        //     withouth copying
        //   - like variables references are immutable by default, so we need to
        //     specify mut to allow the method to change the string's contents
        // - read_line also returns a Result value. Result is an enumeration (enum) 
        //   which is a type that can be in one of multiple possible states. We call 
        //   each possible state a variant. Result’s variants are Ok and Err. 
        //   - The Ok variant indicates the operation was successful, and it 
        //     contains the successfully generated value. 
        //   - The Err variant means the operation failed, and it contains 
        //     information about how or why the operation failed.
        // - An instance of Result has an expect method that you can call. If this 
        //   instance of Result is an Err value, expect will cause the program to 
        //   crash and display the message that you passed as an argument to expect
        //   - If this instance of Result is an Ok value, expect will take the 
        //     return value that Ok is holding and return just that value to you so 
        //     that you can use it. 
        //   - In this case, that value is the number of bytes in the user’s input.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // - lo shadowing, stiamo ridefinendo guess e usiamo il vecchio valore
        //   nell'espressione.
        // - il fatto che abbiamo annotato la definizione con il tipo in quanto
        //   parse() deve sapere a che tipo vogliamo convertire la stringa
        // - we're handling the error by using a match expression on the Result
        //   enum returned by parse()
        //   - if an Ok variant is produced, that Ok value will contain the 
        //     parsed number and will match the first arm's pattern
        //   - otherwise we ask for another input and we ignore the information
        //     about the error produced inside the Err variant value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // match all Err values, no matter what information they have inside
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        // The Ordering type is an enum with the variants Less, Greater, and Equal
        // 
        // here cmp() returns a variant of the Ordering enum and we use a match 
        // expression to decide what to do next based on which variant of Ordering 
        // was returned from the call to cmp with the values in guess and 
        // secret_number.
        // 
        // NB: a match expression ends after the first match
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}