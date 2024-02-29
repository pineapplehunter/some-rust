use core::fmt;

pub struct IOPort;

impl fmt::Write for IOPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        #[cfg(feature = "unicode")]
        for c in s.chars() {
            self.write_char(c)?;
        }

        #[cfg(not(feature = "unicode"))]
        for c in s.as_bytes() {
            self.write_byte(*c);
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let c = if c.is_ascii() { c as u8 } else { b'?' };
        self.write_byte(c);
        Ok(())
    }
}

impl IOPort {
    pub fn write_byte(&mut self, byte: u8) {
        unsafe { (0x1000_0000 as *mut u32).write_volatile(byte as u32) }
    }
}
