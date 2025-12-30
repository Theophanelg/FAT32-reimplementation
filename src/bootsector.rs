#[repr(packed)]
struct BootSector {
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
