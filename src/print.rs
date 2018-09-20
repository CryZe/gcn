use arrayvec::ArrayString;
use core::fmt::{Arguments, Error, Write};
use os;

struct Writer<'a>(&'a mut ArrayString<[u8; 1024]>);

impl<'a> Writer<'a> {
    fn flush(&mut self) {
        self.0.push_str("\0");
        unsafe {
            os::report(self.0.as_ptr());
        }
        self.0.clear();
    }
}

impl<'a> Write for Writer<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for c in s.chars() {
            if c == '\n' {
                self.flush();
            } else {
                if self.0.len() >= 1020 {
                    self.flush();
                }
                self.0.write_char(c)?;
                if c == '%' {
                    self.0.write_char(c)?;
                }
            }
        }
        Ok(())
    }
}

pub fn report(args: &Arguments) {
    let mut message = ArrayString::new();
    let mut writer = Writer(&mut message);
    let _ = write!(writer, "{}\0", args);
    writer.flush();
}

#[macro_export]
macro_rules! report {
    ($msg:expr) => ({
        $crate::print::report(&format_args!($msg))
    });
    ($msg:expr,) => ({
        report!($msg)
    });
    ($fmt:expr, $($arg:tt)+) => ({
        $crate::print::report(&format_args!($fmt, $($arg)+))
    });
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_report {
    ($msg:expr) => ({
        $crate::print::report(&format_args!($msg))
    });
    ($msg:expr,) => ({
        report!($msg)
    });
    ($fmt:expr, $($arg:tt)+) => ({
        $crate::print::report(&format_args!($fmt, $($arg)+))
    });
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_report {
    ($msg:expr) => ({
        drop($msg);
    });
    ($msg:expr,) => ({
        drop($msg);
    });
    ($fmt:expr, $($arg:tt)+) => ({
        drop($($arg)+);
    });
}
