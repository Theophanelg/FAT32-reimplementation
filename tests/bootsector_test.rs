use fat32_reimplementation::BootSector;

#[test]
fn test_bootsector(){
	let mut raw = [0u8; 62];

	// Remplit les offsets utilisés
	raw[0..2].copy_from_slice(&512u16.to_le_bytes());
	raw[2] = 8;
	raw[3] = 32;
	raw[5] = 2;
	raw[25..29].copy_from_slice(&1234u32.to_le_bytes());
	raw[33..37].copy_from_slice(&2u32.to_le_bytes());
	raw[43..51].copy_from_slice(b"FAT32   ");

	let bs = unsafe {
		BootSector::from_bytes(&raw)
	};

	assert_eq!(bs.bytes_per_sector(), 512);
	assert_eq!(bs.reserved_sectors(), 32);
	assert_eq!(bs.number_fat(), 2);
	assert_eq!(bs.sector_per_fat(), 1234);
	assert_eq!(bs.root_cluster(), 2);
	assert_eq!(bs.fs_type(), *b"FAT32   ");
}

#[test]
fn test_fat_offset() {
	let mut raw = [0u8; 62];
	raw[0..2].copy_from_slice(&512u16.to_le_bytes());
	raw[2] = 8;
	raw[3] = 32;
	raw[5] = 2;
	raw[25..29].copy_from_slice(&1234u32.to_le_bytes());
	raw[33..37].copy_from_slice(&2u32.to_le_bytes());

	let bs = unsafe { BootSector::from_bytes(&raw) };

	assert_eq!(bs.fat1_offset(), 16384);
	assert_eq!(bs.fat_entry_offset(2), 16392);
}