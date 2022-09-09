pub fn run() {
    let mut x = 0;
    x = 1;
    x = 2;
    x = 3;
    {
        let mut y = 0;
        y = 1;
        y = 2;
    }
    x = 4;
    x = 5;
}
