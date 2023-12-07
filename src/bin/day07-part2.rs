use anyhow::{anyhow, Result};
use aoc23::load_input_lines_by_name;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Ord, Hash, Clone, Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn value(&self) -> u8 {
        match self {
            Self::Ace => 14,
            Self::King => 13,
            Self::Queen => 12,
            Self::Ten => 10,
            Self::Nine => 9,
            Self::Eight => 8,
            Self::Seven => 7,
            Self::Six => 6,
            Self::Five => 5,
            Self::Four => 4,
            Self::Three => 3,
            Self::Two => 2,
            Self::Joker => 1,
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
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            'J' => Ok(Self::Joker),
            _ => Err(anyhow!("Invalid card char: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, Hash)]
enum Hand {
    /// All cards are the same
    FiveOfAKind,

    /// Four cards are the same and the fifth is different
    ///
    /// The first card is the four, the second is the other.
    FourOfAKind,

    /// Three cards are the same and the other two are the same
    ///
    /// The first card is the three, the second is the two.
    FullHouse,

    /// Three cards are the same and the other two are different
    ///
    /// The first card is the three, the second is the first other,
    /// the third is the second other.
    ThreeOfAKind,

    /// Two pairs of two, plus one other card
    ///
    /// The first card is the first pair, the second is the second
    /// pair, the third is the other.
    TwoPair,

    /// Two cards are the same and the other three are different.
    ///
    /// The first card is the pair, the other three are the rest.
    OnePair,

    /// All the cards are different.
    ///
    /// The cards are sorted from highest to lowest.
    HighCard,
}

impl Hand {
    fn rank(&self) -> usize {
        match self {
            Self::FiveOfAKind => 7,
            Self::FourOfAKind => 6,
            Self::FullHouse => 5,
            Self::ThreeOfAKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
        }
    }
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
        let mut grouped = cards
            .iter()
            .fold(HashMap::<Card, usize>::new(), |mut acc, c| {
                *acc.entry(*c).or_insert(0) += 1;
                acc
            });

        // Take the jokers out and count them...
        let joker_count = grouped.remove(&Card::Joker).unwrap_or(0);
        
        // Count everything else...
        let n_five = grouped.iter().filter(|(_, count)| *count == &5).count();
        let n_four = grouped.iter().filter(|(_, count)| *count == &4).count();
        let n_three = grouped.iter().filter(|(_, count)| *count == &3).count();
        let n_two = grouped.iter().filter(|(_, count)| *count == &2).count();
        let n_one = grouped.iter().filter(|(_, count)| *count == &1).count();

        // If all jokers, it's five of a kind...
        if joker_count == 5 
            || n_five == 1
            || (n_four == 1 && joker_count == 1)
            || (n_three == 1 && joker_count == 2)
            || (n_two == 1 && joker_count == 3)
            || (n_one == 1 && joker_count == 4)
        {
            return Ok(Self::FiveOfAKind);
        }

        // Can we make four of a kind?
        if n_four == 1 
            || joker_count == 4
            || (n_three >= 1 && joker_count >= 1)
            || (n_two >= 1 && joker_count >= 2)
            || (n_one >= 1 && joker_count >= 3)
        {
            return Ok(Self::FourOfAKind);
        }

        // Can we make a full house?
        if (n_three == 1 && n_two == 1)
            || (n_three >= 1 && n_one >= 1 && joker_count >= 1)
            || (n_three >= 1 && joker_count >= 2)
            || (n_two >= 2 && joker_count >= 1)
            || (n_two >= 1 && n_one >= 1 && joker_count >= 2)
            || (n_two >= 1 && joker_count >= 3)
            || (n_one >= 2 && joker_count >= 3)
        {
            return Ok(Self::FullHouse);
        }
        
        // Can we make three of a kind?
        if n_three == 1 
            || (n_two >= 1 && joker_count >= 1)
            || (n_one >= 1 && joker_count >= 2)
            || joker_count >= 3
        {
            // Otherwise, it's three of a kind...
            return Ok(Self::ThreeOfAKind);
        }

        // Can we make two pair?
        if n_two >= 2
            || (n_two >= 1 && n_one >= 1 && joker_count >= 1)
            || (n_two >= 1 && joker_count >= 2)
            || (n_one >= 2 && joker_count >= 2)
            || (n_one >= 1 && joker_count >= 3)
            || joker_count >= 4
        {
            return Ok(Self::TwoPair);
        }


        // Can we make one pair?
        if n_two >= 1
            || (n_one >= 1 && joker_count >= 1)
            || joker_count >= 2
        {
            return Ok(Self::OnePair);
        }

        // Otherwise, it's high card...
        Ok(Self::HighCard)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rank().cmp(&other.rank()))
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
        // Orders by hand type (e.g. full house, two pair, etc.) first,
        // then by the cards in the hand...
        match self.hand.partial_cmp(&other.hand) {
            Some(Ordering::Equal) => {
                // Iterate through the cards zipped together...
                for (left, right) in self.cards.chars().zip(other.cards.chars()) {
                    // If the cards are equal, continue...
                    if left == right {
                        continue;
                    }
                    
                    // Otherwise, convert the cards to Card and compare...
                    let left = Card::try_from(left).ok()?;
                    let right = Card::try_from(right).ok()?;
                    return left.partial_cmp(&right);
                }
                return Some(Ordering::Equal);
            },
            other => other,
        }
    }
}

fn main() -> Result<()> {
    // Parse the input as lines...
    let input_lines = load_input_lines_by_name(file!())?;
    // let input_lines = vec![
    //     "32T3K 765".to_string(),
    //     "T55J5 684".to_string(),
    //     "KK677 28".to_string(),
    //     "KTJJT 220".to_string(),
    //     "QQQJA 483".to_string(),
    // ];
    let mut hands = input_lines
        .into_iter()
        .map(|line| HandAndBid::parse(&line))
        .collect::<Result<Vec<_>>>()?;
    hands.sort();

    // Get the score...
    let score = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum::<usize>();
    println!("Score: {}", score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_parse() -> Result<()> {
        assert_eq!(Card::try_from('A')?, Card::Ace);
        assert_eq!(Card::try_from('J')?, Card::Joker);
        assert_eq!(Card::try_from('K')?, Card::King);
        assert_eq!(Card::try_from('T')?, Card::Ten);
        assert_eq!(Card::try_from('5')?, Card::Five);
        assert_eq!(Card::try_from('2')?, Card::Two);
        Ok(())
    }

    #[test]
    fn test_card_order() -> Result<()> {
        assert!(Card::Ace > Card::King);
        assert!(Card::Ace == Card::Ace);
        assert!(Card::Ten < Card::King);
        assert!(Card::Joker < Card::Ten);
        assert!(Card::Queen > Card::Joker);
        Ok(())
    }

    
    #[test]
    fn test_hand_rank() -> Result<()> {
        assert!(Hand::FiveOfAKind.rank() > Hand::FourOfAKind.rank());
        assert!(Hand::FourOfAKind.rank() > Hand::FullHouse.rank());
        assert!(Hand::FullHouse.rank() > Hand::ThreeOfAKind.rank());
        assert!(Hand::ThreeOfAKind.rank() > Hand::TwoPair.rank());
        assert!(Hand::TwoPair.rank() > Hand::OnePair.rank());
        assert!(Hand::OnePair.rank() > Hand::HighCard.rank());
        assert!(Hand::HighCard == Hand::HighCard);
        Ok(())
    }

    #[test]
    fn test_hand_parse() -> Result<()> {
        assert_eq!(
            Hand::try_from(vec![
                Card::Ace,
                Card::Ace,
                Card::Ace,
                Card::Ace,
                Card::Ace
            ])?,
            Hand::FiveOfAKind
        );
        assert_eq!(
            Hand::try_from(vec![
                Card::Ace,
                Card::Ace,
                Card::King,
                Card::Ace,
                Card::Ace,
            ])?,
            Hand::FourOfAKind
        );
        assert_eq!(
            Hand::try_from(vec![
                Card::King,
                Card::Ace,
                Card::Ace,
                Card::Ace,
                Card::King,
            ])?,
            Hand::FullHouse
        );
        assert_eq!(
            Hand::try_from(vec![
                Card::Queen,
                Card::Ace,
                Card::Ace,
                Card::King,
                Card::Ace,
            ])?,
            Hand::ThreeOfAKind
        );
        assert_eq!(
            Hand::try_from(vec![
                Card::Queen,
                Card::Ace,
                Card::Ace,
                Card::King,
                Card::King,
            ])?,
            Hand::TwoPair
        );
        assert_eq!(
            Hand::try_from(vec![
                Card::Queen,
                Card::Ace,
                Card::Ace,
                Card::King,
                Card::Joker,
            ])?,
            Hand::OnePair
        );
        assert_eq!(
            Hand::try_from(vec![
                Card::Queen,
                Card::Ace,
                Card::King,
                Card::Joker,
                Card::Ten,
            ])?,
            Hand::HighCard
        );
        Ok(())
    }

    #[test]
    fn test_hand_order() -> Result<()> {
        assert!(Hand::FiveOfAKind > Hand::FourOfAKind);
        assert!(Hand::FourOfAKind > Hand::FullHouse);
        assert!(Hand::FullHouse > Hand::ThreeOfAKind);
        assert!(Hand::ThreeOfAKind > Hand::TwoPair);
        assert!(Hand::TwoPair > Hand::OnePair);
        assert!(Hand::OnePair > Hand::HighCard);
        Ok(())
    }


    #[test]
    fn test_handandbid_parse() -> Result<()> {
        assert_eq!(
            HandAndBid::parse("32T3K 765")?,
            HandAndBid {
                cards: "32T3K".to_string(),
                hand: Hand::OnePair,
                bid: 765,
            }
        );
        assert_eq!(
            HandAndBid::parse("T55J5 684")?,
            HandAndBid {
                cards: "T55J5".to_string(),
                hand: Hand::ThreeOfAKind,
                bid: 684,
            }
        );
        assert_eq!(
            HandAndBid::parse("AAAAA 123")?,
            HandAndBid {
                cards: "AAAAA".to_string(),
                hand: Hand::FiveOfAKind,
                bid: 123,
            }
        );
        assert_eq!(
            HandAndBid::parse("25364 321")?,
            HandAndBid {
                cards: "25364".to_string(),
                hand: Hand::HighCard,
                bid: 321,
            }
        );
        Ok(())
    }
    
    #[test]
    fn test_handandbid_order() -> Result<()> {
        let cases = vec![
            ("32T3K 765", "T55J5 684", Ordering::Less),
            ("T55J5 684", "KK677 28",  Ordering::Greater),
            ("KK677 28",  "KTJJT 220", Ordering::Greater),
        ];
        for (i, (left, right, ord)) in cases.into_iter().enumerate() {
            let left = HandAndBid::parse(left)?;
            let right = HandAndBid::parse(right)?;
            let res = left.cmp(&right);
            assert!(
                res == ord, 
                "Case {} failed. Expected {:?}, got {:?}", 
                i, ord, res,
            );
        }
        Ok(())
    }
}
