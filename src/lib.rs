pub fn hello() {
    let x = Some(5);
    match x {
        Some(x) => println!("{x}"),
        None => {}
    }
}
