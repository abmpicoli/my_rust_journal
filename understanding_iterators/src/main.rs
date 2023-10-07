fn main() {
	// 1..5 seems to be the idiomatic expression of an iterator going from 1 to 5.
    // println!("{}",1..5);
	// WHAT AN INTERESTING ERROR! So... ops seems to be an operator library??
	//error[E0277]: `std::ops::Range<{integer}>` doesn't implement `std::fmt::Display`
// --> src/main.rs:3:19
  // |
// 3 |     println!("{}",1..5);
  // |                   ^^^^ `std::ops::Range<{integer}>` cannot be formatted with the default formatter
  // |
  // = help: the trait `std::fmt::Display` is not implemented for `std::ops::Range<{integer}>`
  // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
  // = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

	
	// will a for return the final value of x? 
	let y = for x in 1..5 {
		println!("{}!",x) ;
		
		// ANOTHER INTERESTING ERROR: for *is* an expression, yes, but must return always an empty tuple.
		//x * 2
  // --> src/main.rs:19:3
   // |
// 19 |         x * 2
   // |         ^^^^^ expected `()`, found integer
   // |
// help: you might have meant to break the loop with this value
   // |
// 19 |         break x * 2;
   // |         +++++      +
		()
	} ; 
	
	// ANOTHER INTERESTING ERROR:
	//println!("{}",y);

	// error[E0277]: `()` doesn't implement `std::fmt::Display`
  // --> src/main.rs:33:16
   // |
// 33 |     println!("{}",y);
   // |                   ^ `()` cannot be formatted with the default formatter
   // |
   // = help: the trait `std::fmt::Display` is not implemented for `()`
   // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   // = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

}
