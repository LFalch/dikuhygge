pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let ns: Vec<u64> = read_line().split(' ').map(|n| n.trim().parse().unwrap()).collect();
    match &*ns {
        &[b, b_r, b_s, a, a_s] => {
            let bob_savings = b_s * (b_r - b);
            let mut a_r = a;

            let mut alice_saving = 0;

            while alice_saving <= bob_savings {
                alice_saving += a_s;
                a_r += 1;
            }

            println!("{}", a_r);
        }
        _ => unimplemented!(),
    }
}
