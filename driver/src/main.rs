// This crate provides the core functionality for initializing a hypervisor environment
// within a UEFI application. It demonstrates advanced features such as custom panic handlers,
// early logging, and direct manipulation of loaded image properties for hypervisor initialization.

#![feature(new_uninit)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

extern crate alloc;

use {
    crate::{processor::start_hypervisor_on_all_processors, relocation::zap_relocations},
    hypervisor::{
        intel::{ept::paging::Ept, vm::box_zeroed},
        logger::{self, SerialPort},
    },
    log::*,
    uefi::prelude::*,
};

pub mod processor;
pub mod relocation;
pub mod virtualize;

/// Custom panic handler for the UEFI application.
///
/// # Arguments
///
/// * `info` - Information about the panic, including the location and optional message.
#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // Log the file, line, and column of the panic.
    if let Some(location) = info.location() {
        error!(
            "[-] Panic in {} at ({}, {}):",
            location.file(),
            location.line(),
            location.column()
        );
        // Log the panic message if available.
        if let Some(message) = info.message() {
            error!("[-] {}", message);
        }
    }

    // Enter an infinite loop as the panic handler should not return.
    loop {}
}

/// Entry point for the UEFI application.
///
/// Initializes logging, UEFI services, and attempts to start the hypervisor on all processors.
///
/// # Arguments
///
/// * `_image_handle` - Handle to the loaded image of the application.
/// * `system_table` - Reference to the UEFI System Table.
///
/// # Returns
///
/// The status of the application execution. Returns `Status::SUCCESS` on successful execution,
/// or `Status::ABORTED` if the hypervisor fails to install.
#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Initialize logging with the COM2 port and set the level filter to Trace.
    logger::init(SerialPort::COM2, LevelFilter::Trace);

    // Initialize UEFI services.
    uefi_services::init(&mut system_table).unwrap();
    // allocator::init(&system_table);

    info!("The Matrix is an illusion");

    let boot_services = system_table.boot_services();

    // Attempt to zap relocations in the UEFI environment.
    debug!("Zapping relocations");
    if let Err(e) = zap_relocations(boot_services) {
        error!("Failed to zap relocations: {:?}", e);
        return Status::ABORTED;
    }

    debug!("Allocating primary and secondary EPTs");
    let mut primary_ept = unsafe { box_zeroed::<Ept>() };
    let mut secondary_ept = unsafe { box_zeroed::<Ept>() };

    debug!("Identity mapping primary and secondary EPTs");

    if let Err(e) = primary_ept.build_identity() {
        error!("Failed to identity map primary EPT: {:?}", e);
        return Status::ABORTED;
    }

    if let Err(e) = secondary_ept.build_identity() {
        error!("Failed to identity map secondary EPT: {:?}", e);
        return Status::ABORTED;
    }

    // Attempt to start the hypervisor on all processors.
    debug!("Starting hypervisor on all processors");
    if let Err(e) = start_hypervisor_on_all_processors(boot_services, primary_ept, secondary_ept) {
        error!("Failed to start hypervisor on all processors: {:?}", e);
        return Status::ABORTED;
    }

    // Return success status to UEFI environment.
    Status::SUCCESS
}
