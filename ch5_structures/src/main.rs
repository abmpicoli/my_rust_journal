

struct Event {

    odds: f64,
    event: String,
    min:f64,
    max:f64
  
}

// Can I do that? just specify the fields I want and leave the others out? 

fn build_event(odds: f64, event: &str) {
	Event {
		odds,
		event: String::from(event),
		min:-1,
		max:-1
	}
}

let the_events  = [ 
    build_event(0.01,"A whale falls from the sky and crashes over you."),
	build_event(5.0,"As you walk to the forest, you find a sword and shield!"),
	build_event(25.0,"A pack of giant rats attack you. Prepare for an encounter!")
];
fn main() {
    
    let mut totalOdds:f64=0;
    for f in the_events {
        totalOdds = totalOdds + f.odds;
    }
    println! ( "Total odds: {totalOdds}");
    let mut thisOdds:f64 = 0;
    the_events = the_events.map( | f | {
		let min = thisOdds;
		thisOdds = thisOdds + f.odds / totalOdds ; 
		let max = thisOdds;
		Event { min,max,..f }
		       
	} );
	for f in the_events {
		println("({f.odds}: {f.min} to {f.max}(excl) : {f.event }");
	}
    

}

