fn power_level(x: u32, y: u32, serial: u32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial;
    power *= rack_id;
    let power_str = power.to_string();
    let third_digit: i32 = power_str
        .to_string()
        .chars()
        .rev()
        .nth(2)
        .unwrap_or('0')
        .to_string()
        .parse()
        .unwrap();
    third_digit - 5
}

fn square_finder(size: usize, serial: u32) -> (u32, u32) {
    let ranges = (1..=300).collect::<Vec<u32>>();
    let candidate_windows = ranges
        .windows(size)
        .map(|x| {
            ranges
                .windows(size)
                .map(move |y| iproduct!(x, y).collect::<Vec<(&u32, &u32)>>())
        })
        .flatten();
    let lvls = candidate_windows.map(|q| {
        (
            (*q[0].0, *q[0].1),
            q.iter()
                .map(|(x, y)| power_level(**x, **y, serial))
                .sum::<i32>(),
        )
    });
    lvls.max_by_key(|x| x.1).unwrap().0
}

pub fn run(serial: u32) -> (u32, u32) {
    square_finder(3, serial)
}
