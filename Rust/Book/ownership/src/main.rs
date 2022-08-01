

fn print_type_of<T>(_: &T){
    println!("{}", std::any::type_name::<T>());
}

fn string_push(s:&mut String, app: &str){
    s.push_str(app);
}

fn test(){
    let mut s = String::from("wtf");

    string_push(&mut s, " blabla shii");

    println!("{s}");
}

fn main() {
    test();
}
