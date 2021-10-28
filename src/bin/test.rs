use dnd_charsheet::compendium::{backgrounds, classes, feats, items, races, spells};

fn main() {
    let _races = races::get_races();
    let _classes = classes::get_classes();
    let _backgrounds = backgrounds::get_backgrounds();
    let _items = items::get_items();
    let _spells = spells::get_spells();
    let _feats = feats::get_feats();

    for r in _races.iter() {
        println!("{}", r.name);
        if let Some(r) = r.subraces.clone() {
            for j in r {
                println!("  {}", j.name);
            }
        };
    }

    println!();

    for c in _classes.iter() {
        println!("{}", c.name);
        for i in c.subclasses.clone() {
            println!("  {}", i.name);
        }
    }
}
