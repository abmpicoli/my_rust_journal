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
// abpicoli@DESKTOP-EPFPMPH:/mnt/e/projetos/my_rust_journal/src$ cargo run
   // Compiling learning_rust v0.1.0 (/mnt/e/projetos/my_rust_journal)
// warning: unused variable: `x`
  // --> src/main.rs:12:9
   // |
// 12 |     let x = 2;
   // |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   // |
   // = note: `#[warn(unused_variables)]` on by default

// warning: `learning_rust` (bin "learning_rust") generated 1 warning (run `cargo fix --bin "learning_rust"` to apply 1 suggestion)
    // Finished dev [unoptimized + debuginfo] target(s) in 1.79s
     // Running `/mnt/e/projetos/my_rust_journal/target/debug/learning_rust`
// Hello world 43
// Hello, world!    
    println!("Hello, world {}",x);
}
