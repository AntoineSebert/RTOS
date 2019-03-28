/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

#![allow(dead_code)]

use core::{sync::atomic::AtomicPtr, time::Duration};
use either::Either;
use cmos::RTCDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
	Limbo(Limbo),
	MainMemory(MainMemory),
	SwapSpace(SwapSpace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Limbo {
	Creating,
	Killed,
	Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMemory {
	Ready,
	Running,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapSpace {
	Interrupted,
	Suspended,
	Delayed,
}

pub type Arguments = [[char; 256]; 256];
pub type Main = AtomicPtr<fn(Arguments) -> i64>;

pub type Periodic = (Duration, Duration, Option<RTCDateTime>);		// estimated completion time, interval, delay
pub type Aperiodic = (Duration, RTCDateTime, Option<RTCDateTime>);	// estimated completion time, deadline, delay

pub type Constraint = Option<Either<Periodic, Aperiodic>>;
pub type Info = (State, Duration, RTCDateTime);

pub type Metadata = (Constraint, Info);

pub type Runnable = Main;
pub type Task = (Metadata, Runnable);

pub type Job = (Metadata, [Runnable; 256]);
pub type Group = [Task; 256];

/*
 * Sample job streaming prime numbers on the serial port up to a limit (passed as parameter) less than 2^64
 * On my computer, find all the primes between 0 and 1.000.000 in 2:05 min
 */
#[allow(dead_code)]
fn sample_job(limit: u64, output: bool/*args: Arguments*/) {
	use crate::{println, serial_println};
	use integer_sqrt::IntegerSquareRoot;

	// arg 0 is name
	// arg 1 is --limit=number
	// arg 2 is output=true/false

	if output {
		println!("2");
	} else {
		serial_println!("2");
	}
	let mut candidate: u64 = 3;
	loop {
		if limit < candidate {
			break;
		}
		let mut iterator = 3;
		let mut is_prime = true;
		loop {
			if candidate.integer_sqrt() < iterator {
				break;
			}
			if candidate % iterator == 0 {
				is_prime = false;
				break;
			}
			iterator += 2;
		}
		if is_prime {
			if output {
				println!("{}", candidate);
			} else {
				serial_println!("{}", candidate);
			}
		}
		candidate += 2;
	}
}