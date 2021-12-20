use std::cmp;

pub fn lines(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|x| x.trim().to_string())
        .filter(|x| x != "")
        .collect()
}

pub fn lines_to_i64(lines: Vec<String>) -> Vec<i64> {
    let mut v: Vec<i64> = Vec::new();
    for x in lines {
        let i = x.parse::<i64>();
        match i {
            Err(e) => panic!("Invalid input passed {}: {}", x, e),
            Ok(i_val) => v.push(i_val),
        };
    }

    v
}

pub fn min_max<I>(iter: I) -> Option<(i16, i16)>
where
    I: IntoIterator<Item = i16>,
{
    let mut output: Option<(i16, i16)> = None;

    for v in iter.into_iter() {
        if let Some(current) = output {
            output = Some((cmp::min(current.0, v), cmp::max(current.1, v)));
        } else {
            output = Some((v, v));
        }
    }

    output
}
