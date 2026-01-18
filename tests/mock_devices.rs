use fat32_reimplementation::{BlockDevice, BootSector, FatError};

pub struct MockDevice {
    sector0: [u8;512],
}

impl BlockDevice for MockDevice {
    fn read_sector(&self, num: u64, buffer: &mut [u8]) -> Result<(), FatError> {
        if num != 0 {
            return Err(FatError::BadData);
        }
        if buffer.len() != 512 {
            return Err(FatError::BadData);
        }
        buffer.copy_from_slice(&self.sector0);
        Ok(())
    }
}

#[test]
fn test_mock_device(){
    let sector_bytes = include_bytes!("testdata/boot_sector.bin");
    let mock = MockDevice {
        sector0: sector_bytes[..].try_into().unwrap(),
    };
    let mut buffer = [0u8; 512];
    mock.read_sector(0,&mut buffer).unwrap();
    let bs = unsafe{ BootSector::from_bytes(&buffer[..62])};
    assert_eq!(bs.bytes_per_sector(), 512);
}
