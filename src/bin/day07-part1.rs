use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Ord, Hash, Clone, Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn value(&self) -> u8 {
        match self {
            Self::Ace => 14,
            Self::King => 13,
            Self::Queen => 12,
            Self::Jack => 11,
            Self::Ten => 10,
            Self::Nine => 9,
            Self::Eight => 8,
            Self::Seven => 7,
            Self::Six => 6,
            Self::Five => 5,
            Self::Four => 4,
            Self::Three => 3,
            Self::Two => 2,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(anyhow!("Invalid card char: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, Hash)]
enum Hand {
    /// All cards are the same
    FiveOfAKind(Card),

    /// Four cards are the same and the fifth is different
    ///
    /// The first card is the four, the second is the other.
    FourOfAKind(Card, Card),

    /// Three cards are the same and the other two are the same
    ///
    /// The first card is the three, the second is the two.
    FullHouse(Card, Card),

    /// Three cards are the same and the other two are different
    ///
    /// The first card is the three, the second is the first other,
    /// the third is the second other.
    ThreeOfAKind(Card, Card, Card),

    /// Two pairs of two, plus one other card
    ///
    /// The first card is the first pair, the second is the second
    /// pair, the third is the other.
    TwoPair(Card, Card, Card),

    /// Two cards are the same and the other three are different.
    ///
    /// The first card is the pair, the other three are the rest.
    OnePair(Card, Card, Card, Card),

    /// All the cards are different.
    ///
    /// The cards are sorted from highest to lowest.
    HighCard(Card, Card, Card, Card, Card),
}

impl TryFrom<Vec<Card>> for Hand {
    type Error = anyhow::Error;

    fn try_from(cards: Vec<Card>) -> Result<Self> {
        // Check that the length is what's expected...
        if cards.len() != 5 {
            return Err(anyhow!("Invalid number of cards: {}", cards.len()));
        }

        // Sort the cards from highest to lowest...
        let mut cards = cards;
        cards.sort_by(|a, b| b.cmp(a));

        // Group the cards...
        let grouped = cards
            .iter()
            .fold(HashMap::<Card, usize>::new(), |mut acc, c| {
                *acc.entry(*c).or_insert(0) += 1;
                acc
            });

        // Check for five of a kind...
        if grouped.len() == 1 {
            let first = cards.get(0).unwrap(); // We know there are 5 cards
            return Ok(Self::FiveOfAKind(*first));
        }

        // Check for four of a kind or full house...
        if grouped.len() == 2 {
            // Get the two card types...
            let first = cards.get(0).unwrap(); // We know there are 5 cards
            let second = cards.get(4).unwrap(); // We know there are 5 cards

            // Check for four of a kind...
            if grouped.get(first).unwrap() == &4 {
                return Ok(Self::FourOfAKind(*first, *second));
            }
            if grouped.get(second).unwrap() == &4 {
                return Ok(Self::FourOfAKind(*second, *first));
            }

            // Otherwise, it's a full house. Check the order...
            if grouped.get(first).unwrap() == &3 {
                return Ok(Self::FullHouse(*first, *second));
            }
            if grouped.get(second).unwrap() == &3 {
                return Ok(Self::FullHouse(*second, *first));
            }
            return Err(anyhow!("Unexpected hand: {:?}", grouped));
        }

        // Check for three of a kind or two pair...
        if grouped.len() == 3 {
            // Is it three of a kind?
            if grouped.iter().find(|(_, count)| *count == &3).is_some() {
                // Find the three...
                let three = grouped
                    .iter()
                    .find(|(_, count)| *count == &3)
                    .ok_or(anyhow!("No three"))?
                    .0;

                // Get the rest of the cards...
                let mut rest = grouped
                    .iter()
                    .filter(|(_, count)| *count == &1)
                    .map(|(card, _)| *card)
                    .collect::<Vec<_>>();
                rest.sort_by(|a, b| b.cmp(a));

                // Return the hand...
                return Ok(Self::ThreeOfAKind(
                    *three,
                    *rest.get(0).unwrap(),
                    *rest.get(1).unwrap(),
                ));
            }

            // Otherwise, it must be two pair...
            // Find the pairs...
            let first = grouped
                .iter()
                .find(|(_, count)| *count == &2)
                .ok_or(anyhow!("No first pair"))?
                .0;
            let second = grouped
                .iter()
                .find(|(_, count)| *count == &2)
                .ok_or(anyhow!("No second pair"))?
                .0;

            // Sort the pairs...
            let (first, second) = if first > second {
                (first, second)
            } else {
                (second, first)
            };

            // Get the last cards...
            let last = grouped
                .iter()
                .find(|(_, count)| *count == &1)
                .ok_or(anyhow!("No last card"))?
                .0;

            // Create the hand and return...
            return Ok(Self::TwoPair(*first, *second, *last));
        }

        // Check for one pair...
        if grouped.len() == 3 {
            // Find the pair...
            let pair = grouped
                .iter()
                .find(|(_, count)| *count == &2)
                .ok_or(anyhow!("No pair"))?
                .0;

            // Get the rest of the cards...
            let mut rest = grouped
                .iter()
                .filter(|(_, count)| *count == &1)
                .map(|(card, _)| *card)
                .collect::<Vec<_>>();
            rest.sort_by(|a, b| b.cmp(a));

            // Return the hand...
            return Ok(Self::OnePair(
                *pair,
                *rest.get(0).unwrap(),
                *rest.get(1).unwrap(),
                *rest.get(2).unwrap(),
            ));
        }

        // Otherwise, it must be high card...
        Ok(Self::HighCard(
            *cards.get(0).unwrap(),
            *cards.get(1).unwrap(),
            *cards.get(2).unwrap(),
            *cards.get(3).unwrap(),
            *cards.get(4).unwrap(),
        ))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = match self {
            Hand::FiveOfAKind(h1) => match other {
                Hand::FiveOfAKind(h2) => h1.cmp(h2),
                _ => Ordering::Greater,
            },
            Hand::FourOfAKind(h1four, h1other) => match other {
                Hand::FiveOfAKind(_) => Ordering::Less,
                Hand::FourOfAKind(h2four, h2other) => match h1four.cmp(h2four) {
                    Ordering::Equal => h1other.cmp(h2other),
                    other => other,
                },
                _ => Ordering::Greater,
            },
            Hand::FullHouse(h1trip, h1pair) => match other {
                Hand::FiveOfAKind(_) => Ordering::Less,
                Hand::FourOfAKind(_, _) => Ordering::Less,
                Hand::FullHouse(h2trip, h2pair) => match h1trip.cmp(h2trip) {
                    Ordering::Equal => h1pair.cmp(h2pair),
                    other => other,
                },
                _ => Ordering::Greater,
            },
            Hand::ThreeOfAKind(h1trip, h1other1, h1other2) => match other {
                Hand::FiveOfAKind(_) => Ordering::Less,
                Hand::FourOfAKind(_, _) => Ordering::Less,
                Hand::FullHouse(_, _) => Ordering::Less,
                Hand::ThreeOfAKind(h2trip, h2other1, h2other2) => match h1trip.cmp(h2trip) {
                    Ordering::Equal => match h1other1.cmp(h2other1) {
                        Ordering::Equal => h1other2.cmp(h2other2),
                        other => other,
                    },
                    other => other,
                },
                _ => Ordering::Greater,
            },
            Hand::TwoPair(h1pair1, h1pair2, h1other) => match other {
                Hand::TwoPair(h2pair1, h2pair2, h2other) => match h1pair1.cmp(h2pair1) {
                    Ordering::Equal => match h1pair2.cmp(h2pair2) {
                        Ordering::Equal => h1other.cmp(h2other),
                        other => other,
                    },
                    other => other,
                },
                Hand::HighCard(_, _, _, _, _) => Ordering::Greater,
                _ => Ordering::Less,
            },
            Hand::OnePair(h1pair, h1other1, h1other2, h1other3) => match other {
                Hand::OnePair(h2pair, h2other1, h2other2, h2other3) => match h1pair.cmp(h2pair) {
                    Ordering::Equal => match h1other1.cmp(h2other1) {
                        Ordering::Equal => match h1other2.cmp(h2other2) {
                            Ordering::Equal => h1other3.cmp(h2other3),
                            other => other,
                        },
                        other => other,
                    },
                    other => other,
                },
                Hand::HighCard(_, _, _, _, _) => Ordering::Greater,
                _ => Ordering::Less,
            },
            Hand::HighCard(h1c1, h1c2, h1c3, h1c4, h1c5) => match other {
                Hand::HighCard(h2c1, h2c2, h2c3, h2c4, h2c5) => match h1c1.cmp(h2c1) {
                    Ordering::Equal => match h1c2.cmp(h2c2) {
                        Ordering::Equal => match h1c3.cmp(h2c3) {
                            Ordering::Equal => match h1c4.cmp(h2c4) {
                                Ordering::Equal => h1c5.cmp(h2c5),
                                other => other,
                            },
                            other => other,
                        },
                        other => other,
                    },
                    other => other,
                },
                _ => Ordering::Less,
            },
        };
        Some(res)
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct HandAndBid {
    cards: String,
    hand: Hand,
    bid: usize,
}

impl HandAndBid {
    fn parse(line: &str) -> Result<Self> {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let cards = parts
            .get(0)
            .ok_or(anyhow!("No hand"))?
            .to_string();
        let mut hand = parts
            .get(0)
            .ok_or(anyhow!("No hand"))?
            .chars()
            .map(|c| Card::try_from(c))
            .collect::<Result<Vec<_>>>()?;
        hand.sort_by(|a, b| b.cmp(a));
        let hand = Hand::try_from(hand)?;
        let bid = parts
            .get(1)
            .ok_or(anyhow!("No bid"))?
            .parse::<usize>()
            .map_err(anyhow::Error::from)?;
        Ok(Self { cards, hand, bid })
    }
}

impl PartialOrd for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cards.partial_cmp(&other.cards) {
            Some(Ordering::Equal) => self.bid.partial_cmp(&other.bid),
            other => other,
        }
    }
}

fn main() -> Result<()> {
    let input_lines = load_input_lines_by_name(file!())?;
    todo!();
}
