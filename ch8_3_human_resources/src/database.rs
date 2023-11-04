// this module will contain the data access operations and the data model.
use std::collections::HashMap;
use std::hash::Hash;

const DUPLICATED_ROW:&str="Duplicated row: there is already a {} with this {}" ;

struct Database<'a> {

	employees: Table<'a,&'a str>,
	departments: Table<'a,&'a str>,
	employee_vs_department: Table<'a,(u64,u64)>,
}

#[derive(Eq,PartialEq,Hash)]
struct Row<T> {
	
	id: u64,
	value: T
	
}

struct Table<'a, T:Eq + PartialEq + Hash + Clone > {
	next_primary_key: u64,
	row_id_by_name: HashMap<T,u64>,
	rows_by_id: HashMap<u64,Row<T>>,
	name: &'a str
		
}

impl<'b,T:Eq + PartialEq + Hash + Clone> Table<'b,T> {
	
	fn new(name:&'b str) -> Self {
		
		Table {
			next_primary_key: 0,
			rows_by_id: HashMap::new(),
			row_id_by_name:HashMap::new(),
			name:name
		}
		
	}
	
	fn insert(&mut self,value2: T) -> Result<u64,&str> { 
		let id2 = self.next_primary_key;
		self.next_primary_key+=1;
		let row = Row  {
			id:id2,
			value:value2
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
	fn find(&self,name: T) -> Option<Row<T>> {
		None
	}
	
}

#[cfg(test)]
mod tests {
	use crate::database::*;	
	#[test]
	fn test_table() {
		let mut the_table: Table<&str> = Table::new("my_table");
		the_table.insert("Tommy").expect("Since this is an empty table, the first row should be accepted automatically");
		assert!(the_table.insert("Tommy").is_err(),"Adding a new row with the same name should return an error");
		assert!(the_table.find("Tommy").unwrap().id == 0 );
	}
}

