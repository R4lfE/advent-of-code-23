use std::{error::Error, fs, collections::HashMap};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MachinePart {
    Cool,
    Musical,
    Aerodynamic,
    Shiny
}
use MachinePart::*;

impl From<char> for MachinePart {
    fn from(c: char) -> Self {
        match c {
            'x' => Cool,
            'm' => Musical,
            'a' => Aerodynamic,
            's' => Shiny,
            _ => panic!("Error: Invalid character '{c}'.")
        }
    }
}

struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}

impl Rating {
    fn get(&self, machine_part: MachinePart) -> usize {
        match machine_part {
            Cool => self.x,
            Musical => self.m,
            Aerodynamic => self.a,
            Shiny => self.s
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for Rating {
    fn from(str: &str) -> Self {
        let ratings: Vec<usize> = str.split(',').map(|rating| rating.split('=').collect::<Vec<_>>()[1].replace('}', "").parse().unwrap()).collect();
        
        Self {
            x: ratings[0],
            m: ratings[1],
            a: ratings[2],
            s: ratings[3]
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    Less,
    Greater,
    LessEq,
    GreaterEq
}
use Operator::*;

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '<' => Less,
            '>' => Greater,
            _ => panic!("Error: Invalid character '{c}'.")
        }
    }
}

impl Operator {
    fn other(&self) -> Self {
        match self {
            Less => GreaterEq,
            Greater => LessEq,
            _ => panic!("Error: Invalid operator.")
        }
    }
}

#[derive(Clone, Debug)]
struct Rule {
    part: MachinePart,
    operator: Operator,
    rating: usize,
    next_workflow: String
}

impl From<&str> for Rule {
    fn from(str: &str) -> Self {
        if str.find('<').is_some() {
            let split: Vec<&str> = str.split('<').collect();
            let part = MachinePart::from(split[0].chars().next().unwrap());
            let operator = Less;

            let split: Vec<&str> = split[1].split(':').collect();
            let rating = split[0].parse().unwrap();
            let next_workflow = split[1].to_string();

            Self {
                part,
                operator,
                rating,
                next_workflow
            }
        } else {
            let split: Vec<&str> = str.split('>').collect();
            let part = MachinePart::from(split[0].chars().next().unwrap());
            let operator = Greater;

            let split: Vec<&str> = split[1].split(':').collect();
            let rating = split[0].parse().unwrap();
            let next_workflow = split[1].to_string();

            Self {
                part,
                operator,
                rating,
                next_workflow
            }
        }
    }
}

impl Rule {
    fn other(self) -> Self {
        Self {
            operator: self.operator.other(),
            ..self
        }
    }
}

#[derive(Clone)]
struct Workflow {
    this_workflow: String,
    rules: Vec<Rule>,
    next_workflow: String
}

impl From<&str> for Workflow {
    fn from(str: &str) -> Self {
        let split: Vec<&str> = str.split('{').collect();
        let this_workflow = split[0].to_string();

        let split: Vec<&str> = split[1].split(',').collect();
        let num_rules = split.len() - 1;

        let next_workflow = split.last().unwrap().replace('}', "");
        let rules: Vec<Rule> = split.into_iter().take(num_rules).map(Rule::from).collect();
        
        Self {
            this_workflow,
            rules,
            next_workflow
        }
    }
}

fn read_list(input: &str) -> (Vec<Workflow>, Vec<Rating>) {
    let mut lines = input.lines();

    let mut workflows = Vec::new();
    for line in lines.by_ref() {
        if !line.trim().is_empty() {
            workflows.push(Workflow::from(line.trim()));
        } else {
            break;
        }
    }

    let mut ratings = Vec::new();
    for line in lines {
        if !line.trim().is_empty() {
            ratings.push(Rating::from(line.trim()));
        }
    }

    (workflows, ratings)
} 

fn part1(input: &str) -> usize {
    let (workflows, ratings) = read_list(input);

    let mut map: HashMap<String, Workflow> = HashMap::new();
    for workflow in workflows {
        map.insert(workflow.this_workflow.clone(), workflow);
    }

    let mut sum = 0;
    for rating in ratings {

        let mut current_workflow = String::from("in");
        let a = String::from('A');
        let r = String::from('R');

        while current_workflow != a && current_workflow != r {
            let workflow = map.get(&current_workflow).unwrap();

            for rule in workflow.rules.iter() {
                match rule.operator {
                    Less => if rating.get(rule.part) < rule.rating {
                        current_workflow = rule.next_workflow.clone();
                        break;
                    },
                    Greater => if rating.get(rule.part) > rule.rating {
                        current_workflow = rule.next_workflow.clone();
                        break;
                    },
                    _ => panic!("Error: Invalid operator.")
                }
            }

            if current_workflow == workflow.this_workflow {
                current_workflow = workflow.next_workflow.clone();
            }
        }

        if current_workflow == a {
            sum += rating.sum();
        }
     }

    sum
}

fn find_rules_to_a(map: &HashMap<String, Workflow>, current_workflow: &mut String, rules: &mut Vec<Rule>, accepted_ratings: &mut Vec<Vec<Rule>>) {
    if current_workflow == "A" {
        accepted_ratings.push(rules.clone());
    } else if let Some(workflow) = map.get(current_workflow) {
        let current_workflow_clone = current_workflow.clone();
        for rule in workflow.rules.iter() {
            rules.push(rule.clone());

            current_workflow.clear();
            current_workflow.push_str(&rule.next_workflow);

            find_rules_to_a(map, current_workflow, rules, accepted_ratings);

            rules.pop();

            rules.push(rule.clone().other());
        }

        current_workflow.clear();
        current_workflow.push_str(&workflow.next_workflow);

        find_rules_to_a(map, current_workflow, rules, accepted_ratings);

        while let Some(rule) = rules.last() {
            match rule.operator {
                Less | Greater => break,
                LessEq | GreaterEq => { rules.pop(); }
            }
        }

        current_workflow.clear();
        current_workflow.push_str(&current_workflow_clone);
    }
}

fn part2(input: &str) -> usize {
    let (workflows, _) = read_list(input);
    let mut rules = Vec::new();

    let mut map: HashMap<String, Workflow> = HashMap::new();
    for workflow in workflows {
        map.insert(workflow.this_workflow.clone(), workflow);
    }

    let mut current_workflow = String::from("in");

    let mut accepted_ratings = Vec::new();
    find_rules_to_a(&map, &mut current_workflow, &mut rules, &mut accepted_ratings);

    let bounds_per_flow: Vec<[(usize, usize); 4]> = accepted_ratings
        .into_iter()
        .map(|rules| {
            let mut bounds = [(1, 4000); 4];

            for rule in rules {
                match rule.operator {
                    Less => if rule.rating <= bounds[rule.part as usize].1 {
                        bounds[rule.part as usize].1 = rule.rating - 1;
                    },
                    Greater => if rule.rating >= bounds[rule.part as usize].0 {
                        bounds[rule.part as usize].0 = rule.rating + 1;
                    },
                    LessEq => if rule.rating < bounds[rule.part as usize].1 {
                        bounds[rule.part as usize].1 = rule.rating;
                    },
                    GreaterEq => if rule.rating > bounds[rule.part as usize].0 {
                        bounds[rule.part as usize].0 = rule.rating;
                    }
                }
            }

            bounds
        }).collect();

    bounds_per_flow
        .iter()
        .map(|bounds| bounds
            .iter()
            .fold(0, |acc, bound| {
                match acc {
                    0 => bound.1 - bound.0 + 1,
                    _ => acc * (bound.1 - bound.0 + 1)
                }
            })
        ).sum()
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
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}"#;
        assert_eq!(part1(input), 19114);
    }

    #[test]
    fn part_2() {
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}"#;
        assert_eq!(part2(input), 167_409_079_868_000);
    }
}