use ron;
use crate::compendium::Class;

pub fn get_classes() -> Vec<Class> {
    ron::from_str::<Vec<Class>>(SRD_CLASSES).unwrap()
}

static SRD_CLASSES: &str = r#"[
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
                "You now roll two additional weapon damage dice when determining the extra damage for a critical hit with a melee attack."
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
                "At 20th level, there is no limit to the number of times you may r,age."
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
                "At 15th level, your Bardic Inspiration die changes to a d12."
        },
        16:{
            "Ability Score Improvement":
                "When you reach 16th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat."
        },
        17:{
            "Song of Rest":
                "At 17th level, the extra hit points gained from Song of Rest increases to 1d12.",
        },
        18:{
            "Magical Secrets":
                "At 18th level, choose two additional spells from any class, including this one. A spell you choose must be of a level you can cast, as shown on the Bard table, or a cantrip.\n\nThe chosen spells count as bard spells for you and are included in the number in the Spells Known column of the Bard table."
        },
        19:{
            "Ability Score Improvement":
                "When you reach 19th level you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.\n\nIf your DM allows the use of feats, you may instead take a feat."
        },
        20:{
            "Superior Inspiration":
                "At 20th level, when you roll initiative and have no uses of Bardic Inspiration left, you regain one use."
        },
    },
),
]"#;