// this module will contain the data access operations and the data model.
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;

struct Database<'a> {

	employees: Table<&'a str>,
	departments: Table<&'a str>,
	department_per_employee:HashMap<&'a str,&'a str>	


}

impl <'a> Database<'a> {
	
	fn new() -> Self {
		
		Database {
			employees: Table::new("employees"),
			departments: Table::new("departments"),
			department_per_employee : HashMap::new()
			
		}
		
	}
	
	fn create_department(&mut self, department:&'a str) -> Result<(),&str> {
		
		if self.departments.insert(department).is_err() {
			return Err("department already exists")
		}
		return Ok(())
		
		
		
		
	}
	fn create_person(&mut self, person:&'a str) -> Result<(),&str> {
		
		if self.employees.insert(person).is_err() {
			return Err("employee already registered")
		}
		Ok(())
		
	}
	fn move_person_to_department(&mut self, person:&'a str, department:&'a str) -> Result<(),&str> {
		
		self.department_per_employee.insert(String::from(person).leak(),String::from(department).leak());
		Ok(())
		
	}
	
	fn get_department(&self, person:&str) -> Option<&'a str> {
		None
	}
	
	fn report(&self) -> &str {
		
		"not implemented"
		
	}
	
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
	
	// delete a row by name. 
	// returns the deleted id if successful, or an error with a message on failure.
	fn delete_by_data(&mut self,content: &T ) -> Result<u64,&str> {
		if let Some(row2) = self.find(content) {
			self.row_id_by_name.remove(content);
			self.rows_by_id.remove(&row2.id);
			return Ok(row2.id);
		}
		return Err("Record not found");
		
	}
	
	fn delete_by_id(&mut self,id: u64) -> Result<u64,&str> {
		let idMap = &mut self.rows_by_id;
		if idMap.contains_key(&id) {
			let value = idMap.get(&id).unwrap().value.clone();
			idMap.remove(&id);
			self.row_id_by_name.remove(&value);
			return Ok(id);
		}
		return Err("Record not found");
		
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
		{
			let tommy3:&'a str = String::from("Tommy").leak();
			assert_eq!(0,the_table.delete_by_data(&tommy3).unwrap());
			assert_eq!(1,the_table.insert(tommy3).expect("Now that tommy was deleted, I should be able to add tommy again"),"A new tommy record should be added, with a new id");
			assert_eq!(1,the_table.delete_by_id(1).expect("I should have the row id here, showing the deletion by id did work"));
		};
	}
	#[test]
	fn test_database<'a>() {
		let mut database: Database = Database::new();
		
		// I'm making this mutable, because I don't want to have terminal applications.
		let mut person:&'a str = "use later";
		let mut department:&'a str = "use later";
		{
			department = "Sales";
			database.create_department(department).expect("The Sales department should be created with no issues");
		
		};
	
		{
			person="Tommy";
			database.create_person(person).expect("Tommy should be created.");
			assert!(database.get_department(person).is_none());
			department="Sales";
			database.move_person_to_department(person,department).expect("Tommy should be moved to sales with no issues");
			assert_eq!("Sales",database.get_department("Tommy").unwrap());
			let response = database.move_person_to_department(person,"Accounting");
			assert!(response.is_err(),"The department must exist first. Adding a person to a non-existing department shouldn't happen");
		};
		{
			person = "Sally";
			department = "Sales";
			database.create_person(person).expect("Sally should be created");
			database.create_department(department).expect("Adding an existent department should be a no-op");
			database.move_person_to_department(person,department);
			assert_eq!("Sales",database.get_department("Sally").unwrap());
		};
		
		
		{
			person="Ben";
			department="Accounting";
			assert!(database.move_person_to_department(person,department).is_err(),"It should not be possible to add a non-existing person to a non-existing department");
			database.create_department(department);
			assert!(database.move_person_to_department(person,department).is_err(),"It should not be possible to add a non-existing person to a department");
			database.create_person(person);
			database.move_person_to_department(person,department).expect("Now adding ben to accounting, now that we have ben, and we have the accounting department, should be ok");
			
		};
		
		{ 
			assert_eq!("Accounting:Ben;Sales:Sally,Tommy",database.report(),"The report should have the departments in alphabetic order and the people belonging to the department in alphabetic order as well");
		};
		
	}

}

