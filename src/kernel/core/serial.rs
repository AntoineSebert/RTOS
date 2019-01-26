/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	20/01/2019
 */

extern crate uart_16550;
extern crate spin;
extern crate lazy_static;

use self::uart_16550::SerialPort;
use self::spin::Mutex;
use self::lazy_static::lazy_static;

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
	use core::fmt::Write;
	SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
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