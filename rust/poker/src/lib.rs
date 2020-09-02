use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug, Eq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Ord, PartialOrd, Eq, Debug, PartialEq)]
enum HandRanking {
    HighCard([Rank; 5]),
    OnePair(Rank, [Rank; 3]),
    TwoPair(Rank, Rank, Rank),
    ThreeOfAKind(Rank, [Rank; 2]),
    Straight(Rank),
    Flush([Rank; 5]),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
}

impl PartialEq for Card {
    fn eq(&self, rhs: &Self) -> bool {
        self.rank == rhs.rank
    }
}

impl Ord for Card {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.rank.cmp(&rhs.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(&rhs))
    }
}

#[derive(Eq, Debug)]
struct Hand<'a> {
    as_str: &'a str,
    ranking: HandRanking,
}

impl FromStr for Rank {
    type Err = PokerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Rank::*;
        match s {
            "2" => Ok(Two),
            "3" => Ok(Three),
            "4" => Ok(Four),
            "5" => Ok(Five),
            "6" => Ok(Six),
            "7" => Ok(Seven),
            "8" => Ok(Eight),
            "9" => Ok(Nine),
            "10" => Ok(Ten),
            "J" => Ok(Jack),
            "Q" => Ok(Queen),
            "K" => Ok(King),
            "A" => Ok(Ace),
            _ => Err(PokerError::BadInput(format!("Unrecognized rank: {}", s))),
        }
    }
}

impl TryFrom<char> for Suit {
    type Error = PokerError;
    fn try_from(s: char) -> Result<Self, Self::Error> {
        use Suit::*;
        match s {
            'H' => Ok(Hearts),
            'C' => Ok(Clubs),
            'D' => Ok(Diamonds),
            'S' => Ok(Spades),
            _ => Err(PokerError::BadInput(format!("Unrecognized suit: {}", s))),
        }
    }
}

impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = PokerError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let cards = s
            .split_whitespace()
            .map(|word| {
                let len = word.len();
                let rank = word[..len - 1].parse()?;
                let suit = word
                    .chars()
                    .last()
                    .ok_or_else(|| PokerError::BadInput(format!("no last character: {}", word)))?
                    .try_into()?;
                Ok(Card { rank, suit })
            })
            .collect::<Result<Vec<Card>, Self::Error>>()?;
        let ranking = interpret_hand(cards)?;
        Ok(Self { as_str: s, ranking })
    }
}

#[derive(Debug)]
enum PokerError {
    NoneError(&'static str),
    BadInput(String),
    ConversionError(std::array::TryFromSliceError),
}

impl std::fmt::Display for PokerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PokerError::NoneError(s) => write!(f, "{}", s),
            PokerError::BadInput(s) => write!(f, "{}", s),
            PokerError::ConversionError(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for PokerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PokerError::NoneError(_) | PokerError::BadInput(_) => None,
            PokerError::ConversionError(e) => Some(e),
        }
    }
}

impl From<std::array::TryFromSliceError> for PokerError {
    fn from(err: std::array::TryFromSliceError) -> Self {
        PokerError::ConversionError(err)
    }
}

fn interpret_hand(mut cards: Vec<Card>) -> Result<HandRanking, PokerError> {
    cards.sort();
    let ranks = Hand::ranks(&cards);

    // handle pairs, three, four, and full house
    let (non_singles, singles): (HashMap<_, _>, HashMap<_, _>) =
        ranks.iter().partition(|&(_, &v)| v != 1);

    let highest_count = ranks
        .iter()
        .max_by_key(|(_, &v)| v)
        .ok_or_else(|| PokerError::NoneError("no max"))?;
    let mut singles = singles.into_iter().map(|(k, _)| k).collect::<Vec<_>>();
    singles.sort();

    match (non_singles.len(), highest_count.1) {
        (2, 3) => {
            let three_of = *highest_count.0;
            let two_of = non_singles
                .into_iter()
                .find(|&(_, v)| v == 2)
                .ok_or_else(|| PokerError::BadInput("no pair for this full house".to_string()))?
                .0;
            Ok(HandRanking::FullHouse(three_of, two_of))
        }
        (2, 2) => {
            let mut v: Vec<_> = non_singles.into_iter().collect();
            v.sort();
            let mut iter = v.into_iter();
            let small_pair = iter
                .next()
                .ok_or_else(|| PokerError::BadInput("no small rank in vec".to_string()))?
                .0;
            let large_pair = iter
                .next()
                .ok_or_else(|| PokerError::BadInput("no large rank in vec".to_string()))?
                .0;
            let spare_card = singles[0];
            Ok(HandRanking::TwoPair(large_pair, small_pair, spare_card))
        }
        (1, 4) => {
            let four_of = *highest_count.0;
            let spare_card = singles[0];
            Ok(HandRanking::FourOfAKind(four_of, spare_card))
        }
        (1, 3) => {
            let three_of = *highest_count.0;
            let spare_cards = singles.as_slice().try_into()?;
            Ok(HandRanking::ThreeOfAKind(three_of, spare_cards))
        }
        (1, 2) => {
            let pair_of = *highest_count.0;

            let spare_cards = singles.as_slice().try_into()?;
            Ok(HandRanking::OnePair(pair_of, spare_cards))
        }
        (0, _) => match (Hand::is_flush(&cards), Hand::is_straight(&cards)) {
            (true, true) => {
                let high_card = *singles
                    .last()
                    .ok_or_else(|| PokerError::NoneError("no high card in singles"))?;
                Ok(HandRanking::StraightFlush(high_card))
            }
            (true, false) => {
                let singles = singles.as_slice().try_into()?;
                Ok(HandRanking::Flush(singles))
            }
            (false, true) => {
                let is_aces_low_straight =
                    Some(&Rank::Two) == singles.first() && Some(&Rank::Ace) == singles.last();
                let high_card = if is_aces_low_straight {
                    Rank::Five
                } else {
                    *singles
                        .last()
                        .ok_or_else(|| PokerError::NoneError("no high card in singles"))?
                };
                Ok(HandRanking::Straight(high_card))
            }
            (false, false) => {
                let singles = singles.as_slice().try_into()?;
                Ok(HandRanking::HighCard(singles))
            }
        },
        _ => Err(PokerError::BadInput(format!(
            "wrong number of cards: {:?}",
            cards
        ))),
    }
}

impl Hand<'_> {
    fn ranks(cards: &[Card]) -> HashMap<Rank, u32> {
        let mut hm = HashMap::new();
        for card in cards {
            let count = hm.entry(card.rank).or_insert(0);
            *count += 1;
        }
        hm
    }

    fn is_flush(cards: &[Card]) -> bool {
        cards
            .windows(2)
            .all(|window| window[0].suit == window[1].suit)
    }

    /// Determines of a sorted slice of Cards represents a straight.
    /// NB: Cards must be pre-sorted.
    fn is_straight(cards: &[Card]) -> bool {
        use Rank::*;
        let mut straights = vec![Ace as u32];
        straights.extend(Two as u32..=Ace as u32);
        let mut straights = straights.windows(5);
        let ranks: Vec<_> = cards.iter().map(|c| c.rank as u32).collect();
        straights.any(|straight| {
            let mut straight = straight.to_vec();
            straight.sort();
            ranks.eq(&straight)
        })
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        use HandRanking::*;
        match (&self.ranking, &rhs.ranking) {
            (HighCard(lhs), HighCard(rhs)) => lhs.cmp(&rhs),
            (OnePair(l_pair, l_rem), OnePair(r_pair, r_rem)) => {
                l_pair.cmp(&r_pair).then_with(|| l_rem.cmp(&r_rem))
            }
            (
                TwoPair(l_big_pair, l_small_pair, l_rem),
                TwoPair(r_big_pair, r_small_pair, r_rem),
            ) => l_big_pair
                .cmp(&r_big_pair)
                .then_with(|| l_small_pair.cmp(&r_small_pair))
                .then_with(|| l_rem.cmp(&r_rem)),
            (ThreeOfAKind(l_three, l_rem), ThreeOfAKind(r_three, r_rem)) => {
                l_three.cmp(&r_three).then_with(|| l_rem.cmp(&r_rem))
            }
            (Straight(lhs), Straight(rhs)) => lhs.cmp(&rhs),
            (Flush(lhs), Flush(rhs)) => lhs.cmp(&rhs),
            (FullHouse(l_three, l_two), FullHouse(r_three, r_two)) => {
                l_three.cmp(&r_three).then_with(|| l_two.cmp(&r_two))
            }
            (FourOfAKind(l_four, l_rem), FourOfAKind(r_four, r_rem)) => {
                l_four.cmp(&r_four).then_with(|| l_rem.cmp(&r_rem))
            }
            (StraightFlush(lhs), StraightFlush(rhs)) => lhs.cmp(&rhs),
            _ => self.ranking.cmp(&rhs.ranking),
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(&rhs))
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, rhs: &Self) -> bool {
        self.ranking == rhs.ranking
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    if let Ok(mut hands) = hands
        .iter()
        .map(|&hand| hand.try_into())
        .collect::<Result<Vec<Hand>, _>>()
    {
        hands.sort();
        if let Some(best) = hands.last() {
            let winners = hands
                .iter()
                .filter_map(|hand| {
                    if hand == best {
                        Some(hand.as_str)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            return Some(winners);
        }
    }
    None
}
