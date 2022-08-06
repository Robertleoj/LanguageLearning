fn bla2(s : &mut String) {
    s.push_str("hoho");
}


fn bla(s : &mut String) {
    s.push_str("hehe");
    bla2(s);
}

fn main() {
    let mut b = "heheh ".to_string();

    bla(&mut b);

    println!("{b}");
}
