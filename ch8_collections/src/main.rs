fn main() {

	// Exteriment from chapter 8. Let's see if this project runs - the doc says it wont due to the v.push on 
	// borrowed data.
    println!("Hello, world!");
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

//error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  //--> src/main.rs:10:5
   // |
// 8  |     let first = &v[0];
   // |                  - immutable borrow occurs here
// 9  |
// 10 |     v.push(6);
   // |     ^^^^^^^^^ mutable borrow occurs here
// 11 |
// 12 |     println!("The first element is: {first}");
   // |

//    v.push(6);

    println!("The first element is: {first}");
	
	// changing the experiment a little. 
	// the borrow should end once the variable with the issue is borrowed, right?
	// so, now that the value 
	
	// what is the type of first? : I'm trying an absurd operation here, to see what happens in the error:
	//let second = first + "BANANA!"; 
	// error[E0277]: cannot add `&str` to `&{integer}`
  // --> src/main.rs:31:21
   // |
// 31 |     let second = first + "BANANA!";
   // |                        ^ no implementation for `&{integer} + &str`
   // |
   // = help: the trait `Add<&str>` is not implemented for `&{integer}`
   // = help: the following other types implement trait `Add<Rhs>`:
             // <isize as Add>
             // <isize as Add<&isize>>
             // <i8 as Add>
             // <i8 as Add<&i8>>
             // <i16 as Add>
             // <i16 as Add<&i16>>
             // <i32 as Add>
             // <i32 as Add<&i32>>
           // and 48 others

// For more information about this error, try `rustc --explain E0277`.

  // SO... the vector type is a REFERENCE to an integer. Should an * do the trick?
  let first = *first; // YAY!!! IT WORKS!!! As long as the content is not a complex type that demands heap.
  v.push(6);
  println!("The first element is: {first}");
}
