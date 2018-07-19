fn main() {
	//Explicit polymorphism
	let Farmer: &Human = &Creature::new();
	let Haggard_Dwarf: &Dwarf = &Creature::new();
	let Stupid_Orc: &Orc = &Creature::new();

	//Our fighters in the ring
	let fights: &[&Creature] = &[Farmer, Haggard_Dwarf, Stupid_Orc];

	//We run our fighting function, which forces all creatures to fight. Because we've used polymorphism, all of our dwarves, humans and orcs are creature types and will fight.
	fighting(fights);
	println!("The Orc's health drops by {:?} from the human and {:?} by the dwarf, he has reached {:?} hitpoints!!", Farmer.weapon, Haggard_Dwarf.weapon, (Stupid_Orc.constitution - (Farmer.weapon + Haggard_Dwarf.weapon)));
	println!("The Dwarf is rather unamused, he has {:?} hitpoints remaining", (Haggard_Dwarf.constitution - Stupid_Orc.weapon));
}

//Traits that all of our objects will need. They are able to fight now. 
trait Creature {
	fn fight(&self);
	fn new() -> Self where Self: Sized;
}

//Some Species
pub struct Human{
	constitution : u32,
	weapon : u32,
}

pub struct Dwarf{
	constitution : u32,
	weapon : u32,
}

pub struct Orc{
	constitution : u32,
	weapon : u32,
}

//Implementing the fight function from the creature trait
impl Creature for Human {
	fn fight(&self){
		println!("The human stands tall, swings his sword and hits the orc for {:?}", self.weapon);
	}
	fn new() -> Human {
		Human {constitution : 100, weapon : 10}
	}
}

//Implementing the fight function from the creature trait
impl Creature for Dwarf {
	fn fight(&self){
		println!("The small Dwarf, swings a maul larger than him and hits the orc for {:?}", self.weapon);
	}
	fn new() -> Dwarf {
		Dwarf {constitution : 75, weapon : 15}
	}
}

//Implementing the fight function from the creature trait
impl Creature for Orc {
	fn fight(&self){
		println!("The Orc, with his poorly made weapon, seems to have some fighting knowledge and hits the stubby creature for {:?}", self.weapon);
	}
	fn new() -> Orc {
		Orc {constitution : 150 , weapon : 7}
	}
}
//Here we can see that all of our species are now of creature types, and we can treat them as such to have them fight.
fn fighting(creatures: &[&Creature]) {
		for creature in creatures.iter() {
        	creature.fight();
        }
}








