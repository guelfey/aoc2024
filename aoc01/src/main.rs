use std::io;

fn main() {
    let mut left : Vec<u32> = Vec::new();
    let mut right : Vec<u32> = Vec::new();

    loop {
        let mut line = String::new();
        
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            
        if bytes == 0 {
                break;
        }

        let mut s = line.split_whitespace();
        let a = s.next().expect("line has 1 element");
        let b = s.next().expect("line has 2 elements");
        left.push(a.parse().expect("element 1 is a number"));
        right.push(b.parse().expect("element 2 is a number"));
    }

    left.sort();
    right.sort();

    let mut total = 0;
    let mut ri = 0;
    let mut lastnum = 0;
    let mut lastscore = 0;
    for l in left.iter() {
        let mut count = 0;
        if *l == lastnum {
            total += lastscore;
            continue;
        }
        loop {
            if ri == right.len() || right[ri] > *l {
                break;
            }
            if right[ri] == *l {
                count += 1;
            }
            ri += 1;
        }
        lastscore = count * *l;
        total += lastscore;
        lastnum = *l;
    }

    println!("{total}");
}