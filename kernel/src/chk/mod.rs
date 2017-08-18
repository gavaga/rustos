#[cfg(feature = "vga")]
#[cfg(feature = "uart")]
fn system_output_mutex() {
    compile_error!("Must choose only one of `vga` and `uart` features.");
}
