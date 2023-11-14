use rand::{Rng, seq::SliceRandom};
use rand::distributions::{Distribution, Standard};


#[derive(Debug)]
struct Dwarf {}
#[derive(Debug)]
struct Elf {}
#[derive(Debug)]
struct Human {}
#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}
impl Distribution<Thing> for Standard {
 fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Thing {
    match rng.gen_range(0,2) {
        0 => Thing::Sword,
        1 => Thing::Trinket,
        _ => Thing::Trinket
    } 
 }
}
impl PartialEq for Thing {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}
trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;
    fn enchant(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success);
        print!("{:?} mutters incoherently. ", self);
        if spell_is_successful {
            println!("The {:?} glows brightly.", thing);
        } else {
            if thing != &Thing::Trinket {
                println!("The {:?} fizzes, then turns into a worthless trinket.", thing);
                *thing = Thing::Trinket;
            } else {
                println!("The {:?} fizzes, then turns into dust.", thing);
            }
        }
    }
}
impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5    
    }
}
impl Enchanter for Elf {
    fn competency(&self) -> f64 {
       0.95 
    }

}
impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}
fn main() {
    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};
    let party: Vec<&dyn Enchanter> = vec![&d, &e, &h];
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();
    let mut random_choose_thing = rand::random();
    spellcaster.enchant(&mut random_choose_thing)
}
