use core::result::Result;
use super::{BootSector, BlockDevice, FatError};

pub struct FatVolume<D: BlockDevice> {
    boot: BootSector,
    device: D,
}

impl<D: BlockDevice> FatVolume<D> {
    pub fn new(device: D) -> Result<Self, FatError> {
        let mut sector = [0u8; 512];
        device.read_sector(0, &mut sector)?;
        let boot = unsafe {
            BootSector::from_bytes(&sector[..62])
        };
        if boot.bytes_per_sector() != 512 {
            return Err(FatError::BadData);
        }
        Ok(Self {boot, device})
    }

    pub fn root_cluster(&self) -> u32 {
        self.boot.root_cluster()
    }

    pub fn bytes_per_sector(&self) -> u16 {
        self.boot.bytes_per_sector()
    }
}