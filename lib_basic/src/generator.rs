use rand::Rng;
pub fn gen_ran() {
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen(); // 0 ~ 255
    n
}
