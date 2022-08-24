use crate::common::prelude::DialoguePortrait;

type P = DialoguePortrait;

pub const MUST_TALK_TO_MAYOR: [(P, &str); 1] = [(
    P::Jagerossa,
    "No no! Ya can't leave without finding where the Pirate Lord is!\nThe sea be vast, we'll be huntin' for 'im forever!",
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

pub const DANGEROUS_SEAS: [(P, &str); 1] = [(
    P::Jagerossa,
    "These seas be dangerous!\nOnly a fool would traverse them without a map!",
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
