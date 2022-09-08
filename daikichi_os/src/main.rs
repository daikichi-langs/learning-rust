#![no_std] // Rust標準ライブラリの無効化 don't link the Rust standard library
#![no_main] // 通常のエントリポイントを使いたくないのをコンパイラに伝える disable all Rust-level entry points

use core::panic::PanicInfo;

/// パニック時に呼ばれる関数 This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// 名前修飾無効 don't mangle the name of this function
#[no_mangle]
// エントリポイント this function is the entry point, since the linker looks for a function
// named `_start` by default
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
