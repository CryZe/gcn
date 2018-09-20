#[cfg(any(target_arch = "powerpc", target_arch = "wasm32"))]
#[panic_handler]
pub fn panic(info: &::core::panic::PanicInfo) -> ! {
    debug_report!("{}", info);
    loop {}
}
