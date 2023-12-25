use std::fs;
use std::collections::HashMap;
use regex::Regex;

#[cfg(windows)]
const EMPTY_LINE : &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const EMPTY_LINE : &'static str = "\n\n";

const FILE_PATH: &str = "./input/input.txt";

#[derive(Copy, Clone, Debug)]
struct Datapoint {
    x: u64,
    m: u64,
    a: u64,
    s: u64
}

impl Datapoint {
    fn get(&self, which: &str) -> u64 {
        match which {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => 0
        }
    }

    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl From<Vec<u64>> for Datapoint {
    fn from(item: Vec<u64>) -> Self {
        Datapoint {
            x: item[0],
            m: item[1],
            a: item[2],
            s: item[3]
        }
    }
}

// struct
fn main() { 
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let contents = contents.split(EMPTY_LINE).collect::<Vec<_>>();

    let re = Regex::new(r"(?<name>.+)\{(?<workflow>.+)\}").unwrap();
    let instr_re = Regex::new(r"(?<key>.)(?<op>.)(?<value>.+)").unwrap();

    let mut instructions = HashMap::new();
    contents[0]
        .lines().for_each(|l| {
            let caps = re.captures(l).unwrap();
            let name = caps.name("name").unwrap().as_str();
            let workflow = caps
                .name("workflow")
                .unwrap()
                .as_str()
                .split(",")
                .map(|i| i
                    .split(":")
                    .collect::<Vec<&str>>()
                )
                .map(|i| if i.len() == 1 {
                    ("", "", 0, i[0])
                } else {
                    let caps = instr_re.captures(i[0]).unwrap();
                    let key = caps.name("key").unwrap().as_str();
                    let op = caps.name("op").unwrap().as_str();
                    let value = caps.name("value").unwrap().as_str().parse::<u64>().unwrap();
                    (key, op, value, i[1])
                })
                .collect::<Vec<(&str, &str, u64, &str)>>();
            instructions.insert(name, workflow);
    });
    instructions.clone().into_iter().for_each(|(k, mut v)| {
        v.reverse();
        v.dedup_by(|a, b| a.3 == b.3);
        v.reverse();
        instructions.insert(k, v);
    });
    
    let re = Regex::new(r"\{(?<match>.*)\}").unwrap();
    let data = contents[1]
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();

            let d = caps.get(1).unwrap()
                .as_str()
                .split(",")
                .map(|x| {
                    let out = x.split("=").collect::<Vec<&str>>();
                    let value = u64::from_str_radix(out[1], 10).unwrap();
                    value
                }
                ).collect::<Vec<u64>>();
            Datapoint::from(d)

        })
        .collect::<Vec<Datapoint>>();

        let task1 = data.iter().map(|d| {
            let mut res = String::from("");
            let mut instr_name = String::from("in");
            let end_states = ["A".to_string(), "R".to_string()];
            while !end_states.contains(&res) {
                let mut instr = instructions.get(&instr_name[..]).unwrap().clone();
                res = process_instruction(d, &mut instr);
                instr_name = res.clone();
            }
            if res == "A" { d.sum() } else {0}
        }).sum::<u64>();
    println!("Task 1: {}", task1);

    instructions.clone().keys().for_each(|k| {
        let i: &Vec<(&str, &str, u64, &str)> = instructions.get(*k).unwrap();
        let length = i.len() - 1;
        let mut inst = i[length];
        if length == 0 {
            inst.0 = "x";
            inst.1 = ">";
        } else {
            let (k, op, val, _) = i[length - 1];
            inst.0 = k;
            if op == ">" {
                inst.1 = "<";
                inst.2 = val + 1;

            } else {
                inst.1 = ">";
                inst.2 = val - 1;
            };
        }
        let mut new_inst = i.clone();
        new_inst[length] = inst;
        instructions.insert(k, new_inst);
    });
    println!("{:?}", instructions);

    // let mut all_paths: Vec<(&str, (&str, &str, u64, &str))> = vec![];
    // let mut path: Vec<(&str, (&str, &str, u64, &str))> = vec![];
    // let end_states = ["A".to_string(), "R".to_string()];
    // let mut res = String::from("in");
    // while !end_states.contains(&res) {
    //     let mut instr = instructions.get(&res[..]).unwrap().clone();
    //     res = get_path(d, &mut instr);
    //     instr_name = res.clone();

    // }
}

fn process_instruction(d: &Datapoint, instruction: &mut Vec<(&str, &str, u64, &str)>) -> String {
    if instruction.len() == 1 { return instruction[0].3.to_string()}
    let (key, op, value, target) = instruction[0];
    let res = if op == ">" {
        d.get(key) > value
    } else {
        d.get(key) < value
    };
    if res {
        return target.to_string();
    } else {
        process_instruction(d, &mut instruction[1..].to_vec())
    }
}
