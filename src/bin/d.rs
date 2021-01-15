pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn is_arithmetic(arr: &[i64]) -> bool {
    if arr.len() > 2 {
        let a = arr[0];
        let b = arr[1];
        let diff = b - a;
        let mut last = b;
        for &c in &arr[2..] {
            if c - last != diff {
                return false;
            }
            last = c;
        }
        true
    } else {
        true
    }
}

fn main() {
    let n: usize = read_line().trim().parse().unwrap();

    for _ in 0..n {
        let line = read_line();
        let mut line = line.trim().split(' ');

        let _: u8 = line.next().unwrap().parse().unwrap();

        let mut seq: Vec<i64> = line.map(|n| n.parse().unwrap()).collect();

        if is_arithmetic(&seq) {
            println!("arithmetic");
        } else {
            seq.sort_unstable();
            if is_arithmetic(&seq) {
                println!("permuted arithmetic");
            } else {
                println!("non-arithmetic");
            }
        }
    }
}
