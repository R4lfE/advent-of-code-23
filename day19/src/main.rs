use std::{error::Error, fs, collections::HashMap};

#[derive(Clone, Copy)]
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

enum Operator {
    Less,
    Greater
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
                    }
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

fn part2(input: &str) -> usize {
    let (workflows, _) = read_list(input);
    todo!()
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
        assert_eq!(part2(input), 167409079868000);
    }
}