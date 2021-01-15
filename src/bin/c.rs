use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let n: usize = read_line().trim().parse().unwrap();

    let mut friends: HashMap<String, (u64, String)> = HashMap::with_capacity(n);

    for _ in 0..n {
        let line = read_line();
        let mut line = line.trim().split(' ');
        let name = line.next().unwrap().to_owned();
        let c: u64 = line.next().unwrap().parse().unwrap();
        let date = line.next().unwrap().to_owned();

        match friends.entry(date) {
            Entry::Occupied(mut occupied) => {
                if occupied.get().0 < c {
                    occupied.insert((c, name));
                }
            }
            Entry::Vacant(vacant) => {
                vacant.insert((c, name));
            }
        }
    }
    let mut friends_to_remember: Vec<_> = friends.into_iter().map(|(_, (_, name))| name).collect();
    friends_to_remember.sort();
    println!("{}", friends_to_remember.len());
    for name in friends_to_remember {
        println!("{}", name);
    }
}
