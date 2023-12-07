use std::{cmp::Ordering, error::Error, fs};

#[derive(Debug, Eq, Ord)]
enum Hand {
    Five([u8; 5], u32),
    Four([u8; 5], u32),
    FullHouse([u8; 5], u32),
    Three([u8; 5], u32),
    TwoPair([u8; 5], u32),
    Pair([u8; 5], u32),
    High([u8; 5], u32)
}

impl Hand {
    fn from(s: &str, jokers: u8) -> Self {
        let split: Vec<&str> = s
            .split_whitespace()
            .collect();

        let mut hand: [u8; 5] = [0; 5];
        for (i, c) in split[0].chars().enumerate() {
            hand[i] = match c {
                'A' => 14 - jokers,
                'K' => 13 - jokers,
                'Q' => 12 - jokers,
                'J' => 11 - 10 * jokers,
                'T' => 10,
                c if c.is_numeric() => c.to_digit(10).unwrap() as u8,
                _ => panic!("Error: Not a valid card.")
            }
        }

        let bid: u32 = split[1].parse().unwrap();

        let mut counter = vec![0; 13];
        for card in hand.iter() {
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

        match counter.iter().skip(jokers as usize).max() {
            Some(max) => match max {
                5 => Hand::Five(hand, bid),
                4 => Hand::Four(hand, bid),

                3 => match counter.iter().skip(jokers as usize).filter(|count| **count != 0).min() {
                    Some(min) => match min {
                        2 => Hand::FullHouse(hand, bid),
                        1 => Hand::Three(hand, bid),
                        _ => panic!("Error: Too many cards after three of a kind.")
                    },
                    None => panic!("Error: Not enough cards after three of a kind."),
                },
                2 => match counter.iter().skip(jokers as usize).filter(|count| **count == 2).count() {
                    2 => Hand::TwoPair(hand, bid),
                    1 => Hand::Pair(hand, bid),
                    0 => match jokers {
                        1 => Hand::Pair(hand, bid),
                        _ => panic!("Error: Wrong number of cards after pair with jokers.")
                    }, 
                    _ => panic!("Error: Wrong number of cards after pair.")
                },
                1 => Hand::High(hand, bid),
                x => panic!("Error: Found {x} cards.")
            },
            None => panic!("Error: Counter is broken."),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Five(l0, _), Self::Five(r0, _)) => l0 == r0,
            (Self::Four(l0, _), Self::Four(r0, _)) => l0 == r0,
            (Self::FullHouse(l0, _), Self::FullHouse(r0, _)) => l0 == r0,
            (Self::Three(l0, _), Self::Three(r0, _)) => l0 == r0,
            (Self::TwoPair(l0, _), Self::TwoPair(r0, _)) => l0 == r0,
            (Self::Pair(l0, _), Self::Pair(r0, _)) => l0 == r0,
            (Self::High(l0, _), Self::High(r0, _)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Hand::Five(l, _), Hand::Five(r, _))
            | (Hand::Four(l, _), Hand::Four(r, _))
            | (Hand::FullHouse(l, _), Hand::FullHouse(r, _))
            | (Hand::Three(l, _), Hand::Three(r, _))
            | (Hand::TwoPair(l, _), Hand::TwoPair(r, _))
            | (Hand::Pair(l, _), Hand::Pair(r, _))
            | (Hand::High(l, _), Hand::High(r, _)) => {
                for i in 0..l.len() {
                    if l[i] < r[i] {
                        return Some(Ordering::Less);
                    } else if l[i] > r[i] {
                        return Some(Ordering::Greater);
                    }
                } 
                return Some(Ordering::Equal);
            },
            (Hand::Five(_, _), Hand::Four(_, _))
            | (Hand::Five(_, _), Hand::FullHouse(_, _))
            | (Hand::Five(_, _), Hand::Three(_, _))
            | (Hand::Five(_, _), Hand::TwoPair(_, _))
            | (Hand::Five(_, _), Hand::Pair(_, _))
            | (Hand::Five(_, _), Hand::High(_, _))
            | (Hand::Four(_, _), Hand::FullHouse(_, _))
            | (Hand::Four(_, _), Hand::Three(_, _))
            | (Hand::Four(_, _), Hand::TwoPair(_, _))
            | (Hand::Four(_, _), Hand::Pair(_, _))
            | (Hand::Four(_, _), Hand::High(_, _))
            | (Hand::FullHouse(_, _), Hand::Three(_, _))
            | (Hand::FullHouse(_, _), Hand::TwoPair(_, _))
            | (Hand::FullHouse(_, _), Hand::Pair(_, _))
            | (Hand::FullHouse(_, _), Hand::High(_, _))
            | (Hand::Three(_, _), Hand::TwoPair(_, _))
            | (Hand::Three(_, _), Hand::Pair(_, _))
            | (Hand::Three(_, _), Hand::High(_, _))
            | (Hand::TwoPair(_, _), Hand::Pair(_, _))
            | (Hand::TwoPair(_, _), Hand::High(_, _))
            | (Hand::Pair(_, _), Hand::High(_, _)) => Some(Ordering::Greater),
            _ => Some(Ordering::Less)
        }
    }
}

fn both(input: &str, jokers: u8) -> u32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Hand::from(line, jokers))
        .collect();
    
    hands.sort();

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + match hand {
            Hand::Five(_, bid)
            | Hand::Four(_, bid)
            | Hand::FullHouse(_, bid)
            | Hand::Three(_, bid)
            | Hand::TwoPair(_, bid)
            | Hand::Pair(_, bid)
            | Hand::High(_, bid) => bid * (i as u32 + 1)
        })
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    
    dbg!(both(&input, 0));
    dbg!(both(&input, 1));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card() {
        assert_eq!(Hand::from("JQQQQ 1", 1), Hand::Five([1,11,11,11,11], 1));
        assert_eq!(Hand::from("JQ4QQ 1", 1), Hand::Four([1,11,4,11,11], 1));
        assert_eq!(Hand::from("JQQ44 1", 1), Hand::FullHouse([1,11,11,4,4], 1));
        assert_eq!(Hand::from("JQQ53 1", 1), Hand::Three([1,11,11,5,3], 1));
        assert_eq!(Hand::from("J32QK 1", 1), Hand::Pair([1,3,2,11,12], 1));
    }

    #[test]
    fn part_1() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        assert_eq!(both(&input, 0), 6440);
    }

    #[test]
    fn part_2() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        assert_eq!(both(&input, 1), 5905);
    }
}