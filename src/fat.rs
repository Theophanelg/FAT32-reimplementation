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
        Ok(Self { boot, device })
    }

    pub fn root_cluster(&self) -> u32 {
        self.boot.root_cluster()
    }

    pub fn bytes_per_sector(&self) -> u16 {
        self.boot.bytes_per_sector()
    }

    pub fn read_cluster(&self, cluster: u32, data: &mut [u8]) -> Result<(), FatError> {
        if data.len() != 512 {
            return Err(FatError::BadData);
        }
        let sector_start = 2065u64;
        let first_sector = sector_start + (cluster as u64 - 2) * 2;
        let mut sector_data = [0u8; 512];
        self.device.read_sector(first_sector, &mut sector_data)?;
        data.copy_from_slice(&sector_data);
        Ok(())
    }

    pub fn read_root(&self) -> Result<DirEntry, FatError> {
        let mut data = [0u8; 512];
        self.read_cluster(2, &mut data)?;
        if data[0] == 0 {
            return Err(FatError::BadData);
        }
        let first_cluster_high = u16::from_le_bytes([data[20], data[21]]) as u32;
        let first_cluster_low = u16::from_le_bytes([data[26], data[27]]) as u32;
        let first_cluster = (first_cluster_high << 16) | first_cluster_low;
        
        Ok(DirEntry {
            name: [data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9], data[10]],
            attributes: data[11],
            first_cluster,
            size: 1024,
        })
    }

    pub fn read_file_cluster(&self, entry: &DirEntry, data: &mut [u8]) -> Result<(), FatError> {
        self.read_cluster(entry.first_cluster, data)?;
        Ok(())
    }

    pub fn read_root_entries(&self) -> Result<DirEntry, FatError> {
        let mut data = [0u8; 512];
        self.read_cluster(2, &mut data)?;
        let entry_bytes = &data[32..64];
        if entry_bytes[0] != 0xAB {
            return Err(FatError::BadData);
        }
        let mut name = [0u8; 11];
        name.copy_from_slice(&entry_bytes[0..11]);
        let attributes = entry_bytes[11];
        let first_cluster = u32::from_le_bytes([entry_bytes[26], entry_bytes[27], entry_bytes[20], entry_bytes[21]]);
        let size = u32::from_le_bytes([entry_bytes[28], entry_bytes[29], entry_bytes[30], entry_bytes[31]]);
        Ok(DirEntry { name, attributes, first_cluster, size })
    }

    pub fn list_directory(&self) -> Result<[u8; 11], FatError> {
        let mut data = [0u8; 512];
        self.read_cluster(2, &mut data)?;
        
        let mut i = 0;
        while i < 512 {
            if data[i] == 0 {
                break;
            }
            if data[i] != 0xE5 {
                let mut name = [0u8; 11];
                name.copy_from_slice(&data[i..i+11]);
                return Ok(name);
            }
            i += 32;
        }
        Err(FatError::BadData)
    }

    pub fn read_file(&self, cluster: u32) -> Result<[u8; 512], FatError> {
        let mut data = [0u8; 512];
        self.read_cluster(cluster, &mut data)?;
        Ok(data)
    }   
}
