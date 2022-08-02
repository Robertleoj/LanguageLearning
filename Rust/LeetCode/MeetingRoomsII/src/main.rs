use std::collections::BTreeMap;
use std::cmp;
struct Solution;

#[derive(Debug)]
struct IntervalNode {
    starts:i32,
    ends: i32,
}

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        // insert into hasbtmap
        let mut btmap = BTreeMap::<i32, IntervalNode>::new();

        for intval in intervals {
            let (b, e) = (intval[0], intval[1]);

            match btmap.get_mut(&b) {
                Some(n) => {
                    n.starts += 1;
                },
                None => {
                    btmap.insert(b, IntervalNode {
                        starts: 1,
                        ends: 0,
                    });
                }
            }

            match btmap.get_mut(&e) {
                Some(n) => {
                    n.ends += 1;
                },
                None => {
                    btmap.insert(e, IntervalNode {
                        starts: 0,
                        ends: 1,
                    });
                }
            }
        }

        let mut mx : i32 = 0;
        let mut current: i32 = 0;

        for (val, i) in btmap.iter() {

            // println!("{} {:?}", val, i);
            current += i.starts;
            current -= i.ends;

            mx = cmp::max(current, mx);
        }

        assert_eq!(current, 0);

        mx

    }
}

fn main() {

    let v = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
    println!("{:?}", Solution::min_meeting_rooms(v))

}
