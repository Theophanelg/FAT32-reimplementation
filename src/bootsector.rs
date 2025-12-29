
#[repr(packed)]
struct BootSector {
	/// Offset 11-12 : bytes par secteur (512)
	bytes_per_sector: u16,

	/// Offset 13 : secteurs par cluster
	sectors_per_cluster: u8,
	
	/// Padding jusqu'à l'offset 44
	padding1: [u8; 30],
	
	/// Offset 44-47 : premier cluster du répertoire root (FAT32)
	root_cluster: u32,
	
	/// Padding jusqu'à offset 54 
	padding2: [u8; 6],
	
	/// Offset 54-61 : "FAT32   "
	fs_type: [u8; 8],
}
