use core::fmt;

use crate::io::MAIN_OUTPUT;

#[inline(never)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut output = MAIN_OUTPUT.lock();
    output.write_fmt(args).unwrap();
}

/// print a string
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::macros::_print(format_args!($($arg)*)));
}

/// print a string followed by a new line
#[macro_export]
macro_rules! println {
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

/// show content of a variable
#[macro_export]
macro_rules! dbg {
    () => {{
        debug!();
    }};
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                debug!(concat!(stringify!($val)," ="), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! ignore {
    ($_any:expr) => {};
    ($_any:expr,$replace:expr) => {
        $replace
    };
}
#[macro_export(local_inner_macros)]
macro_rules! log_macro_impl {
    ($level:ident, $color:literal, $label:expr $(,$e:expr)*) => {
        // let thread_id = riscv::register::mhartid::read();
        let thread_id = get_thread_id();
        // let cycle = riscv::register::cycle::read();
        $crate::println!(
            core::concat!("[thread={} ",::core::file!(),":",::core::line!()," {}] ", $color, core::stringify!($level),"\x1b[0m {}" $(,ignore!($e," {:#?}"))*),
            thread_id,
            0,
            $label,
            $($e),*
        );
    };
}

#[macro_export(local_inner_macros)]
macro_rules! debug {
    () => {
        debug!("no message")
    };
    ($label:expr $(,$e:expr)*) => {
        log_macro_impl!(DEBUG,"\x1b[47m",$label $(,$e)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! info {
    () => {
        info!("no message")
    };
    ($label:expr $(,$e:expr)*) => {
        log_macro_impl!(INFO,"\x1b[46m",$label $(,$e)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! warn {
    () => {
        warn!("no message")
    };
    ($label:expr $(,$e:expr)*) => {
        log_macro_impl!(WARN,"\x1b[43m",$label $(,$e)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! error {
    () => {
        warn!("no message")
    };
    ($label:expr $(,$e:expr)*) => {
        log_macro_impl!(ERROR,"\x1b[41m",$label $(,$e)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! fatal {
    () => {
        warn!("no message")
    };
    ($label:expr $(,$e:expr)*) => {
        log_macro_impl!(FATAL,"\x1b[40m",$label $(,$e)*)
    };
}
