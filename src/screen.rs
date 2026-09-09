pub fn clear_screen(vga_buffer: *mut u8) {
    unsafe {
        for i in (0..(80 * 25)).step_by(2) {
            vga_buffer.add(1).write_volatile(0x00);
            vga_buffer.add(i + 1).write_volatile(0x00);
        }
    }
}
