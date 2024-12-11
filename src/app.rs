use rand::Rng;

pub enum MenuSelection {
    Facts,
    About,
}

pub struct App {
    pub fact: String,
    pub menu_selection: MenuSelection,
}

impl App {
    pub fn new() -> App {
        Self {
            fact: get_random_fact(),
            menu_selection: MenuSelection::Facts,
        }
    }
}

const FACTS_BANK: &[&str] = &[
    "Norwood? More like Nandwood",
    "Did you know Kris is an endangered species?",
    "Yeah I can't go to lunch, I just got here, can you bring me something back tho?",
    "I'm not gnome like though, more dwarf without the beard",
    "Kris enjoys: cat'ing out logs in his terminal to look busy.",
    "Kris is a good man",
    "Hold my diapers!",
    "As Krispy as a Krispy Cracker",
    "The body does not turn because the face is not confident'",
    "Woooo!",
    "I wish I was on Team Vector!",
    "Rest in peps",
    "You guys don't?",
    "How often do men throw money at me? Often enough.",
    "I'm rebooting the server guys give me a second.",
    "Don't talk about my body, I don't have any armor.",
    "That's too bad.",
    "Bruuuuuh",
    "I love Gabe. He has Cooper's hair.",
    "I don't like almonds, but I'm a fan of the almond brothers.",
    "That's a good point, I didn't think about that.",
    "That's fair.",
    "Ahhh yeah that's right!",
    "Interesting...",
    "YIP",
    "I don't pay attention to me.",
    "Sorry, I was moonlighting.",
    "Yes, that's the joke.",
    "I agree with that.",
    "Who wants to go to Oily Mikes?",
    "Jacob is a good man.",
    "Cooper is a good man.",
    "My mouth has never been so full.",
    "Jacob is like me, but with pants.",
    "I've got the Fuman, I just need 'Chu.",
];

pub fn get_random_fact() -> String {
    let len = FACTS_BANK.len();
    let index = rand::thread_rng().gen_range(0..len);
    let chosen_fact = FACTS_BANK.get(index);
    if let Some(fact) = chosen_fact {
        return String::from(*fact);
    } else {
        return String::from("Fact not found :cry:");
    }
}
