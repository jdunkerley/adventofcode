pub fn lines(input: &str) -> Vec<String> {
    input.lines()
        .map(|x| x.to_string())
        .filter(|x| x.trim() != "")
        .collect()
}

pub fn lines_to_i64(lines: Vec<String>) -> Vec<i64>
{
    let mut v: Vec<i64> = Vec::new();
    for x in lines {
        let i = x.parse::<i64>();
        match i {
            Err(e) => panic!("Invalid input passed {}: {}", x, e),
            Ok(i_val) => v.push(i_val)
        };
    }

    v
}
