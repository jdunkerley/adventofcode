use crate::shared::utils::lines;
use std::rc::{Rc, Weak};
use std::str::FromStr;

#[derive(Debug)]
enum SnailFishType {
    Number(i32),
    Pair,
}

#[derive(Debug)]
struct SnailFish {
    parent: Option<Weak<SnailFish>>,
    fish_type: SnailFishType,
    left: Option<Rc<SnailFish>>,
    right: Option<Rc<SnailFish>>,
}

impl SnailFish {
    pub fn new_number(value: i32) -> Rc<SnailFish> {
        Rc::new(SnailFish {
            parent: None,
            fish_type: SnailFishType::Number(value),
            left: None,
            right: None,
        })
    }

    pub fn new_pair(mut left: Rc<SnailFish>, mut right: Rc<SnailFish>) -> Rc<SnailFish> {
        let mut output = SnailFish {
            parent: None,
            fish_type: SnailFishType::Pair,
            left: None,
            right: None,
        };

        let rc = Rc::new(output);
        left.set_parent(Rc::downgrade(&rc));
        right.set_parent(Rc::downgrade(&rc));
        output.left = Some(left);
        output.right = Some(right);

        rc
    }

    pub fn set_parent(&mut self, parent_weak: Weak<SnailFish>) {
        self.parent = Some(parent_weak);
    }

    /*
    fn add(self: &Self, new: &SnailFish) -> SnailFish {
        let mut result = SnailFish::Pair(Box::new(*self), Box::new(*new));

        while result.explode(0).is_some() {}

        result
    }

    fn add_left(self: &mut Self, value: i32) {
        if let SnailFish::Number(ref mut number) = self {
            *number += value;
        } else if let SnailFish::Pair(_, right) = self {
            right.add_right(value);
        }
    }

    fn add_right(self: &mut Self, value: i32) {
        if let SnailFish::Number(ref mut number) = self {
            *number += value;
        } else if let SnailFish::Pair(left, _) = self {
            left.add_left(value);
        }
    }

    fn explode(self: &mut Self, depth: u8) -> Option<(i32, i32)> {
        match self {
            SnailFish::Number(_) => None,
            SnailFish::Pair(left, right) => {
                if depth == 3 {
                    match left {
                        SnailFish::Number(left_value) => match right {
                            SnailFish::Number(right_value) => {}
                            _ => panic!("Uh Oh - Right: {:?}", right),
                        },
                        _ => panic!("Uh Oh - Left: {:?}", left),
                    }
                } else {
                    if let Some((l, r)) = left.explode(depth + 1) {
                        right.add_left(r);
                        Some((l, 0))
                    } else if let Some((l, r)) = right.explode(depth + 1) {
                        left.add_right(l);
                        Some((0, r))
                    } else {
                        None
                    }
                }
            }
        }
    }
    */
}

impl FromStr for SnailFish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn inner_parse(s: &str, i: usize) -> Result<(SnailFish, usize), ()> {
            let len = s.len();
            if s[i..i + 1] == *"[" {
                match inner_parse(s, i + 1) {
                    Err(e) => Err(e),
                    Ok((left, j)) => {
                        if s[j..j + 1] != *"," {
                            Err(())
                        } else {
                            match inner_parse(s, j + 1) {
                                Err(e) => Err(e),
                                Ok((right, k)) => {
                                    if s[k..k + 1] != *"]" {
                                        Err(())
                                    } else {
                                        let result = SnailFish::new_pair(left, right);
                                        Ok((result, k + 1))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let mut j = i;
                while j < len && s[j..j + 1] != *"," && s[j..j + 1] != *"]" {
                    j += 1;
                }
                let value = s[i..j].parse::<i32>();
                match value {
                    Ok(v) => Ok((SnailFish::new_number(v), j)),
                    Err(_) => Err(()),
                }
            }
        }

        let temp = inner_parse(s, 0);
        if temp.is_err() {
            Err(())
        } else {
            Ok(temp.unwrap().0)
        }
    }
}

fn process(input: &str) {
    let lines = lines(input);

    let mut current = SnailFish::from_str(&lines[0]);
    for line in lines.iter().skip(1) {
        let new = SnailFish::from_str(line);
    }
}

pub fn run() {
    let a = SnailFish::from_str("[1,[2,[4,5]]").unwrap();
    println!("{:?}", a);

    let b = SnailFish::from_str("[2,2]").unwrap();
    println!("{:?}", b);

    let test_input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
    process(test_input);
}
