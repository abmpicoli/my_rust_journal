// this module will contain the data access operations and the data model.
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;


#[derive(Eq,PartialEq,Hash,Debug,Clone)]
struct IdMapping {
	
	key: u64,
	value: u64,
}

impl Display for IdMapping {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.key, self.value)
    }
	
}

struct Database<'a> {

	employees: Table<&'a str>,
	departments: Table<&'a str>,
	employee_vs_department: Table<IdMapping>,
}

#[derive(Eq,PartialEq,Hash,Debug,Clone)]
struct Row<T:Display + Debug + Clone> {
	
	id: u64,
	value: T
	
}

struct Table<T:Eq + PartialEq + Hash + Clone + Display + Debug> {
	next_primary_key: u64,
	row_id_by_name: HashMap<T,u64>,
	rows_by_id: HashMap<u64,Row<T>>,
	name: String
		
}

impl<T:Eq + PartialEq + Hash + Clone + Display +Debug > Table<T> {
	
	fn new(name:&str) -> Self {
		
		Table {
			next_primary_key: 0,
			rows_by_id: HashMap::new(),
			row_id_by_name:HashMap::new(),
			name: String::from(name)
		}
		
	}
	
	fn insert(&mut self,value2: T) -> Result<u64,String> { 
		let id2 = self.next_primary_key;
		
		if let Some(existing) = self.row_id_by_name.get(&value2) {
				return Err(String::from(format!("Value {} already exists in the database as row  {:?}",value2,existing)));
		};
		
		self.next_primary_key+=1;
		let row = Row  {
			id:id2,
			value:value2.clone()
		};
		let valueKey = row.value.clone();
		
		self.rows_by_id.insert(id2,row);
		self.row_id_by_name.insert(valueKey,id2);
		
		Ok(id2)
		
		
	
	}
	
	// dele
	fn update(&mut self) -> Result<Row<T>,&str> {
		Err("boom!")
	}
	
	// delete a row by name
	fn delete_by_data(&mut self,row: T ) -> Result<u64,&str> {
		Err("boom!")
		
	}
	
	fn delete_by_id(&mut self,id: u64) -> Result<u64,&str> {
		Err("boom!")
		
	}
	
	// find a row with the provided name.
	fn find(&self,name: &T) -> Option<Row<T>> {
		if self.row_id_by_name.contains_key(name) {
			return Some(self.rows_by_id.get(self.row_id_by_name.get(name).unwrap()).unwrap().clone());
		};
		None
	}
	
}

#[cfg(test)]
mod tests {
	use crate::database::*;	
	#[test]
	fn test_table() {
		let mut the_table: Table<String> = Table::new("my_table");

		// making the test to happen in an inner scope. Will the string references survive? 
		
		
		{ 
			let tommy1 = String::from("Tommy");
			let value = the_table.insert(tommy1).expect("Since this is an empty table, the first row should be accepted automatically");
			assert_eq!(0,value);
		};
		{
			let tommy2 = String::from("Tommy");
			let response = the_table.insert(tommy2);
			if let Err(ref msg) = response {
				println!("{}",msg);	
			}
			
			assert!(response.is_err(),"Adding a new row with the same name should return an error");
		};
		let found_tommy = the_table.find(&String::from("Tommy"));
		assert!(found_tommy.is_some() , "The find function should find Tommy under id 0");
	}
	
	#[test]
	fn test_table_with_str<'a>() {
		let mut the_table: Table<&'a str> = Table::new("my_table");

		{ 
			let tommy1:&'a str = String::from("Tommy").leak();
			let value = the_table.insert(tommy1).expect("Since this is an empty table, the first row should be accepted automatically");
			assert_eq!(0,value);
		};
		{
			let tommy2:&'a str = String::from("Tommy").leak();
			let response = the_table.insert(tommy2);
			if let Err(ref msg) = response {
				println!("{}",msg);	
			}
			assert!(response.is_err(),"Adding a new row with the same name should return an error");
		};
		
	}
}

