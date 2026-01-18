#![no_std]
pub mod bootsector;
pub mod fat;

pub use bootsector::BootSector;
pub use fat::FatVolume;

/// Erreurs possibles lors de la lecture du système de fichiers
#[derive(Debug, Copy, Clone)]
pub enum FatError {
    BadData,
}

/// Représente une entrée de répertoire
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: [u8; 11],
    pub attributes: u8,
    pub first_cluster: u32,
    pub size: u32,
}

/// Structure contenant la liste des entrées d'un répertoire
pub struct Directory {
    pub entries: [Option<[u8; 11]>; 16],
    pub count: usize,
}

/// Trait pour lire des secteurs depuis un périphérique
pub trait BlockDevice {
    fn read_sector(&self, num: u64, buffer: &mut [u8]) -> Result<(), FatError>;
}
