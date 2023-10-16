pub mod player {

pub const NAKED="You are naked";
pub const RAGGED_CLOTHING="You wear a ragged shirt and holey pants";

}

pub struct Player {
	
	// the tags will be a cute list consisting of 
	tags:String
}

impl Player {
	
	pub fn new()  {
		return Player {
			tags: String::from("")
		}
	}
	
	pub fn add_tag(self,tag:&str) -> Player {
		if ! contains_tag(self,tag) {
			return Player {
				tags:self.tags.clone() + format_tag(tag)
			}
		}
		self
	}
	
	pub fn contains_tag(self,tag:&str) -> bool {
		self.tags.contains(format_tag(tag))
	}
	pub fn remove_tag(self,tag:&str) -> Player {
		
		Player {
			tags: self.tag.clone().replace(format_tag(tag),"")
		}
		
	}
	pub fn format_tag(tag:&str)->&str {
		return "♦ " + tag + "\n"
	}
}

#[cfg(test)]
mod tests {
	
	#[test]
	fn test_tags() {
		let x = Player::new();
		x = x.add_tag(NAKED);
		assert!(x.contains_tag(NAKED),"The player should have the naked tag now {}",x.tags);
		x = x.add_tag(NAKED);
		assert_eq!(x.tags,format_tag(NAKED),"adding multiple tags the same tag should provide only one tag. It is providing instead {}",x.tags);
		x = x.remove_tag(NAKED);
		assert_eq!(x.tags,"","removing the only tag should provide an empty string. Instead it was {}",x.tags);
		x = x.add_tag(NAKED);
		x = x.add_tag(RAGGED_CLOTHING);
		assert_eq!(x.tags,format_tag(NAKED)+format_tag(RAGGED_CLOTHING));
	}
	
}
