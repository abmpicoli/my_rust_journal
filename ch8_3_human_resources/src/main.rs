use ch8_3_human_resources::database::Database;
use std::io::*;
fn main() {
    
	let mut db = Database::new();
	loop {
		println!("\nAvailable options:\nCREATE PERSON <<person>>\nCREATE DEPT <<department>>\nMOVE <<person>> TO <<department>>\nREPORT\n<CTRL-C to exit>\n");
		let mut line = String::new();
		if let Err(s) = stdin().read_line(&mut line) {
			println!("Line couldn't be read. {}. Exitting.",s);
			break;
		}
		if line.starts_with("CREATE PERSON ") {
			
			let the_person_option=line.strip_prefix("CREATE PERSON ");
			if let Some(name) = the_person_option {
				if let Err(x) = db.create_person(&name.trim()) {
				
					println!("Error creating person {name}: {x}"); 
					continue;
				}
				println!("Operation successful.");
				continue;
			}
			println!("You need to provide a person name");
			continue;
		}
		if line.starts_with("CREATE DEPT ") {
			let the_dept_option=line.strip_prefix("CREATE DEPT ");
			if let Some(name) = the_dept_option {
				if let Err(x) = db.create_department(&name.trim()) {
				
					println!("Error creating department {name}: {x}"); 
					continue;
				}
				println!("Operation successful.");
				continue;
			}
			println!("You need to provide a department name");
			continue;
		}
		if line.starts_with("MOVE ")  {
			let the_move_clause_option = line.strip_prefix("MOVE ");
			if let Some(the_move_clause) = the_move_clause_option {
				let parts:Vec<&str> = the_move_clause.split("TO").collect();
				if parts.len() != 2 { 
					println!("Error: Move clause must have only a single 'TO' separator.");
					continue;
				}
				let person = parts[0].trim();
				let department=parts[1].trim();
				if let Err(x) = db.move_person_to_department(person,department) {
					println!("Error moving {person} to {department}: {x}");
					continue;
				}
				println!("Operation successful");
				continue;
			}
			println!("Error: missing complement to the MOVE operation");
			continue;
			
		}
		if line.starts_with("REPORT") {
			println!("{}",db.report());
			continue;
		}
		println!("Unrecognized command: >>{line}<<");
		
	}
	
	
	
	
	
	
	
	
}
