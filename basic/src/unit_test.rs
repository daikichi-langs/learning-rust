struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn compare(&self, other: &Rectangle) -> bool {
        &self.height * &self.width > other.height * other.width
    }
}

fn is_double(i: i32) -> i32 {
    i * 2
}
fn is_greeting(name: &str) -> String {
    format!("Hello! {}!!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_larger_test() {
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
    fn is_smaller_test() {
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
    fn is_double_test() {
        assert_eq!(6, is_double(3));
    }
    #[test]
    fn is_greeting_test() {
        let res = is_greeting("rust");
        assert!(res.contains("rust"));
    }
}
