

struct Event {

    odds: f64,
    text: String,
    min:f64,
    max:f64
  
}

impl Event {
	
// Can I do that? just specify the fields I want and leave the others out? 

	fn new(odds: f64, text: &str) -> Self {
		Self {
			odds,
			text: String::from(text),
			min:-1.0,
			max:-1.0
		}
	}
}

//   |
//23 | let the_events  = [
//   | ^^^ consider using `const` or `static` instead of `let` for global variables

// Global variables seems to be required to know the final value at compile time:
// error[E0015]: cannot call non-const fn `build_event` in statics
  // --> src/main.rs:27:5
   // |
// 27 |     build_event(0.01,"A whale falls from the sky and crashes over you."),
   // |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   // |
   // = note: calls in statics are limited to constant functions, tuple structs and tuple variants
   // = note: consider wrapping this expression in `Lazy::new(|| ...)` from the `once_cell` crate: https://crates.io/crates/once_cell

//static the_events:[Event;3]  = [ 
//    build_event(0.01,"A whale falls from the sky and crashes over you."),
//	build_event(5.0,"As you walk to the forest, you find a sword and shield!"),
//	build_event(25.0,"A pack of giant rats attack you. Prepare for an encounter!")
//];
fn main() {
    let the_events:[Event;3]  = [ 
     Event::new(0.01,"A whale falls from the sky and crashes over you."),
	 Event::new(5.0,"As you walk to the forest, you find a sword and shield!"),
	 Event::new(25.0,"A pack of giant rats attack you. Prepare for an encounter!")
    ];
    let mut total_odds:f64=0.0;
//...oh man, the borrowing is a pain...
// 49 |     for f in the_events {
   // |              ---------- `the_events` moved due to this implicit call to `.into_iter()`
//...
    for f in &the_events {
        total_odds = total_odds + f.odds;
    }
    println! ( "Total odds: {total_odds}");
    let mut this_odds:f64 = 0.0;
    let the_events = the_events.map( | f | {
		let min = this_odds;
		this_odds = this_odds + f.odds / total_odds ; 
		let max = this_odds;
		Event { min,max,..f }
		       
	} );
	for f in the_events {
		let odds = f.odds;
		let min = f.min;
		let max = f.max;
		let text = f.text;
		println!("({odds}: {min} to {max}(excl) : {text}");
	}
    

}

