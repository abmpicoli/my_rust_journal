use crate::player::*;
use crate::scene::*;

pub struct Choice {
	
	is_applicable: fn (&Scene,&Player) -> boolean,
	apply: fn(Scene,Player)->(Scene,Player),
	describe: fn(&Player) -> String
	
}