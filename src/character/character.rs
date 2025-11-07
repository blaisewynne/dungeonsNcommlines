use std::io;

pub fn new_character() {
    print!("\x1B[2J\x1b[1;1H");
    character_name();
    character_class_init();
} 

fn character_name() {
    println!("\x1b[36mWhat is your character's name?");
    let mut character_name = String::new();
    io::stdin().read_line(&mut character_name).expect("");
    let character_name = character_name.trim();
    println!("{}", character_name);
}

fn character_class_init() {
   println!(" \x1b[1;34mWhat is your character's class?");
   println!(" \x1b[1;31m(1) Barbarian");
   println!(" \x1b[1;95m(2) Bard");
   println!(" \x1b[1;33m(3) Cleric");
   println!(" \x1b[1;92m(4) Druid");
   println!(" \x1b[1;96m(5) Fighter");
   println!(" \x1b[1;97m(6) Monk");
   println!(" \x1b[1;93m(7) Paladin");
   println!(" \x1b[1;34m(8) Ranger");
   println!(" \x1b[1;37m(9) Rogue");
   println!("\x1b[1;36m(10) Sorcerer");
   println!("\x1b[1;91m(11) Warlock");
   println!("\x1b[1;94m(12) Wizard");
   let mut character_class = String::new();
   io::stdin().read_line(&mut character_class).expect("");
   let character_class = character_class.trim();
   let character_choice: i32 = character_class.parse().expect("");
   character_class_fwd(character_choice);
}

fn character_class_fwd(choice: i32) {
    print!("{}", choice);
}
