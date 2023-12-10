const WORDS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn main() {
    let result = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            (0..line.len()).find_map(|i| get_num(line, i)).unwrap() * 10
                + (0..line.len())
                    .rev()
                    .find_map(|i| get_num(line, i))
                    .unwrap()
        })
        .sum::<usize>();

    println!("{}", result)
}

fn get_num(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as usize)
        .or(WORDS
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(num, _)| num + 1))
}
