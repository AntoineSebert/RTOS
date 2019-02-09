/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	20/01/2019
 */

// crates
extern crate lazy_static;
extern crate spin;
extern crate uart_16550;
extern crate x86_64;

// uses
use self::{lazy_static::lazy_static, spin::Mutex, uart_16550::SerialPort};

lazy_static! {
	pub static ref SERIAL1: Mutex<SerialPort> = {
		let mut serial_port = SerialPort::new(0x3F8);
		serial_port.init();
		Mutex::new(serial_port)
	};
}

/*
 * Macros
 */

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
	use self::x86_64::instructions::interrupts;
	use core::fmt::Write;

	interrupts::without_interrupts(|| {
		SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
	});
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
	($($arg:tt)*) => {
		$crate::serial::_print(format_args!($($arg)*));
	};
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
	() => ($crate::serial_print!("\n"));
	($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}