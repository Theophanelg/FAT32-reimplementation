#![no_std]
pub mod bootsector;
pub mod fat;

pub use bootsector::BootSector;
pub use fat::FatVolume;


#[derive(Debug, Copy, Clone)]
pub enum FatError {
    BadData,
}

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: [u8; 11],
    pub attributes: u8,
    pub first_cluster: u32,
    pub size: u32,
}

pub trait BlockDevice {
    fn read_sector(&self, num: u64, buffer: &mut [u8]) -> Result<(), FatError>;
}
