fn main() {
    let first = String::from("Ferris"); // THIS IS A NON-MUTABLE VARIABLE!!!
    let full = add_suffix(first); // AND I CAN USE IT INTO A MUTABLE FUNCTION!
    println!("{full}");
}

fn add_suffix(name: String) -> String {
    let mut name = name;
	name.push_str(" Jr.");
    name
}
