struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn compare(&self, other: &Rectangle) -> bool {
        &self.height * &self.width > other.height * other.width
    }
}

fn double(i: i32) -> i32 {
    i * 2
}
fn greet(name: &str) -> String {
    format!("Hello! {}!!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_larger() {
        let rect = Rectangle {
            height: 5,
            width: 5,
        };
        let rectangle = Rectangle {
            height: 3,
            width: 3,
        };
        assert!(rect.compare(&rectangle));
    }
    #[test]
    fn is_smaller() {
        let rect = Rectangle {
            height: 3,
            width: 3,
        };
        let rectangle = Rectangle {
            height: 5,
            width: 5,
        };
        assert!(!(rect.compare(&rectangle)));
    }
    #[test]
    fn is_double() {
        assert_eq!(6, double(3));
    }
    #[test]
    fn is_greeting() {
        let res = greet("rust");
        assert!(res.contains("rust"));
    }
}
