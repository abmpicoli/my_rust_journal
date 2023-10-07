fn main() {
    // are arrays mutable? are contents mutable?

    // arrays can be initialized with a value:
    //
    let x = [ 3; 5 ] ;
    // can i do this? Update the content of the NOT MUTABLE variable x?
    
	//NOPE CAN'T DO IT!
	
	//x[2]=142;
	// error[E0594]: cannot assign to `x[_]`, as `x` is not declared as mutable
	 // --> src/main.rs:8:5
	  // |
	// 8 |     x[2]=142;
	  // |     ^^^^^^^^ cannot assign
	  // |
	// help: consider changing this to be mutable
	  // |
	// 6 |     let mut x = [ 3; 5 ] ;
	  // |         +++	

	//AND WHAT IF I PLACE A LET ?
	
	// NOPE: the code is not considered syntatically valid.
	//let x[2]=142;
// error: expected one of `:`, `;`, `=`, `@`, or `|`, found `[`
  // --> src/main.rs:24:7
   // |
// 24 |     let x[2]=142;
   // |          ^ expected one of `:`, `;`, `=`, `@`, or `|`
	
    println!("Hello, world!");
}
