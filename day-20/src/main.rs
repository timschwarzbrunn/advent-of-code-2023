use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default)]
enum Task {
    #[default]
    First,
    Second,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum SignalType {
    LOW,
    HIGH,
}

#[derive(Debug, PartialEq, Eq)]
struct Signal {
    sender: String,
    receiver: String,
    signal_type: SignalType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType {
    // Broadcaster: broadcaster
    // Sends the pulse it receives to all of its connected outputs.
    Broadcaster,
    // FlipFlop: %
    // A FlipFlop is either ON or OFF (default: OFF).
    // If it receives a HIGH pulse, it is ignored and nothing happens.
    // If it receives a LOW pulse, it flips between ON and OFF.
    // If it changes from OFF -> ON it sends a HIGH pulse.
    // If it changes from ON -> OFF it sends a LOW pulse.
    FlipFlop(bool),
    // Conjunction: &
    // Remembers what signal ALL his inputs send to it (default: LOW).
    // If all remembered inputs are HIGH, it sends a LOW pulse.
    // Otherwise it sends a HIGH pulse.
    // So for this one we need to check how many other modules transfer
    // signals to it.
    Conjunction(HashMap<String, SignalType>),
}

impl ModuleType {
    fn process_signal(&mut self, signal: Signal) -> Option<SignalType> {
        match self {
            ModuleType::Broadcaster => {
                return Some(signal.signal_type);
            }
            ModuleType::FlipFlop(flip_flop_state) => match signal.signal_type {
                SignalType::HIGH => {
                    return None;
                }
                SignalType::LOW => {
                    *flip_flop_state = !*flip_flop_state;
                    match flip_flop_state {
                        true => return Some(SignalType::HIGH),
                        false => return Some(SignalType::LOW),
                    }
                }
            },
            ModuleType::Conjunction(saved_input_signals) => {
                // Update the last received signal from this sender.
                let senders_last_signal_type = saved_input_signals.get_mut(&signal.sender).unwrap();
                *senders_last_signal_type = signal.signal_type;
                // Check if all remembered last signals are HIGH or not.
                if saved_input_signals
                    .values()
                    .all(|signal_type| *signal_type == SignalType::HIGH)
                {
                    return Some(SignalType::LOW);
                } else {
                    return Some(SignalType::HIGH);
                };
            }
        }
    }
}

#[derive(Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    outputs: Vec<String>,
}

impl Module {
    fn from_line(line: String) -> Self {
        // Example lines:
        // broadcaster -> a, b, c
        // %a -> inv, con
        // &con -> output
        let mut parts = line.split(" -> ");
        let mut name: String = parts.next().unwrap().to_string();
        let module_type = match name.chars().nth(0).unwrap() {
            '%' => ModuleType::FlipFlop(false),
            '&' => ModuleType::Conjunction(HashMap::new()),
            _ => ModuleType::Broadcaster,
        };
        if module_type != ModuleType::Broadcaster {
            // Remove the % or & at the start.
            name.remove(0);
        }
        let mut outputs: Vec<String> = parts
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(str::to_string)
            .collect();
        // Remove the ','.
        for idx in 0..outputs.len() - 1 {
            outputs[idx].pop();
        }
        Self {
            name,
            module_type,
            outputs,
        }
    }

    fn process_signal(&mut self, signal: Signal) -> Vec<Signal> {
        if let Some(signal_type) = self.module_type.process_signal(signal) {
            return self
                .outputs
                .iter()
                .map(|receiver| Signal {
                    sender: self.name.clone(),
                    receiver: (*receiver).clone(),
                    signal_type: signal_type.clone(),
                })
                .collect();
        } else {
            return Vec::new();
        }
    }
}

#[derive(Debug)]
struct ModuleCollection {
    modules: HashMap<String, Module>,
}

impl ModuleCollection {
    fn from_reader<B: BufRead>(reader: B) -> Self {
        let mut modules: HashMap<String, Module> = reader
            .lines()
            .map(Result::unwrap)
            .map(Module::from_line)
            .map(|module| (module.name.clone(), module))
            .collect();
        // Now we also need to check what modules are inputs for
        // a conjunction module.
        // First, find the names of all conjunction modules.
        let conjunction_modules: Vec<String> = modules
            .values()
            .filter(|module| match module.module_type {
                ModuleType::Conjunction(_) => true,
                _ => false,
            })
            .map(|module| module.name.clone())
            .collect();
        for conjunction_module in conjunction_modules.iter() {
            let inputs: HashMap<String, SignalType> = modules
                .values()
                .filter(|module| module.outputs.contains(conjunction_module))
                .map(|module| (module.name.clone(), SignalType::LOW))
                .collect();
            modules.get_mut(conjunction_module).unwrap().module_type =
                ModuleType::Conjunction(inputs);
        }
        Self { modules }
    }

    fn process_signal(&mut self, signal: Signal) -> Vec<Signal> {
        // Note that not all receivers must exist as a sender.
        if let Some(receiver) = self.modules.get_mut(&signal.receiver) {
            (*receiver).process_signal(signal)
        } else {
            Vec::new()
        }
    }

    fn count_signals(&mut self, number_of_button_presses: usize) -> usize {
        let mut n_press = 0;
        let mut number_of_low_signals = 0;
        let mut number_of_high_signals = 0;
        while n_press < number_of_button_presses {
            // Send the first signal to the broadcaster.
            n_press += 1;
            let mut signals_to_process: VecDeque<Signal> = VecDeque::new();
            signals_to_process.push_back(Signal {
                sender: "button".to_string(),
                receiver: "broadcaster".to_string(),
                signal_type: SignalType::LOW,
            });
            while !signals_to_process.is_empty() {
                let signal = signals_to_process.pop_front().unwrap();
                match signal.signal_type {
                    SignalType::LOW => number_of_low_signals += 1,
                    SignalType::HIGH => number_of_high_signals += 1,
                }
                signals_to_process.extend(self.process_signal(signal));
            }
        }
        number_of_low_signals * number_of_high_signals
    }

    fn find_signal_to_rx(&mut self) -> usize {
        // At first find the module which sends the signal to "rx".
        let sender_to_rx: Vec<String> = self
            .modules
            .values()
            .filter(|module| module.outputs.contains(&"rx".to_string()))
            .map(|module| module.name.clone())
            .collect();
        if sender_to_rx.len() != 1 {
            panic!("Too many rx!")
        }
        let sender_to_rx: String = sender_to_rx[0].clone();
        // It is just one conjunction module. We want the inputs of this conjunction
        // module so that we can calculate the number of presses for each of them
        // and calculate the least common multiple of these presses.
        let mut inputs_to_sender_to_rx: HashMap<String, usize>;
        if let ModuleType::Conjunction(inputs) =
            self.modules.get(&sender_to_rx).unwrap().module_type.clone()
        {
            inputs_to_sender_to_rx = inputs.keys().map(|key| (key.clone(), 0)).collect();
        } else {
            panic!("Expected conjunction module!");
        }
        let mut n_press = 0;
        loop {
            n_press += 1;
            // Send the first signal to the broadcaster.
            let mut signals_to_process: Vec<Signal> = vec![Signal {
                sender: "button".to_string(),
                receiver: "broadcaster".to_string(),
                signal_type: SignalType::LOW,
            }];
            while !signals_to_process.is_empty() {
                let signal = signals_to_process.remove(0);
                if inputs_to_sender_to_rx.contains_key(&signal.receiver)
                    && signal.signal_type == SignalType::LOW
                {
                    // It is one of the senders.
                    let current_value = inputs_to_sender_to_rx.get_mut(&signal.receiver).unwrap();
                    if *current_value == 0 {
                        *current_value = n_press;
                        // Check if we can finish.
                        if inputs_to_sender_to_rx.values().all(|value| *value != 0) {
                            // We need the least common multiple but the product of all is also valid since in this case its also the lcm.
                            return inputs_to_sender_to_rx.values().fold(1, |acc, &x| acc * x);
                        }
                    }
                }
                signals_to_process.extend(self.process_signal(signal));
            }
        }
    }
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let mut module_collection = ModuleCollection::from_reader(reader);
    module_collection.count_signals(1000)
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let mut module_collection = ModuleCollection::from_reader(reader);
    module_collection.find_signal_to_rx()
}

fn main() {
    let mut args = std::env::args().skip(1);
    let filename = args.next().unwrap_or_else(|| String::from("./input"));
    let task = args
        .next()
        .map(|arg| match arg.as_str() {
            "first" => Task::First,
            "second" => Task::Second,
            _ => unreachable!(),
        })
        .unwrap_or_default();
    let reader = BufReader::new(File::open(filename).expect("Input file not found."));

    println!(
        "{:?} task solution: {:?}",
        task,
        match task {
            Task::First => solve_first_task(reader),
            Task::Second => solve_second_task(reader),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_task_first_example() {
        let reader = BufReader::new(File::open("./input1.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 32000000);
    }

    #[test]
    fn test_first_task_second_example() {
        let reader = BufReader::new(File::open("./input2.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 11687500);
    }
}
