use fat32_reimplementation::{FatVolume, FatError, BlockDevice};

#[derive(Default)]
struct MockDevice;

impl BlockDevice for MockDevice {
    fn read_sector(&self, num: u64, buffer: &mut [u8]) -> Result<(), FatError> {
        match num {
            0 => {
                buffer[11] = 0x00; buffer[12] = 0x02;
                buffer[13] = 0x08;
                buffer[14] = 0x01;
                buffer[15] = 0x02;
                buffer[16] = 0x00; buffer[17] = 0x00;
                buffer[44] = 0x02; buffer[45] = 0x00; buffer[46] = 0x00; buffer[47] = 0x00;
                Ok(())
            }
            2065 => { buffer[0] = 0xf8; Ok(()) }
            _ => Err(FatError::BadData),
        }
    }
}


#[test]
fn test_read_cluster_simple() {
    let mock = MockDevice {};
    let volume = FatVolume::new(mock).unwrap();
    let mut cluster1 = [0u8;512];
    volume.read_cluster(2, &mut cluster1).unwrap();
    assert_eq!(cluster1[0], 0xf8);
}

#[test]
fn test_read_root() {
    let mock = MockDevice;
    let volume = FatVolume::new(mock).unwrap();
    let root_entry = volume.read_root().unwrap();
    assert_eq!(root_entry.name[0], 0xf8);
    assert_eq!(root_entry.first_cluster, 3);
}