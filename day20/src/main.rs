use std::{error::Error, fs, collections::{HashMap, VecDeque}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    High,
    Low
}
use Pulse::*;

#[derive(Debug, Eq)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Broadcaster,
    Button
}
use ModuleType::*;
use num::Integer;

impl From<char> for ModuleType {
    fn from(c: char) -> Self {
        match c {
            '%' => FlipFlop(false),
            '&' => Conjunction(HashMap::new()),
            'b' => Broadcaster,
            _ => panic!("Error: Invalid character '{c}'.")
        }
    }
}

impl PartialEq for ModuleType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            FlipFlop(_) => matches!(other, FlipFlop(_)),
            Conjunction(_) => matches!(other, Conjunction(_)),
            Broadcaster => matches!(other, Broadcaster),
            Button => matches!(other, Button),
        }
    }
}

#[derive(Debug)]
struct Module {
    name: String,
    type_: ModuleType,
    destinations: Vec<String>
}

impl From<String> for Module {
    fn from(str: String) -> Self {
        let split: Vec<&str> = str.split_whitespace().collect();

        let type_ = ModuleType::from(split[0].chars().next().unwrap());

        let name = match type_ {
            Broadcaster => split[0],
            _ => &split[0][1..split[0].len()]
        }.to_string();

        let destinations: Vec<String> = split.into_iter().skip(2).map(|str| str.to_string()).collect();

        Module {
            name,
            type_,
            destinations
        }
    }
}

impl Module {
    fn button() -> Self {
        Module {
            name: String::from("button"),
            type_: Button,
            destinations: vec![String::from("broadcaster")]
        }
    }

    fn flip(&mut self) {
        match self.type_ {
            FlipFlop(is_on) => {
                self.type_ = FlipFlop(!is_on);
            },
            _ => panic!("Error: Tried to flip an incorrect module.")
        }
    }

    fn handle_pulse(&mut self, source: String, pulse: Pulse, pulse_order: &mut VecDeque<(String, String, Pulse)>) -> Option<(Pulse, usize)> {
        match self.type_ {
            FlipFlop(is_on) => {
                match pulse {
                    High => None,
                    Low => {
                        let pulse = match is_on {
                            true => Low,
                            false => High
                        };

                        for destination in self.destinations.iter() {
                            pulse_order.push_back((self.name.clone(), destination.clone(), pulse));
                        }

                        self.flip();
                        Some((pulse, self.destinations.len()))
                    }
                }
            },
            Conjunction(ref mut sources) => {
                sources.insert(source, pulse);

                let pulse = if sources.iter().all(|source| *source.1 == High) {
                    Low
                } else {
                    High
                };

                for destination in self.destinations.iter() {
                    pulse_order.push_back((self.name.clone(), destination.clone(), pulse));
                }
                Some((pulse, self.destinations.len()))
            },
            Broadcaster => {
                for destination in self.destinations.iter() {
                    pulse_order.push_back((self.name.clone(), destination.clone(), pulse));
                }
                Some((pulse, self.destinations.len()))
            },
            Button => {
                pulse_order.push_back((self.name.clone(), String::from("broadcaster"), Low));
                Some((Low, 1))
            }
        }
    }
}

fn init_modules(input: &str) -> Vec<Module> {
    let mut modules: Vec<Module> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().replace(',', ""))
        .map(Module::from)
        .collect();

    let conjunctions: Vec<String> = modules
        .iter()
        .filter(|module| module.type_ == Conjunction(HashMap::new()))
        .map(|module| module.name.clone())
        .collect();

    for conjunction in conjunctions.iter() {
        let sources: HashMap<String, Pulse> = modules
            .iter()
            .filter(|module| module.destinations.contains(conjunction))
            .map(|module| (module.name.clone(), Low))
            .collect();

        modules.iter_mut().find(|module| &module.name == conjunction).unwrap().type_ = Conjunction(sources);
    }

    modules
}

fn part1(input: &str) -> usize {
    let mut modules = init_modules(input)
        .into_iter()
        .map(|module| (module.name.clone(), module))
        .collect::<HashMap<String, Module>>();

    let mut pulses = [0; 2];
    let mut pulse_order: VecDeque<(String, String, Pulse)> = VecDeque::new();

    let mut button = Module::button();

    for _ in 0..1000 {
        pulses[Low as usize] += button.handle_pulse(button.name.clone(), Low, &mut pulse_order).unwrap().1;

        while let Some(next) = pulse_order.pop_front() {
            if let Some(module) = modules.get_mut(&next.1) {
                if let Some(result) = module.handle_pulse(next.0, next.2, &mut pulse_order) {
                    pulses[result.0 as usize] += result.1;
                }
            }
        }
    }

    pulses[0] * pulses[1]
}

fn part2(input: &str) -> usize {
    let mut modules = init_modules(input)
        .into_iter()
        .map(|module| (module.name.clone(), module))
        .collect::<HashMap<String, Module>>();

    let mut pulse_order: VecDeque<(String, String, Pulse)> = VecDeque::new();
    let mut button = Module::button();

    let mut presses = 0;
    let mut cycle_lenghts: [Option<usize>; 4] = [None; 4];
    let conjunction_modules = ["kd", "zf", "vg", "gs"];

    while cycle_lenghts.iter().any(|cycle| cycle.is_none()) {
        button.handle_pulse(button.name.clone(), Low, &mut pulse_order);
        presses += 1;

        while let Some(next) = pulse_order.pop_front() {
            for (i, module) in conjunction_modules.iter().enumerate() {
                if &next.0 == module && next.2 == High && cycle_lenghts[i].is_none() {
                    cycle_lenghts[i] = Some(presses);
                }
            }

            if let Some(module) = modules.get_mut(&next.1) {
                module.handle_pulse(next.0, next.2, &mut pulse_order);
            }
        }
    }

    cycle_lenghts.into_iter().fold(0, |acc, cycle_length| match acc {
        0 => cycle_length.unwrap(),
        _ => Integer::lcm(&acc, &cycle_length.unwrap())
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    
    dbg!(part1(&input));
    dbg!(part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a"#;
        assert_eq!(part1(input), 32000000);

        let input = r#"broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output"#;
        assert_eq!(part1(input), 11687500);
    }

    #[test]
    fn part_2() {
        let input = r#""#;
        assert_eq!(part2(input), 0);
    }
}