fn main() {
    let result: u32 = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .enumerate()
        .filter_map(|(game_id, line)| {
            line.splitn(2, |b| b == &b':')
                .nth(1)
                .unwrap()
                .split(|b| b == &b';' || b == &b',')
                .all(|item| {
                    let i = match item[1..].splitn(2, |b| *b == b' ').nth(1).unwrap() {
                        b"red" => 0usize,
                        b"green" => 1,
                        b"blue" => 2,
                        _ => unreachable!(),
                    };
                    let count: u32 = atoi::atoi(&item[1..]).unwrap();
                    count <= 12 + i as u32
                })
                .then_some((game_id + 1) as u32)
        })
        .sum();

    println!("{}", result);
}
