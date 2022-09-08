enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}

pub fn run() {
    let windows = OS::Windows(1985, String::from("MicroSoft"));
    print_os_info(windows);
    let mac = OS::Mac(2001, String::from("Apple"));
    print_os_info(mac);
    let linux = OS::Linux(1991, String::from("Linus"));
    print_os_info(linux);
}
fn print_os_info(os: OS) {
    match os {
        OS::Windows(birthyear, developer) => {
            println!(
                "Windows: First release in {} years by {}.",
                birthyear, developer
            )
        }
        OS::Mac(birthyear, developer) => {
            println!(
                "Mac: First release in {} years by {}.",
                birthyear, developer
            )
        }
        OS::Linux(birthyear, developer) => {
            println!(
                "Linux: First release in {} years by {}.",
                birthyear, developer
            )
        }
    }
}
