fn main() {
    // can I declare a mutable variable and then shadow it with a non-mutable one?
    //
    //
    let mut x = 1;

    x = x + 42;
  
    println!("Hello world {}",x);

    // I'm shadowing the x with a non-mutable scope. Can I do it?
    let x = 2;
    // YES: I can. It is silly, though. And the compiler will warn about it.
    
    println!("Hello, world!");
}
