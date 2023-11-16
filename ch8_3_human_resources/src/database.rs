// this module will contain the data access operations and the data model.
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;



pub struct Database {

	department_per_employee:HashMap<String,Option<String>>,
	employees_per_department:BTreeMap<String,BTreeSet<String>>

}

impl Database {
	
	pub fn new() -> Self {
		
		Database {
			department_per_employee: HashMap::new(),
			employees_per_department: BTreeMap::new()
		}
		
	}
	
	pub fn create_department(&mut self, department:&str) -> Result<(),&str> {
		let x = &mut self.employees_per_department;
		if x.contains_key(department) {
			
			return Ok(());
			
		}
		x.insert(String::from(department),BTreeSet::new());
		Ok(())
		
	}
	pub fn create_person(&mut self, person:&str) -> Result<(),&str> {
		let x = &mut self.department_per_employee;
		if x.contains_key(person) {
			return Err("Person already exists");
		}
		x.insert(String::from(person),None);
		
		Ok(())
		
	}
	pub fn move_person_to_department(&mut self, person:&str, department:&str) -> Result<(),&str> {
		let  employee_list = &mut self.department_per_employee;
		let  department_list = &mut self.employees_per_department;
		if employee_list.contains_key(person) && department_list.contains_key(department) {
			if let Some(old_department) = employee_list.get(person) {
				if let Some(old_department_name) = old_department {
					if let  Some(ref mut old_department_roster) = &mut department_list.get_mut(old_department_name) {
						old_department_roster.remove(person);
					};
				};
			};
			let department_roster = &mut department_list.get_mut(department).unwrap();
			department_roster.insert(String::from(person));
			employee_list.insert(String::from(person),Some(String::from(department)));
			return Ok(());
		};

		Err("Either the person or department do not exist")
		
	}
	
	pub fn get_department(&self, person:&str) -> Option<String> {
		if let Some(entry) = self.department_per_employee.get(person) {
			if let Some(entry2) = entry {
				return Some(String::from(entry2));
			};
		};
		None
		
	}
	
	pub fn report(&self) -> String {
		let mut the_report = String::new();	
		let mut department_divisor= "";
		for(department,roster) in self.employees_per_department.iter() {
			the_report.push_str(department_divisor);
			department_divisor=";";
			let mut employee_divisor="";
			the_report.push_str(department);
			the_report.push(':');
			for employee in roster.iter() {
				the_report.push_str(employee_divisor);
				the_report.push_str(employee);
				employee_divisor=",";
			}

		
		}
		return the_report;
	}
	
}



#[cfg(test)]
mod tests {
	use crate::database::*;	
	#[test]
	fn test_database() {
		let mut database: Database = Database::new();
		
		// I'm making this mutable, because I don't want to have terminal applications.
		let mut person:&str;
		let mut department:&str;
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
			database.move_person_to_department(person,department).expect("This operation should succeed");
			assert_eq!("Sales",database.get_department("Sally").unwrap());
		};
		
		
		{
			person="Ben";
			department="Accounting";
			assert!(database.move_person_to_department(person,department).is_err(),"It should not be possible to add a non-existing person to a non-existing department");
			database.create_department(department).expect("Departments should be accepted, even if it is a no-op");
			assert!(database.move_person_to_department(person,department).is_err(),"It should not be possible to add a non-existing person to a department");
			database.create_person(person).expect("Ben is a new person, it should be accepted");
			database.move_person_to_department(person,department).expect("Now adding ben to accounting, now that we have ben, and we have the accounting department, should be ok");
			
		};
		
		{ 
			assert_eq!("Accounting:Ben;Sales:Sally,Tommy",database.report(),"The report should have the departments in alphabetic order and the people belonging to the department in alphabetic order as well");
		};
		
	}

}

