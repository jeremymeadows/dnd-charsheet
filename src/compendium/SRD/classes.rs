use std::lazy::SyncLazy;
use ron;
use crate::compendium::Class;

static CLASSES: SyncLazy<Vec<Class>> = SyncLazy::new(|| {
    ron::from_str::<Vec<Class>>(SRD_CLASSES).unwrap()
});

pub fn get_classes() -> Vec<Class> {
    CLASSES.to_vec()
}

pub fn get_class(name: &str) -> Option<Class> {
    CLASSES.iter().find(|e| e.name == name).cloned()
}

static SRD_CLASSES: &str = r#"[
Class(
    name: "test",
    hit_die: 20,
    saves:(
        strength: Proficient,
        wisdom: Proficient,
    ),
    features:{
        1:{
            ".abilities.dexterity": "100",
            ".skills.athletics": "Proficient",
        },
        2:{
            ".abilities.constitution": "100",
            ".skills.athletics": "Expert",
            ".skills.acrobatics": "Proficient",
        },
    },
),
Class(
    name: "Barbarian",
    hit_die: 12,
    saves:(
        strength: Proficient,
        constitution: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: light armor, medium armor, shields\nWeapons: simple weapons, martial weapons\nTools: none\nSkills: Choose two from Animal Handling, Athletics, Intimidation, Nature, Perception, and Survival",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) a greataxe or (b) any martial melee weapon\n\t• (a) two handaxes or (b) any simple weapon\n\t• An explorer's pack, and four javelins\n\nAlternatively, you may start with 2d4 x 10 gp to buy your own equipment.",
            "Rage":
                "In battle, you fight with primal ferocity. On your turn, you can enter a rage as a bonus action.\n\nWhile raging, you gain the following benefits if you aren't wearing heavy armor:\n\t• You have advantage on Strength checks and Strength saving throws.\n\t• When you make a melee weapon attack using Strength, you gain a +2 bonus to the damage roll. This bonus increases as you level.\n\t• You have resistance to bludgeoning, piercing, and slashing damage.\n\nIf you are able to cast spells, you can't cast them or concentrate on them while raging.\n\nYour rage lasts for 1 minute. It ends early if you are knocked unconscious or if your turn ends and you haven't attacked a hostile creature since your last turn or taken damage since then. You can also end your rage on your turn as a bonus action.\n\nOnce you have raged the maximum number of times for your barbarian level, you must finish a long rest before you can rage again. You may rage 2 times at 1st level, 3 at 3rd, 4 at 6th, 5 at 12th, and 6 at 17th.",
            "Unarmored Defense":
                "While you are not wearing any armor, your Armor Class equals 10 + your Dexterity modifier + your Constitution modifier. You can use a shield and still gain this benefit.",
        },
        2:{
            "Danger Sense":
                "At 2nd level, you gain an uncanny sense of when things nearby aren't as they should be, giving you an edge when you dodge away from danger. You have advantage on Dexterity saving throws against effects that you can see, such as traps and spells. To gain this benefit, you can't be blinded, deafened, or incapacitated.",
            "Reckless Attack":
                "Starting at 2nd level, you can throw aside all concern for defense to attack with fierce desperation. When you make your first attack on your turn, you can decide to attack recklessly. Doing so gives you advantage on melee weapon attack rolls using Strength during this turn, but attack rolls against you have advantage until your next turn.",
        },
        3:{
            "Increased Rages":
                "You may now rage 3 times before a long rest.",
            "Primal Path":
                "At 3rd level, you choose a path that shapes the nature of your rage. Choose the Path of the Berserker or the Path of the Totem Warrior, both detailed at the end of the class description. Your choice grants you features at 3rd level and again at 6th, 10th, and 14th levels.",
            "Primal Path: Path of the Berserker":
                "For some barbarians, rage is a means to an end -- that end being violence. The Path of the Berserker is a path of untrammeled fury, slick with blood. As you enter the berserker's rage, you thrill in the chaos of battle, heedless of your own health or well-being.",
            "Primal Path: Path of the Totem Warrior":
                "The Path of the Totem Warrior is a spiritual journey, as the barbarian accepts a spirit animal as guide, protector, and inspiration. In battle, your totem spirit fills you with supernatural might, adding magical fuel to your barbarian rage.\n\nMost barbarian tribes consider a totem animal to be kin to a particular clan. In such cases, it is unusual for an individual to have more than one totem animal spirit, though exceptions exist.",
            "Path of the Berserker: Frenzy":
                "Starting when you choose this path at 3rd level, you can go into a frenzy when you rage. If you do so, for the duration of your rage you can make a single melee weapon attack as a bonus action on each of your turns after this one. When your rage ends, you suffer one level of exhaustion.",
            "Path of the Totem Warrior: Spirit Seeker":
                "Yours is a path that seeks attunement with the natural world, giving you a kinship with beasts. At 3rd level when you adopt this path, you gain the ability to cast the beast sense and speak with animals spells, but only as rituals, as described in chapter 10.",
            "Path of the Totem Warrior: Totem Spirit":
                "At 3rd level, when you adopt this path, you choose a totem spirit and gain its feature. You must make or acquire a physical totem object- an amulet or similar adornment — that incorporates fur or feathers, claws, teeth, or bones of the totem animal. At your option, you also gain minor physical attributes that are reminiscent of your totem spirit. For example, if you have a bear totem spirit, you might be unusually hairy and thick-skinned, or if your totem is the eagle, your eyes turn bright yellow.\n\nYour totem animal might be an animal related to those listed here but more appropriate to your homeland. For example, you could choose a hawk or vulture in place of an eagle.",
            "Totem Spirit (Bear)":
                "While raging, you have resistance to all damage except psychic damage. The spirit of the bear makes you tough enough to stand up to any punishment.",
            "Totem Spirit (Eagle)":
                "While you're raging and aren't wearing heavy armor, other creatures have disadvantage on opportunity attack rolls against you, and you can use the Dash action as a bonus action on your turn. The spirit of the eagle makes you into a predator who can weave through the fray with ease.",
            "Totem Spirit (Elk)":
                "While you're raging and aren't wearing heaving armor, your walking speed increases by 15 feet. The spirit of the elk makes you extraordinarily swift.",
            "Totem Spirit (Tiger)":
                "While raging, you can add 10 feet to your long jump distance and 3 feet to your high jump distance. The spirit of the tiger empowers your leaps.",
            "Totem Spirit (Wolf)":
                "While you're raging, your friends have advantage on melee attack rolls against any creature within 5 feet of you that is hostile to you. The spirit of the wolf makes you a leader of hunters.",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        5:{
            "Extra Attack":
                "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn.",
            "Fast Movement":
                "Starting at 5th level, your speed increases by 10 feet while you aren't wearing heavy armor.",
        },
        6:{
            "Increased Rages":
                "You may now rage 4 times before a long rest.",
            "Path of the Berserker: Mindless Rage":
                "Beginning at 6th level, you can't be charmed or frightened while raging. If you are charmed or frightened when you enter your rage, the effect is suspended for the duration of the rage.",
            "Path of the Totem Warrior: Aspect of the Beast":
                "At 6th level, you gain a magical benefit based on the totem animal of your choice. You can choose the same animal you selected at 3rd level or a different one.",
            "Aspect of the Beast: Bear":
                "You gain the might of a bear. Your carrying capacity (including maximum load and maximum lift) is doubled, and you have advantage on Strength checks made to push, pull, lift, or break objects.",
            "Aspect of the Beast: Eagle":
                "You gain the eyesight of an eagle. You can see up to 1 mile away with no difficulty, able to discern even fine details as though looking at something no more than 100 feet away from you. Additionally, dim light doesn't impose disadvantage on your Wisdom (Perception) checks.",
            "Aspect of the Beast: Elk":
                "Whether mounted or on foot, your travel pace is doubled, as is the travel pace of up to ten companions while they're within 60 feet of you and you're not incapacitated. The elk spirit helps you roam far and fast.",
            "Aspect of the Beast: Tiger":
                "You gain proficiency in two skills from the following list: Athletics, Acrobatics, Stealth, and Survival. The cat spirit hones your survival instincts.",
            "Aspect of the Beast: Wolf":
                "You gain the hunting sensibilities of a wolf. You can track other creatures while traveling at a fast pace, and you can move stealthily while traveling at a normal pace.",
        },
        7:{
            "Feral Instinct":
                "By 7th level, your instincts are so honed that you have advantage on initiative rolls.\n\nAdditionally, if you are surprised at the beginning of combat and aren't incapacitated, you can act normally on your first turn, but only if you enter your rage before doing anything else on that turn.",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM uses the optional Feats, you can instead take a feat.",
        },
        9:{
            "Brutal Critical":
                "Beginning at 9th level, you can roll one additional weapon damage die when determining the extra damage for a critical hit with a melee attack.\n\nThis increases to two additional dice at 13th level and three additional dice at 17th level.",
            "Increased Rage Damage":
                "You now have a +3 bonus to melee damage rolls while raging.",
        },
        10:{
            "Path of the Berserker: Intimidating Presence":
                "Beginning at 10th level, you can use your action to frighten someone with your menacing presence. When you do so, choose one creature that you can see within 30 feet of you. If the creature can see or hear you, it must succeed on a Wisdom saving throw (DC equal to 8 + your proficiency bonus + your Charisma modifier) or be frightened of you until the end of your next turn. On subsequent turns, you can use your action to extend the duration of this effect on the frightened creature until the end of your next turn. This effect ends if the creature ends its turn out of line of sight or more than 60 feet away from you.\n\nIf the creature succeeds on its saving throw, you can't use this feature on that creature again for 24 hours.",
            "Path of the Totem Warrior: Spirit Walker":
                "At 10th level, you can cast the commune with nature spell, but only as a ritual. When you do so, a spiritual version of one of the animals you chose for Totem Spirit or Aspect of the Beast appears to you to convey the information you seek.",
        },
        11:{
            "Relentless Rage":
                "Starting at 11th level, your rage can keep you fighting despite grievous wounds. If you drop to 0 hit points while you're raging and don't die outright, you can make a DC 10 Constitution saving throw. If you succeed, you drop to 1 hit point instead.\n\nEach time you use this feature after the first, the DC increases by 5. When you finish a short or long rest, the DC resets to 10.",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM uses the optional Feats, you can instead take a feat.",
            "Increased Rages":
                "You may now rage 5 times before a long rest.",
        },
        13:{
            "Brutal Critical":
                "You now roll two additional weapon damage dice when determining the extra damage for a critical hit with a melee attack.",
        },
        14:{
            "Path of the Berserker: Retaliation":
                "Starting at 14th level, when you take damage from a creature that is within 5 feet of you. you can use your reaction to make a melee weapon attack against that creature.",
            "Path of the Totem Warrior: Totemic Attunement":
                "At 14th level, you gain a magical benefit based on a totem animal of your choice. You can choose the same animal you selected previously or a different one.",
            "Totemic Attunement: Bear":
                "While you're raging, any creature within 5 feet of you that's hostile to you has disadvantage on attack rolls against targets other than you or another character with this feature. An enemy is immune to this effect if it can't see or hear you or if it can't be frightened.",
            "Totemic Attunement: Eagle":
                "While raging, you have a flying speed equal to your current walking speed. This benefit works only in short bursts; you fall if you end your turn in the air and nothing else is holding you aloft.",
            "Totemic Attunement: Elk":
                "While raging, you can use a bonus action during your move to pass through the space of a Large or smaller creature. That creature must succeed on a Strength saving throw (DC 8 + your Strength bonus + your proficiency bonus) or be knocked prone and take bludgeoning damage equal to 1d12 + your strength modifier.",
            "Totemic Attunement: Tiger":
                "While you're raging, if you move at least 20 feet in a straight line toward a Large or smaller target right before making a melee weapon attack against it, you can use a bonus action to make an additional melee weapon attack against it.",
            "Totemic Attunement: Wolf":
                "While you're raging, you can use a bonus action on your turn to knock a Large or smaller creature prone when you hit it with melee weapon attack.",
        },
        15:{
            "Persistent Rage":
                "Beginning at 15th level, your rage is so fierce that it ends early only if you fall unconscious or if you choose to end it.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM uses the optional Feats, you can instead take a feat.",
            "Increased Rage Damage":
                "You now have a +4 bonus to melee damage rolls while raging.",
        },
        17:{
            "Brutal Critical":
                "You now roll three additional weapon damage dice when determining the extra damage for a critical hit with a melee attack.",
            "Increased Rages":
                "You may now rage 6 times before a long rest",
        },
        18:{
            "Indomitable Might":
                "Beginning at 18th level, if your total for a Strength check is less than your Strength score, you can use that score in place of the total.",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM uses the optional Feats, you can instead take a feat.",
        },
        20:{
            "Primal Champion":
                "At 20th level, you embody the power of the wilds. Your Strength and Constitution scores increase by 4. Your maximum for those scores is now 24.",
            "Unlimited Rages":
                "At 20th level, there is no limit to the number of times you may rage.",
        },
    },
),
Class(
    name: "Bard",
    hit_die: 8,
    spellcasting_ability: "Charisma",
    saves:(
        dexterity: Proficient,
        charisma: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: light armor\nWeapons: simple weapons, hand crossbows, longswords, rapiers, shortswords\nTools: three musical instruments of your choice\nSkills: Choose any three.",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) a rapier, (b) a longsword, or (c) any simple weapon\n\t• (a) a diplomat's pack or (b) an entertainer's pack\n\t• (a) a lute or (b) any other musical instrument\n\t• Leather armor, and a dagger\n\nAlternatively, you may start with 5d4 x 10 gp to buy your own equipment.",
            "Bardic Inspiration":
                "You can inspire others through stirring words or music. To do so, you use a bonus action on your turn to choose one creature other than yourself within 60 feet of you who can hear you. That creature gains one Bardic Inspiration die, a d6.\n\nOnce within the next 10 minutes, the creature can roll the die and add the number rolled to one ability check, attack roll, or saving throw it makes. The creature can wait until after it rolls the d20 before deciding to use the Bardic Inspiration die, but must decide before the DM says whether the roll succeeds or fails. Once the Bardic Inspiration die is rolled, it is lost. A creature can have only one Bardic Inspiration die at a time.\n\nYou can use this feature a number of times equal to your Charisma modifier (a minimum of once). You regain any expended uses when you finish a long rest.\n\nYour Bardic Inspiration die changes when you reach certain levels in this class. The die becomes a d8 at 5th level, a d10 at 10th level, and a d12 at 15th level.",
            "Spellcasting":
                "You have learned to untangle and reshape the fabric of reality in harmony with your wishes and music. Your spells are part of your vast repertoire, magic that you can tune to different situations. See chapter 10 for the general rules of spellcasting and chapter 11 for the bard spell list.\n\nCantrips\nYou know two cantrips of your choice from the bard spell list. You learn additional bard cantrips of your choice at higher levels, learning a 3rd cantrip at 4th level and a 4th at 10th level.\n\nSpell Slots\nThe Bard table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\n\nFor example, if you know the 1st-level spell cure wounds and have a 1st-level and a 2nd-level spell slot available, you can cast cure wounds using either slot.\n\nSpells Known of 1st Level and Higher\nYou know four 1st-level spells of your choice from the bard spell list.\n\nYou learn an additional bard spell of your choice at each level except 12th, 16th, 19th, and 20th. Each of these spells must be of a level for which you have spell slots. For instance, when you reach 3rd level in this class, you can learn one new spell of 1st or 2nd level.\n\nAdditionally, when you gain a level in this class, you can choose one of the bard spells you know and replace it with another spell from the bard spell list, which also must be of a level for which you have spell slots.\n\nSpellcasting Ability\nCharisma is your spellcasting ability for your bard spells. Your magic comes from the heart and soul you pour into the performance of your music or oration. You use your Charisma whenever a spell refers to your spellcasting ability. In addition, you use your Charisma modifier when setting the saving throw DC for a bard spell you cast and when making an attack roll with one.\n\n\tSpell save DC = 8 + your proficiency bonus + your Charisma modifier\n\tSpell attack modifier = your proficiency bonus + your Charisma modifier\n\nRitual Casting\nYou can cast any bard spell you know as a ritual if that spell has the ritual tag.\n\nSpellcasting Focus\nYou can use a musical instrument (found in chapter 5) as a spellcasting focus for your bard spells.",
        },
        2:{
            "Jack of All Trades":
                "Starting at 2nd level, you can add half your proficiency bonus, rounded down, to any ability check you make that doesn't already include your proficiency bonus.",
            "Song of Rest":
                "Beginning at 2nd level, you can use soothing music or oration to help revitalize your wounded allies during a short rest. If you or any friendly creatures who can hear your performance regain hit points by spending Hit Dice at the end of the short rest, each of those creatures regains an extra 1d6 hit points.\n\nThe extra hit points increase when you reach certain levels in this class: to 1d8 at 9th level, to 1d10 at 13th level, and to 1d12 at 17th level.",
        },
        3:{
            "Bard College":
                "At 3rd level, you delve into the advanced techniques of a bard college of your choice: the College of Lore or the College of Valor, both detailed at the end of the class description. Your choice grants you features at 3rd level and again at 6th and 14th level.",
            "Bard College: College of Lore":
                "Bards of the College of Lore know something about most things, collecting bits of knowledge from sources as diverse as scholarly tomes and peasant tales. Whether singing folk ballads in taverns or elaborate compositions in royal courts, these bards use their gifts to hold audiences spellbound. When the applause dies down, the audience members might find themselves questioning everything they held to be true, from their faith in the priesthood of the local temple to their loyalty to the king.\n\nThe loyalty of these bards lies in the pursuit of beauty and truth, not in fealty to a monarch or following the tenets of a deity. A noble who keeps such a bard as a herald or advisor knows that the bard would rather be honest than politic.\n\nThe college's members gather in libraries and sometimes in actual colleges, complete with classrooms and dormitories, to share their lore with one another. They also meet at festivals or affairs of state, where they can expose corruption, unravel lies, and poke fun at self-important figures of authority.",
            "Expertise":
                "At 3rd level, choose two of your skill proficiencies. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.\n\nAt 10th level, you can choose another two skill proficiencies to gain this benefit.",
            "College of Lore: Bonus Proficiencies":
                "When you join the College of Lore at 3rd level, you gain proficiency with three skills of your choice.",
            "College of Lore: Cutting Words":
                "Also at 3rd level, you learn how to use your wit to distract, confuse, and otherwise sap the confidence and competence of others. When a creature that you can see within 60 feet of you makes an attack roll, an ability check, or a damage roll, you can use your reaction to expend one of your uses of Bardic Inspiration, rolling a Bardic Inspiration die and subtracting the number rolled from the creature's roll. You can choose to use this feature after the creature makes its roll, but before the DM determines whether the attack roll or ability check succeeds or fails, or before the creature deals its damage. The creature is immune if it can't hear you or if it's immune to being charmed.",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        5:{
            "Bardic Inspiration":
                "At 5th level, your Bardic Inspiration die changes to a d8.",
            "Font of Inspiration":
                "Beginning when you reach 5th level, you regain all of your expended uses of Bardic Inspiration when you finish a short or long rest.",
        },
        6:{
            "Countercharm":
                "At 6th level, you gain the ability to use musical notes or words of power to disrupt mind-influencing effects. As an action, you can start a performance that lasts until the end of your next turn. During that time, you and any friendly creatures within 30 feet of you have advantage on saving throws against being frightened or charmed. A creature must be able to hear you to gain this benefit. The performance ends early if you are incapacitated or silenced or if you voluntarily end it (no action required).",
            "College of Lore: Additional Magical Secrets":
                "At 6th level, you learn two spells of your choice from any class. A spell you choose must be of a level you can cast, as shown on the Bard table, or a cantrip. The chosen spells count as bard spells for you but don't count against the number of bard spells you know.",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        9:{
            "Song of Rest":
                "At 9th level, the extra hit points gained from Song of Rest increases to 1d8.",
        },
        10:{
            "Bardic Inspiration":
                "At 10th level, your Bardic Inspiration die changes to a d10.",
            "Expertise":
                "At 10th level, you can choose another two skill proficiencies. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.",
            "Magical Secrets":
                "By 10th level, you have plundered magical knowledge from a wide spectrum of disciplines. Choose two spells from any class, including this one. A spell you choose must be of a level you can cast, as shown on the Bard table, or a cantrip.\n\nThe chosen spells count as bard spells for you and are included in the number in the Spells Known column of the Bard table.\n\nYou learn two additional spells from any class at 14th level and again at 18th level.",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        13:{
            "Song of Rest":
                "At 13th level, the extra hit points gained from Song of Rest increases to 1d10.",
        },
        14:{
            "Magical Secrets":
                "At 14th level, choose two additional spells from any class, including this one. A spell you choose must be of a level you can cast, as shown on the Bard table, or a cantrip.\n\nThe chosen spells count as bard spells for you and are included in the number in the Spells Known column of the Bard table.",
            "College of Lore: Peerless Skill":
                "Starting at 14th level, when you make an ability check, you can expend one use of Bardic Inspiration. Roll a Bardic Inspiration die and add the number rolled to your ability check. You can choose to do so after you roil the die for the ability check, but before the DM tells you whether you succeed or fail.",
        },
        15:{
            "Bardic Inspiration":
                "At 15th level, your Bardic Inspiration die changes to a d12.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        17:{
            "Song of Rest":
                "At 17th level, the extra hit points gained from Song of Rest increases to 1d12.",
        },
        18:{
            "Magical Secrets":
                "At 18th level, choose two additional spells from any class, including this one. A spell you choose must be of a level you can cast, as shown on the Bard table, or a cantrip.\n\nThe chosen spells count as bard spells for you and are included in the number in the Spells Known column of the Bard table.",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        20:{
            "Superior Inspiration":
                "At 20th level, when you roll initiative and have no uses of Bardic Inspiration left, you regain one use.",
        },
    },
),
Class(
    name: "Cleric",
    hit_die: 8,
    spellcasting_ability: "Wisdom",
    saves:(
        wisdom: Proficient,
        charisma: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: light armor, medium armor, shields\nWeapons: simple weapons\nTools: none\nSkills: Choose two from History, Insight, Medicine, Persuasion, and Religion",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) a mace or (b) a warhammer (if proficient)\n\t• (a) scale mail, (b) leather armor, or (c) chain mail (if proficient)\n\t• (a) a light crossbow and 20 bolts or (b) any simple weapon\n\t• (a) a priest's pack or (b) an explorer's pack\n\t• A shield and a holy symbol\n\nAlternatively, you may start with 5d4 x 10 gp to buy your own equipment.",
            "Spellcasting":
                "As a conduit for divine power, you can cast cleric spells. See chapter 10 for the general rules of spellcasting and chapter 11 for a selection of cleric spells.\n\nCantrips\nAt 1st level, you know three cantrips of your choice from the cleric spell list. You learn additional cleric cantrips of your choice at higher levels, as shown in the Cantrips Known column of the Cleric table.\n\nPreparing and Casting Spells\nThe Cleric table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\n\nYou prepare the list of cleric spells that are available for you to cast, choosing from the cleric spell list. When you do so, choose a number of cleric spells equal to your Wisdom modifier + your cleric level (minimum of one spell). The spells must be of a level for which you have spell slots.\n\nFor example, if you are a 3rd-level cleric, you have four 1st-level and two 2nd-level spell slots. With a Wisdom of 16, your list of prepared spells can include six spells of 1st or 2nd level, in any combination. If you prepare the 1st-level spell cure wounds, you can cast it using a 1st-level or 2nd-level slot. Casting the spell doesn't remove it from your list of prepared spells.\n\nYou can change your list of prepared spells when you finish a long rest. Preparing a new list of cleric spells requires time spent in prayer and meditation: at least 1 minute per spell level for each spell on your list.\n\nSpellcasting Ability\nWisdom is your spellcasting ability for your cleric spells. The power of your spells comes from your devotion to your deity. You use your Wisdom whenever a cleric spell refers to your spellcasting ability. In addition, you use your Wisdom modifier when setting the saving throw DC for a cleric spell you cast and when making an attack roll with one.\n\n\tSpell save DC = 8 + your proficiency bonus + your Wisdom modifier\n\tSpell attack modifier = your proficiency bonus + your Wisdom modifier\n\nRitual Casting\nYou can cast a cleric spell as a ritual if that spell has the ritual tag and you have the spell prepared.\n\nSpellcasting Focus\nYou can use a holy symbol (found in chapter 5) as a spellcasting focus for your cleric spells.",
            "Divine Domain":
                "Choose one domain related to your deity: Arcana, Knowledge, Life, Light, Nature, Tempest, Trickery, or War. Each domain is detailed in their own feature, and each one provides examples of gods associated with it. Your choice grants you domain spells and other features when you choose it at 1st level. It also grants you additional ways to use Channel Divinity when you gain that feature at 2nd level, and additional benefits at 6th, 8th, and 17th levels.\n\nDomain Spells\nEach domain has a list of spells-its domain spells-that you gain at the cleric levels noted in the domain description. Once you gain a domain spell, you always have it prepared, and it doesn't count against the number of spells you can prepare each day.\n\nIf you have a domain spell that doesn't appear on the cleric spell list, the spell is nonetheless a cleric spell for you.",
            "Divine Domain: Life":
                "The Life domain focuses on the vibrant positive energy - one of the fundamental forces of the universe -  that sustains all life. The gods of life promote vitality and health through healing the sick and wounded, caring for those in need, and driving away the forces of death and undeath. Almost any non-evil deity can claim influence over this domain, particularly agricultural deities (such as Chauntea, Arawai, and Demeter), sun gods (such as Lathander, Pelor, and Re-Horakhty), gods of healing or endurance (such as Ilmater, Mishakal, Apollo, and Diancecht), and gods of home and community (such as Hestia, Hathor, and Boldrei).\n\nLife Domain Spells: At each indicated cleric level, you add the listed spells to your spells prepared.\n\n1st - bless, cure wounds\n3rd - lesser restoration, spiritual weapon\n5th - beacon of hope, revivify\n7th - death ward, guardian of faith\n9th - mass cure wounds, raise dead",
            "Life Domain: Bonus Proficiency":
                "When you choose this domain at 1st level, you gain proficiency with heavy armor.",
            "Life Domain: Disciple of Life":
                "Also starting at 1st level, your healing spells are more effective. Whenever you use a spell of 1st level or higher to restore hit points to a creature, the creature regains additional hit points equal to 2 + the spell's level.",
        },
        2:{
            "Channel Divinity":
                "At 2nd level, you gain the ability to channel divine energy directly from your deity, using that energy to fuel magical effects. You start with two such effects: Turn Undead and an effect determined by your domain. Some domains grant you additional effects as you advance in levels, as noted in the domain description.\n\nWhen you use your Channel Divinity, you choose which effect to create. You must then finish a short or long rest to use your Channel Divinity again\n\nSome Channel Divinity effects require saving throws. When you use such an effect from this class, the DC equals your cleric spell save DC.\n\nBeginning at 6th level, you can use your Channel Divinity twice between rests, and beginning at 18th level, you can use it three times between rests. When you finish a short or long rest, you regain your expended uses.",
            "Channel Divinity: Turn Undead":
                "As an action, you present your holy symbol and speak a prayer censuring the undead. Each undead that can see or hear you within 30 feet of you must make a Wisdom saving throw. If the creature fails its saving throw, it is turned for 1 minute or until it takes any damage.\n\nA turned creature must spend its turns trying to move as far away from you as it can, and it can't willingly move to a space within 30 feet of you. It also can't take reactions. For its action, it can use only the Dash action or try to escape from an effect that prevents it from moving. If there's nowhere to move, the creature can use the Dodge action.",
            "Life Domain Channel Divinity: Preserve Life":
                "Starting at 2nd level, you can use your Channel Divinity to heal the badly injured.\n\nAs an action, you present your holy symbol and evoke healing energy that can restore a number of hit points equal to five times your cleric level. Choose any creatures within 30 feet of you, and divide those hit points among them. This feature can restore a creature to no more than half of its hit point maximum. You can't use this feature on an undead or a construct.",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        5:{
            "Destroy Undead (CR 1/2)":
                "Starting at 5th level, when an undead of CR 1/2 or lower fails its saving throw against your Turn Undead feature, the creature is instantly destroyed.",
        },
        6:{
            "Channel Divinity (2/rest)":
                "Beginning at 6th level, you can use your Channel Divinity twice between rests.",
            "Life Domain: Blessed Healer":
                "Beginning at 6th level, the healing spells you cast on others heal you as well. When you cast a spell of 1st level or higher that restores hit points to a creature other than you, you regain hit points equal to 2 + the spell's level.",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
            "Destroy Undead (CR 1)":
                "Starting at 8th level, when an undead of CR 1 or lower fails its saving throw against your Turn Undead feature, the creature is instantly destroyed.",
            "Life Domain: Divine Strike":
                "At 8th level, you gain the ability to infuse your weapon strikes with divine energy. Once on each of your turns when you hit a creature with a weapon attack, you can cause the attack to deal an extra 1d8 radiant damage to the target. When you reach 14th level, the extra damage increases to 2d8.",
        },
        10:{
            "Divine Intervention":
                "Beginning at 10th level, you can call on your deity to intervene on your behalf when your need is great.\n\nImploring your deity's aid requires you to use your action. Describe the assistance you seek, and roll percentile dice. If you roll a number equal to or lower than your cleric level, your deity intervenes. The DM chooses the nature of the intervention; the effect of any cleric spell or cleric domain spell would be appropriate. If your deity intervenes, you can't use this feature again for 7 days. Otherwise, you can use it again after you finish a long rest.\n\nAt 20th level, your call for intervention succeeds automatically, no roll required.",
        },
        11:{
            "Destroy Undead (CR 2)":
                "Starting at 11th level, when an undead of CR 2 or lower fails its saving throw against your Turn Undead feature, the creature is instantly destroyed.",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        14:{
            "Destroy Undead (CR 3)":
                "Starting at 14th level, when an undead of CR 3 or lower fails its saving throw against your Turn Undead feature, the creature is instantly destroyed.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        17:{
            "Destroy Undead (CR 4)":
                "Starting at 17th level, when an undead of CR 4 or lower fails its saving throw against your Turn Undead feature, the creature is instantly destroyed.",
            "Life Domain: Supreme Healing":
                "Starting at 17th level, when you would normally roll one or more dice to restore hit points with a spell, you instead use the highest number possible for each die. For example, instead of restoring 2d6 hit points to a creature, you restore 12.",
        },
        18:{
            "Channel Divinity (3/rest)":
                "Beginning at 18th level, you can use your Channel Divinity three times between rests.",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        20:{
            "Divine Intervention Improvement":
                "At 20th level, your call for intervention succeeds automatically, no roll required.",
        },
    },
),
Class(
    name: "Druid",
    hit_die: 8,
    spellcasting_ability: "Wisdom",
    saves:(
        intelligence: Proficient,
        wisdom: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: light armor, medium armor, shields (druids will not wear armor or use shields made of metal)\nWeapons: clubs, daggers, darts, javelins, maces, quarterstaffs, scimitars, sickles, slings, spears\nTools: herbalism kit\nSkills: Choose two from Arcana, Animal Handling, Insight, Medicine, Nature, Perception, Religion, and Survival",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) a wooden shield or (b) any simple weapon\n\t• (a) a scimitar or (b) any simple melee weapon\n\t• Leather armor, an explorer's pack, and a druidic focus\n\nAlternatively, you may start with 2d4 x 10 gp to buy your own equipment.",
            "Druidic":
                "You know Druidic, the secret language of druids. You can speak the language and use it to leave hidden messages. You and others who know this language automatically spot such a message. Others spot the message's presence with a successful DC 15 Wisdom (Perception) check but can't decipher it without magic.",
            "Spellcasting":
                "Drawing on the divine essence of nature itself, you can cast spells to shape that essence to your will. See chapter 10 for the general rules of spellcasting and chapter 11 for the druid spell list.\n\nCantrips\nAt 1st level, you know two cantrips of your choice from the druid spell list. You learn additional druid cantrips of your choice at higher levels, as shown in the Cantrips Known column of the Druid table.\n\nPreparing and Casting Spells\nThe Druid table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these druid spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\n\nYou prepare the list of druid spells that are available for you to cast, choosing from the druid spell list. When you do so, choose a number of druid spells equal to your Wisdom modifier + your druid level (minimum of one spell). The spells must be of a level for which you have spell slots.\n\nFor example, if you are a 3rd-level druid, you have four 1st-level and two 2nd-level spell slots. With a Wisdom of 16, your list of prepared spells can include six spells of 1st or 2nd level, in any combination. If you prepare the 1st-level spell cure wounds, you can cast it using a 1st-level ar 2nd-level slot. Casting the spell doesn't remove it from your list of prepared spells.\n\nYou can also change your list of prepared spells when you finish a long rest. Preparing a new list of druid spells requires time spent in prayer and meditation: at least 1 minute per spell level for each spell on your list.\n\nSpellcasting Ability\nWisdom is your spellcasting ability for your druid spells, since your magic draws upon your devotion and attunement to nature. You use your Wisdom whenever a spell refers to your spellcasting ability. In addition, you use your Wisdom modifier when setting the saving throw DC for a druid spell you cast and when making an attack roll with one.\n\n\tSpell save DC = 8 + your proficiency bonus + your Wisdom modifier\n\n\tSpell attack modifier = your proficiency bonus + your Wisdom modifier\n\nRitual Casting\nYou can cast a druid spell as a ritual if that spell has the ritual tag and you have the spell prepared.\n\nSpellcasting Focus\nYou can use a druidic focus (found in chapter 5) as a spellcasting focus for your druid spells",
        },
        2:{
            "Wild Shape":
                "Starting at 2nd level, you can use your action to magically assume the shape of a beast that you have seen before. You can use this feature twice. You regain expended uses when you finish a short or long rest.\n\nYour druid level determines the beasts you can transform into, as shown in the Beast Shapes table. At 2nd level, for example, you can transform into any beast that has a challenge rating of 1/4 or lower that doesn't have a flying or swimming speed.\n\nBeast Shapes\nLevel    Max. CR    Limitations    Example\n2nd    1/4 No flying or swimming speed    Wolf\n4th    1/2 No flying speed    Crocodile\n8th    1    —    Giant eagle\n\nYou can stay in a beast shape for a number of hours equal to half your druid level (rounded down). You then revert to your normal form unless you expend another use of this feature. You can revert to your normal form earlier by using a bonus action on your turn. You automatically revert if you fall unconscious, drop to 0 hit points, or die.\n\nWhile you are transformed, the following rules apply:\n\t• Your game statistics are replaced by the statistics of the beast, but you retain your alignment, personality, and Intelligence, Wisdom, and Charisma scores. You also retain all of your skill and saving throw proficiencies, in addition to gaining those of the creature. If the creature has the same proficiency as you and the bonus in its stat block is higher than yours, use the creature's bonus instead of yours. If the creature has any legendary or lair actions, you can't use them.\n\t• When you transform, you assume the beast's hit points and Hit Dice. When you revert to your normal form, you return to the number of hit points you had before you transformed. However, if you revert as a result of dropping to 0 hit points, any excess damage carries over to your normal form. For example, if you take 10 damage in animal form and have only 1 hit point left, you revert and take 9 damage. As long as the excess damage doesn't reduce your normal form to 0 hit points, you aren't knocked unconscious.\n\t• You can't cast spells, and your ability to speak or take any action that requires hands is limited to the capabilities of your beast form. Transforming doesn't break your concentration on a spell you've already cast, however, or prevent you from taking actions that are part of a spell, such as call lightning, that you've already cast.\n\t• You retain the benefit of any features from your class, race, or other source and can use them if the new form is physically capable of doing so. However, you can't use any of your special senses, such as darkvision, unless your new form also has that sense.\n\t• You choose whether your equipment falls to the ground in your space, merges into your new form, or is worn by it. Worn equipment functions as normal, but the DM decides whether it is practical for the new form to wear a piece of equipment, based on the creature's shape and size. Your equipment doesn't change size or shape to match the new form, and any equipment that the new form can't wear must either fall to the ground or merge with it. Equipment that merges with the form has no effect until you leave the form.",
            "Druid Circle":
                "At 2nd level, you choose to identify with a circle of druids: the Circle of the Land or the Circle of the Moon, both detailed at the end of the class description. Your choice grants you features at 2nd level and again at 6th, 10th, and 14th level.",
            "Druid Circle: Circle of the Land":
                "The Circle of the Land is made up of mystics and sages who safeguard ancient knowledge and rites through a vast oral tradition. These druids meet within sacred circles of trees or standing stones to whisper primal secrets in Druidic. The circle's wisest members preside as the chief priests of communities that hold to the Old Faith and serve as advisors to the rulers of those folk. As a member of this circle, your magic is influenced by the land where you were initiated into the circle's mysterious rites.",
            "Circle of Land: Bonus Cantrip":
                "You learn one additional druid cantrip of your choice.",
            "Circle of Land: Natural Recovery":
                "Starling at 2nd level, you can regain some of your magical energy by sitting in meditation and communing with nature. During a short rest, you choose expended spell slots to recover. The spell slots can have a combined level that is equal to or less than half your druid level (rounded up), and none of the slots can be 6th level or higher. You can't use this feature again until you finish a long rest\n\nFor example, when you are a 4th-level druid, you can recover up to two levels worth of spell slots. You can recover either a 2nd-level slot or two 1st-level slots.",
            "Circle of Land: Circle Spells":
                "Your mystical connection to the land infuses you with the ability to cast certain spells. At 3rd, 5th, 7th, and 9th level you gain access to circle spells connected to the land where you became a druid. Choose that land - arctic, coast, desert, forest, grassland, mountain, swamp, or Underdark - and consult the associated list of spells.\n\nOnce you gain access to a circle spell, you always have it prepared, and it doesn't count against the number of spells you can prepare each day. If you gain access to a spell that doesn't appear on the druid spell list, the spell is nonetheless a druid spell for you.",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        6:{
            "Circle of Land: Land's Stride":
                "Starting at 6th level, moving through nonmagical difficult terrain costs you no extra movement. You can also pass through nonmagical plants without being slowed by them and without taking damage from them if they have thorns, spines, or a similar hazard.\n\nIn addition, you have advantage on saving throws against plants that are magically created or manipulated to impede movement, such those created by the entangle spell.",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        10:{
            "Circle of Land: Nature's Ward":
                "When you reach 10th level, you can't be charmed or frightened by elementals or fey, and you are immune to poison and disease.",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        14:{
            "Circle of Land: Nature's Sanctuary":
                "When you reach 14th level, creatures of the natural world sense your connection to nature and become hesitant to attack you. When a beast or plant creature attacks you, that creature must make a Wisdom saving throw against your druid spell save DC. On a failed save, the creature must choose a different target, or the attack automatically misses. On a successful save, the creature is immune to this effect for 24 hours.\n\nThe creature is aware of this effect before it makes its attack against you.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        18:{
            "Timeless Body":
                "Starting at 18th level, the primal magic that you wield causes you to age more slowly. For every 10 years that pass, your body ages only 1 year.",
            "Beast Spells":
                "Beginning at 18th level, you can cast many of your druid spells in any shape you assume using Wild Shape. You can perform the somatic and verbal components of a druid spell while in a beast shape, but you aren't able to provide material components.",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        20:{
            "Archdruid":
                "At 20th level, you can use your Wild Shape an unlimited number of times.\n\nAdditionally, you can ignore the verbal and somatic components of your druid spells, as well as any material components that lack a cost and aren't consumed by a spell. You gain this benefit in both your normal shape and your beast shape from Wild Shape.",
        },
    },
),
Class(
    name: "Fighter",
    hit_die: 10,
    saves:(
        strength: Proficient,
        constitution: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: light armor, medium armor, heavy armor, shields\nWeapons: simple weapons, martial weapons\nTools: none\nSkills: Choose two skills from Acrobatics, Animal Handling, Athletics, History, Insight, Intimidation, Perception, and Survival",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) chain mail or (b) leather, longbow, and 20 arrows\n\t• (a) a martial weapon and a shield or (b) two martial weapons\n\t• (a) a light crossbow and 20 bolts or (b) two handaxes\n\t• (a) a dungeoneer's pack or (b) an explorer's pack\n\nAlternatively, you may start with 5d4 x 10 gp to buy your own equipment.",
            "Fighting Style":
                "You adopt a particular style of fighting as your specialty. Choose a fighting style from the list of optional features. You can't take the same Fighting Style option more than once, even if you get to choose again.",
            "Fighting Style: Archery":
                "You gain a +2 bonus to attack rolls you make with ranged weapons.",
            "Fighting Style: Defense":
                "While you are wearing armor, you gain a +1 bonus to AC.",
            "Fighting Style: Dueling":
                "When you are wielding a melee weapon in one hand and no other weapons, you gain a +2 bonus to damage rolls with that weapon.",
            "Fighting Style: Great Weapon Fighting":
                "When you roll a 1 or 2 on a damage die for an attack you make with a melee weapon that you are wielding with two hands, you can reroll the die and must use the new roll, even if the new roll is a 1 or a 2. The weapon must have the two-handed or versatile property for you to gain this benefit.",
            "Fighting Style: Protection":
                "When a creature you can see attacks a target other than you that is within 5 feet of you, you can use your reaction to impose disadvantage on the attack roll. You must be wielding a shield.",
            "Fighting Style: Two-Weapon Fighting":
                "When you engage in two-weapon fighting, you can add your ability modifier to the damage of the second attack.",
            "Second Wind":
                "You have a limited well of stamina that you can draw on to protect yourself from harm. On your turn, you can use a bonus action to regain hit points equal to 1d10 + your fighter level.\n\nOnce you use this feature, you must finish a short or long rest before you can use it again.",
        },
        2:{
            "Action Surge":
                "Starting at 2nd level, you can push yourself beyond your normal limits for a moment. On your turn, you can take one additional action on top of your regular action and a possible bonus action.\n\nOnce you use this feature, you must finish a short or long rest before you can use it again. Starting at 17th level, you can use it twice before a rest, but only once on the same turn.",
        },
        3:{
            "Martial Archetype":
                "At 3rd level, you choose an archetype that you strive to emulate in your combat styles and techniques. Choose Champion, Battle Master, or Eldritch Knight, all detailed at the end of the class description. The archetype you choose grants you features at 3rd level and again at 7th, 10th, 15th, and 18th level.",
            "Martial Archetype: Champion":
                "The archetypal Champion focuses on the development of raw physical power honed to deadly perfection. Those who model themselves on this archetype combine rigorous training with physical excellence to deal devastating blows.",
            "Champion: Improved Critical":
                "Beginning when you choose this archetype at 3rd level, your weapon attacks score a critical hit on a roll of 19 or 20.",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        5:{
            "Extra Attack":
                "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn.\n\nThe number of attacks increases to three when you reach 11th level in this class and to four when you reach 20th level in this class.",
        },
        6:{
            "Ability Score Improvement":
                "When you reach 6th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        7:{
            "Champion: Remarkable Athlete":
                "Starting at 7th level, you can add half your proficiency bonus (round up) to any Strength, Dexterity, or Constitution check you make that doesn't already use your proficiency bonus.\n\nIn addition, when you make a running long jump, the distance you can cover increases by a number of feet equal to your Strength modifier.",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        9:{
            "Indomitable":
                "Beginning at 9th level, you can reroll a saving throw that you fail. If you do so, you must use the new roll, and you can't use this feature again until you finish a long rest.\n\nYou can use this feature twice between long rests starting at 13th level and three times between long rests starting at 17th level.",
        },
        10:{
            "Champion: Additional Fighting Style":
                "At 10th level, you can choose a second option from the Fighting Style class feature.",
        },
        11:{
            "Extra Attack (2)":
                "At 11th level, you can attack three times whenever you take the Attack action on your turn.",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        13:{
            "Indomitable (two uses)":
                "At 13th level, you can use Indomitable twice between long rests.",
        },
        14:{
            "Ability Score Improvement":
                "When you reach 14th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        15:{
            "Champion: Superior Critical":
                "Starting at 15th level, your weapon attacks score a critical hit on a roll of 18-20.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        17:{
            "Action Surge (two uses)":
                "At 17th level, you can use Action Surge twice before a rest, but only once on the same turn.",
            "Indomitable (three uses)":
                "At 17th level, you can use Indomitable three times between long rests.",
        },
        18:{
            "Champion: Survivor":
                "At 18th level, you attain the pinnacle of resilience in battle. At the start of each of your turns, you regain hit points equal to 5 + your Constitution modifier if you have no more than half of your hit points left. You don't gain this benefit if you have 0 hit points.",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        20:{
            "Extra Attack (3)":
                "At 20th level, you can attack four times whenever you take the Attack action on your turn.",
        },
    },
),
Class(
    name: "Monk",
    hit_die: 8,
    saves:(
        strength: Proficient,
        dexterity: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: none\nWeapons: simple weapons, shortswords\nTools: any one type of artisan's tools or any one musical instrument of your choice\nSkills: Choose two from Acrobatics, Athletics, History, Insight, Religion, and Stealth",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) a shortsword or (b) any simple weapon\n\t• (a) a dungeoneer's pack or (b) an explorer's pack\n\t• 10 darts\n\nAlternatively, you may start with 5d4 gp to buy your own equipment.",
            "Unarmored Defense":
                "Beginning at 1st level, while you are wearing no armor and not wielding a shield, your AC equals 10 + your Dexterity modifier + your Wisdom modifier.",
            "Martial Arts":
                "Your practice of martial arts gives you mastery of combat styles that use unarmed strikes and monk weapons, which are shortswords and any simple melee weapons that don't have the two-handed or heavy property.\n\nYou gain the following benefits while you are unarmed or wielding only monk weapons and you aren't wearing armor or wielding a shield.\n\t• You can use Dexterity instead of Strength for the attack and damage rolls of your unarmed strikes and monk weapons.\n\t• You can roll a d4 in place of the normal damage of your unarmed strike or monk weapon.\n\t• When you use the Attack action with an unarmed strike or a monk weapon on your turn, you can make one unarmed strike as a bonus action. For example, if you take the Attack action and attack with a quarterstaff, you can also make an unarmed strike as a bonus action, assuming you haven't already taken a bonus action this turn.\n\nCertain monasteries use specialized forms of the monk weapons. For example, you might use a club that is two lengths of wood connected by a short chain (called a nunchaku) or a sickle with a shorter, straighter blade (called a kama).",
        },
        2:{
            "Ki":
                "Starting at 2nd level, your training allows you to harness the mystic energy of ki. Your access to this energy is represented by a number of ki points. Your monk level determines the number of points you have, as shown in the Ki Points column of the Monk table.\n\nYou can spend these points to fuel various ki features. You start knowing three such features: Flurry of Blows, Patient Defense, and Step of the Wind. You learn more ki features as you gain levels in this class.\n\nWhen you spend a ki point, it is unavailable until you finish a short or long rest, at the end of which you draw all of your expended ki back into yourself. You must spend at least 30 minutes of the rest meditating to regain your ki points.\n\nSome of your ki features require your target to make a saving throw to resist the feature's effects. The saving throw DC is calculated as follows: Ki save DC = 8 + your proficiency bonus + your Wisdom modifier",
            "Flurry of Blows":
                "Immediately after you take the Attack action on your turn, you can spend 1 ki point to make two unarmed strikes as a bonus action.",
            "Patient Defense":
                "You can spend 1 ki point to take the Dodge action as a bonus action on your turn.",
            "Step of the Wind":
                "You can spend 1 ki point to take the Disengage or Dash action as a bonus action on your turn, and your jump distance is doubled for the turn.",
            "Unarmored Movement":
                "Starting at 2nd level, your speed increases by 10 feet while you are not wearing armor or wielding a shield. This bonus increases when you reach certain monk levels, as shown in the Monk table.\n\nAt 9th level, you gain the ability to move along vertical surfaces and across liquids on your turn without falling during the move.",
        },
        3:{
            "Monastic Tradition":
                "When you reach 3rd level, you commit yourself to a monastic tradition: the Way of the Open Hand, the Way of Shadow, or the Way of the Four Elements. Your tradition grants you features at 3rd level and again at 6th, 11th, and 17th level.",
            "Monastic Tradition: Way of the Open Hand":
                "Monks of the Way of the Open Hand are the ultimate masters of martial arts combat, whether armed or unarmed. They learn techniques to push and trip their opponents, manipulate ki to heal damage to their bodies, and practice advanced meditation that can protect them from harm.",
            "Way of the Open Hand: Open Hand Technique":
                "You can manipulate your enemy's ki when you harness your own. Whenever you hit a creature with one of the attacks granted by your Flurry of Blows, you can impose one of the following effects on that target.\n\t• It must succeed on a Dexterity saving throw or be knocked prone.\n\t• It must make a Strength saving throw. If it fails, you can push it up to 15 feet away from you.\n\t• It can't take reactions until the end of your next turn.",
            "Deflect Missiles":
                "Starting at 3rd level, you can use your reaction to deflect or catch the missile when you are hit by a ranged weapon attack. When you do so, the damage you take from the attack is reduced by 1d 10 + your Dexterity modifier + your monk level.\n\nIf you reduce the damage to 0, you can catch the missile if it is small enough for you to hold in one hand and you have at least one hand free. If you catch a missile in this way, you can spend 1 ki point to make a ranged attack (range 20 feet/60 feet) with the weapon or piece of ammunition you just caught, as part of the same reaction. You make this attack with proficiency, regardless of your weapon proficiencies, and the missile counts as a monk weapon for the attack.",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
            "Slow Fall":
                "Beginning at 4th level, you can use your reaction when you fall to reduce any falling damage you take by an amount equal to five times your monk level.",
        },
        5:{
            "Extra Attack":
                "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn.",
            "Stunning Strike":
                "Starting at 5th level, you can interfere with the flow of ki in an opponent's body. When you hit another creature with a melee weapon attack, you can spend 1 ki point to attempt a stunning strike. The target must succeed on a Constitution saving throw or be stunned until the end of your next turn.",
            "Martial Arts (1d6)":
                "You can  now roll a d6 in the place of the normal damage of your unarmed strike or monk weapon.",
        },
        6:{
            "Ki-Empowered Strikes":
                "Starting at 6th level, your unarmed strikes count as magical for the purpose of overcoming resistance and immunity to nonmagical attacks and damage.",
            "Unarmed Movement (+ 15 ft.)":
                "The bonus to your speed increases to 15 feet while you are not wearing armor or wielding a shield.",
            "Way of the Open Hand: Wholeness of Body":
                "You gain the ability to heal yourself. As an action, you can regain hit points equal to three times your monk level. You must finish a long rest before you can use this feature again.",
        },
        7:{
            "Stillness of Mind":
                "Starting at 7th level, you can use your action to end one effect on yourself that is causing you to be charmed or frightened.",
            "Evasion":
                "At 7th level, your instinctive agility lets you dodge out of the way of certain area effects, such as a blue dragon's lightning breath or a fireball spell. When you are subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail.",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        9:{
            "Unarmored Movement (Vertical Surfaces)":
                "You gain the ability to move along vertical surfaces and across liquids on your turn without falling during the move.",
        },
        10:{
            "Purity of Body":
                "At 10th level, your mastery of the ki flowing through you makes you immune to disease and poison.",
            "Unarmed Movement (+ 20 ft.)":
                "The bonus to your speed increases to 20 feet while you are not wearing armor or wielding a shield.",
        },
        11:{
            "Way of the Open Hand: Tranquility":
                "Beginning at 11th level, you can enter a special meditation that surrounds you with an aura of peace. At the end of a long rest, you gain the effect of a sanctuary spell that lasts until the start of your next long rest (the spell can end early as normal). The saving throw DC for the spell equals 8 + your Wisdom modifier + your Proficiency bonus.\n\nSanctuary gives this effect: You ward a creature within range against attack. Until the spell ends, any creature who targets the warded creature with an attack or a harmful spell must first make a Wisdom saving throw. On a failed save, the creature must choose a new target or lose the attack or spell. This spell doesn't protect the warded creature from area effects, such as the explosion of a fireball.\n\nIf the warded creature makes an attack or casts a spell that affects an enemy creature, this spell ends.",
            "Martial Arts (1d8)":
                "You can  now roll a d8 in the place of the normal damage of your unarmed strike or monk weapon.",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        13:{
            "Tongue of the Sun and Moon":
                "Starting at 13th level, you learn to touch the ki of other minds so that you understand all spoken languages. Moreover, any creature that can understand a language can understand what you say.",
        },
        14:{
            "Diamond Soul":
                "Beginning at 14th level, your mastery of ki grants you proficiency in all saving throws.\n\nAdditionally, whenever you make a saving throw and fail, you can spend 1 ki point to reroll it and take the second result.",
            "Unarmed Movement (+ 25 ft.)":
                "The bonus to your speed increases to 25 feet while you are not wearing armor or wielding a shield.",
        },
        15:{
            "Timeless Body":
                "At 15th level, your ki sustains you so that you suffer none of the frailty of old age, and you can't be aged magically. You can still die of old age, however. In addition, you no longer need food or water.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        17:{
            "Martial Arts (1d10)":
                "You can  now roll a d10 in the place of the normal damage of your unarmed strike or monk weapon.",
            "Way of the Open Hand: Quivering Palm":
                "You gain the ability to set up lethal vibrations in someone's body. When you hit a creature with an unarmed strike, you can spend 3 ki points to start these imperceptible vibrations, which last for a number of days equal to your monk level. The vibrations are harmless unless you use your action to end them. To do so, you and the target must be on the same plane of existence. When you use this action, the creature must make a Constitution saving throw. If it fails, it is reduced to 0 hit points. If it succeeds, it takes 10d10 necrotic damage.\n\nYou can have only one creature under the effect of this feature at a time. You can choose to end the vibrations harmlessly without using an action.",
        },
        18:{
            "Empty Body":
                "Beginning at 18th level, you can use your action to spend 4 ki points to become invisible for 1 minute. During that time, you also have resistance to all damage but force damage.\n\nAdditionally, you can spend 8 ki points to cast the astral projection spell, without needing material components. When you do so, you can't take any other creatures with you.",
            "Unarmed Movement (+ 30 ft.)":
                "The bonus to your speed increases to 30 feet while you are not wearing armor or wielding a shield.",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        20:{
            "Perfect Soul":
                "At 20th level, when you roll for initiative and have no ki points remaining, you regain 4 ki points.",
        },
    },
),
Class(
    name: "Paladin",
    hit_die: 10,
    spellcasting_ability: "Charisma",
    saves:(
        wisdom: Proficient,
        charisma: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: light armor, medium armor, heavy armor, shields\nWeapons: simple weapons, martial weapons\nTools: none\nSkills: Choose two from Athletics, Insight, Intimidation, Medicine, Persuasion, and Religion",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) a martial weapon and a shield or (b) two martial weapons\n\t• (a) five javelins or (b) any simple melee weapon\n\t• (a) a priest's pack or (b) an explorer's pack\n\t• Chain mail and a holy symbol\n\tAlternatively, you may start with 5d4 x 10 gp to buy your own equipment.",
            "Divine Sense":
                "The presence of strong evil registers on your senses like a noxious odor, and powerful good rings like heavenly music in your ears. As an action, you can open your awareness to detect such forces. Until the end of your next turn, you know the location of any celestial, fiend, or undead within 60 feet of you that is not behind total cover. You know the type (celestial, fiend, or undead) of any being whose presence you sense, but not its identity (the vampire Count Strahd von Zarovich, for instance). Within the same radius, you also detect the presence of any place or object that has been consecrated or desecrated, as with the hallow spell.\n\nYou can use this feature a number of times equal to 1 + your Charisma modifier. When you finish a long rest, you regain all expended uses.",
            "Lay on Hands":
                "Your blessed touch can heal wounds. You have a pool of healing power that replenishes when you take a long rest. With that pool, you can restore a total number of hit points equal to your paladin level x 5.\n\nAs an action, you can touch a creature and draw power from the pool to restore a number of hit points to that creature, up to the maximum amount remaining in your pool.\n\nAlternatively, you can expend 5 hit points from your pool of healing to cure the target of one disease or neutralize one poison affecting it. You can cure multiple diseases and neutralize multiple poisons with a single use of Lay on Hands, expending hit points separately for each one.\n\nThis feature has no effect on undead and constructs.",
        },
        2:{
            "Divine Smite":
                "Starting at 2nd level, when you hit a creature with a melee weapon attack, you can expend one spell slot to deal radiant damage to the target, in addition to the weapon's damage. The extra damage is 2d8 for a 1st-level spell slot, plus 1d8 for each spell level higher than 1st, to a maximum of 5d8. The damage increases by 1d8 if the target is an undead or a fiend.",
            "Fighting Style: Defense":
                "While you are wearing armor, you gain a +1 bonus to AC.",
            "Fighting Style: Dueling":
                "When you are wielding a melee weapon in one hand and no other weapons, you gain a +2 bonus to damage rolls with that weapon.",
            "Fighting Style: Great Weapon Fighting":
                "When you roll a 1 or 2 on a damage die for an attack you make with a melee weapon that you are wielding with two hands, you can reroll the die and must use the new roll. The weapon must have the two-handed or versatile property for you to gain this benefit.",
            "Fighting Style: Protection":
                "When a creature you can see attacks a target other than you that is within 5 feet of you, you can use your reaction to impose disadvantage on the attack roll. You must be wielding a shield.",
            "Spellcasting":
                "By 2nd level, you have learned to draw on divine magic through meditation and prayer to cast spells as a cleric does. See chapter 10 for the general rules of spellcasting and chapter 11 for the paladin spell list.\n\nPreparing and Casting Spells\nThe Paladin table shows how many spell slots you have to cast your spells. To cast one of your paladin spells of 1st level or higher, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\n\nYou prepare the list of paladin spells that are available for you to cast, choosing from the paladin spell list. When you do so, choose a number of paladin spells equal to your Charisma modifier + half your paladin level, rounded down (minimum of one spell). The spells must be of a level for which you have spell slots.\n\nFor example, if you are a 5th-level paladin, you have four 1st-level and two 2nd-level spell slots. With a Charisma of 14, your list of prepared spells can include four spells of 1st or 2nd level, in any combination. If you prepare the 1st-level spell cure wounds, you can cast it using a 1st-level or a 2nd-level slot. Casting the spell doesn't remove it from your list of prepared spells.\n\nYou can change your list of prepared spells when you finish a long rest. Preparing a new list of paladin spells requires time spent in prayer and meditation: at least 1 minute per spell level for each spell on your list.\n\nSpellcasting Ability\nCharisma is your spellcasting ability for your paladin spells, since their power derives from the strength of your convictions. You use your Charisma whenever a spell refers to your spellcasting ability. In addition, you use your Charisma modifier when setting the saving throw DC for a paladin spell you cast and when making an attack roll with one.\n\n\tSpell save DC = 8 + your proficiency bonus + your Charisma modifier\n\n\tSpell attack modifier = your proficiency bonus + your Charisma modifier\n\nSpellcasting Focus\nYou can use a holy symbol (found in chapter 5) as a spellcasting focus for your paladin spells.",
        },
        3:{
            "Divine Health":
                "By 3rd level, the divine magic flowing through you makes you immune to disease.",
            "Sacred Oath":
                "When you reach 3rd level, you swear the oath that binds you as a paladin forever. Up to this time you have been in a preparatory stage, committed to the path but not yet sworn to it. Now you choose the Oath of Devotion, the Oath of the Ancients, the Oath of the Crown, or the Oath of Vengeance, all detailed at the end of the c1ass description.\n\nYour choice grants you features at 3rd level and again at 7th, 15th, and 20th level. Those features include oath spells and the Channel Divinity feature.",
            "Sacred Oath: Oath of Devotion":
                "The Oath of Devotion binds a paladin to the loftiest ideals of justice, virtue, and order. Sometimes called cavaliers, white knights, or holy warriors, these paladins meet the ideal of the knight in shining armor, acting with honor in pursuit of justice and the greater good. They hold themselves to the highest standards of conduct, and some, for better or worse, hold the rest of the world to the same standards. Many who swear this oath are devoted to gods of law and good and use their gods' tenets as the measure of their devotion. They hold angels - the perfect servants of good - as their ideals, and incorporate images of angelic wings into their helmets or coats of arms.\n\nTenets of Devotion\nThough the exact words and strictures of the Oath of Devotion vary, paladins of this oath share these tenets.\n\tHonesty. Don't lie or cheat. Let your word be your promise.\n\tCourage. Never fear to act, though caution is wise.\n\tCompassion. Aid others, protect the weak, and punish those who threaten them. Show mercy to your foes, but temper it with wisdom.\n\tHonor. Treat others with fairness, and let your honorable deeds be an example to them. Do as much good as possible while causing the least amount of harm.\n\tDuty. Be responsible for your actions and their consequences, protect those entrusted to your care, and obey those who have just authority over you.",
            "Oath of Devotion Channel Divinity: Sacred Weapon":
                "As an action, you can imbue one weapon that you are holding with positive energy, using your Channel Divinity. For 1 minute, you add your Charisma modifier to attack rolls made with that weapon (with a minimum bonus of +1). The weapon also emits bright light in a 20-foot radius and dim light 20 feet beyond that. If the weapon is not already magical, it becomes magical for the duration.\n\nYou can end this effect on your turn as part of any other action. If you are no longer holding or carrying this weapon, or if you fall unconscious, this effect ends.",
            "Oath of Devotion Channel Divinity: Turn the Unholy":
                "As an action, you present your holy symbol and speak a prayer censuring fiends and undead, using your Channel Divinity. Each fiend or undead that can see or hear you within 30 feet of you must make a Wisdom saving throw. If the creature fails its saving throw, it is turned for 1 minute or until it takes damage.\n\nA turned creature must spend its turns trying to move as far away from you as it can, and it can't willingly move to a space within 30 feet of you. It also can't take reactions. For its action, it can use only the Dash action or try to escape from an effect that prevents it from moving. If there's nowhere to move, the creature can use the Dodge action.",
            "Oath of Devotion: Oath Spells":
                "You gain oath spells at the paladin levels listed.\n\n3rd - protection from evil and good, sanctuary\n5th - lesser restoration, zone of truth\n9th - beacon of hope, dispel magic\n13th - freedom of movement, guardian of faith\n17th - commune, flame strike",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        5:{
            "Extra Attack":
                "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn.",
        },
        6:{
            "Aura of Protection":
                "Starting at 6th level, whenever you or a friendly creature within 10 feet of you must make a saving throw, the creature gains a bonus to the saving throw equal to your Charisma modifier (with a minimum bonus of +1). You must be conscious to grant this bonus.\n\nAt 18th level, the range of this aura increases to 30 feet.",
        },
        7:{
            "Oath of Devotion: Aura of Devotion":
                "Starting at 7th level, you and friendly creatures within 10 feet of you can't be charmed while you are conscious.\n\nAt 18th level, the range of this aura increases to 30 feet.",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        10:{
            "Aura of Courage":
                "Starting at 10th level, you and friendly creatures within 10 feet of you can't be frightened while you are conscious.\n\nAt 18th level, the range of this aura increases to 30 feet.",
        },
        11:{
            "Improved Divine Smite":
                "By 11th level, you are so suffused with righteous might that all your melee weapon strikes carry divine power with them. Whenever you hit a creature with a melee weapon, the creature takes an extra 1d8 radiant damage. If you also use your Divine Smite with an attack, you add this damage to the extra damage of your Divine Smite.",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        14:{
            "Cleansing Touch":
                "Beginning at 14th level, you can use your action to end one spell on yourself or on one willing creature that you touch.\n\nYou can use this feature a number of times equal to your Charisma modifier (a minimum of once). You regain expended uses when you finish a long rest.",
        },
        15:{
            "Oath of Devotion: Purity of Spirit":
                "Beginning at 15th level, you are always under the effects of a protection from evil and good spell.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        18:{
            "Aura of Protection Improvement":
                "At 18th level, the range of your Aura of Protection increases to 30 feet.",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        20:{
            "Oath of Devotion: Holy Nimbus":
                "At 20th level, as an action, you can emanate an aura of sunlight. For 1 minute, bright light shines from you in a 30-foot radius, and dim light shines 30 feet beyond that.\n\nWhenever an enemy creature starts its turn in the bright light, the creature takes 10 radiant damage.\n\nIn addition, for the duration, you have advantage on saving throws against spells cast by fiends or undead.\n\nOnce you use this feature, you can't use it again until you finish a long rest.",
        },
    },
),
Class(
    name: "Ranger",
    hit_die: 10,
    spellcasting_ability: "Wisdom",
    saves:(
        strength: Proficient,
        dexterity: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: light armor, medium armor, shields\nWeapons: simple weapons, martial weapons\nTools: none\nSkills: Choose three from Animal Handling, Athletics, Insight, Investigation, Nature, Perception, Stealth, and Survival",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) scale mail or (b) leather armor\n\t• (a) two shortswords or (b) two simple melee weapons\n\t• (a) a dungeoneer's pack or (b) an explorer's pack\n\t• A longbow and a quiver of 20 arrows\n\nAlternatively, you may start with 5d4 x 10 gp to buy your own equipment.",
            "Favored Enemy":
                "Beginning at 1st level, you have significant experience studying, tracking, hunting, and even talking to a certain type of enemy.\n\nChoose a type of favored enemy: aberrations, beasts, celestials, constructs, dragons, elementals, fey, fiends, giants, monstrosities, oozes, plants, or undead. Alternatively, you can select two races of humanoid (such as gnolls and orcs) as favored enemies.\n\nYou have advantage on Wisdom (Survival) checks to track your favored enemies, as well as on Intelligence checks to recall information about them.\n\nWhen you gain this feature, you also learn one language of your choice that is spoken by your favored enemies, if they speak one at all.\n\nYou choose one additional favored enemy, as well as an associated language, at 6th and 14th level. As you gain levels, your choices should reflect the types of monsters you have encountered on your adventures.",
            "Natural Explorer":
                "You are particularly familiar with one type of natural environment and are adept at traveling and surviving in such regions. Choose one type of favored terrain: arctic, coast, desert, forest, grassland, mountain, swamp, or the Underdark. When you make an Intelligence or Wisdom check related to your favored terrain, your proficiency bonus is doubled if you are using a skill that you're proficient in.\n\nWhile traveling for an hour or more in your favored terrain, you gain the following benefits:\n\t• Difficult terrain doesn't slow your group's travel.\n\t• Your group can't become lost except by magical means.\n\t• Even when you are engaged in another activity while traveling (such as foraging, navigating, or tracking), you remain alert to danger.\n\t• If you are traveling alone, you can move stealthily at a normal pace.\n\t• When you forage, you find twice as much food as you normally would.\n\t• While tracking other creatures, you also learn their exact number, their sizes, and how long ago they passed through the area.\n\nYou choose additional favored terrain types at 6th and 10th level.",
        },
        2:{
            "Spellcasting":
                "By the time you reach 2nd level, you have learned to use the magical essence of nature to cast spells, much as a druid does. See chapter 10 for the general rules of spellcasting and chapter 11 for the ranger spell list.\n\nSpell Slots\nThe Ranger table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\n\nFor example, if you know the 1st-level spell animal friendship and have a 1st-level and a 2nd-level spell slot available, you can cast animal friendship using either slot.\n\nSpells Known of 1st Level and Higher\nYou know two 1st-level spells of your choice from the ranger spell list.\n\nYou learn an additional ranger spell of your choice at each odd numbered level thereafter. Each of these spells must be of a level for which you have spell slots. For instance, when you reach 5th level in this class, you can learn one new spell of 1st or 2nd level.\n\nAdditionally, when you gain a level in this class, you can choose one of the ranger spells you know and replace it with another spell from the ranger spell list, which also must be of a level for which you have spell slots.\n\nSpellcasting Ability\nWisdom is your spellcasting ability for your ranger spells, since your magic draws on your attunement to nature. You use your Wisdom whenever a spell refers to your spellcasting ability. In addition, you use your Wisdom modifier when setting the saving throw DC for a ranger spell you cast and when making an attack roll with one.\n\n\tSpell save DC = 8 + your proficiency bonus + your Wisdom modifier\n\tSpell attack modifier = your proficiency bonus + your Wisdom modifier",
            "Fighting Style":
                "At 2nd level, you adopt a particular style of fighting as your specialty. Choose one of the following options: Archery, Defense, Dueling, Mariner, or Two-Weapon Fighting. You can't take a Fighting Style option more than once, even if you later get to choose again.",
            "Fighting Style: Archery":
                "You gain a +2 bonus to attack rolls you make with ranged weapons.",
            "Fighting Style: Defense":
                "While you are wearing armor, you gain a +1 bonus to AC.",
            "Fighting Style: Dueling":
                "When you are wielding a melee weapon in one hand and no other weapons, you gain a +2 bonus to damage rolls with that weapon.",
            "Fighting Style: Two-Weapon Fighting":
                "When you engage in two-weapon fighting, you can add your ability modifier to the damage of the second attack.",
        },
        3:{
            "Ranger Archetype":
                "At 3rd level, you choose an archetype that you strive to emulate: Beast Master, Deep Stalker, or Hunter. Your choice grants features at 3rd level, and again at 7th, 11th, and 15th level.",
            "Ranger Archetype: Hunter":
                "Emulating the Hunter archetype means accepting your place as a bulwark between civilization and the terrors of the wilderness. As you walk the Hunter's path, you learn specialized techniques for fighting the threats you face, from rampaging ogres and hordes of orcs to towering giants and terrifying dragons.",
            "Hunter: Hunter's Prey":
                "At 3rd level, you gain one of the following features of your choice: Colossus Slayer, Giant Killer, or Horde Breaker.",
            "Hunter's Prey: Colossus Slayer":
                "Your tenacity can wear down the most potent foes. When you hit a creature with a weapon attack, the creature takes an extra 1d8 damage if it's below its hit point maximum. You can deal this extra damage only once per turn.",
            "Hunter's Prey: Giant Killer":
                "When a Large or larger creature within 5 feet of you hits or misses you with an attack, you can use your reaction to attack that creature immediately after its attack, provided that you can see the creature.",
            "Hunter's Prey: Horde Breaker":
                "Once on each of your turns when you make a weapon attack, you can make another attack with the same weapon against a different creature that is within 5 feet of the original target and within range of your weapon.",
            "Primeval Awareness":
                "Beginning at 3rd level, you can use your action and expend one ranger spell slot to focus your awareness on the region around you. For 1 minute per level of the spell slot you expend, you can sense whether the following types of creatures are present within 1 mile of you (or within up to 6 miles if you are in your favored terrain): aberrations, celestials, dragons, elementals, fey, fiends, and undead. This feature doesn't reveal the creatures' location or number.",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        5:{
            "Extra Attack":
                "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn.",
        },
        6:{
            "Additional Favored Terrains":
                "You gain an additional favored terrain.",
            "Additional Favored Enemy":
                "You choose one additional favored enemy, as well as an associated language. Your choice should reflect the types of monsters you have encountered on your adventures.",
        },
        7:{
            "Hunter: Defensive Tactics":
                "At 7th level, you gain one of the following features of your choice: Escape the Horde, Multiattack Defense, or Steel Will.",
            "Defensive Tactics: Escape the Horde":
                "Opportunity attacks against you are made with disadvantage.",
            "Defensive Tactics: Multiattack Defense":
                "When a creature hits you with an attack, you gain a +4 bonus to AC against all subsequent attacks made by that creature for the rest of the turn.",
            "Defensive Tactics: Steel Will":
                "You have advantage on saving throws against being frightened.",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
            "Land's Stride":
                "Starting at 8th level, moving through nonmagical difficult terrain costs you no extra movement. You can also pass through nonmagical plants without being slowed by them and without taking damage from them if they have thorns, spines, or a similar hazard.\n\nIn addition, you have advantage on saving throws against plants that are magically created or manipulated to impede movement, such those created by the entangle spell.",
        },
        10:{
            "Hide in Plain Sight":
                "Starting at 10th level, you can spend 1 minute creating camouflage for yourself. You must have access to fresh mud, dirt, plants, soot, and other naturally occurring materials with which to create your camouflage.\n\nOnce you are camouflaged in this way, you can try to hide by pressing yourself up against a solid surface, such as a tree or wall, that is at least as tall and wide as you are. You gain a +10 bonus to Dexterity (Stealth) checks as long as you remain there without moving or taking actions. Once you move or take an action or a reaction, you must camouflage yourself again to gain this benefit.",
            "Additional Favored Terrains":
                "You gain an additional favored terrain.",
        },
        11:{
            "Hunter: Multiattack":
                "At 11th level, you gain one of the following features of your choice: Volley or Whirlwind Attack.",
            "Multiattack: Volley":
                "You can use your action to make a ranged attack against any number of creatures within 10 feet of a point you can see within your weapon's range. You must have ammunition for each target, as normal, and you make a separate attack roll for each target.",
            "Multiattack: Whirlwind Attack":
                "You can use your action to make a melee attack against any number of creatures within 5 feet of you, with a separate attack roll for each target.",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        14:{
            "Vanish":
                "Starting at 14th level, you can use the Hide action as a bonus action on your turn. Also, you can't be tracked by nonmagical means, unless you choose to leave a trail.",
            "Additional Favored Enemy":
                "You choose one additional favored enemy, as well as an associated language. Your choice should reflect the types of monsters you have encountered on your adventures.",
        },
        15:{
            "Hunter: Superior Hunter's Defense":
                "At 15th level, you gain one of the following features of your choice: Evasion, Stand Against the Tide, or Uncanny Dodge.",
            "Superior Hunter's Defense: Evasion":
                "You can nimbly dodge out of the way of certain area effects, such as a red dragon's fiery breath or a lightning bolt spell. When you are subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail.",
            "Superior Hunter's Defense: Stand Against the Tide":
                "When a hostile creature misses you with a melee attack, you can use your reaction to force that creature to repeat the same attack against another creature (other than itself) of your choice. ",
            "Superior Hunter's Defense: Uncanny Dodge":
                "When an attacker that you can see hits you with an attack, you can use your reaction to halve the attack's damage against you.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        18:{
            "Feral Senses":
                "At 18th level, you gain preternatural senses that help you fight creatures you can't see. When you attack a creature you can't see, your inability to see it doesn't impose disadvantage on your attack rolls against it. You are also aware of the location of any invisible creature within 30 feet of you, provided that the creature isn't hidden from you and you aren't blinded or deafened.",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        20:{
            "Foe Slayer":
                "At 20th level, you become an unparalleled hunter of your enemies. Once on each of your turns, you can add your Wisdom modifier to the attack roll or the damage roll of an attack you make against one of your favored enemies. You can choose to use this feature before or after the roll, but before any effects of the roll are applied.",
        },
    },
),
Class(
    name: "Rogue",
    hit_die: 8,
    saves:(
        dexterity: Proficient,
        intelligence: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: light armor\nWeapons: simple weapons, hand crossbows, longswords, rapiers, shortswords\nTools: thieves' tools\nSkills: Choose four from Acrobatics, Athletics, Deception, Insight, Intimidation, Investigation, Perception, Performance. Persuasion, Sleight of Hand, and Stealth",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) a rapier or (b) a shortsword\n\t• (a) a shortbow and quiver of 20 arrows or (b) a shortsword\n\t• (a) a burglar's pack, (b) a dungeoneer's pack, or (c) an explorer's pack\n\t• Leather armor, two daggers, and thieves' tools\n\nAlternatively, you may start with 4d4 x 10 gp to buy your own equipment.",
            "Expertise":
                "At 1st level, choose two of your skill proficiencies, or one of your skill proficiencies and your proficiency with thieves' tools. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.\n\nAt 6th level, you can choose two more of your proficiencies (in skills or with thieves' tools) to gain this benefit.",
            "Sneak Attack":
                "Beginning at 1st level, you know how to strike subtly and exploit a foe's distraction. Once per turn, you can deal an extra 1d6 damage to one creature you hit with an attack if you have advantage on the attack roll. The attack must use a finesse or a ranged weapon.\n\nYou don't need advantage on the attack roll if another enemy of the target is within 5 feet of it, that enemy isn't incapacitated, and you don't have disadvantage on the attack roll.\n\nThe amount of the extra damage increases as you gain levels in this class, as shown in the Sneak Attack column of the Rogue table.",
            "Thieves' Cant":
                "During your rogue training you learned thieves' cant, a secret mix of dialect, jargon, and code that allows you to hide messages in seemingly normal conversation. Only another creature that knows thieves' cant understands such messages. It takes four times longer to convey such a message than it does to speak the same idea plainly.\n\nIn addition, you understand a set of secret signs and symbols used to convey short, simple messages, such as whether an area is dangerous or the territory of a thieves' guild, whether loot is nearby, or whether the people in an area are easy marks or will provide a safe house for thieves on the run.",
        },
        2:{
            "Cunning Action":
                "Starting at 2nd level, your quick thinking and agility allow you to move and act quickly. You can take a bonus action on each of your turns in combat. This action can be used only to take the Dash, Disengage, or Hide action.",
        },
        3:{
            "Roguish Archetype":
                "At 3rd level, you choose an archetype that you emulate in the exercise of your rogue abilities: Thief, Assassin, or Arcane Trickster, all detailed at the end of the class description. Your archetype choice grants you features at 3rd level and then again at 9th, 13th, and 17th level.",
            "Roguish Archetype: Thief":
                "You hone your skills in the larcenous arts. Burglars, bandits, cutpurses, and other criminals typically follow this archetype, but so do rogues who prefer to think of themselves as professional treasure seekers, explorers, delvers, and investigators. In addition to improving your agility and stealth, you learn skills useful for delving into ancient ruins, reading unfamiliar languages, and using magic items you normally couldn't employ.\n\nYour archetype grants you features at 3rd level and then again at 9th, 13th, and 17th level.",
            "Thief: Fast Hands":
                "Starting at 3rd level, you can use the bonus action granted by your Cunning Action to make a Dexterity (Sleight of Hand) check, use your thieves' tools to disarm a trap or open a lock, or take the Use an Object action.",
            "Thief: Second-Story Work":
                "When you choose this archetype at 3rd level, you gain the ability to climb faster than normal; climbing no longer costs you extra movement.\n\nIn addition, when you make a running jump, the distance you cover increases by a number of feet equal to your Dexterity modifier.",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        5:{
            "Uncanny Dodge":
                "Starting at 5th level, when an attacker that you can see hits you with an attack, you can use your reaction to halve the attack's damage against you.",
        },
        7:{
            "Evasion":
                "Beginning at 7th level, you can nimbly dodge out of the way of certain area effects, such as a red dragon's fiery breath or an ice storm spell. When you are subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail.",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        9:{
            "Thief: Supreme Sneak":
                "Starting at 9th level, you have advantage on a Dexterity (Stealth) check if you move no more than half your speed on the same turn.",
        },
        10:{
            "Ability Score Improvement":
                "When you reach 10th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        11:{
            "Reliable Talent":
                "By 11th level, you have refined your chosen skills until they approach perfection. Whenever you make an ability check that lets you add your proficiency bonus, you can treat a d20 roll of 9 or lower as a 10.",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        13:{
            "Thief: Use Magic Device":
                "By 13th level, you have learned enough about the workings of magic that you can improvise the use of items even when they are not intended for you. You ignore all class, race, and level requirements on the use of magic items.",
        },
        14:{
            "Blindsense":
                "Starting at 14th level, if you are able to hear, you are aware of the location of any hidden or invisible creature within 10 feet of you.",
        },
        15:{
            "Slippery Mind":
                "By 15th level, you have acquired greater mental strength. You gain proficiency in Wisdom saving throws.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        17:{
            "Thief: Thief's Reflexes":
                "When you reach 17th level, you have become adept at laying ambushes and quickly escaping danger. You can take two turns during the first round of any combat. You take your first turn at your normal initiative and your second turn at your initiative minus 10. You can't use this feature when you are surprised.",
        },
        18:{
            "Elusive":
                "Beginning at 18th level, you are so evasive that attackers rarely gain the upper hand against you. No attack roll has advantage against you while you aren't incapacitated.",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        20:{
            "Stroke of Luck":
                "At 20th level, you have an uncanny knack for succeeding when you need to. If your attack misses a target within range, you can turn the miss into a hit. Alternatively, if you fail an ability check, you can treat the d20 roll as a 20.\n\nOnce you use this feature, you can't use it again until you finish a short or long rest.",
        },
    },
),
Class(
    name: "Sorcerer",
    hit_die: 6,
    spellcasting_ability: "Charisma",
    saves:(
        constitution: Proficient,
        charisma: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: none\nWeapons: daggers, darts, slings, quarterstaffs, light crossbows\nTools: none\nSkills: Choose two from Arcana, Deception, Insight, Intimidation, Persuasion, and Religion",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) a light crossbow and 20 bolts or (b) any simple weapon\n\t• (a) a component pouch or (b) an arcane focus\n\t• (a) a dungeoneer's pack or (b) an explorer's pack\n\t• Two daggers\n\nAlternatively, you may start with 3d4 x 10 gp to buy your own equipment.",
            "Spellcasting":
                "An event in your past, or in the life of a parent or ancestor, left an indelible mark on you, infusing you with arcane magic. This font of magic, whatever its origin, fuels your spells. See chapter 10 for the general rules of spellcasting and chapter 11 for the sorcerer spell list.\n\nCantrips\nAt 1st level, you know four cantrips of your choice from the sorcerer spell list. You learn an additional sorcerer cantrip of your choice at 4th level and another at 10th level.\n\nSpell Slots\nThe Sorcerer table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these sorcerer spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\n\nFor example, if you know the 1st-level spell burning hands and have a 1st-level and a 2nd-level spell slot available, you can cast burning hands using either slot.\n\nSpells Known of 1st Level and Higher\nYou know two 1st-level spells of your choice from the sorcerer spell list.\n\nYou learn an additional sorcerer spell of your choice at each level except 12th, 14th, 16th, 18th, 19th, and 20th. Each of these spells must be of a level for which you have spell slots. For instance, when you reach 3rd level in this class, you can learn one new spell of 1st or 2nd level.\n\nAdditionally, when you gain a level in this class, you can choose one of the sorcerer spells you know and replace it with another spell from the sorcerer spell list, which also must be of a level for which you have spell slots.\n\nSpellcasting Ability\nCharisma is your spellcasting ability for your sorcerer spells, since the power of your magic relies on your ability to project your will into the world. You use your Charisma whenever a spell refers to your spellcasting ability. In addition, you use your Charisma modifier when setting the saving throw DC for a sorcerer spell you cast and when making an attack roll with one.\n\n\tSpell save DC = 8 + your proficiency bonus + your Charisma modifier\n\n\tSpell attack modifier = your proficiency bonus + your Charisma modifier",
            "Sorcerous Origin":
                "Choose a sorcerous origin, which describes the source of your innate magical power: Draconic Bloodline, Wild Magic, Favored Soul, or Storm Sorcerer.\n\nYour choice grants you features when you choose it at 1st level and again at 6th, 14th, and 18th level.",
            "Sorcerous Origin: Draconic Bloodline":
                "Your innate magic comes from draconic magic that was mingled with your blood or that of your ancestors. Most often, sorcerers with this origin trace their descent back to a mighty sorcerer of ancient times who made a bargain with a dragon or who might even have claimed a dragon parent. Some of these bloodlines are well established in the world, but most are obscure. Any given sorcerer could be the first of a new bloodline, as a result of a pact or some other exceptional circumstance.",
            "Dragon Ancestor":
                "At 1st level, you choose one type of dragon as your ancestor. The damage type associated with each dragon is used by features you gain later.\n\nDraconic Ancestor: Black (Acid)\nDraconic Ancestor: Blue (Lightning)\nDraconic Ancestor: Brass (Fire)\nDraconic Ancestor: Bronze (Lightning)\nDraconic Ancestor: Copper (Acid)\nDraconic Ancestor: Gold (Fire)\nDraconic Ancestor: Green (Poison)\nDraconic Ancestor: Red (Fire)\nDraconic Ancestor: Silver (Cold)\nDraconic Ancestor: White (Cold)\n\nYou can speak, read, and write Draconic. Additionally, whenever you make a Charisma check when interacting with dragons, your Proficiency bonus is doubled if it applies to the check.",
            "Draconic Bloodline: Draconic Resilience":
                "As magic flows through your body, it causes physical traits of your dragon ancestors to emerge. At 1st level, your hit point maximum increases by 1 and increases by 1 again whenever you gain a level in this class.\n\nAdditionally, parts of your skin are covered by a thin sheen of dragon-like scales. When you aren't wearing armor, your AC equals 13 + your Dexterity modifier.",
        },
        2:{
            "Sorcery Points":
                "You have 2 sorcery points, and you gain one additional point every time you level up, to a maximum of 20 at level 20. You can never have more sorcery points than shown on the table for your level. You regain all spent sorcery points when you finish a long rest.",
            "Flexible Casting":
                "You can use your sorcery points to gain additional spell slots, or sacrifice spell slots to gain additional sorcery points. You learn other ways to use your sorcery points as you reach higher levels.\n\nCreating Spell Slots. You can transform unexpended sorcery points into one spell slot as a bonus action on your turn. The created spell slots vanish at the end of a long rest. The Creating Spell Slots table shows the cost of creating a spell slot of a given level. You can create spell slots no higher in level than 5th.\n\nCreating Spell Slots\nSpell Slot Level  —  Sorcery Point Cost\n1st —  2\n2nd —  3\n3rd —  5\n4th —  6\n5th —  7\n\nConverting a Spell Slot to Sorcery Points. As a bonus action on your turn, you can expend one spell slot and gain a number of sorcery points equal to the slot's level.",
        },
        3:{
            "Metamagic":
                "At 3rd level, you gain the ability to twist your spells to suit your needs. You gain two of the following Metamagic options of your choice. You gain another one at 10th and 17th level.\n\nYou can use only one Metamagic option on a spell when you cast it, unless otherwise noted.",
            "Metamagic: Careful Spell":
                "When you cast a spell that forces other creatures to make a saving throw, you can protect some of those creatures from the spell's full force. To do so, you spend 1 sorcery point and choose a number of those creatures up to your Charisma modifier (minimum of one creature). A chosen creature automatically succeeds on its saving throw against the spell.",
            "Metamagic: Distant Spell":
                "When you cast a spell that has a range of 5 feet or greater, you can spend 1 sorcery point to double the range of the spell.\n\nWhen you cast a spell that has a range of touch, you can spend 1 sorcery point to make the range of the spell 30 feet.",
            "Metamagic: Empowered Spell":
                "When you roll damage for a spell, you can spend 1 sorcery point to reroll a number of the damage dice up to your Charisma modifier (minimum of one). You must use the new rolls.\n\nYou can use Empowered Spell even if you have already used a different Metamagic option during the casting of the spell.",
            "Metamagic: Extended Spell":
                "When you cast a spell that has a duration of 1 minute or longer, you can spend 1 sorcery point to double its duration, to a maximum duration of 24 hours.",
            "Metamagic: Heightened Spell":
                "When you cast a spell that forces a creature to make a saving throw to resist its effects, you can spend 3 sorcery points to give one target of the spell disadvantage on its first saving throw made against the spell.",
            "Metamagic: Quickened Spell":
                "When you cast a spell that has a casting time of 1 action, you can spend 2 sorcery points to change the casting time to 1 bonus action for this casting.",
            "Metamagic: Subtle Spell":
                "When you cast a spell, you can spend 1 sorcery point to cast it without any somatic or verbal components.",
            "Metamagic: Twinned Spell":
                "When you cast a spell that doesn't have a range of self and is incapable of targeting more than one creature at the spell's current level, you can spend a number of sorcery points equal to the spell's level to target a second creature in range with the same spell (1 sorcery point if the spell is a cantrip).",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        6:{
            "Draconic Bloodline: Elemental Affinity":
                "Starting at 6th level, when you cast a spell that deals damage of the type associated with your draconic ancestry, add your Charisma modifier to one damage roll.\n\nIn addition, you can spend 1 sorcery point to gain resistance to that damage type for 1 hour.",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        10:{
            "Metamagic Option (3rd)":
                "At 10th level, you learn an additional metamagic option.",
            "Metamagic: Careful Spell":
                "When you cast a spell that forces other creatures to make a saving throw, you can protect some of those creatures from the spell's full force. To do so, you spend 1 sorcery point and choose a number of those creatures up to your Charisma modifier (minimum of one creature). A chosen creature automatically succeeds on its saving throw against the spell.",
            "Metamagic: Distant Spell":
                "When you cast a spell that has a range of 5 feet or greater, you can spend 1 sorcery point to double the range of the spell.\n\nWhen you cast a spell that has a range of touch, you can spend 1 sorcery point to make the range of the spell 30 feet.",
            "Metamagic: Empowered Spell":
                "When you roll damage for a spell, you can spend 1 sorcery point to reroll a number of the damage dice up to your Charisma modifier (minimum of one). You must use the new rolls.\n\nYou can use Empowered Spell even if you have already used a different Metamagic option during the casting of the spell.",
            "Metamagic: Extended Spell":
                "When you cast a spell that has a duration of 1 minute or longer, you can spend 1 sorcery point to double its duration, to a maximum duration of 24 hours.",
            "Metamagic: Heightened Spell":
                "When you cast a spell that forces a creature to make a saving throw to resist its effects, you can spend 3 sorcery points to give one target of the spell disadvantage on its first saving throw made against the spell.",
            "Metamagic: Quickened Spell":
                "When you cast a spell that has a casting time of 1 action, you can spend 2 sorcery points to change the casting time to 1 bonus action for this casting.",
            "Metamagic: Subtle Spell":
                "When you cast a spell, you can spend 1 sorcery point to cast it without any somatic or verbal components.",
            "Metamagic: Twinned Spell":
                "When you cast a spell that targets only one creature and doesn't have a range of self, you can spend a number of sorcery points equal to the spell's level to target a second creature in range with the same spell (1 sorcery point if the spell is a cantrip).",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        14:{
            "Draconic Bloodline: Dragon Wings":
                "At 14th level, you gain the ability to sprout a pair of dragon wings from your back, gaining a flying speed equal to your current speed. You can create these wings as a bonus action on your turn. They last until you dismiss them as a bonus action on your turn.\n\nYou can't manifest your wings while wearing armor unless the armor is made to accommodate them, and clothing not made to accommodate your wings might be destroyed when you manifest them.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        17:{
            "Metamagic Option (4th)":
                "At 17th level, you learn an additional metamagic option.",
            "Metamagic: Careful Spell":
                "When you cast a spell that forces other creatures to make a saving throw, you can protect some of those creatures from the spell's full force. To do so, you spend 1 sorcery point and choose a number of those creatures up to your Charisma modifier (minimum of one creature). A chosen creature automatically succeeds on its saving throw against the spell.",
            "Metamagic: Distant Spell":
                "When you cast a spell that has a range of 5 feet or greater, you can spend 1 sorcery point to double the range of the spell.\n\nWhen you cast a spell that has a range of touch, you can spend 1 sorcery point to make the range of the spell 30 feet.",
            "Metamagic: Empowered Spell":
                "When you roll damage for a spell, you can spend 1 sorcery point to reroll a number of the damage dice up to your Charisma modifier (minimum of one). You must use the new rolls.\n\nYou can use Empowered Spell even if you have already used a different Metamagic option during the casting of the spell.",
            "Metamagic: Extended Spell":
                "When you cast a spell that has a duration of 1 minute or longer, you can spend 1 sorcery point to double its duration, to a maximum duration of 24 hours.",
            "Metamagic: Heightened Spell":
                "When you cast a spell that forces a creature to make a saving throw to resist its effects, you can spend 3 sorcery points to give one target of the spell disadvantage on its first saving throw made against the spell.",
            "Metamagic: Quickened Spell":
                "When you cast a spell that has a casting time of 1 action, you can spend 2 sorcery points to change the casting time to 1 bonus action for this casting.",
            "Metamagic: Subtle Spell":
                "When you cast a spell, you can spend 1 sorcery point to cast it without any somatic or verbal components.",
            "Metamagic: Twinned Spell":
                "When you cast a spell that targets only one creature and doesn't have a range of self, you can spend a number of sorcery points equal to the spell's level to target a second creature in range with the same spell (1 sorcery point if the spell is a cantrip).",
        },
        18:{
            "Draconic Bloodline: Draconic Presence":
                "Beginning at 18th level, you can channel the dread presence of your dragon ancestor, causing those around you to become awestruck or frightened. As an action, you can spend 5 sorcery points to draw on this power and exude an aura of awe or fear (your choice) to a distance of 60 feet. For 1 minute or until you lose your concentration (as if you were casting a concentration spell), each hostile creature that starts its turn in this aura must succeed on a Wisdom saving throw or be charmed (if you chose awe) or frightened (if you chose fear) until the aura ends. A creature that succeeds on this saving throw is immune to your aura for 24 hours.",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        20:{
            "Sorcerous Restoration":
                "At 20th level, you regain 4 expended sorcery points whenever you finish a short rest.",
        },
    },
),
Class(
    name: "Warlock",
    hit_die: 8,
    spellcasting_ability: "Charisma",
    saves:(
        wisdom: Proficient,
        charisma: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: light armor\nWeapons: simple weapons\nTools: none\nSkills: Choose two skills from Arcana, Deception, History, Intimidation, Investigation, Nature, and Religion",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) a light crossbow and 20 bolts or (b) any simple weapon\n\t• (a) a component pouch or (b) an arcane focus\n\t• (a) a scholar's pack or (b) a dungeoneer's pack\n\t• Leather armor, any simple weapon, and two daggers\n\nAlternatively, you may start with 4d4 x 10 gp to buy your own equipment.",
            "Otherworldly Patron":
                "At 1st level, you have struck a bargain with an otherworldly being of your choice - the Archfey, the Fiend, or the Great Old One, each of which is detailed at the end of the class description. Your choice grants you features at 1st level and again at 6th, 10th, and 14th level.",
            "Otherworldly Patron: The Fiend":
                "You have made a pact with a fiend from the lower planes of existence, a being whose aims are evil, even if you strive against those aims. Such beings desire the corruption or destruction of all things, ultimately including you. Fiends powerful enough to forge a pact include demon lords such as Demogorgon, Orcus, Fraz'Urb-luu, and Baphomet; archdevils such as Asmodeus, Dispater, Mephistopheles, and Belial; pit fiends and balors that are especially mighty; and ultroloths and other lords of the yugoloths.\n\nExpanded Spell List: The Fiend lets you choose from an expanded list of spells when you learn a warlock spell. The following spells are added to the warlock spell list for you.\n\n1st - burning hands, command\n2nd - blindness/deafness, scorching ray\n3rd - fireball, stinking cloud\n4th - fire shield, wall of fire\n5th - flame strike, hallow",
            "The Fiend: Dark One's Blessing":
                "Starting at 1st level, when you reduce a hostile creature to 0 hit points, you gain temporary hit points equal to your Charisma modifier + your warlock level (minimum of 1).",
            "Pact Magic":
                "Your arcane research and the magic bestowed on you by your patron have given you facility with spells. See chapter 10 for the general rules of spellcasting and chapter 11 for the warlock spell list.\n\nCantrips\nYou know two cantrips of your choice from the warlock spell list. You learn additional warlock cantrips of your choice at higher levels, as shown in the Cantrips Known column of the Warlock table.\n\nSpell Slots\nThe Warlock table shows how many spell slots you have. The table also shows what the level of those slots is; all of your spell slots are the same level. To cast one of your warlock spells of 1st level or higher, you must expend a spell slot. You regain all expended spell slots when you finish a short or long rest.\n\nFor example, when you are 5th level, you have two 3rd-level spell slots. To cast the 1st-level spell thunderwave, you must spend one of those slots, and you cast it as a 3rd-level spell.\n\nSpells Known of 1st Level and Higher\nAt 1st level, you know two 1st-level spells of your choice from the warlock spell list.\n\nYou learn a new warlock spell every time you gain a level from 2 through 9, as well as at level 19. A spell you choose must be of a level no higher than what's shown in the table's Slot Level column for your level. When you reach 6th level, for example, you learn a new warlock spell, which can be 1st, 2nd, or 3rd level.\n\nAdditionally, when you gain a level in this class, you can choose one of the warlock spells you know and replace it with another spell from the warlock spell list, which also must be of a level for which you have spell slots.\n\nSpellcasting Ability\nCharisma is your spellcasting ability for your warlock spells, so you use your Charisma whenever a spell refers to your spellcasting ability. In addition, you use your Charisma modifier when setting the saving throw DC for a warlock spell you cast and when making an attack roll with one.\n\n\tSpell save DC: 8 + Proficiency bonus + Charisma modifier\n\tSpell attack modifier: Proficiency bonus + Charisma modifier\n\nSpellcasting Focus\nYou can use an arcane focus (found in chapter 5) as a spellcasting focus for your warlock spells.",
        },
        2:{
            "Eldritch Invocations":
                "In your study of occult lore, you have unearthed eldritch invocations, fragments of forbidden knowledge that imbue you with an abiding magical ability.\n\nAt 2nd level, you gain two eldritch invocations of your choice. Your invocation options are detailed at the end of the class description. When you gain certain warlock levels, you gain additional invocations of your choice.\n\nAdditionally, when you gain a level in this class, you can choose one of the invocations you know and replace it with another invocation that you could learn at that level. A level prerequisite in an invocation refers to warlock level, not character level.",
        },
        3:{
            "Pact Boon: Pact of the Blade":
                "You can use your action to create a pact weapon in your empty hand. You can choose the form that this melee weapon takes each time you create it (see chapter 5 for weapon options). You are proficient with it while you wield it. This weapon counts as magical for the purpose of overcoming resistance and immunity to nonmagical attacks and damage.\n\nYour pact weapon disappears if it is more than 5 feet away from you for 1 minute or more. It also disappears if you use this feature again, if you dismiss the weapon (no action required), or if you die.\n\nYou can transform one magic weapon into your pact weapon by performing a special ritual while you hold the weapon. You perform the ritual over the course of 1 hour, which can be done during a short rest. You can then dismiss the weapon, shunting it into an extradimensional space, and it appears whenever you create your pact weapon thereafter. You can't affect an artifact or a sentient weapon in this way. The weapon ceases being your pact weapon if you die, if you perform the 1-hour ritual on a different weapon, or if you use a 1-hour ritual to break your bond to it. The weapon appears at your feet if it is in the extradimensional space when the bond breaks.",
            "Pact Boon: Pact of the Chain":
                "You learn the find familiar spell and can cast it as a ritual. The spell doesn't count against your number of spells known.\n\nWhen you cast the spell, you can choose one of the normal forms for your familiar or one of the following special forms: imp, pseudodragon, quasit, or sprite.\n\nAdditionally, when you take the Attack action, you can forgo one of your own attacks to allow your familiar to use its reaction to make one attack of its own.",
            "Pact Boon: Pact of the Tome":
                "Your patron gives you a grimoire called a Book of Shadows. When you gain this feature, choose three cantrips from any class's spell list. The cantrips do not need to be from the same spell list. While the book is on your person, you can cast those cantrips at will. They don't count against your number of cantrips known. Any cantrip you cast with this feature is considered a warlock cantrip for you. If you lose your Book of Shadows, you can perform a 1-hour ceremony to receive a replacement from your patron. This ceremony can be performed during a short or long rest, and it destroys the previous book. The book turns to ash when you die.",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        5:{
            "Eldritch Invocation":
                "You learn an additional eldritch invocation",
        },
        6:{
            "The Fiend: Dark One's Own Luck":
                "Starting at 6th level, you can call on your patron to alter fate in your favor. When you make an ability check or a saving throw, you can use this feature to add a d10 to your roll. You can do so after seeing the initial roll but before any of the roll's effects occur.\n\nOnce you use this feature, you can't use it again until you finish a short or long rest",
        },
        7:{
            "Eldritch Invocation":
                "You learn an additional eldritch invocation",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        9:{
            "Eldritch Invocation":
                "You learn an additional eldritch invocation",
        },
        10:{
            "The Fiend: Fiendish Resilience":
                "Starting at 10th level, you can choose one damage type when you finish a short or long rest. You gain resistance to that damage type until you choose a different one with this feature. Damage from magical weapons or silver weapons ignores this resistance.",
        },
        11:{
            "Mystic Arcanum (6th level)":
                "At 11th level, your patron bestows upon you a magical secret called an arcanum. Choose one 6th-level spell from the warlock spell list as this arcanum.\n\nYou can cast your arcanum spell once without expending a spell slot. You must finish a long rest before you can do so again.\n\nAt higher levels, you gain more warlock spells of your choice that can be cast in this way: one 7th-level spell at 13th level, one 8th-level spell at 15th level, and one 9th-level spell at 17th level. You regain all uses of your Mystic A rcanum when you finish a long rest.",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
            "Eldritch Invocation":
                "You learn an additional eldritch invocation",
        },
        13:{
            "Mystic Arcanum (7th level)":
                "At 13th level, your patron bestows upon you a magical secret called an arcanum. Choose one 7th-level spell from the warlock spell list as this arcanum.\n\nYou can cast your arcanum spell once without expending a spell slot. You must finish a long rest before you can do so again.",
        },
        14:{
            "The Fiend: Hurl Through Hell":
                "Starting at 14th level, when you hit a creature with an attack, you can use this feature to instantly transport the target through the lower planes. The creature disappears and hurtles through a nightmare landscape.\n\nAt the end of your next turn, the target returns to the space it previously occupied, or the nearest unoccupied space. If the target is not a fiend, it takes 10d 10 psychic damage as it reels from its horrific experience.\n\nOnce you use this feature, you can't use it again until you finish a long rest.",
        },
        15:{
            "Eldritch Invocation":
                "You learn an additional eldritch invocation",
            "Mystic Arcanum (8th level)":
                "At 15th level, your patron bestows upon you a magical secret called an arcanum. Choose one 8th-level spell from the warlock spell list as this arcanum.\n\nYou can cast your arcanum spell once without expending a spell slot. You must finish a long rest before you can do so again.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        17:{
            "Mystic Arcanum (9th level)":
                "At 17th level, your patron bestows upon you a magical secret called an arcanum. Choose one 9th-level spell from the warlock spell list as this arcanum.\n\nYou can cast your arcanum spell once without expending a spell slot. You must finish a long rest before you can do so again.",
        },
        18:{
            "Eldritch Invocation":
                "You learn an additional eldritch invocation",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        20:{
            "Eldritch Master":
                "At 20th level, you can draw on your inner reserve of mystical power while entreating your patron to regain expended spell slots. You can spend 1 minute entreating your patron for aid to regain all your expended spell slots from your Pact Magic feature. Once you regain spell slots with this feature, you must finish a long rest before you can do so again.",
        },
        2:{
            "Eldritch Invocation: Agonizing Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, add your Charisma modifier to the damage it deals on a hit.",
            "Eldritch Invocation: Armor of Shadows":
                "You can cast mage armor on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Beast Speech":
                "You can cast speak with animals at will, without expending a spell slot.",
            "Eldritch Invocation: Beguiling Influence":
                "You gain proficiency in the Deception and Persuasion skills.",
            "Eldritch Invocation: Book of Ancient Secrets":
                "Prerequisite: Pact of the Tome feature\n\nYou can now inscribe magical rituals in your Book of Shadows. Choose two 1st-level spells that have the ritual tag from any class's spell list; they do not have to be from the same class's spell list. The spells appear in the book and don't count against the number of spells you know. With your Book of Shadows in hand, you can cast the chosen spells as rituals. You can't cast the spells except as rituals, unless you've learned them by some other means. You can also cast a warlock spell you know as a ritual if it has the ritual tag.\n\nOn your adventures, you can add other ritual spells to your Book of Shadows. When you find such a spell, you can add it to the book if the spell's level is equal to or less than half your warlock level (rounded up) and if you can spare the time to transcribe the spell. For each level of the spell, the transcription process takes 2 hours and costs 50 gp for the rare inks needed to inscribe it.",
            "Eldritch Invocation: Devil's Sight":
                "You can see normally in darkness, both magical and nonmagical, to a distance of 120 feet.",
            "Eldritch Invocation: Eldritch Sight":
                "You can cast detect magic at will, without expending a spell slot.",
            "Eldritch Invocation: Eldritch Spear":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, its range is 300 feet.",
            "Eldritch Invocation: Eyes of the Rune Keeper":
                "You can read all writing.",
            "Eldritch Invocation: Fiendish Vigor":
                "You can cast false life on yourself at will as a 1st-level spell, without expending a spell slot or material components.",
            "Eldritch Invocation: Gaze of Two Minds":
                "You can use your action to touch a willing humanoid and perceive through its senses until the end of your next turn. As long as the creature is on the same plane of existence as you, you can use your action on subsequent turns to maintain this connection, extending the duration until the end of your next turn. While perceiving through the other creature's senses, you benefit from any special senses possessed by that creature, and you are blinded and deafened to your own surroundings.",
            "Eldritch Invocation: Mask of Many Faces":
                "You can cast disguise self at will, without expending a spell slot.",
            "Eldritch Invocation: Misty Visions":
                "You can cast silent image at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Repelling Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you hit a creature with eldritch blast, you can push the creature up to 10 feet away from you in a straight line.",
            "Eldritch Invocation: Thief of Five Fates":
                "You can cast bane once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Voice of the Chain Master":
                "Prerequisite: Pact of the Chain feature\n\nYou can communicate telepathically with your familiar and perceive through your familiar's senses as long as you are on the same plane of existence. Additionally, while perceiving through your familiar's senses, you can also speak through your familiar in your own voice, even if your familiar is normally incapable of speech.",
        },
        5:{
            "Eldritch Invocation: Agonizing Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, add your Charisma modifier to the damage it deals on a hit.",
            "Eldritch Invocation: Armor of Shadows":
                "You can cast mage armor on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Beast Speech":
                "You can cast speak with animals at will, without expending a spell slot.",
            "Eldritch Invocation: Beguiling Influence":
                "You gain proficiency in the Deception and Persuasion skills.",
            "Eldritch Invocation: Book of Ancient Secrets":
                "Prerequisite: Pact of the Tome feature\n\nYou can now inscribe magical rituals in your Book of Shadows. Choose two 1st-level spells that have the ritual tag from any class's spell list. The spells appear in the book and don't count against the number of spells you know. With your Book of Shadows in hand, you can cast the chosen spells as rituals. You can't cast the spells except as rituals, unless you've learned them by some other means. You can also cast a warlock spell you know as a ritual if it has the ritual tag.\n\nOn your adventures, you can add other ritual spells to your Book of Shadows. When you find such a spell, you can add it to the book if the spell's level is equal to or less than half your warlock level (rounded up) and if you can spare the time to transcribe the spell. For each level of the spell, the transcription process takes 2 hours and costs 50 gp for the rare inks needed to inscribe it.",
            "Eldritch Invocation: Devil's Sight":
                "You can see normally in darkness, both magical and nonmagical, to a distance of 120 feet.",
            "Eldritch Invocation: Eldritch Sight":
                "You can cast detect magic at will, without expending a spell slot.",
            "Eldritch Invocation: Eldritch Spear":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, its range is 300 feet.",
            "Eldritch Invocation: Eyes of the Rune Keeper":
                "You can read all writing.",
            "Eldritch Invocation: Fiendish Vigor":
                "You can cast false life on yourself at will as a 1st-level spell, without expending a spell slot or material components.",
            "Eldritch Invocation: Gaze of Two Minds":
                "You can use your action to touch a willing humanoid and perceive through its senses until the end of your next turn. As long as the creature is on the same plane of existence as you, you can use your action on subsequent turns to maintain this connection, extending the duration until the end of your next turn. While perceiving through the other creature's senses, you benefit from any special senses possessed by that creature, and you are blinded and deafened to your own surroundings.",
            "Eldritch Invocation: Mask of Many Faces":
                "You can cast disguise self at will, without expending a spell slot.",
            "Eldritch Invocation: Mire the Mind":
                "Prerequisite: 5th level\n\nYou can cast slow once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Misty Visions":
                "You can cast silent image at will, without expending a spell slot or material components.",
            "Eldritch Invocation: One with Shadows":
                "Prerequisite: 5th level\n\nWhen you are in an area of dim light or darkness, you can use your action to become invisible until you move or take an action or a reaction.",
            "Eldritch Invocation: Repelling Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you hit a creature with eldritch blast, you can push the creature up to 10 feet away from you in a straight line.",
            "Eldritch Invocation: Signs of Ill Omen":
                "Prerequisite: 5th level\n\nYou can cast bestow curse once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thief of Five Fates":
                "You can cast bane once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thirsting Blade":
                "Prerequisite: 5th level, Pact of the Blade feature\n\nYou can attack with your pact weapon twice, instead of once, whenever you take the Attack action on your turn.",
            "Eldritch Invocation: Voice of the Chain Master":
                "Prerequisite: Pact of the Chain feature\n\nYou can communicate telepathically with your familiar and perceive through your familiar's senses as long as you are on the same plane of existence. Additionally, while perceiving through your familiar's senses, you can also speak through your familiar in your own voice, even if your familiar is normally incapable of speech.",
        },
        7:{
            "Eldritch Invocation: Agonizing Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, add your Charisma modifier to the damage it deals on a hit.",
            "Eldritch Invocation: Armor of Shadows":
                "You can cast mage armor on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Beast Speech":
                "You can cast speak with animals at will, without expending a spell slot.",
            "Eldritch Invocation: Beguiling Influence":
                "You gain proficiency in the Deception and Persuasion skills.",
            "Eldritch Invocation: Bewitching Whispers":
                "Prerequisite: 7th level\n\nYou can cast compulsion once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Book of Ancient Secrets":
                "Prerequisite: Pact of the Tome feature\n\nYou can now inscribe magical rituals in your Book of Shadows. Choose two 1st-level spells that have the ritual tag from any class's spell list. The spells appear in the book and don't count against the number of spells you know. With your Book of Shadows in hand, you can cast the chosen spells as rituals. You can't cast the spells except as rituals, unless you've learned them by some other means. You can also cast a warlock spell you know as a ritual if it has the ritual tag.\n\nOn your adventures, you can add other ritual spells to your Book of Shadows. When you find such a spell, you can add it to the book if the spell's level is equal to or less than half your warlock level (rounded up) and if you can spare the time to transcribe the spell. For each level of the spell, the transcription process takes 2 hours and costs 50 gp for the rare inks needed to inscribe it.",
            "Eldritch Invocation: Devil's Sight":
                "You can see normally in darkness, both magical and nonmagical, to a distance of 120 feet.",
            "Eldritch Invocation: Dreadful Word":
                "Prerequisite: 7th level\n\nYou can cast confusion once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Eldritch Sight":
                "You can cast detect magic at will, without expending a spell slot.",
            "Eldritch Invocation: Eldritch Spear":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, its range is 300 feet.",
            "Eldritch Invocation: Eyes of the Rune Keeper":
                "You can read all writing.",
            "Eldritch Invocation: Fiendish Vigor":
                "You can cast false life on yourself at will as a 1st-level spell, without expending a spell slot or material components.",
            "Eldritch Invocation: Gaze of Two Minds":
                "You can use your action to touch a willing humanoid and perceive through its senses until the end of your next turn. As long as the creature is on the same plane of existence as you, you can use your action on subsequent turns to maintain this connection, extending the duration until the end of your next turn. While perceiving through the other creature's senses, you benefit from any special senses possessed by that creature, and you are blinded and deafened to your own surroundings.",
            "Eldritch Invocation: Mask of Many Faces":
                "You can cast disguise self at will, without expending a spell slot.",
            "Eldritch Invocation: Mire the Mind":
                "Prerequisite: 5th level\n\nYou can cast slow once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Misty Visions":
                "You can cast silent image at will, without expending a spell slot or material components.",
            "Eldritch Invocation: One with Shadows":
                "Prerequisite: 5th level\n\nWhen you are in an area of dim light or darkness, you can use your action to become invisible until you move or take an action or a reaction.",
            "Eldritch Invocation: Repelling Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you hit a creature with eldritch blast, you can push the creature up to 10 feet away from you in a straight line.",
            "Eldritch Invocation: Sculptor of Flesh":
                "Prerequisite: 7th level\n\nYou can cast polymorph once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Signs of Ill Omen":
                "Prerequisite: 5th level\n\nYou can cast bestow curse once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thief of Five Fates":
                "You can cast bane once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thirsting Blade":
                "Prerequisite: 5th level, Pact of the Blade feature\n\nYou can attack with your pact weapon twice, instead of once, whenever you take the Attack action on your turn.",
            "Eldritch Invocation: Voice of the Chain Master":
                "Prerequisite: Pact of the Chain feature\n\nYou can communicate telepathically with your familiar and perceive through your familiar's senses as long as you are on the same plane of existence. Additionally, while perceiving through your familiar's senses, you can also speak through your familiar in your own voice, even if your familiar is normally incapable of speech.",
        },
        9:{
            "Eldritch Invocation: Agonizing Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, add your Charisma modifier to the damage it deals on a hit.",
            "Eldritch Invocation: Armor of Shadows":
                "You can cast mage armor on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Ascendant Step":
                "Prerequisite: 9th level\n\nYou can cast levitate on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Beast Speech":
                "You can cast speak with animals at will, without expending a spell slot.",
            "Eldritch Invocation: Beguiling Influence":
                "You gain proficiency in the Deception and Persuasion skills.",
            "Eldritch Invocation: Bewitching Whispers":
                "Prerequisite: 7th level\n\nYou can cast compulsion once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Book of Ancient Secrets":
                "Prerequisite: Pact of the Tome feature\n\nYou can now inscribe magical rituals in your Book of Shadows. Choose two 1st-level spells that have the ritual tag from any class's spell list. The spells appear in the book and don't count against the number of spells you know. With your Book of Shadows in hand, you can cast the chosen spells as rituals. You can't cast the spells except as rituals, unless you've learned them by some other means. You can also cast a warlock spell you know as a ritual if it has the ritual tag.\n\nOn your adventures, you can add other ritual spells to your Book of Shadows. When you find such a spell, you can add it to the book if the spell's level is equal to or less than half your warlock level (rounded up) and if you can spare the time to transcribe the spell. For each level of the spell, the transcription process takes 2 hours and costs 50 gp for the rare inks needed to inscribe it.",
            "Eldritch Invocation: Devil's Sight":
                "You can see normally in darkness, both magical and nonmagical, to a distance of 120 feet.",
            "Eldritch Invocation: Dreadful Word":
                "Prerequisite: 7th level\n\nYou can cast confusion once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Eldritch Sight":
                "You can cast detect magic at will, without expending a spell slot.",
            "Eldritch Invocation: Eldritch Spear":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, its range is 300 feet.",
            "Eldritch Invocation: Eyes of the Rune Keeper":
                "You can read all writing.",
            "Eldritch Invocation: Fiendish Vigor":
                "You can cast false life on yourself at will as a 1st-level spell, without expending a spell slot or material components.",
            "Eldritch Invocation: Gaze of Two Minds":
                "You can use your action to touch a willing humanoid and perceive through its senses until the end of your next turn. As long as the creature is on the same plane of existence as you, you can use your action on subsequent turns to maintain this connection, extending the duration until the end of your next turn. While perceiving through the other creature's senses, you benefit from any special senses possessed by that creature, and you are blinded and deafened to your own surroundings.",
            "Eldritch Invocation: Mask of Many Faces":
                "You can cast disguise self at will, without expending a spell slot.",
            "Eldritch Invocation: Minions of Chaos":
                "Prerequisite: 9th level\n\nYou can cast conjure elemental once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Mire the Mind":
                "Prerequisite: 5th level\n\nYou can cast slow once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Misty Visions":
                "You can cast silent image at will, without expending a spell slot or material components.",
            "Eldritch Invocation: One with Shadows":
                "Prerequisite: 5th level\n\nWhen you are in an area of dim light or darkness, you can use your action to become invisible until you move or take an action or a reaction.",
            "Eldritch Invocation: Otherworldly Leap":
                "Prerequisite: 9th level\n\nYou can cast jump on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Repelling Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you hit a creature with eldritch blast, you can push the creature up to 10 feet away from you in a straight line.",
            "Eldritch Invocation: Sculptor of Flesh":
                "Prerequisite: 7th level\n\nYou can cast polymorph once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Signs of Ill Omen":
                "Prerequisite: 5th level\n\nYou can cast bestow curse once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thief of Five Fates":
                "You can cast bane once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thirsting Blade":
                "Prerequisite: 5th level, Pact of the Blade feature\n\nYou can attack with your pact weapon twice, instead of once, whenever you take the Attack action on your turn.",
            "Eldritch Invocation: Voice of the Chain Master":
                "Prerequisite: Pact of the Chain feature\n\nYou can communicate telepathically with your familiar and perceive through your familiar's senses as long as you are on the same plane of existence. Additionally, while perceiving through your familiar's senses, you can also speak through your familiar in your own voice, even if your familiar is normally incapable of speech.",
            "Eldritch Invocation: Whispers of the Grave":
                "Prerequisite: 9th level\n\nYou can cast speak with dead at will, without expending a spell slot.",
        },
        12:{
            "Eldritch Invocation: Agonizing Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, add your Charisma modifier to the damage it deals on a hit.",
            "Eldritch Invocation: Armor of Shadows":
                "You can cast mage armor on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Ascendant Step":
                "Prerequisite: 9th level\n\nYou can cast levitate on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Beast Speech":
                "You can cast speak with animals at will, without expending a spell slot.",
            "Eldritch Invocation: Beguiling Influence":
                "You gain proficiency in the Deception and Persuasion skills.",
            "Eldritch Invocation: Bewitching Whispers":
                "Prerequisite: 7th level\n\nYou can cast compulsion once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Book of Ancient Secrets":
                "Prerequisite: Pact of the Tome feature\n\nYou can now inscribe magical rituals in your Book of Shadows. Choose two 1st-level spells that have the ritual tag from any class's spell list. The spells appear in the book and don't count against the number of spells you know. With your Book of Shadows in hand, you can cast the chosen spells as rituals. You can't cast the spells except as rituals, unless you've learned them by some other means. You can also cast a warlock spell you know as a ritual if it has the ritual tag.\n\nOn your adventures, you can add other ritual spells to your Book of Shadows. When you find such a spell, you can add it to the book if the spell's level is equal to or less than half your warlock level (rounded up) and if you can spare the time to transcribe the spell. For each level of the spell, the transcription process takes 2 hours and costs 50 gp for the rare inks needed to inscribe it.",
            "Eldritch Invocation: Devil's Sight":
                "You can see normally in darkness, both magical and nonmagical, to a distance of 120 feet.",
            "Eldritch Invocation: Dreadful Word":
                "Prerequisite: 7th level\n\nYou can cast confusion once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Eldritch Sight":
                "You can cast detect magic at will, without expending a spell slot.",
            "Eldritch Invocation: Eldritch Spear":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, its range is 300 feet.",
            "Eldritch Invocation: Eyes of the Rune Keeper":
                "You can read all writing.",
            "Eldritch Invocation: Fiendish Vigor":
                "You can cast false life on yourself at will as a 1st-level spell, without expending a spell slot or material components.",
            "Eldritch Invocation: Gaze of Two Minds":
                "You can use your action to touch a willing humanoid and perceive through its senses until the end of your next turn. As long as the creature is on the same plane of existence as you, you can use your action on subsequent turns to maintain this connection, extending the duration until the end of your next turn. While perceiving through the other creature's senses, you benefit from any special senses possessed by that creature, and you are blinded and deafened to your own surroundings.",
            "Eldritch Invocation: Lifedrinker":
                "Prerequisite: 12th level\n\nWhen you hit a creature with your pact weapon, the creature takes extra necrotic damage equal to your Charisma modifier (minimum 1).",
            "Eldritch Invocation: Mask of Many Faces":
                "You can cast disguise self at will, without expending a spell slot.",
            "Eldritch Invocation: Minions of Chaos":
                "Prerequisite: 9th level\n\nYou can cast conjure elemental once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Mire the Mind":
                "Prerequisite: 5th level\n\nYou can cast slow once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Misty Visions":
                "You can cast silent image at will, without expending a spell slot or material components.",
            "Eldritch Invocation: One with Shadows":
                "Prerequisite: 5th level\n\nWhen you are in an area of dim light or darkness, you can use your action to become invisible until you move or take an action or a reaction.",
            "Eldritch Invocation: Otherworldly Leap":
                "Prerequisite: 9th level\n\nYou can cast jump on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Repelling Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you hit a creature with eldritch blast, you can push the creature up to 10 feet away from you in a straight line.",
            "Eldritch Invocation: Sculptor of Flesh":
                "Prerequisite: 7th level\n\nYou can cast polymorph once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Signs of Ill Omen":
                "Prerequisite: 5th level\n\nYou can cast bestow curse once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thief of Five Fates":
                "You can cast bane once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thirsting Blade":
                "Prerequisite: 5th level, Pact of the Blade feature\n\nYou can attack with your pact weapon twice, instead of once, whenever you take the Attack action on your turn.",
            "Eldritch Invocation: Voice of the Chain Master":
                "Prerequisite: Pact of the Chain feature\n\nYou can communicate telepathically with your familiar and perceive through your familiar's senses as long as you are on the same plane of existence. Additionally, while perceiving through your familiar's senses, you can also speak through your familiar in your own voice, even if your familiar is normally incapable of speech.",
            "Eldritch Invocation: Whispers of the Grave":
                "Prerequisite: 9th level\n\nYou can cast speak with dead at will, without expending a spell slot.",
        },
        15:{
            "Eldritch Invocation: Agonizing Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, add your Charisma modifier to the damage it deals on a hit.",
            "Eldritch Invocation: Armor of Shadows":
                "You can cast mage armor on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Ascendant Step":
                "Prerequisite: 9th level\n\nYou can cast levitate on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Beast Speech":
                "You can cast speak with animals at will, without expending a spell slot.",
            "Eldritch Invocation: Beguiling Influence":
                "You gain proficiency in the Deception and Persuasion skills.",
            "Eldritch Invocation: Bewitching Whispers":
                "Prerequisite: 7th level\n\nYou can cast compulsion once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Book of Ancient Secrets":
                "Prerequisite: Pact of the Tome feature\n\nYou can now inscribe magical rituals in your Book of Shadows. Choose two 1st-level spells that have the ritual tag from any class's spell list. The spells appear in the book and don't count against the number of spells you know. With your Book of Shadows in hand, you can cast the chosen spells as rituals. You can't cast the spells except as rituals, unless you've learned them by some other means. You can also cast a warlock spell you know as a ritual if it has the ritual tag.\n\nOn your adventures, you can add other ritual spells to your Book of Shadows. When you find such a spell, you can add it to the book if the spell's level is equal to or less than half your warlock level (rounded up) and if you can spare the time to transcribe the spell. For each level of the spell, the transcription process takes 2 hours and costs 50 gp for the rare inks needed to inscribe it.",
            "Eldritch Invocation: Chains of Carceri":
                "Prerequisite: 15th level, Pact of the Chain feature\n\nYou can cast hold monster at will—targeting a celestial, fiend, or elemental—without expending a spell slot or material components. You must finish a long rest before you can use this invocation on the same creature again.",
            "Eldritch Invocation: Devil's Sight":
                "You can see normally in darkness, both magical and nonmagical, to a distance of 120 feet.",
            "Eldritch Invocation: Dreadful Word":
                "Prerequisite: 7th level\n\nYou can cast confusion once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Eldritch Sight":
                "You can cast detect magic at will, without expending a spell slot.",
            "Eldritch Invocation: Eldritch Spear":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, its range is 300 feet.",
            "Eldritch Invocation: Eyes of the Rune Keeper":
                "You can read all writing.",
            "Eldritch Invocation: Fiendish Vigor":
                "You can cast false life on yourself at will as a 1st-level spell, without expending a spell slot or material components.",
            "Eldritch Invocation: Gaze of Two Minds":
                "You can use your action to touch a willing humanoid and perceive through its senses until the end of your next turn. As long as the creature is on the same plane of existence as you, you can use your action on subsequent turns to maintain this connection, extending the duration until the end of your next turn. While perceiving through the other creature's senses, you benefit from any special senses possessed by that creature, and you are blinded and deafened to your own surroundings.",
            "Eldritch Invocation: Lifedrinker":
                "Prerequisite: 12th level\n\nWhen you hit a creature with your pact weapon, the creature takes extra necrotic damage equal to your Charisma modifier (minimum 1).",
            "Eldritch Invocation: Mask of Many Faces":
                "You can cast disguise self at will, without expending a spell slot.",
            "Eldritch Invocation: Master of Myriad Forms":
                "Prerequisite: 15th level\n\nYou can cast alter self at will, without expending a spell slot.",
            "Eldritch Invocation: Minions of Chaos":
                "Prerequisite: 9th level\n\nYou can cast conjure elemental once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Mire the Mind":
                "Prerequisite: 5th level\n\nYou can cast slow once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Misty Visions":
                "You can cast silent image at will, without expending a spell slot or material components.",
            "Eldritch Invocation: One with Shadows":
                "Prerequisite: 5th level\n\nWhen you are in an area of dim light or darkness, you can use your action to become invisible until you move or take an action or a reaction.",
            "Eldritch Invocation: Otherworldly Leap":
                "Prerequisite: 9th level\n\nYou can cast jump on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Repelling Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you hit a creature with eldritch blast, you can push the creature up to 10 feet away from you in a straight line.",
            "Eldritch Invocation: Sculptor of Flesh":
                "Prerequisite: 7th level\n\nYou can cast polymorph once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Signs of Ill Omen":
                "Prerequisite: 5th level\n\nYou can cast bestow curse once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thief of Five Fates":
                "You can cast bane once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thirsting Blade":
                "Prerequisite: 5th level, Pact of the Blade feature\n\nYou can attack with your pact weapon twice, instead of once, whenever you take the Attack action on your turn.",
            "Eldritch Invocation: Visions of Distant Realms":
                "Prerequisite: 15th level\n\nYou can cast arcane eye at will, without expending a spell slot.",
            "Eldritch Invocation: Voice of the Chain Master":
                "Prerequisite: Pact of the Chain feature\n\nYou can communicate telepathically with your familiar and perceive through your familiar's senses as long as you are on the same plane of existence. Additionally, while perceiving through your familiar's senses, you can also speak through your familiar in your own voice, even if your familiar is normally incapable of speech.",
            "Eldritch Invocation: Whispers of the Grave":
                "Prerequisite: 9th level\n\nYou can cast speak with dead at will, without expending a spell slot.",
            "Eldritch Invocation: Witch Sight":
                "Prerequisite: 15th level\n\nYou can see the true form of any shapechanger or creature concealed by illusion or transmutation magic while the creature is within 30 feet of you and within line of sight.",
        },
        18:{
            "Eldritch Invocation: Agonizing Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, add your Charisma modifier to the damage it deals on a hit.",
            "Eldritch Invocation: Armor of Shadows":
                "You can cast mage armor on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Ascendant Step":
                "Prerequisite: 9th level\n\nYou can cast levitate on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Beast Speech":
                "You can cast speak with animals at will, without expending a spell slot.",
            "Eldritch Invocation: Beguiling Influence":
                "You gain proficiency in the Deception and Persuasion skills.",
            "Eldritch Invocation: Bewitching Whispers":
                "Prerequisite: 7th level\n\nYou can cast compulsion once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Book of Ancient Secrets":
                "Prerequisite: Pact of the Tome feature\n\nYou can now inscribe magical rituals in your Book of Shadows. Choose two 1st-level spells that have the ritual tag from any class's spell list. The spells appear in the book and don't count against the number of spells you know. With your Book of Shadows in hand, you can cast the chosen spells as rituals. You can't cast the spells except as rituals, unless you've learned them by some other means. You can also cast a warlock spell you know as a ritual if it has the ritual tag.\n\nOn your adventures, you can add other ritual spells to your Book of Shadows. When you find such a spell, you can add it to the book if the spell's level is equal to or less than half your warlock level (rounded up) and if you can spare the time to transcribe the spell. For each level of the spell, the transcription process takes 2 hours and costs 50 gp for the rare inks needed to inscribe it.",
            "Eldritch Invocation: Chains of Carceri":
                "Prerequisite: 15th level, Pact of the Chain feature\n\nYou can cast hold monster at will—targeting a celestial, fiend, or elemental—without expending a spell slot or material components. You must finish a long rest before you can use this invocation on the same creature again.",
            "Eldritch Invocation: Devil's Sight":
                "You can see normally in darkness, both magical and nonmagical, to a distance of 120 feet.",
            "Eldritch Invocation: Dreadful Word":
                "Prerequisite: 7th level\n\nYou can cast confusion once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Eldritch Sight":
                "You can cast detect magic at will, without expending a spell slot.",
            "Eldritch Invocation: Eldritch Spear":
                "Prerequisite: eldritch blast cantrip\n\nWhen you cast eldritch blast, its range is 300 feet.",
            "Eldritch Invocation: Eyes of the Rune Keeper":
                "You can read all writing.",
            "Eldritch Invocation: Fiendish Vigor":
                "You can cast false life on yourself at will as a 1st-level spell, without expending a spell slot or material components.",
            "Eldritch Invocation: Gaze of Two Minds":
                "You can use your action to touch a willing humanoid and perceive through its senses until the end of your next turn. As long as the creature is on the same plane of existence as you, you can use your action on subsequent turns to maintain this connection, extending the duration until the end of your next turn. While perceiving through the other creature's senses, you benefit from any special senses possessed by that creature, and you are blinded and deafened to your own surroundings.",
            "Eldritch Invocation: Lifedrinker":
                "Prerequisite: 12th level\n\nWhen you hit a creature with your pact weapon, the creature takes extra necrotic damage equal to your Charisma modifier (minimum 1).",
            "Eldritch Invocation: Mask of Many Faces":
                "You can cast disguise self at will, without expending a spell slot.",
            "Eldritch Invocation: Master of Myriad Forms":
                "Prerequisite: 15th level\n\nYou can cast alter self at will, without expending a spell slot.",
            "Eldritch Invocation: Minions of Chaos":
                "Prerequisite: 9th level\n\nYou can cast conjure elemental once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Mire the Mind":
                "Prerequisite: 5th level\n\nYou can cast slow once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Misty Visions":
                "You can cast silent image at will, without expending a spell slot or material components.",
            "Eldritch Invocation: One with Shadows":
                "Prerequisite: 5th level\n\nWhen you are in an area of dim light or darkness, you can use your action to become invisible until you move or take an action or a reaction.",
            "Eldritch Invocation: Otherworldly Leap":
                "Prerequisite: 9th level\n\nYou can cast jump on yourself at will, without expending a spell slot or material components.",
            "Eldritch Invocation: Repelling Blast":
                "Prerequisite: eldritch blast cantrip\n\nWhen you hit a creature with eldritch blast, you can push the creature up to 10 feet away from you in a straight line.",
            "Eldritch Invocation: Sculptor of Flesh":
                "Prerequisite: 7th level\n\nYou can cast polymorph once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Signs of Ill Omen":
                "Prerequisite: 5th level\n\nYou can cast bestow curse once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thief of Five Fates":
                "You can cast bane once using a warlock spell slot. You can't do so again until you finish a long rest.",
            "Eldritch Invocation: Thirsting Blade":
                "Prerequisite: 5th level, Pact of the Blade feature\n\nYou can attack with your pact weapon twice, instead of once, whenever you take the Attack action on your turn.",
            "Eldritch Invocation: Visions of Distant Realms":
                "Prerequisite: 15th level\n\nYou can cast arcane eye at will, without expending a spell slot.",
            "Eldritch Invocation: Voice of the Chain Master":
                "Prerequisite: Pact of the Chain feature\n\nYou can communicate telepathically with your familiar and perceive through your familiar's senses as long as you are on the same plane of existence. Additionally, while perceiving through your familiar's senses, you can also speak through your familiar in your own voice, even if your familiar is normally incapable of speech.",
            "Eldritch Invocation: Whispers of the Grave":
                "Prerequisite: 9th level\n\nYou can cast speak with dead at will, without expending a spell slot.",
            "Eldritch Invocation: Witch Sight":
                "Prerequisite: 15th level\n\nYou can see the true form of any shapechanger or creature concealed by illusion or transmutation magic while the creature is within 30 feet of you and within line of sight.",
        },
    },
),
Class(
    name: "Wizard",
    hit_die: 6,
    spellcasting_ability: "Intelligence",
    saves:(
        intelligence: Proficient,
        wisdom: Proficient,
    ),
    features:{
        1:{
            "Starting Proficiencies":
                "You are proficient with the following items, in addition to any proficiencies provided by your race or background.\n\nArmor: none\nWeapons: daggers, darts, slings, quarterstaffs, light crossbows\nTools: none\nSkills: Choose two from Arcana, History, Insight, Investigation, Medicine, and Religion",
            "Starting Equipment":
                "You start with the following items, plus anything provided by your background.\n\t• (a) a quarterstaff or (b) a dagger\n\t• (a) a component pouch or (b) an arcane focus\n\t• (a) a scholar's pack or (b) an explorer's pack\n\t• A spellbook\n\nAlternatively, you may start with 4d4 x 10 gp to buy your own equipment.",
            "Arcane Recovery":
                "You have learned to regain some of your magical energy by studying your spellbook. Once per day when you finish a short rest, you can choose expended spell slots to recover. The spell slots can have a combined level that is equal to or less than half your wizard level (rounded up), and none of the slots can be 6th level or higher.\n\nFor example, if you're a 4th-level wizard, you can recover up to two levels worth of spell slots.\n\nYou can recover either a 2nd-level spell slot or two 1st-level spell slots.",
            "Spellcasting":
                "As a student of arcane magic, you have a spellbook containing spells that show the first glimmerings of your true power. See chapter 10 for the general rules of spellcasting and chapter 11 for the wizard spell list.\n\nCantrips\nAt 1st level, you know three cantrips of your choice from the wizard spell list. You learn additional wizard cantrips of your choice at higher levels, as shown in the Cantrips Known column of the Wizard table.\n\nSpellbook\nAt 1st level, you have a spellbook containing six 1st-level wizard spells of your choice. Your spellbook is the repository of the wizard spells you know, except your cantrips, which are fixed in your mind.\n\nPreparing and Casting Spells\nThe Wizard table shows how many spell slots you have to cast your spells of 1st level and higher. To cast one of these spells, you must expend a slot of the spell's level or higher. You regain all expended spell slots when you finish a long rest.\n\nYou prepare the list of wizard spells that are available for you to cast. To do so, choose a number of wizard spells from your spellbook equal to your Intelligence modifier + your wizard level (minimum of one spell). The spells must be of a level for which you have spell slots.\n\nFor example, if you're a 3rd-level wizard, you have four 1st-level and two 2nd-level spell slots. With an Intelligence of 16, your list of prepared spells can include six spells of 1st or 2nd level, in any combination, chosen from your spellbook. If you prepare the 1st-level spell magic missile, you can cast it using a 1st-level or a 2nd-level slot. Casting the spell doesn't remove it from your list of prepared spells.\n\nYou can change your list of prepared spells when you finish a long rest. Preparing a new list of wizard spells requires time spent studying your spellbook and memorizing the incantations and gestures you must make to cast the spell: at least 1 minute per spell level for each spell on your list.\n\nSpellcasting Ability\nIntelligence is your spellcasting ability for your wizard spells, since you learn your spells through dedicated study and memorization. You use your Intelligence whenever a spell refers to your spellcasting ability. In addition, you use your Intelligence modifier when setting the saving throw DC for a wizard spell you cast and when making an attack roll with one.\n\n\tSpell save DC = 8 + your proficiency bonus + your Intelligence modifier\n\tSpell attack modifier = your proficiency bonus + your Intelligence modifier\n\nRitual Casting\nYou can cast a wizard spell as a ritual if that spell has the ritual tag and you have the spell in your spellbook. You don't need to have the spell prepared.\n\nSpellcasting Focus\nYou can use an arcane focus (found in chapter 5) as a spellcasting focus for your wizard spells.",
        },
        2:{
            "Arcane Tradition":
                "When you reach 2nd level, you choose an arcane tradition, shaping your practice of magic through one of eight schools: Abjuration, Bladesinging, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, or Transmutation, all detailed at the end of the class description. Your choice grants you features at 2nd level and again at 6th, 10th, and 14th level.",
            "Arcane Tradition: Evocation":
                "You focus your study on magic that creates powerful elemental effects such as bitter cold, searing flame, rolling thunder, crackling lightning, and burning acid. Some evokers find employment in military forces, serving as artillery to blast enemy armies from afar. Others use their spectacular power to protect the weak, while some seek their own gain as bandits, adventurers, or aspiring tyrants.",
            "School of Evocation: Evocation Savant":
                "Beginning when you select this school at 2nd level, the gold and time you must spend to copy an evocation spell into your spellbook is halved.",
            "School of Evocation: Sculpt Spells":
                "Beginning at 2nd level, you can create pockets of relative safety within the effects of your evocation spells. When you cast an evocation spell that affects other creatures that you can see, you can choose a number of them equal to 1+the spell's level. The chosen creatures automatically succeed on their saving throws against the spell, and they take no damage if they would normally take half damage on a successful save.",
        },
        4:{
            "Ability Score Improvement":
                "When you reach 4th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        6:{
            "School of Evocation: Potent Cantrip":
                "Starting at 6th level, your damaging cantrips affect even creatures that avoid the brunt of the effect. When a creature succeeds on a saving throw against your cantrip, the creature takes half the cantrip's damage (if any) but suffers no additional effect from the cantrip.",
        },
        8:{
            "Ability Score Improvement":
                "When you reach 8th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        10:{
            "School of Evocation: Empowered Evocation":
                "Beginning at 10th level, you can add your Intelligence modifier to one damage roll of any wizard evocation spell you cast.",
        },
        12:{
            "Ability Score Improvement":
                "When you reach 12th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        14:{
            "School of Evocation: Overchannel":
                "Starting at 14th level, you can increase the power of your simpler spells. When you cast a wizard spell of 1st through 5th level that deals damage, you can deal maximum damage with that spell.\n\nThe first time you do so, you suffer no adverse effect. If you use this feature again before you finish a long rest, you take 2d12 necrotic damage for each level of the spell, immediately after you cast it. Each time you use this feature again before finishing a long rest, the necrotic damage per spell level increases by 1d12. This damage ignores resistance and immunity.",
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        18:{
            "Spell Mastery":
                "At 18th level, you have achieved such mastery over certain spells that you can cast them at will. Choose a 1st-level wizard spell and a 2nd-level wizard spell that are in your spellbook. You can cast those spells at their lowest level without expending a spell slot when you have them prepared. If you want to cast either spell at a higher level, you must expend a spell slot as normal.\n\nBy spending 8 hours in study, you can exchange one or both of the spells you chose for different spells of the same levels.",
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat.",
        },
        20:{
            "Signature Spells":
                "When you reach 20th level, you gain mastery over two powerful spells and can cast them with little effort. Choose two 3rd-level wizard spells in your spellbook as your signature spells. You always have these spells prepared, they don't count against the number of spells you have prepared, and you can cast each of them once at 3rd level without expending a spell slot. When you do so, you can't do so again until you finish a short or long rest.\n\nIf you want to cast either spell at a higher level, you must expend a spell slot as normal.",
        },
    },
),
]"#;
