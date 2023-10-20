use crate::player::*;
use crate::scene::*;


pub const MAKE_MAKESHIFT_CLOTHES_DESCRIPTION:&str="Use straw , reed, whatever you can find to make something to cover your nakedness.";

pub struct Choice {
	
	is_applicable: fn (&Scene,&Player) -> bool,
	apply: fn(Scene,Player)->(Scene,Player),
	describe: fn(&Player) -> String
	
}

fn make_makeshift_is_applicable(s:&Scene,p:&Player)-> bool {
	
	p.contains_tag(NAKED)
	
}

fn make_makeship_apply(s:Scene,p:Player) -> (Scene,Player) {
	(s,p.remove_tag(NAKED).add_tag(STRAW_CLOTHING))
	
}

fn make_makeship_describe(p:&Player) -> String {
	
	String::from(MAKE_MAKESHIFT_CLOTHES_DESCRIPTION)
}

pub fn make_makeshift_clothes_with_straw() -> Choice {
	 Choice {
		is_applicable: self::make_makeshift_is_applicable,
		apply: self::make_makeship_apply,
		describe: self::make_makeship_describe
	}
		
}
	




#[cfg(test)]
mod tests {
	use crate::player::*;	
	use crate::scene::*;
	use crate::choice::*;
	#[test]
	fn test_tags() {
		let player = Player::new();
		let player = player.add_tag(NAKED);
		let scene = Scene { } ;
		let choice = make_makeshift_clothes_with_straw();
		assert_eq!(MAKE_MAKESHIFT_CLOTHES_DESCRIPTION,(choice.describe)(&player),"The description should match the static value");
		assert!((choice.is_applicable)(&scene,&player));
		let (scene,player) = (choice.apply)(scene,player);
		assert!(player.contains_tag(STRAW_CLOTHING),"the scene effect should have been applied");
		assert!(! (choice.is_applicable)(&scene,&player),"Now that the player is no longer naked, the choice is no longer applicable");
		
	}
}
