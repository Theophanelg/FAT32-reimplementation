#![no_std]
pub mod bootsector;
pub mod fat;

pub use bootsector::BootSector;
pub use fat::FatVolume;


#[derive(Debug, Copy, Clone)]
pub enum FatError {
    BadData,
}

pub trait BlockDevice {
    fn read_sector(&self, num: u64, buffer: &mut [u8]) -> Result<(), FatError>;
}

