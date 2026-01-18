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

    pub fn read_cluster(&self, cluster: u32, data: &mut [u8]) -> Result<(), FatError> {
        let sector_start = 2065;
        if data.len() != 512 {
            return Err(FatError::BadData);
        }

        self.device.read_sector(sector_start + (cluster as u64 - 2), data)?;
        Ok(())
    }
}