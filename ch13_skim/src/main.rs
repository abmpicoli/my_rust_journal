fn main() {
    println!("Hello, world!");
	let throw_away = |x| -> () { println!("Will not throw away, right? {:?}",x) };
	let something_that_is_moved=vec!["1","2","3"];
	throw_away(something_that_is_moved);
	println!("This is not moved, right? {:?}",&something_that_is_moved);
	
}
