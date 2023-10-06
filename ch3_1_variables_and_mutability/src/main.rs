mod functions;

const HELLO_WORLD: i32=10000000;

// let mut x = 2000;

   // Compiling ch3_1_variables_and_mutability v0.1.0 (/mnt/e/projetos/my_rust_journal/ch3_1_variables_and_mutability)
    // error: expected item, found keyword `let`
     // --> src/main.rs:5:1
       // |
       // 5 | let mut x = 2000;
         // | ^^^ expected item

fn main() {
    println!("Hello, world!");

    // can use mutable variables and reassign them
    let mut x = 100;

    x = x + HELLO_WORLD;

    // y is not mutable
    let y = HELLO_WORLD;

    // even though I did add the "mod functions" on top of this file,
	// it is not like a C dumb #include directive that accepts anything.
	// variables declared in the functions.rs can't be used directly, 
	// without some sort of "exposition" first.
    println! ( "{x} {y} {HELLO_WORLD}") ; 


    let y = y * 2; // rust accepts this because it is a "shadowing in the same scope". 
                   // AS LONG AS THE VARIABLE IS NOT MUTABLE.
    println! ( "New y = {y}");


// Cant use variables from other modules directly! the "mod" keyword on the first line is 
// not a blind C/C++ #include directive
//
    //println! ( "{x} {y} {HELLO_WORLD} {ANOTHER_THING}") ;  
// error[E0425]: cannot find value `ANOTHER_THING` in this scope
  // --> src/main.rs:17:40
   // |
// 17 |     println! ( "{x} {y} {HELLO_WORLD} {ANOTHER_THING}") ;
   // |                                        ^^^^^^^^^^^^^ not found in this scope
   // |
// note: constant `crate::functions::ANOTHER_THING` exists but is inaccessible
  // --> src/functions.rs:1:1
   // |
// 1  | const ANOTHER_THING: i32 = 2000000;
   // | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

//For more information about this error, try `rustc --explain E0425`.

// rust doesn't accept overflow in debug mode.
//
   //let mut y:u8 = 255 ;
   //y = y + 1 ; 
   //println!("Yet another y = {y}");
   // gives kernel panic



// CHARS ARE NOT INT. HENCE THIS CODE WON'T COMPILE
	println!("The UNICODE TABLE!")

	println!("HEX:     chars");

	let mut iCode:u32 = 32;
	let mut theChar:char = iCode;
	loop {
		if ( iCode % 256 == 0 ) {
			println!("-------");
		}
		
		if ( iCode % 32 == 0 ) {
			print!("\n{:#08x}",iCode)
		}
		if (iCode % 8 == 0) {
			print!(" ")
		}
		print!("{theChar}");
		iCode = iCode + 1;
		theChar = iCode;
		if (iCode == 0xffffffff) {
			break;
		}
	}
}
