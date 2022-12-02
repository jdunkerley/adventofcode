fn process(input: &str) {
    let ((l, r), (t, b)) = get_ranges(input);

    let mut max_height = 0;
    for vx in 0..=r {
        let mut s = 0;
        let mut x = 0;
        while x <= r {}
    }
}

fn get_ranges(input: &str) -> ((i16, i16), (i16, i16)) {
    let x_middle = input.find("..").unwrap();
    let x_end = input.find(",").unwrap();
    let left = input[15..x_middle].parse::<i16>().unwrap();
    let right = input[x_middle + 2..x_end].parse::<i16>().unwrap();

    let y_middle = input[x_end + 4..].find("..").unwrap();
    let top = input[x_end + 4..x_end + 4 + y_middle]
        .parse::<i16>()
        .unwrap();
    let bottom = input[x_end + 6 + y_middle..].parse::<i16>().unwrap();
    ((left, right), (top, bottom))
}

pub fn run() {
    let test_input = "target area: x=20..30, y=-10..-5";
    println!("Test Input:");
    process(test_input);

    let input = "target area: x=253..280, y=-73..-46";
    println!("Real Input:");
    process(input);
}
