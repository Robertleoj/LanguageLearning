struct Solution;

impl Solution {



    fn get_expr(tokens: &Vec<String>, idx: i32) -> (i32, i32) {

        let curr_token = tokens[idx as usize].as_str();

        match curr_token {
            "+" | "-" | "*" | "/" => {
                let (num1, i1) = Self::get_expr(tokens, idx - 1);
                let (num2, i2) = Self::get_expr(tokens, i1);
                return (match curr_token {
                    "+" => num2 + num1,
                    "-" => num2 - num1,
                    "*" => num2 * num1,
                    "/" => num2 / num1,
                    _ => {panic!("Invalid token");}
                }, i2)
            },
            _ => {
                let num: i32 = curr_token.parse().expect("Invalid number");
                return (num, idx - 1);
            }
        }
    }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        return Self::get_expr(&tokens, tokens.len() as i32 - 1).0;
    }

}
fn main() {
    let arr = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
        .map(|x| x.to_string())
        .to_vec();

    println!("{:#?}", Solution::eval_rpn(arr));
}
