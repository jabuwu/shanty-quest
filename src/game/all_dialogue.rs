use crate::common::prelude::DialoguePortrait;

type P = DialoguePortrait;

pub const MUST_TALK_TO_MAYOR: [(P, &str); 1] = [(
    P::Jagerossa,
    "No no! Ya can't leave without finding where the Pirate Lord is!\nThe sea be vast, we'll be huntin' for 'im forever!",
)];

pub const MUST_TALK_TO_BARKEEP: [(P, &str); 1] = [(
    P::Jagerossa,
    "Yer a brave soul no doubt! But I advise a visit to the tavern over yonder,\nand stocking up on barrels o' rum! How else can a ship sail!?",
)];

pub const UPGRADE_MENU: [(P, &str); 1] = [(
    P::Jagerossa,
    "This here is how ya make yer ship and musicians better! Ye can't be the\nPirate King with a rottin' hulk untuned instrument, eh?",
)];

pub const JAGEROSSA1: [(P, &str); 2] = [
    (
        P::Jagerossa,
        "Ha-ha! Sailed right into me ambush ya bilge rat!\nI'll paint ya ship black with gunpowder!"
    ),
    (
        P::Jagerossa,
        "Then I'll take yer instrument from your scorched corpse!"
    ),
];

pub const JAGEROSSA2: [(P, &str); 3] = [
    (
        P::Jagerossa,
        "Well! Ya can't always get what you want... But wait, don't kill me yet!",
    ),
    (
        P::Jagerossa,
"Have some sympathy fer me, poor devil...\nHow about we combine our powers?! Ha?\nWith 2 instruments, yer ship we'll be unstoppable!"
    ),
    (
        P::Jagerossa,
        "Other Pirate Lords will scatter like tumblin' dice before our\ncombined might!\nSet sail, onwards! We need to find a town."
    )
];

pub const JAGEROSSA_AFTER_VICTORY: [(P, &str); 2] = [
    (
        P::Jagerossa,
        "Har! Ya got what ya wanted! Now yer can sail the high seas as the\nPirate King!",
    ),
    (
        P::Jagerossa,
        "But the question is... How long 'til the Royal Navy tries to take the\ncolonies back?",
    ),
];

pub const DANGEROUS_SEAS: [(P, &str); 1] = [(
    P::Jagerossa,
    "These seas be dangerous!\nOnly a fool would traverse them without a map!\n(Press M to open map)",
)];

pub const RINGO_MAYOR: [(P, &str); 4] = [
    (
        P::Mayor,
        "Ah! The account of your triumph precedes you, Pirate Lord! The town\nof Portallica celebrates the defeat of your rival!"
    ),
    (
        P::Mayor,
        "Now that I have your attention... How about you conquer your other\nrivals, o mighty Pirate Lord? It would do wonders for my purse...\nI mean, for trade! Trade!"
    ),
    (
        P::Mayor,
        "Just imagine it! With all the other Lords defeated, all their instruments\ncombined... Why! You could be Pirate King!"
    ),
    (
        P::Mayor,
        "Luckily for you, I know where that bastard Ringo Yarr has set anchor...\nI'll provide you with his location, if you promise to remember your good\nfriend. Quid pro quo, as Latins say!"
    )
];

pub const RINGO1: [(P, &str); 1] = [(
    P::Ringo,
    "Here ye are, sailin' helter-skelter right into me guns! Ha!",
)];

pub const RINGO2: [(P, &str); 2] = [
    (
        P::Ringo,
        "Ach! I should've known better!",
    ),
    (
        P::Ringo,
        "How about we just let it be, eh? Forgive our past grievances. Think for\nyerself! I'll give ya my instrument and you can combine all their powers!",
    )
];

pub const PLANK_MAYOR: [(P, &str); 3] = [
    (
        P::Mayor,
        "Oh, Pirate Lord! My humble town celebrates your arrival..."
    ),
    (
        P::Mayor,
        "Of course, of course. I will get straight into business!\nHere is where Captain Plank Presley has set anchor!"
    ),
    (
        P::Mayor,
        "Beware of his dashing hair and wicked dance moves!\nAnd... do remember your good friends, the Genes!"
    )
];

pub const PLANK1: [(P, &str); 1] = [(
    P::Plank,
    "Yaar! It's now or never! All yer instruments will be mine!",
)];

pub const PLANK2: [(P, &str); 1] = [
    (
        P::Plank,
        "Huh... I'm all shook up! Come on, don't be cruel! My instrument fer me\nlife, a fair accord! You can combine their powers... And just let me be!",
    ),
];

pub const DAVY_MAYOR: [(P, &str); 3] = [
    (
        P::Mayor,
        "Welcome, welcome Pirate Lord! Or, dare I say it? Pirate King!\nYour achievements know no bounds."
    ),
    (
        P::Mayor,
        "Only one Lord left, the maniac Captain Davy Bowie!\nHis ship is anchored nearby. Beware of his many-colored eyes!\nIt is said they can turn you to stone!"
    ),
    (
        P::Mayor,
        "Once he's... dispatched, and his instrument combined with your\norchestra... Then we can do business!"
    )
];

pub const DAVY1: [(P, &str); 1] = [(
    P::Davy,
    "Oh! Bring me the self-proclaimed Pirate King! Let's dance, ya bilge rat!",
)];

pub const DAVY2: [(P, &str); 2] = [
    (
        P::Davy,
        "My... My golden years must be behind me! Well. Ashes to ashes.\nMy instrument fer my life!",
    ),
    (P::Davy, "Yer truly are the Pirate King..."),
];

pub const MAYOR_RANDOM1: [(P, &str); 1] = [(
    P::Mayor,
    "Of course I am a spitting image of the other Governor!\nWhy, we're identical siblings!",
)];

pub const MAYOR_RANDOM2: [(P, &str); 1] =
    [(P::Mayor, "When you're the Pirate King, trade will flow!")];

pub const MAYOR_RANDOM3: [(P, &str); 1] = [(
    P::Mayor,
    "Darn, darn, darn! Look at these accounts!\nWe'll be ruined if the Pirate Lords aren't... Oh, hello!",
)];

pub const MAYOR_RANDOM4: [(P, &str); 1] = [(P::Mayor, "Ah, tax counting day. My favorite!")];

pub const MAYOR_RANDOM5: [(P, &str); 1] = [(
    P::Mayor,
    "Where is that rumship? Townsfolk are getting restless...",
)];

pub const MAYOR_AFTER_VICTORY1: [(P, &str); 1] =
    [(P::Mayor, "Oh, Pirate King! How can the Genes serve you?")];

pub const MAYOR_AFTER_VICTORY2: [(P, &str); 1] = [(P::Mayor, "Enjoying your reign, Pirate King?")];

pub const MAYOR_AFTER_VICTORY3: [(P, &str); 1] = [(
    P::Mayor,
    "Me? Of course I haven't sent that intercepted missive meant for the\nRoyal Navy!",
)];

pub const MAYOR_AFTER_VICTORY4: [(P, &str); 1] = [(
    P::Mayor,
    "Ah, yes! The trade is blossoming again. But someone has to do\nsomething about those pesky sea monsters!",
)];

pub const MAYOR_AFTER_VICTORY5: [(P, &str); 1] = [(P::Mayor, "Pirate King! Welcome to our town.")];

pub const MAYOR_AFTER_VICTORY6: [(P, &str); 1] = [(
    P::Mayor,
    "Yes, oh yes! The trade flows. Rum, sugar, cotton!",
)];

pub const BARKEEP1: [(P, &str); 3] = [
    (P::Barkeep, "Eh? Piss off scoundrel! We have no more rum..."),
    (
        P::Barkeep,
        "Oh... Thousand apologies, Pirate Lord!\nBoy! Fetch ye the best caskets of rum!",
    ),
    (
        P::Barkeep,
        "Not that, stupid! That's bilge water! The best caskets I said!",
    ),
];

pub const BARKEEP_RANDOM1: [(P, &str); 1] = [(
    P::Barkeep,
    "The more I clean this glass the dirtier it gets...",
)];

pub const BARKEEP_RANDOM2: [(P, &str); 1] = [(
    P::Barkeep,
    "Should've chosen a trade with real retirement options.",
)];

pub const BARKEEP_RANDOM3: [(P, &str); 1] = [(
    P::Barkeep,
    "Boy! Are those rats-on-sticks done?! Guests be waiting!",
)];

pub const BARKEEP_RANDOM4: [(P, &str); 1] = [(
    P::Barkeep,
    "Water?! Ye want to drink water?! That's fer washing, not drinking! Out!",
)];

pub const BARKEEP_RANDOM5: [(P, &str); 1] = [(
    P::Barkeep,
    "Dirty water is a copper, clean water is a silver, jug o' rum is two silver!",
)];

pub const BARKEEP_RANDOM6: [(P, &str); 1] =
    [(P::Barkeep, "Eh. Should've attracted better clientele...")];
