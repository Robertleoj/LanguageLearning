fn test1(){
    let mut k = 0;

    let x = 'bla: loop {
        k += 1;
        if k > 10{
            break 'bla k + 2;
        }
    };

    println!("x = {x}");
}


fn test2(){
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("element = {element}");
    }
}

fn test3() {
    let a = [0..10];
    for i in a{
        print!("{i} ")
    }
}

fn main() {
    test3();
}
