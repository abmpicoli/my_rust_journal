
pub const NAKED:&str="You are naked";
pub const RAGGED_CLOTHING:&str ="You wear a ragged shirt and holey pants";


pub struct Player {
	
	// the tags will be a cute list consisting of 
	tags:String
}

pub fn format_tag(tag:&str)->String{
	let x = String::from("♦ ") + tag + "\n" ;
	x
}

impl Player {
	
	pub fn new() -> Player {
		return Player {
			tags: String::from("")
		}
	}
	
	pub fn add_tag(self,tag:&str) -> Player {
		if ! self.contains_tag(tag) {
			let mut x:String = self.tags.clone();
			x.push_str(&format_tag(tag));
			return Player {
				tags:x.clone()
			}
		}
		self
	}
	
	pub fn contains_tag(&self,tag:&str) -> bool {
		self.tags.contains(format_tag(tag).as_str())
	}
	pub fn remove_tag(self,tag:&str) -> Player {
		
		Player {
			tags: self.tags.clone().replace(format_tag(tag).as_str(),"")
		}
		
	}
	
}

#[cfg(test)]
mod tests {
	use crate::player::*;	
	#[test]
	fn test_tags() {
		let x = Player::new();
		let x = x.add_tag(NAKED);
		assert!(x.contains_tag(NAKED),"The player should have the naked tag now {}",x.tags);
		let x = x.add_tag(NAKED);
		assert_eq!(x.tags,format_tag(NAKED),"adding multiple tags the same tag should provide only one tag. It is providing instead {}",x.tags);
		let x = x.remove_tag(NAKED);
		assert_eq!(x.tags,"","removing the only tag should provide an empty string. Instead it was {}",x.tags);
		let x = x.add_tag(NAKED);
		let x = x.add_tag(RAGGED_CLOTHING);
		assert_eq!(x.tags,format_tag(NAKED).to_owned()+&format_tag(RAGGED_CLOTHING));
	}
	
}
