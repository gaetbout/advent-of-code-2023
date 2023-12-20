use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug, PartialEq)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction,
    Broadcaster,
}

// TODO This could be a true/false, but way more readable that way
#[derive(Debug, Clone, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    name: String,
    outputs: Vec<String>,
}

impl Module {
    fn new(module_type: ModuleType, name: &str) -> Module {
        Module {
            module_type,
            name: name.to_owned(),
            outputs: vec![],
        }
    }
}
#[derive(Debug)]
struct Signal {
    pulse: Pulse,
    from: String,
    to: String,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut modules = vec![];
    for line in input.trim_end().split('\n') {
        let mut all = line.split_ascii_whitespace();
        let first = all.next().unwrap();
        all.next();
        let outputs = all.map(|e| e.replace(",", "")).collect();
        let module_type = if first.contains('%') {
            ModuleType::FlipFlop(false)
        } else if first.contains('&') {
            ModuleType::Conjunction
        } else {
            ModuleType::Broadcaster
        };
        let mut module = Module::new(module_type, &first[1..first.len()]);
        module.outputs = outputs;
        modules.push(module);
    }

    let mut conj_memory_size: HashMap<String, usize> = HashMap::new();
    for module in &modules {
        for output in &module.outputs {
            if let Some(module) = modules.iter().find(|m| m.name.eq(output)) {
                if module.module_type == ModuleType::Conjunction {
                    let size = conj_memory_size.entry(output.clone()).or_default();
                    *size = *size + 1;
                }
            }
        }
    }

    let mut to_process: VecDeque<Signal> = VecDeque::new();
    let mut low = 0;
    let mut high = 0;
    let mut conj_memory: HashMap<String, HashSet<String>> = HashMap::new();
    for _ in 0..1000 {
        to_process.push_back(Signal {
            pulse: Pulse::Low,
            to: "roadcaster".to_string(), // Typo intended because I slice the first character above
            from: "button".to_string(),
        });

        'dir: while let Some(signal) = to_process.pop_front() {
            match signal.pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            }

            // println!("{} -{:?}-> {}", signal.from, signal.pulse, signal.to);

            let module = modules.iter_mut().find(|m| m.name == (signal.to));
            if module.is_none() {
                continue 'dir;
            }
            let module = module.unwrap();
            match module.module_type {
                ModuleType::FlipFlop(bool_type) => {
                    match signal.pulse {
                        Pulse::Low => {
                            let pulse = if bool_type { Pulse::Low } else { Pulse::High };

                            for to in &module.outputs {
                                to_process.push_back(Signal {
                                    pulse: pulse.clone(),
                                    to: to.to_string(),
                                    from: module.name.clone(),
                                })
                            }
                            module.module_type = ModuleType::FlipFlop(!bool_type);
                        }
                        Pulse::High => {} // Ignore
                    }
                }
                ModuleType::Conjunction => {
                    let memory = conj_memory.entry(module.name.clone()).or_default();
                    let str = &signal.from.clone();
                    match signal.pulse {
                        Pulse::Low => memory.remove(str),
                        Pulse::High => memory.insert(str.to_string()),
                    };
                    let pulse = if memory.len()
                        == *conj_memory_size.entry(module.name.clone()).or_default()
                    {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for to in &module.outputs {
                        to_process.push_back(Signal {
                            pulse: pulse.clone(),
                            to: to.to_string(),
                            from: module.name.clone(),
                        })
                    }
                }
                ModuleType::Broadcaster => {
                    for to in &module.outputs {
                        to_process.push_back(Signal {
                            pulse: Pulse::Low,
                            to: to.to_string(),
                            from: module.name.clone(),
                        })
                    }
                }
            }
        }
    }

    // println!("Low: {} High: {}", low, high);
    Some(low * high)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(32000000));
        assert_eq!(result, Some(886701120));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
