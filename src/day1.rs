use crate::solve::{Scanner, Solution};

pub struct Day1Pt1;

impl Solution for Day1Pt1 {
    fn solve(&self) {
        let mut reader = Scanner::default();

        let mut list1 = Vec::<i64>::new();
        let mut list2 = Vec::<i64>::new();
        let sz: usize = 1000;

        for _ in 0..sz {
            list1.push(reader.next());
            list2.push(reader.next());
        }

        list1.sort();
        list2.sort();

        let mut sum: i64 = 0;
        for i in 0..sz {
            sum += (list1[i] - list2[i]).abs();
        }

        println!("{sum}");
    }
}

pub struct Day1Pt2;

impl Solution for Day1Pt2 {
    fn solve(&self) {
        let mut reader = Scanner::default();

        let mut list1 = Vec::<i64>::new();
        let mut freqs2 = HashMap::<i64, i32>::new();
        let sz: usize = 1000;

        for _ in 0..sz {
            list1.push(reader.next());
            let n2: i64 = reader.next();

            match freqs2.get(&n2) {
                Some(n2_val) => {
                    freqs2.insert(n2, n2_val + 1);
                }
                None => {
                    freqs2.insert(n2, 1);
                }
            }
        }

        let mut sum: i64 = 0;
        for i in 0..sz {
            let n1 = list1[i];
            sum += match freqs2.get(&n1) {
                Some(freq) => n1 * (*freq as i64),
                None => 0,
            }
        }

        println!("{sum}");
    }
}
