/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	20/01/2019
 */

/*
run
	cls && bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04
tests
	cls && bootimage test
both
	cls && bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 && bootimage test

format
	cargo +nightly fmt
lint
	cargo clippy
both
	cargo +nightly fmt && cargo clippy

bootable USB
	dd if=target/x86_64-dandelion/debug/bootimage-dandelion.bin of=/dev/sdX && sync

misc
	https://giphy.com/gifs/love-cute-adorable-RExphJPPMEVeo
*/

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]
#![deny(clippy::all)]

extern crate bootloader;
extern crate dandelion;
extern crate integer_sqrt;
extern crate pic8259_simple;
extern crate x86_64;

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use dandelion::{hlt_loop, println};

/*
 * OS entry point override
 */
entry_point!(kernel_main);

#[cfg(not(test))]
#[no_mangle]
#[allow(clippy::print_literal)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use dandelion::{
		gdt,
		interrupts::{init_idt, PICS},
		memory::{self, create_mapping, init_frame_allocator},
	};
	use x86_64::instructions::interrupts::enable;

	println!("Hello World{}", "!");

	gdt::init();
	init_idt();
	unsafe { PICS.lock().initialize() };
	enable();

	let mut recursive_page_table = unsafe { memory::init(boot_info.p4_table_addr as usize) };
	let mut frame_allocator = init_frame_allocator(&boot_info.memory_map);

	create_mapping(&mut recursive_page_table, &mut frame_allocator);
	unsafe { (0x0dea_dbea_f900 as *mut u64).write_volatile(0xf021_f077_f065_f04e) };

	sample_job(1_000, false);

	println!("It did not crash!");
	hlt_loop();
}

/*
 * This function is called on panic.
 * @param	info	information about the panic error
 */
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	hlt_loop();
}

/*
 * Sample job streaming prime numbers on the serial port up to a limit (passed as parameter) less than 2^64
 * On my computer, find all the primes between 0 and 1.000.000 in 2:05 min
 */
fn sample_job(limit: u64, output: bool) {
	use dandelion::{println, serial_println};
	use integer_sqrt::IntegerSquareRoot;

	if output {
		println!("2");
	} else {
		serial_println!("2");
	}
	let mut counter: u64 = 3;
	loop {
		if limit < counter {
			break;
		}
		let mut counter2 = 3;
		let mut is_prime = true;
		loop {
			if counter.integer_sqrt() < counter2 {
				break;
			}
			if counter % counter2 == 0 {
				is_prime = false;
				break;
			}
			counter2 += 2;
		}
		if is_prime {
			if output {
				println!("{}", counter);
			} else {
				serial_println!("{}", counter);
			}
		}
		counter += 2;
	}
}
