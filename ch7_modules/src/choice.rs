use crate::player::*;
use crate::scene::*;

pub struct Choice {
	
	is_applicable: fn (&Scene,&Player) -> bool,
	apply: fn(Scene,Player)->(Scene,Player),
	describe: fn(&Player) -> String
	
}

pub fn make_makeshift_clothes_with_straw() -> Choice {
	
	return Choice {
		is_applicable: fn(s:&Scene,p:&Player)-> bool {
			
			p.contains_tag(NAKED)
			
		},
		apply: fn(Scene,Player) -> (Scene,Player) {
			(Scene,Player.remove_tag(player::NAKED).add_tag(player::STRAW_CLOTHING))
		},
		describe: fn(&Player) -> String {
			String::from("Use straw , reed
		}
		
	}
	
}



#[cfg(test)]
mod tests {
	use crate::player::*;	
	use crate::scene::*;
	#[test]
	fn test_tags() {
		let x = Player::new();
		let x = x.add_tag(NAKED);
		let Choice = make_makeshift_clothes_with_straw();
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
