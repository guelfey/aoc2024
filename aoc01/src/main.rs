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
    for (i, l) in left.iter().enumerate() {
        let r = right[i];
        total += if r > *l {
            r - *l
        } else {
            *l - r
        };
    }

    println!("{total}");
}