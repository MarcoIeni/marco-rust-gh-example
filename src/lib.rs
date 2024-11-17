#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

/// Print a [`Poin`] struct
pub fn print_point() {
    let x = Some(Point { x: 1, y: 2 });
    if let Some(x) = x {
        println!("point {}, {}", x.x, x.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        print_point();
    }
}
