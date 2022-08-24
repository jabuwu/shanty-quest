use crate::common::prelude::DialoguePortrait;

type P = DialoguePortrait;

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
