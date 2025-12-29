use crate::BootSector;

#[test]
fn test_bootsector_parsing() {
	let raw = include_bytes!("test_data/bootsector.bin");
	let bootsector = BootSector::from_bytes(raw);
	assert_eq!(bootsector.fs_type, *b"FAT32   ");
}
