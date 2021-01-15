pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let mut ns: Vec<u8> = read_line().split(' ').map(|n| n.trim().parse().unwrap()).collect();

    ns.sort_unstable();

    let s: String = read_line()
        .trim()
        .chars()
        .map(|c| ns[(c as u8 - b'A') as usize])
        .map(|c| format!("{} ", c))
        .collect();

    println!("{}", s.trim());
}
