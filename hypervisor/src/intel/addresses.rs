//! Abstraction over physical addresses with utility functions for address conversion.
//!
//! This module introduces the `PhysicalAddress` structure that simplifies operations around
//! physical addresses. It provides conversions between virtual addresses (VAs) and physical addresses (PAs),
//! as well as methods for extracting page frame numbers (PFNs) and other address-related information.

use {
    core::ops::{Deref, DerefMut},
    x86::bits64::paging::{PAddr, BASE_PAGE_SHIFT},
};

/// A representation of physical addresses.
///
/// Provides utility methods to work with physical addresses,
/// including conversions between physical and virtual addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalAddress(PAddr);

impl PhysicalAddress {
    /// Constructs a `PhysicalAddress` from a given physical address.
    pub fn from_pa(pa: u64) -> Self {
        Self(PAddr::from(pa))
    }

    /// Constructs a `PhysicalAddress` from a given page frame number (PFN).
    pub fn from_pfn(pfn: u64) -> Self {
        Self(PAddr::from(pfn << BASE_PAGE_SHIFT))
    }

    /// Retrieves the page frame number (PFN) for the physical address.
    pub fn pfn(&self) -> u64 {
        self.0.as_u64() >> BASE_PAGE_SHIFT
    }

    /// Retrieves the physical address.
    pub fn pa(&self) -> u64 {
        self.0.as_u64()
    }
}

impl const Deref for PhysicalAddress {
    type Target = PAddr;

    /// Dereferences the `PhysicalAddress` to retrieve the underlying `PAddr`.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl const DerefMut for PhysicalAddress {
    /// Provides mutable access to the underlying `PAddr`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
