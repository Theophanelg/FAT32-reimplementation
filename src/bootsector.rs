#![allow(dead_code)]
use core::ptr;

/// Structure qui représente le secteur de démarrage FAT32
/// Contient les informations sur la taille des secteurs, clusters, etc.
#[repr(packed)]
pub struct BootSector {
    _boot_code: [u8; 11],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u8,
    padding0: [u8; 1],
    number_fat: u8,
    padding1: [u8; 19],
    sector_per_fat: u32,
    padding2: [u8; 4],
    root_cluster: u32,
    padding3: [u8; 6],
    fs_type: [u8; 8],
}

impl BootSector {
    /// Retourne le nombre d'octets par secteur (normalement 512)
    pub fn bytes_per_sector(&self) -> u16 {
        u16::from_le_bytes(self.bytes_per_sector.to_le_bytes())
    }

    /// Retourne le nombre de secteurs par cluster
    pub fn sectors_per_cluster(&self) -> u8 {
        self.sectors_per_cluster
    }
    
    /// Retourne le nombre de secteurs réservés
    pub fn reserved_sectors(&self) -> u8 {
        self.reserved_sectors
    }
    
    /// Retourne le nombre de tables FAT (généralement 2)
    pub fn number_fat(&self) -> u8 {
        self.number_fat
    }
    
    /// Retourne le nombre de secteurs par table FAT
    pub fn sector_per_fat(&self) -> u32 {
        u32::from_le_bytes(self.sector_per_fat.to_le_bytes())
    }
    
    /// Retourne le numéro du cluster racine (généralement 2)
    pub fn root_cluster(&self) -> u32 {
        u32::from_le_bytes(self.root_cluster.to_le_bytes())
    }
    
    /// Retourne le type de système de fichiers ("FAT32   ")
    pub fn fs_type(&self) -> [u8; 8] {
        self.fs_type
    }

    /// Calcule l'offset de la première table FAT
    pub fn fat1_offset(&self) -> u64 {
        (self.reserved_sectors() as u64) * (self.bytes_per_sector() as u64)
    }

    /// Calcule l'offset d'une entrée FAT pour un cluster donné
    pub fn fat_entry_offset(&self, cluster: u32) -> u64 {
        self.fat1_offset() + (cluster as u64 * 4)
    }

    /// Lit un secteur de démarrage depuis des octets bruts
	/// 
    /// # Safety
    /// Les octets doivent contenir exactement 62 octets valides
    pub unsafe fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 62);
        ptr::read_unaligned(bytes.as_ptr() as *const BootSector)
    }
}
