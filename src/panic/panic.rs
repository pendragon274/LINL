use core::panic::PanicInfo;
use crate::println;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> !{
    println!("\n\\c4fKERNEL PANIC OCCURRED:\\c0f\n{:?}", panic_info);

    loop{}
}