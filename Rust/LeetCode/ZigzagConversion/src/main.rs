struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows : i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows: usize = num_rows as usize;

        let chars: Vec<_> = s.chars().collect();

        let mut num_between_bottom:usize = (num_rows - 2) * 2 + 1;
        let mut num_between_top: usize = 0;

        let mut out = String::new();


        for row in 0..num_rows {
            println!("row: {row} num_between_bottom: {num_between_bottom} num_between_top: {num_between_top}");

            let mut down = true;
            let mut i = row;
            while i < chars.len() {
                out.push(chars[i]);
                if down {
                    if row == num_rows -1 {
                        i += num_between_top + 1;
                    } else {
                        down = !down;
                        i += num_between_bottom + 1;
                    }
                } else {
                    if row == 0 {
                        i += num_between_bottom + 1;
                    } else {
                        down = !down;
                        i += num_between_top + 1;
                    }
                }
            }
            if row != num_rows -1 {
                num_between_top += if num_between_top == 0 {1} else {2};
                num_between_bottom -= if num_between_bottom == 1 {1} else {2};
            }
        }
        out

    }
}
fn main() {
    let s = "PAYPALISHIRING".to_string();
    println!("{}", Solution::convert(s, 3));
}
