use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {

        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = [
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
        ];

        let mut cards = vec![];


        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        let deck = Deck { cards };

        return deck;
    }

    fn shuffle(&mut self) {

        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, numsize: usize) -> Vec<String> {

        self.cards.split_off(self.cards.len() - numsize)

    }
}

fn main() {

    let mut deck = Deck::new();

    // deck.shuffle();

    let cards = deck.deal(5);

    println!("Heres your deck: {:#?}", deck);
    println!("Heres your hand: {:#?}", cards);
}
