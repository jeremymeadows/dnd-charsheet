use ron;
use crate::compendium::Race;

pub fn get_races() -> Vec<Race> {
    ron::from_str::<Vec<Race>>(SRD_RACES).unwrap()
}

static SRD_RACES: &str = r#"[
Race(
    name: "Dragonborn",
    size: Medium,
    speed: 30,
    abilities:(
        strength: 2,
        dexterity: 0,
        constitution: 0,
        intelligence: 0,
        wisdom: 0,
        charisma: 1,
    ),
    traits:{
        "Draconic Ancestry":
            "You have draconic ancestry. Choose one type of dragon from the Draconic Ancestry table. Your breath weapon and damage resistance are determined by the dragon type, as shown in the table. (Player's Handbook p. 34)",
        "Breath Weapon":
            "You can use your action to exhale destructive energy. Your draconic ancestry determines the size, shape, and damage type of the exhalation.\n\nWhen you use your breath weapon, each creature in the area of the exhalation must make a saving throw, the type of which is determined by your draconic ancestry. The DC for this saving throw equals 8 + your Constitution modifier + your proficiency bonus. A creature takes 2d6 damage on a failed save, and half as much damage on a successful one. The damage increases to 3d6 at 6th level, 4d6 at 11th level, and 5d6 at 16th level.\n\nAfter you use your breath weapon, you can't use it again until you complete a short or long rest.",
        "Damage Resistance":
            "You have resistance to the damage type associated with your draconic ancestry.",
        "Languages":
            "You can speak, read, and write Common and Draconic. Draconic is thought to be one of the oldest languages and is often used in the study of magic. The language sounds harsh to most other creatures and includes numerous hard consonants and sibilants.",
    },
),
Race(
    name: "Dwarf (Hill)",
    size: Medium,
    speed: 25,
    abilities:(
        strength: 0,
        dexterity: 0,
        constitution: 2,
        intelligence: 1,
        wisdom: 0,
        charisma: 0,
    ),
    traits:{
        "Darkvision":
            "Accustomed to life underground, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
        "Dwarven Resilience":
            "You have advantage on saving throws against poison, and you have resistance against poison damage.",
        "Dwarven Combat Training":
            "You have proficiency with the battleaxe, handaxe, light hammer, and warhammer.",
        "Tool Proficiency":
            "You gain proficiency with the artisan's tools of your choice: smith's tools, brewer's supplies, or mason's tools.",
        "Stonecunning":
            "Whenever you make an Intelligence (History) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check, instead of your normal proficiency bonus.",
        "Languages":
            "You can speak, read, and write Common and Dwarvish. Dwarvish is full of hard consonants and guttural sounds, and those characteristics spill over into whatever other language a dwarf might speak.",
        "Dwarven Toughness":
            "Your hit point maximum increases by 1, and it increases by 1 every time you gain a level.",
    },
),
Race(
    name: "Dwarf (Mountain)",
    size: Medium,
    speed: 30,
    abilities:(
        strength: 2,
        dexterity: 0,
        constitution: 2,
        intelligence: 0,
        wisdom: 0,
        charisma: 0,
    ),
    traits:{
        "Darkvision":
            "Accustomed to life underground, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
        "Dwarven Combat Training":
            "You have proficiency with the battleaxe, handaxe, light hammer, and warhammer.",
        "Tool Proficiency":
            "You gain proficiency with the artisan's tools of your choice: smith's tools, brewer's supplies, or mason's tools.",
        "Stonecunning":
            "Whenever you make an Intelligence (History) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check, instead of your normal proficiency bonus.",
        "Languages":
            "You can speak, read, and write Common and Dwarvish. Dwarvish is full of hard consonants and guttural sounds, and those characteristics spill over into whatever other language a dwarf might speak.",
        "Dwarven Armor Training":
            "You have proficiency with light and medium armor.",
    },
),
Race(
    name: "Elf (High)",
    size: Medium,
    speed: 30,
    abilities:(
        strength: 0,
        dexterity: 2,
        constitution: 0,
        intelligence: 1,
        wisdom: 0,
        charisma: 0,
    ),
    traits:{
        "Darkvision":
            "Accustomed to twilit forests and the night sky, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
        "Keen Senses":
            "You have proficiency in the Perception skill.",
        "Fey Ancestry":
            "You have advantage on saving throws against being charmed, and magic can't put you to sleep.",
        "Trance":
            "Elves don't need to sleep. Instead, they meditate deeply, remaining semiconscious, for 4 hours a day. (The Common word for such meditation is \"trance.\") While meditating, you can dream after a fashion; such dreams are actually mental exercises that have become reflexive through years of practice. After resting in this way, you gain the same benefit that a human does from 8 hours of sleep.",
        "Languages":
            "You can speak, read, and write Common, Elvish, and one extra language of your choice. Elvish is fluid, with subtle intonations and intricate grammar. Elven literature is rich and varied, and their songs and poems are famous among other races. Many bards learn their language so they can add Elvish ballads to their repertoires.",
        "Elf Weapon Training":
            "You have proficiency with the longsword, shortsword, shortbow, and longbow.",
        "Cantrip":
            "You know one cantrip of your choice from the wizard spell list. Intelligence is your spellcasting ability for it.",
    },
    skills:(
        perception: true
    ),
),
Race(
    name: "Elf (Wood)",
    size: Medium,
    speed: 30,
    abilities:(
        strength: 0,
        dexterity: 2,
        constitution: 0,
        intelligence: 0,
        wisdom: 1,
        charisma: 0,
    ),
    traits:{
        "Darkvision":
            "Accustomed to twilit forests and the night sky, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
        "Keen Senses":
            "You have proficiency in the Perception skill.",
        "Fey Ancestry":
            "You have advantage on saving throws against being charmed, and magic can't put you to sleep.",
        "Trance":
            "Elves don't need to sleep. Instead, they meditate deeply, remaining semiconscious, for 4 hours a day. (The Common word for such meditation is \"trance.\") While meditating, you can dream after a fashion; such dreams are actually mental exercises that have become reflexive through years of practice. After resting in this way, you gain the same benefit that a human does from 8 hours of sleep.",
        "Languages":
            "You can speak, read, and write Common and Elvish. Elvish is fluid, with subtle intonations and intricate grammar. Elven literature is rich and varied, and their songs and poems are famous among other races. Many bards learn their language so they can add Elvish ballads to their repertoires.",
        "Elf Weapon Training":
            "You have proficiency with the longsword, shortsword, shortbow, and longbow.",
        "Fleet of Foot":
            "Your base walking speed increases to 35 feet.",
        "Mask of the Wild":
            "You can attempt to hide even when you are only lightly obscured by foliage, heavy rain, falling snow, mist, and other natural phenomena.",
    },
    skills:(
        perception: true
    ),
),
Race(
    name: "Gnome (Rock)",
    size: Medium,
    speed: 30,
    abilities:(
        strength: 0,
        dexterity: 0,
        constitution: 1,
        intelligence: 2,
        wisdom: 0,
        charisma: 0,
    ),
    traits:{
        "Darkvision":
            "Accustomed to life underground, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
        "Gnome Cunning":
            "You have advantage on all Intelligence, Wisdom, and Charisma saving throws against magic.",
        "Languages":
            "You can speak, read, and write Common and Gnomish. The Gnomish language, which uses the Dwarvish script, is renowned for its technical treatises and its catalogs of knowledge about the natural world.",
        "Artificer's Lore":
            "Whenever you make an Intelligence (History) check related to magic items, alchemical objects, or technological devices, you can add twice your proficiency bonus, instead of any proficiency bonus you normally apply.",
        "Tinker":
            "You have proficiency with artisan's tools (tinker's tools). Using those tools, you can spend 1 hour and 10 gp worth of materials to construct a Tiny clockwork device (AC 5, 1 hp). The device ceases to function after 24 hours (unless you spend 1 hour repairing it to keep the device functioning), or when you use your action to dismantle it; at that time, you can reclaim the materials used to create it. You can have up to three such devices active at a time.\n\nWhen you create a device, choose one of the following options:\n\n\tClockwork Toy. This toy is a clockwork animal, monster, or person, such as a frog, mouse, bird, dragon, or soldier. When placed on the ground, the toy moves 5 feet across the ground on each of your turns in a random direction. It makes noises as appropriate to the creature it represents.\n\n\tFire Starter. The device produces a miniature flame, which you can use to light a candle, torch, or campfire. Using the device requires your action.\n\n\tMusic Box. When opened, this music box plays a single song at a moderate volume. The box stops playing when it reaches the song's end or when it is closed.",
    },
),
Race(
    name: "Halfling (Lightfoot)",
    size: Medium,
    speed: 30,
    abilities:(
        strength: 0,
        dexterity: 2,
        constitution: 0,
        intelligence: 0,
        wisdom: 0,
        charisma: 1,
    ),
    traits:{
        "Lucky":
            "When you roll a 1 on an attack roll, ability check, or saving throw, you can reroll the die and must use the new roll.",
        "Brave":
            "You have advantage on saving throws against being frightened.",
        "Halfling Nimbleness":
            "You can move through the space of any creature that is of a size larger than yours.",
        "Languages":
            "You can speak, read, and write Common and Halfling. The Halfling language isn't secret, but halflings are loath to share it with others. They write very little, so they don't have a rich body of literature. Their oral tradition, however, is very strong. Almost all halflings speak Common to converse with the people in whose lands they dwell or through which they are traveling.",
        "Naturally Stealthy":
            "You can attempt to hide even when you are obscured only by a creature that is at least one size larger than you.",
    },
),
Race(
    name: "Halfling (Stout)",
    size: Medium,
    speed: 30,
    abilities:(
        strength: 0,
        dexterity: 2,
        constitution: 1,
        intelligence: 0,
        wisdom: 0,
        charisma: 0,
    ),
    traits:{
        "Lucky":
            "When you roll a 1 on an attack roll, ability check, or saving throw, you can reroll the die and must use the new roll.",
        "Brave":
            "You have advantage on saving throws against being frightened.",
        "Halfling Nimbleness":
            "You can move through the space of any creature that is of a size larger than yours.",
        "Languages":
            "You can speak, read, and write Common and Halfling. The Halfling language isn't secret, but halflings are loath to share it with others. They write very little, so they don't have a rich body of literature. Their oral tradition, however, is very strong. Almost all halflings speak Common to converse with the people in whose lands they dwell or through which they are traveling.",
        "Stout Resilience":
            "You have advantage on saving throws against poison, and you have resistance against poison damage.",
    },
),
Race(
    name: "Half-Elf",
    size: Medium,
    speed: 30,
    abilities:(
        strength: 0,
        dexterity: 0,
        constitution: 0,
        intelligence: 0,
        wisdom: 0,
        charisma: 2,
    ),
    traits:{
        "Ability Score Increase":
            "Two different ability scores of your choice increase by 1.",
        "Darkvision":
            "Thanks to your elf blood, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
        "Fey Ancestry":
            "You have advantage on saving throws against being charmed, and magic can't put you to sleep.",
        "Skill Versatility":
            "You gain proficiency in two skills of your choice.",
        "Languages":
            "You can speak, read, and write Common, Elvish, and one extra language of your choice.",
    },
),
Race(
    name: "Half-Orc",
    size: Medium,
    speed: 30,
    abilities:(
        strength: 2,
        dexterity: 0,
        constitution: 1,
        intelligence: 0,
        wisdom: 0,
        charisma: 0,
    ),
    traits:{
        "Darkvision":
            "Thanks to your orc blood, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
        "Menacing":
            "You gain proficiency in the Intimidation skill.",
        "Relentless Endurance":
            "When you are reduced to 0 hit points but not killed outright, you can drop to 1 hit point instead. You can't use this feature again until you finish a long rest.",
        "Savage Attacks":
            "When you score a critical hit with a melee weapon attack, you can roll one of the weapon's damage dice one additional time and add it to the extra damage of the critical hit.",
        "Languages":
            "You can speak, read, and write Common and Orc. Orc is a harsh, grating language with hard consonants. It has no script of its own but is written in the Dwarvish script.",
    },
    modifiers:{
        "skills.intimidation": "proficient",
    },
),
Race(
    name: "Human",
    size: Medium,
    speed: 30,
    abilities:(
        strength: 1,
        dexterity: 1,
        constitution: 1,
        intelligence: 1,
        wisdom: 1,
        charisma: 1,
    ),
    traits:{
        "Languages":
            "You can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
    },
    modifiers:{
        "language": "Common",
    }
),
Race(
    name: "Tiefling",
    size: Medium,
    speed: 30,
    abilities:(
        strength: 0,
        dexterity: 0,
        constitution: 0,
        intelligence: 1,
        wisdom: 0,
        charisma: 2,
    ),
    traits:{
        "Darkvision":
            "Thanks to your infernal heritage, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
        "Hellish Resistance":
            "You have resistance to fire damage.",
        "Infernal Legacy":
            "You know the thaumaturgy cantrip. Once you reach 3rd level, you can cast the hellish rebuke spell as a 2nd-level spell; you must finish a long rest in order to cast the spell again using this trait. Once you reach 5th level, you can also cast the darkness spell; you must finish a long rest in order to cast the spell again using this trait. Charisma is your spellcasting ability for these spells.",
        "Languages":
            "You can speak, read, and write Common and Infernal.",
    },
),
]"#;