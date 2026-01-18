use fat32_reimplementation::{BlockDevice, FatError};

pub struct MockDevice;

impl BlockDevice for MockDevice {
    fn read_sector(&self, num: u64, buffer: &mut [u8]) -> Result<(), FatError> {
        if buffer.len() != 512 {
            return Err(FatError::BadData);
        }
        
        match num {
            0 => {
                buffer[11] = 0x00;
                buffer[12] = 0x02;
                buffer[13] = 0x01;
                buffer[14] = 0x01;
                buffer[15] = 0x02;
                buffer[44..48].copy_from_slice(&2u32.to_le_bytes());
            }
            2065 => {
                buffer[0] = 0xf8;
                buffer[20] = 0x00;
                buffer[21] = 0x00;
                buffer[26] = 0x03;
                buffer[27] = 0x00;
                buffer[32] = 0xAB;
                buffer[52] = 0x00;
                buffer[53] = 0x00;
                buffer[58] = 0x04;
                buffer[59] = 0x00;
            }
            2066 => {
                buffer[0] = 0xAB;
            }
            2067 => {
                buffer[0] = 0xAA;
            }
            2069 => {
                buffer[0] = 0xAA;
            }
            _ => return Err(FatError::BadData),
        }
        Ok(())
    }
}
