use std::fs;

#[derive(Debug)]
struct Packet {
    v: u8,
    t: u8,
    literal: i64,
    children: Vec<Packet>,
}

impl Packet {
    fn v_total(self: &Self) -> u64 {
        self.v as u64 + self.children.iter().map(|c| c.v_total()).sum::<u64>()
    }

    fn eval(self: &Self) -> i64 {
        match self.t {
            0 => self.children.iter().map(|c| c.eval()).sum(),
            1 => self.children.iter().map(|c| c.eval()).product(),
            2 => self.children.iter().map(|c| c.eval()).min().unwrap(),
            3 => self.children.iter().map(|c| c.eval()).max().unwrap(),
            4 => self.literal,
            5 => {
                if self.children[0].eval() > self.children[1].eval() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if self.children[0].eval() < self.children[1].eval() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if self.children[0].eval() == self.children[1].eval() {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Uh Oh!"),
        }
    }
}

fn map(hex: char) -> u8 {
    match hex {
        '0'..='9' => (hex as i8 - '0' as i8) as u8,
        'A'..='F' => (hex as i8 - 'A' as i8 + 10) as u8,
        _ => panic!("Invalid Character {}", hex),
    }
}

fn parse(bits: &Vec<char>, index: usize) -> (Packet, usize) {
    let v = read_number(bits, index, 3) as u8;
    let t = read_number(bits, index + 3, 3) as u8;

    let mut cur_index = index + 6;
    let mut children = Vec::new();
    if t == 4 {
        // Literal
        let (literal, new_index) = read_literal(bits, cur_index);
        return (
            Packet {
                v,
                t,
                literal,
                children,
            },
            new_index,
        );
    }

    if bits[cur_index] == '0' {
        // Set of literals
        let end_index = read_number(bits, cur_index + 1, 15) as usize + 16 + cur_index;
        cur_index += 16;
        while cur_index < end_index {
            let (child, new_index) = parse(bits, cur_index);
            children.push(child);
            cur_index = new_index;
        }
    } else {
        // Set of children
        let len = read_number(bits, cur_index + 1, 11) as usize;
        cur_index += 12;
        for _ in 0..len {
            let (child, new_index) = parse(bits, cur_index);
            children.push(child);
            cur_index = new_index;
        }
    }

    (
        Packet {
            v,
            t,
            literal: 0,
            children,
        },
        cur_index,
    )
}

fn read_literal(bits: &Vec<char>, index: usize) -> (i64, usize) {
    let mut text = String::new();

    let mut cur_index = index;
    let mut finished = false;

    while !finished {
        finished = bits[cur_index] == '0';
        text.push(bits[cur_index + 1]);
        text.push(bits[cur_index + 2]);
        text.push(bits[cur_index + 3]);
        text.push(bits[cur_index + 4]);
        cur_index += 5;
    }

    (i64::from_str_radix(&text, 2).unwrap(), cur_index)
}

fn read_number(bits: &Vec<char>, index: usize, len: usize) -> u64 {
    let mut value = 0;
    for i in index..index + len {
        value = 2 * value + if bits[i] == '1' { 1 } else { 0 };
    }

    value
}

fn process(input: &str) {
    let bits = input
        .chars()
        .map(|h| format!("{:04b}", map(h)))
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>();

    let parsed = parse(&bits, 0);
    println!("{} {} {:?}", parsed.0.v_total(), parsed.0.eval(), parsed.0);
}

pub fn run() {
    let test_input = "D2FE28";
    println!("Test Input:");
    process(test_input);
    process("38006F45291200");
    process("EE00D40C823060");
    process("8A004A801A8002F478");

    let input = fs::read_to_string("data/day16.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}
