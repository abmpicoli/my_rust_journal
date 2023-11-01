// this module will contain the data access operations and the data model.
use std::collections::HashMap;


const DUPLICATED_ROW="Duplicated row: there is already a {} with this {}" ;

struct Database {

	employees: Table<&str>,
	departents: Table<&str>,
	employee_vs_department: Table<(u64,u64)>,
	

}

struct Row<T> {
	
	id: u64,
	value: T
	
}




struct Table<T> {
	
	rows_by_name: HashMap<T,Row<T>>,
	rows_by_id: HashMap<u64,Row<T>>,
	name: &str
		
}

impl<T> Table<T> {
	
	fn new(name:&str) -> Self {
		
		Table {
			next_primary_key: 0,
			rows_by_id: HashMap::new(),
			rows_by_name:HashMap::new(),
			name:&str
		}
		
	}
	
	fn insert(name: T) -> Result<Row<T>,&str> { 
	
		Err("boom!")
	
	}
	
	// dele
	fn update() -> Result<Row<T>,&str> {
		Err("boom!")
	}
	
	// delete a row by name
	fn delete(row: T ) -> Result<u64,&str> {
		Err("boom!")
		
	}
	
	fn delete(id: u64) -> Result<u64,&str> {
		Err("boom!")
		
	}
	
	// find a row with the provided name.
	fn find(name: T) -> Option<Row> {
		None
	}
	
}

#[cfg(test)]
mod tests {
	use crate::database::*;	
	#[test]
	fn test_table() {
		let mut the_table = Table::new();
		the_table.insert("Tommy").expect("Since this is an empty table, the first row should be accepted automatically");
		assert!(the_table.insert("Tommy").is_err(),"Adding a new row with the same name should return an error");
		assert!(the_table.find("Tommy").unwrap().id == 0 );
	}
}

