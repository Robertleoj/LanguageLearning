use rand::Rng;

fn num(x: i32) -> i32 {
    let y = x * 10;
    y + 8
}

fn rand_int(min:i32, max:i32) -> i32 {
    rand::thread_rng().gen_range(min..max)
}


fn main() {
    let y = {
        let x = if rand_int(0, 10) < 10 {0} else {40};
        num(x)
    };

    println!("{y}");
}
