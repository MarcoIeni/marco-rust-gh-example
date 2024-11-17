pub fn hello() {
    let x = Some(5);
    if let Some(x) = x {
        println!("{x}")
    }
}
