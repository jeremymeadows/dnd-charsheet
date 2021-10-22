use dnd_charsheet::compendium::srd;

fn main() {
    let _races = srd::get_races();
    let classes = srd::get_classes();
    let _backgrounds = srd::get_backgrounds();

    for i in classes.iter().map(|x| x.name.clone()) {
      println!("{}", i);
    }
    println!("\n{:?}", classes[0].saves);
}
