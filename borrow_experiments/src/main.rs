fn main() {
    
	
	// scenario 1.
	// only one variable can have access to a reference at any single time.
	
	// The java concern about having a string as an immutable object 
	// DOES EXIST IN RUST. BY DEFAULT. For all types, in principle.
	// And it works for reference values;
	
	let some_string="Hello!";
	
	// FROM THE FIRST EXPERIMENT, we deduct that we could also write the line above as:
	let some_string: &str ="Hello!"; 
	
	// The error message from this attempt shows an interesting message!
	
	
	
	// some_string.push_str("Hola!");
	   // |
	// 11 |     some_string.push_str("Hola!");
	   // |                 ^^^^^^^^ method not found in `&str`
	
	// the type of "some string" is a SLICE OF A STRING.
	// A slice is read-only. Can only inspect it's value.
	
	// EXPERIMENT 2:
	// Now we are trying to assing a String to a Slice. Let's try it:
	//let some_string: String = "Hello!";
	// error[E0308]: mismatched types
  // --> src/main.rs:30:28
   // |
// 30 |     let some_string: String = "Hello!";
   // |                      ------   ^^^^^^^^- help: try using a conversion method: `.to_string()`
   // |                      |        |
   // |                      |        expected `String`, found `&str`
   // |                      expected due to this
 
   // RUST DOESN'T HOLD HANDS. You should have learned that by now. There are no automatic conversions and promotions.

	// EXPERIMENT 3: I assume that a to_string will make a COPY of the slice, right? So I can println! both variables.
	let some_other_string: String = some_string.to_string();
	println!("some_string={}.some_other_string={}",some_string,some_other_string);
	//let yet_some_another_string = some_other_string;
	
	// a dummy experiment. Going to invoke an unsupported method. I want to see what is the type inferred here.
	//yet_some_another_string.banana();
   // |
// 48 |     yet_some_another_string.banana();
   // |                             ^^^^^^ method not found in `String`
	// OK, rust considers it a String instance anyway.

	// No borrowing happened, right? RIGHT?? WRONG!
   // println!("some_other_string = {some_other_string}; yet some another string={yet_some_another_string} ; some_string={some_string}");
// |  |
// 43 |     let some_other_string: String = some_string.to_string();
   // |         ----------------- move occurs because `some_other_string` has type `String`, which does not implement the `Copy` trait
// 44 |     println!("some_string={}.some_other_string={}",some_string,some_other_string);
// 45 |     let yet_some_another_string = some_other_string;
   // |                                   ----------------- value moved here
// ...
// 55 |     println!("some_other_string = {some_other_string}; yet some another string={yet_some_another_string} ; some_string={some_string}");
   // |                                   ^^^^^^^^^^^^^^^^^^^ value borrowed here after move
   // |
   // = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
   // |
// 45 |     let yet_some_another_string = some_other_string.clone();
   // |                                                    ++++++++

// EXPERIMENT 4: the fix for experiment 3.
   // let yet_some_other_string: &String = some_other_string;
   //println!("some_other_string = {some_other_string}; yet some another string={yet_some_another_string} ; some_string={some_string}");
   
   // Again , a String is different than a reference to a string.
// 73 |    let yet_some_other_string: &String = some_other_string;
   // |                               -------   ^^^^^^^^^^^^^^^^^ expected `&String`, found `String`
   // |                               |

}
