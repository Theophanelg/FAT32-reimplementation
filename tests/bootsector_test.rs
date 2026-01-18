use fat32_reimplementation::{BootSector};
#[test]
fn test_bootsector(){
	let mut raw = [0u8; 62];

	raw[11..13].copy_from_slice(&512u16.to_le_bytes());
    raw[13] = 8;
    raw[14] = 32;
    raw[16] = 2;
    raw[36..40].copy_from_slice(&1234u32.to_le_bytes());
    raw[44..48].copy_from_slice(&2u32.to_le_bytes());
    raw[54..62].copy_from_slice(b"FAT32   ");

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
	raw[11..13].copy_from_slice(&512u16.to_le_bytes());
    raw[13] = 8;
    raw[14] = 32;
    raw[16] = 2;
    raw[36..40].copy_from_slice(&1234u32.to_le_bytes());
    raw[44..48].copy_from_slice(&2u32.to_le_bytes());

	let bs = unsafe { BootSector::from_bytes(&raw) };

	assert_eq!(bs.fat1_offset(), 16384);
	assert_eq!(bs.fat_entry_offset(2), 16392);
}

#[test]
fn test_real_boot_sector() -> Result<(), Box<dyn std::error::Error>> {
    let sector = std::fs::read("tests/testdata/boot_sector.bin")?;
    let bs = unsafe { BootSector::from_bytes(&sector[..62]) };
    
    assert_eq!(bs.bytes_per_sector(), 512);
    assert_eq!(bs.reserved_sectors(), 32);
    assert_eq!(bs.number_fat(), 2);
    assert_eq!(bs.sector_per_fat(), 1009);
    assert_eq!(bs.root_cluster(), 2);
    
    Ok(())
}