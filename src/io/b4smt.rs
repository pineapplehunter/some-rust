use core::fmt;

pub struct IOPort;

impl fmt::Write for IOPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        if c.is_ascii() {
            unsafe { (0x1000_0000 as *mut u32).write_volatile(c as u32) }
        }
        Ok(())
    }
}
