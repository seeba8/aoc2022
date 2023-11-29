use std::{collections::HashMap, fmt::Display, str::FromStr};

use crate::util::read_input;
use {once_cell::sync::Lazy, regex::Regex};

pub fn solve() {
    let input = read_input("day21.txt");
    let mut monkeys: Monkeys = input.parse().unwrap();
    println!("Day 21 part 1: {}", monkeys.solve("root").unwrap());

    let mut monkeys: Monkeys = input.parse().unwrap();
    monkeys.monkeys.entry("root".to_string()).and_modify(|m| {
        if let Action::Calculation(c) = m {
            c.op = Operator::Eq;
        }
    });
    monkeys
        .monkeys
        .entry("humn".to_string())
        .and_modify(|h| *h = Action::Human);
    let Action::Calculation(root) = monkeys.monkeys.get("root").unwrap().clone() else {
        panic!()
    };
    let result = if let Action::Number(_) = monkeys.monkeys.get(&root.lhs).unwrap() {
        monkeys.solve_equation(&root.rhs, &root.lhs)
    } else {
        monkeys.solve_equation(&root.lhs, &root.rhs)
    };
    println!("Day 21 part 2: {result}");
}

type Number = isize;

#[derive(Debug, PartialEq, Eq)]
pub struct Monkey {
    name: String,
    action: Action,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action {
    Number(Number),
    Calculation(Calculation),
    Human,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl Operator {
    pub fn apply(self, lhs: Number, rhs: Number) -> Number {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
            Self::Eq => panic!("Equal shouldn't happen"),
        }
    }

    pub const fn get_inverse(self) -> Self {
        match self {
            Self::Add => Self::Sub,
            Self::Sub => Self::Add,
            Self::Mul => Self::Div,
            Self::Div => Self::Mul,
            Self::Eq => Self::Eq,
        }
    }

    pub fn is_commutative(self) -> bool {
        self == Self::Add || self == Self::Mul
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Calculation {
    lhs: String,
    op: Operator,
    rhs: String,
}

impl FromStr for Operator {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            x => Err(color_eyre::eyre::eyre!("Cannot parse operator {x}")),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Add => write!(f, " + "),
            Self::Sub => write!(f, " - "),
            Self::Mul => write!(f, " * "),
            Self::Div => write!(f, " / "),
            Self::Eq => write!(f, " = "),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkeys {
    monkeys: HashMap<String, Action>,
}

impl FromStr for Monkeys {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Vec<Monkey> = s
            .trim()
            .lines()
            .map(str::parse)
            .collect::<color_eyre::Result<Vec<_>>>()?;
        let mut result = HashMap::new();
        for monkey in res {
            result.insert(monkey.name, monkey.action);
        }
        Ok(Self { monkeys: result })
    }
}

impl FromStr for Monkey {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(\w+): (\w+)(?: ([\+\-\*\/]) (\w+))?$").unwrap());
        let cap = RE
            .captures(s)
            .ok_or_else(|| color_eyre::eyre::eyre!("Cannot parse line {s}"))?;
        if cap.iter().filter(Option::is_some).count() > 3 {
            Ok(Self {
                name: cap[1].to_owned(),
                action: Action::Calculation(Calculation {
                    lhs: cap[2].to_owned(),
                    op: cap[3].parse()?,
                    rhs: cap[4].to_owned(),
                }),
            })
        } else {
            Ok(Self {
                name: cap[1].to_owned(),
                action: Action::Number(cap[2].parse()?),
            })
        }
    }
}

impl Monkeys {
    pub fn solve(&mut self, name: &str) -> Option<Number> {
        if let Action::Calculation(calculation) = self.monkeys.get(name)?.clone() {
            let lhs = self.solve(&calculation.lhs)?;
            let rhs = self.solve(&calculation.rhs)?;
            let v = calculation.op.apply(lhs, rhs);
            self.monkeys.insert(name.to_owned(), Action::Number(v));
        }
        if let Action::Number(v) = self.monkeys.get(name)? {
            return Some(*v);
        }
        None
    }

    pub fn contains(&self, root: &str, child: &str) -> bool {
        if root == child {
            return true;
        }
        let Action::Calculation(calc) = self.monkeys.get(root).unwrap() else {
            return false;
        };
        self.contains(&calc.lhs, child) || self.contains(&calc.rhs, child)
    }

    pub fn get_equation(&mut self) -> String {
        let Action::Calculation(root) = self.monkeys.get("root").unwrap().clone() else {
            panic!("Nonono")
        };
        let lhs_contains_humn = self.contains(&root.lhs, "humn");
        if lhs_contains_humn {
            self.solve(&root.rhs);
        } else {
            self.solve(&root.lhs);
        }
        self.get_equation_part("root")
    }

    pub fn solve_equation(&mut self, variable_side: &str, fix_side: &str) -> Number {
        let _ = self.get_equation();
        // println!("\n{}", );
        match self.monkeys.get(variable_side).unwrap().clone() {
            Action::Number(x) => x,
            Action::Human => {
                let Action::Number(n) = self.monkeys.get(fix_side).unwrap() else {
                    panic!()
                };
                *n
            }
            Action::Calculation(r) => {
                if matches!(self.monkeys.get(&r.lhs).unwrap(), Action::Number(_))
                    && matches!(self.monkeys.get(&r.rhs).unwrap(), Action::Number(_))
                {
                    let Action::Number(x) = self.monkeys.get(&r.lhs).unwrap() else {
                        panic!()
                    };
                    return *x;
                }
                if let Action::Number(num) = self.monkeys.get(&r.lhs).unwrap().clone() {
                    // Number is on the left hand of the statement.
                    // If the operation is commutative, it does not matter.
                    // For example: 5 + x = 7 is the same as x + 5 = 7
                    let inverse = r.op.get_inverse();
                    if r.op.is_commutative() {
                        // println!("{} {}", inverse, num);
                        let Action::Number(n) = self.monkeys.get_mut(fix_side).unwrap() else {
                            panic!()
                        };
                        *n = inverse.apply(*n, num);
                        let other_side = self.monkeys.get(&r.rhs).unwrap().clone();
                        self.monkeys
                            .entry(variable_side.to_string())
                            .and_modify(|e| *e = other_side);
                    } else {
                        // It's not commutative (- or /)
                        // For example:
                        //  v
                        // (5 - x) = 7
                        // Here, we should move the variable side to the other side of the equals via the inverse operation
                        // 5 = 7 + x
                        let new_calc = Action::Calculation(Calculation {
                            lhs: r.lhs.clone(),
                            op: inverse,
                            rhs: r.rhs.clone(),
                        });
                        let new_fix = Action::Number(num);
                        let Action::Number(old_fix) = self.monkeys.get(fix_side).unwrap().clone()
                        else {
                            panic!()
                        };
                        self.monkeys
                            .entry(r.lhs.to_string())
                            .and_modify(|f| *f = Action::Number(old_fix));
                        self.monkeys
                            .entry(variable_side.to_string())
                            .and_modify(|f| *f = new_calc);
                        self.monkeys
                            .entry(fix_side.to_string())
                            .and_modify(|f| *f = new_fix);
                    }
                    return self.solve_equation(variable_side, fix_side);
                }
                let Action::Calculation(r) = self.monkeys.get(variable_side).unwrap().clone()
                else {
                    panic!()
                };
                let Action::Number(num) = self.monkeys.get(&r.rhs).unwrap().clone() else {
                    panic!()
                };
                // Number is on the right hand of the statement. Thus, we can inverse the operation on the lhs of the equals sign
                let inverse = r.op.get_inverse();
                // println!("{} {}", inverse, num);
                let Action::Number(n) = self.monkeys.get_mut(fix_side).unwrap() else {
                    panic!()
                };
                *n = inverse.apply(*n, num);
                let other_side = self.monkeys.get(&r.lhs).unwrap().clone();
                self.monkeys
                    .entry(variable_side.to_string())
                    .and_modify(|e| *e = other_side);
                self.solve_equation(variable_side, fix_side)
            }
        }
    }

    fn get_equation_part(&mut self, root: &str) -> String {
        let r = self.monkeys.get(root).unwrap().clone();
        match r {
            Action::Number(x) => x.to_string(),
            Action::Calculation(calc) => {
                if !self.contains(&calc.lhs, "humn") {
                    self.solve(&calc.lhs);
                }
                if !self.contains(&calc.rhs, "humn") {
                    self.solve(&calc.rhs);
                }
                format!(
                    "({} {} {})",
                    self.get_equation_part(&calc.lhs),
                    calc.op,
                    self.get_equation_part(&calc.rhs)
                )
            }
            Action::Human => root.to_owned(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::{read_example, read_input};

    use super::*;

    #[test]
    fn it_parses_monkey() -> color_eyre::Result<()> {
        let input = "root: pppw + sjmn";
        let monkey: Monkey = input.parse()?;
        assert_eq!("root", monkey.name);
        assert_eq!(
            Action::Calculation(Calculation {
                lhs: "pppw".to_owned(),
                op: Operator::Add,
                rhs: "sjmn".to_owned()
            }),
            monkey.action
        );

        let input = "dbpl: 5";
        let monkey: Monkey = input.parse()?;
        assert_eq!(
            Monkey {
                name: "dbpl".to_owned(),
                action: Action::Number(5)
            },
            monkey
        );
        Ok(())
    }

    #[test]
    fn it_solves() {
        let input = read_example("day21.txt");
        let mut monkeys: Monkeys = input.parse().unwrap();
        assert_eq!(Some(5), monkeys.solve("dbpl"));
        assert_eq!(Some(32), monkeys.solve("hmdt"));
        assert_eq!(Some(30), monkeys.solve("drzm"));
        assert_eq!(Some(152), monkeys.solve("root"));
    }
    #[test]
    fn it_solves_part1() {
        let input = read_input("day21.txt");
        let mut monkeys: Monkeys = input.parse().unwrap();
        assert_eq!(Some(194058098264286), monkeys.solve("root"));
    }

    #[test]
    fn it_finds_node_in_subtree() {
        let input = read_example("day21.txt");
        let monkeys: Monkeys = input.parse().unwrap();
        assert!(monkeys.contains("pppw", "humn"));
        assert!(!monkeys.contains("sjmn", "humn"));
    }

    #[test]
    fn that_real_input_does_not_have_humn_on_both_sides() {
        let input = read_input("day21.txt");
        let monkeys: Monkeys = input.parse().unwrap();
        let Action::Calculation(root) = monkeys.monkeys.get("root").unwrap() else {
            panic!("Nonono")
        };
        assert_ne!(
            monkeys.contains(&root.lhs, "humn"),
            monkeys.contains(&root.rhs, "humn")
        );
    }

    #[test]
    fn it_can_do_equation_where_operator_is_not_commutative() {
        // ( 5 - x = 3)
        let mut monkeys = HashMap::new();
        monkeys.insert(
            "root".to_string(),
            Action::Calculation(Calculation {
                lhs: "calc".to_string(),
                op: Operator::Eq,
                rhs: "three".to_string(),
            }),
        );
        monkeys.insert(
            "calc".to_string(),
            Action::Calculation(Calculation {
                lhs: "five".to_string(),
                op: Operator::Sub,
                rhs: "humn".to_string(),
            }),
        );
        monkeys.insert("humn".to_string(), Action::Human);
        monkeys.insert("three".to_string(), Action::Number(3));
        monkeys.insert("five".to_string(), Action::Number(5));
        let mut monkeys = Monkeys { monkeys };
        let x = monkeys.solve_equation("calc", "three");
        dbg!(x);
    }

    #[test]
    fn it_solves_equation_from_example() {
        let input = read_example("day21.txt");
        let mut monkeys: Monkeys = input.parse().unwrap();
        monkeys.monkeys.entry("root".to_string()).and_modify(|m| {
            if let Action::Calculation(c) = m {
                c.op = Operator::Eq
            }
        });
        monkeys
            .monkeys
            .entry("humn".to_string())
            .and_modify(|h| *h = Action::Human);
        let Action::Calculation(root) = monkeys.monkeys.get("root").unwrap().clone() else {
            panic!()
        };
        let result = if let Action::Number(_) = monkeys.monkeys.get(&root.lhs).unwrap() {
            monkeys.solve_equation(&root.rhs, &root.lhs)
        } else {
            monkeys.solve_equation(&root.lhs, &root.rhs)
        };
        assert_eq!(301, result);
    }

    #[test]
    fn it_solves_equation_from_input() {
        let input = read_input("day21.txt");
        let mut monkeys: Monkeys = input.parse().unwrap();
        monkeys.monkeys.entry("root".to_string()).and_modify(|m| {
            if let Action::Calculation(c) = m {
                c.op = Operator::Eq
            }
        });
        monkeys
            .monkeys
            .entry("humn".to_string())
            .and_modify(|h| *h = Action::Human);
        let Action::Calculation(root) = monkeys.monkeys.get("root").unwrap().clone() else {
            panic!()
        };
        let result = if let Action::Number(_) = monkeys.monkeys.get(&root.lhs).unwrap() {
            monkeys.solve_equation(&root.rhs, &root.lhs)
        } else {
            monkeys.solve_equation(&root.lhs, &root.rhs)
        };
        assert_eq!(3592056845086, result);
    }
}
