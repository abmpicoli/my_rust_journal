use std::io::stdin;

struct Player {
	flags:String
	
}

struct Action {
	
	description:String,
	action: fn(&Scene,&Player)->Option<(&Scene,&Player)>
	
}


struct Scene {
	description:String,
	choices: [Option<Action>;3]
	
}



fn north_scene_action(s:&Scene, p:&Player) -> Option<(&Scene,&Player)> {
	
	if p.flags.contains("naked") {
		
		return Some((Scene {
			description: String::from("As you go north you reach a big encampment. It seems to be a caravan of some sort. They took pity on you all naked and gave you some ragged shirt to wear"),
			choices:[None,None,None]
			
		},Player {
			flags:String::from("You wear ragged shirts and pants")
		}));
	};
	None
}


fn main() {
	
	let mut player=Player {
		flags:String::from("You are naked");
	}
	
	let initial_square=Scene {
		description:String::from(r#"As your slowly open your eyes, you find yourself in a crossroad in the middle of a thick jungle. 
		The brambles and thorny branches you see in other places shows only 3 possible paths.

		A clearing to the east shows a small path leading to some sort of small settlement in the distance.
		The south path seems to lead to an even darker forest.

		There is evidence of tracks going from the south path to the north, showing a big group of people and 
		wheeled carts/ chariots, and horse hoofprints. 
		
		There are a few footprints in both directions from the east path. But they seem older, somehow.
		"#),
		choices: [ None,None,Some(Action {
			description:String::from("Go north"),
			action:north_scene_action
		})]
	}
	
	let mut scene: &Scene = &initial_square;
	loop {
		println!("{}\n-----\n{}\nWhat do you do?",player.flags,scene.description);
		for i in &(scene.choices) {
			if let i =Some(option) {
				println!("* {}",option.0);
			}
		}
		let mut choice=String::new()
		let mut consequence:&Option<fn(&Scene,Player)->Option<(&Scene,Player)>=None;
		stdin().read_line(&mut choice).expect("This doesn't happen. Error reading line??");
		for i in &(scene.choices) {
			
			if let Some(candidate)= i {
				if candidate.description.contains(&choice) {
					println!("You have chosen {}",candidate.description);
					consequence = Some(candidate.action);
					break;
				}
			}
		}
		match consequence {
			Some(t) => match t(&scene,&player) {
				Some(t2) => (scene,player) = t2,
				None => { 
					println!("The consequence of the action was not recognized. So the state is unknown. The game end");
					break;
				}
			},
			None => { 
				println!("Either the option was not recognized, or there is really nothing to do. The game ends.");
				break;
			}
		}
	}
	println!("THE ADVENTURE ENDS.");
	
}