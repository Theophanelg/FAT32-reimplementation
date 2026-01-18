use fat32_reimplementation::{FatVolume, BlockDevice, FatError};

struct MockDevice;

impl BlockDevice for MockDevice {
    fn read_sector(&self, num: u64, buffer: &mut [u8]) -> Result<(), FatError> {
        match num {
            0 => {
                buffer[11] = 0x00; buffer[12] = 0x02;
                buffer[13] = 0x01;
                buffer[44..48].copy_from_slice(&2u32.to_le_bytes());
                Ok(())
            }
            2065 => {
                buffer[0] = 0xf8;
                buffer[26] = 0x03;
                Ok(())
            }
            2067 => {
                buffer[0] = 0xAA;
                Ok(())
            }
            _ => Err(FatError::BadData),
        }
    }
}

fn main() {
    let mut volume = FatVolume::new(MockDevice).unwrap();
    
    println!("=== Test ls ===");
    let list = volume.list_directory().unwrap();
    println!("Fichiers trouvés: {}", list.count);
    
    println!("\n=== Test cd ===");
    volume.change_directory(3).unwrap();
    println!("Cluster actuel: {}", volume.current_directory());
    
    println!("\n=== Test cat ===");
    let data = volume.read_file(3).unwrap();
    println!("Premier byte: 0x{:02x}", data[0]);
}
