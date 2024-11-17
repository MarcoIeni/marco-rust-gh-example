pub fn hello() {
    let x = Some(5);
    if let Some(x) = x {
        println!("{x}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        hello();
    }
}
