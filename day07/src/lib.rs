use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five = 6,
    Four = 5,
    FullHouse = 4,
    Three = 3,
    TwoPair = 2,
    Pair = 1,
    High = 0
}

#[derive(Debug, Eq)]
struct Hand {
    bid: u32,
    cards: [u8; 5],
    hand_type: HandType
}

impl Hand {
    fn from(s: &str, jokers: u8) -> Self {
        let split: Vec<&str> = s
            .split_whitespace()
            .map(|line| line.trim())
            .collect();

        let bid: u32 = split[1].parse().unwrap();

        let mut cards: [u8; 5] = [0; 5];
        for (i, c) in split[0].chars().enumerate() {
            cards[i] = match c {
                'A' => 14 - jokers,
                'K' => 13 - jokers,
                'Q' => 12 - jokers,
                'J' => 11 - 10 * jokers,
                'T' => 10,
                c if c.is_numeric() => c.to_digit(10).unwrap() as u8,
                _ => panic!("Error: Not a valid card.")
            }
        }

        let mut counter = [0; 13];
        for card in cards.iter() {
            counter[*card as usize + jokers as usize - 2] += 1;
        }

        if jokers == 1 {
            let (max_index, _) = counter
                .iter()
                .enumerate()
                .skip(1)
                .max_by(|a, b| a.1.cmp(b.1))
                .unwrap();
            counter[max_index] += counter[0];
        }

        let hand_type = match counter.iter().skip(jokers as usize).max() {
            Some(max) => match max {
                5 => HandType::Five,
                4 => HandType::Four,

                3 => match counter.iter().skip(jokers as usize).filter(|count| **count != 0).min() {
                    Some(min) => match min {
                        2 => HandType::FullHouse,
                        1 => HandType::Three,
                        _ => panic!("Error: Too many cards after three of a kind.")
                    },
                    None => panic!("Error: Not enough cards after three of a kind."),
                },
                2 => match counter.iter().skip(jokers as usize).filter(|count| **count == 2).count() {
                    2 => HandType::TwoPair,
                    1 => HandType::Pair,
                    0 => match jokers {
                        1 => HandType::Pair,
                        _ => panic!("Error: Wrong number of cards after pair with jokers.")
                    }, 
                    _ => panic!("Error: Wrong number of cards after pair.")
                },
                1 => HandType::High,
                x => panic!("Error: Found {x} cards.")
            },
            None => panic!("Error: Counter is broken."),
        };

        Hand {
            bid,
            cards,
            hand_type,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare cards
        if self.hand_type == other.hand_type {
            for i in 0..self.cards.len() {
                match self.cards[i].cmp(&other.cards[i]) {
                    Ordering::Equal => (),
                    ordering => { return ordering; }
                }
            }
            Ordering::Equal

        // compare hand types
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn both(input: &str, jokers: u8) -> u32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Hand::from(line, jokers))
        .collect();
    
    hands.sort();

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i as u32 + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card() {
        assert_eq!(Hand::from("JQQQQ 1", 1).hand_type, HandType::Five);
        assert_eq!(Hand::from("JQ4QQ 1", 1).hand_type, HandType::Four);
        assert_eq!(Hand::from("JQQ44 1", 1).hand_type, HandType::FullHouse);
        assert_eq!(Hand::from("JQQ53 1", 1).hand_type, HandType::Three);
        assert_eq!(Hand::from("J32QK 1", 1).hand_type, HandType::Pair);
    }

    #[test]
    fn part_1() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        assert_eq!(both(input, 0), 6440);
    }

    #[test]
    fn part_2() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        assert_eq!(both(input, 1), 5905);
    }
}