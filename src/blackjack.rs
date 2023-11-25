use rand::seq::SliceRandom;

enum Suit {
    Hearts       = 0,
    Clubs        = 1,
    Diamonds     = 2,
    Spades       = 3
}

impl Suit {
    fn from_i32(n: i32) -> Option<Suit> {
        match n {
            0 => Some(Suit::Hearts),
            1 => Some(Suit::Clubs),
            2 => Some(Suit::Diamonds),
            3 => Some(Suit::Spades),
            _ => None,
        }
    }

    fn to_string(s: &Suit) -> String {
        match s {
            Suit::Hearts    => "♥".to_string(),
            Suit::Clubs     => "♣".to_string(),
            Suit::Diamonds  => "♦".to_string(),
            Suit::Spades    => "♠".to_string(),
        }
    }
}

struct Card {
    rank: i32,
    suit: Suit,
}

impl Card {
    fn to_string(&self) -> String {
        Suit::to_string(&self.suit) + &self.rank.to_string()
    }
}

fn generate_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = vec!();

    for s in 0..=3 {
        for r in 1..=13 {
            let suit = Suit::from_i32(s);
            if suit.is_none() {
                continue;
            }

            deck.push(Card {
                rank: r,
                suit: suit.unwrap(),
            });
        }
    }

    deck
}

fn shuffle_deck(deck: &mut Vec<Card>) -> () {
    let mut rng = rand::thread_rng();

    deck.shuffle(&mut rng);
    println!("{}", deck[0].to_string());
}

pub fn play() {
    let mut deck = generate_deck();
    shuffle_deck(&mut deck);
}
