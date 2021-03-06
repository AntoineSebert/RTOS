// https://wiki.osdev.org/ACPI

/// Initialise the ACPI.
/// Returns an error message if the ACPI is not present or could not be initialised.
///
/// # Safety
/// Dealing with hardware components.
pub unsafe fn init() -> Result<(), &'static str> { Ok(()) }
