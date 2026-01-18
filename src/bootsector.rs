#![allow(dead_code)]
use core::ptr;


#[repr(packed)]

pub struct BootSector {
	_boot_code: [u8; 11],
	bytes_per_sector: u16,		// Offset 11-12 : bytes par secteur (512)
	sectors_per_cluster: u8, 	// Offset 13 : secteurs par cluster (8)	
	reserved_sectors: u8,		// Offset 14 : secteurs réservés (32)
	padding0: [u8; 1],
	number_fat: u8, 		// Offset 16 : nombre de FAT (2)	
	padding1: [u8; 19],
	sector_per_fat: u32,	// Offset 36-39 : secteurs par FAT (1234)	
	padding2: [u8; 4],
	root_cluster: u32,		// Offset 44-47 : cluster racine (2)	
	padding3: [u8; 6],
	fs_type: [u8; 8],		// Offset 54-61 : type de système de fichiers ("FAT32   ")
}

impl BootSector {
	pub fn bytes_per_sector(&self) -> u16{
		u16::from_le_bytes(self.bytes_per_sector.to_le_bytes())
	}

	pub fn sectors_per_cluster(&self) -> u8 {
        self.sectors_per_cluster
    }
    
    pub fn reserved_sectors(&self) -> u8 {
       self.reserved_sectors
    }
    
    pub fn number_fat(&self) -> u8 {
		self.number_fat
    }
    
    pub fn sector_per_fat(&self) -> u32 {
        u32::from_le_bytes(self.sector_per_fat.to_le_bytes())
    }
    
    pub fn root_cluster(&self) -> u32 {
		u32::from_le_bytes(self.root_cluster.to_le_bytes())
    }
    
    pub fn fs_type(&self) -> [u8; 8] {
        self.fs_type
    }

	pub fn fat1_offset(&self) -> u64 {
		(self.reserved_sectors() as u64) * (self.bytes_per_sector() as u64)
	}

	pub fn fat_entry_offset(&self, cluster: u32) -> u64 {
		self.fat1_offset() + (cluster as u64 * 4)
	}

	/// Analyser les octets bruts du secteur d'amorçage dans BootSector
	/// bytes doit contenir **exactement 62 octets** valides du secteur d'amorçage
	pub unsafe fn from_bytes(bytes: &[u8]) -> Self {
		assert_eq!(bytes.len(), 62);
		ptr::read_unaligned(bytes.as_ptr() as *const BootSector)
	}
}
