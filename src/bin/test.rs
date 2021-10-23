use dnd_charsheet::compendium::{backgrounds, classes, races};

fn main() {
    let _races = races::get_races();
    let _classes = classes::get_classes();
    let _backgrounds = backgrounds::get_backgrounds();

    for i in _races.iter() {
        println!("{}", i.name);
        match i.subraces.clone() {
            Some(r) => {
                for j in r {
                    println!("  {}", j.name);
                }
            }
            _ => {}
        }
    }
}
