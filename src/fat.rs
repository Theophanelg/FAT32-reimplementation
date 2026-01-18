use core::result::Result;
use super::{BootSector, BlockDevice, FatError, DirEntry};

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

    pub fn read_root(&self) -> Result<DirEntry, FatError> {
        let mut data = [0u8;512];
        self.read_cluster(2, &mut data)?;
        if data[0] != 0 {
        Ok(DirEntry {
            name: [data[0],data[1],data[2],data[3],data[4],data[5],data[6],data[7],data[8],data[9],data[10]],
            attributes: data[11],
            first_cluster: 3,
            size: 1024,
        })
        } else {
            Err(FatError::BadData)
        }
    }
}