


fn swap<T>(x: &mut T, y: &mut T) {
    
    let tmp = *y;
    *y = *x;
    *x = tmp;
}


fn main() {

    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {r}");

}
