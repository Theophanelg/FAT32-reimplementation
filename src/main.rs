mod bootsector;
use std::fs::File;
use std::io::{Read, Result};
use fat32_reimplementation::BootSector;


fn main() -> Result<()> {
    let mut file = File::open("disque.img")?; // image FAT32
    let mut sector = [0u8; 512]; // secteur
    file.read_exact(&mut sector)?;
    
    println!("Sector[0..16]     = {:02x?}", &sector[0..16]);
    println!("Sector[11..20]    = {:02x?}", &sector[11..20]);
    println!("Sector[36..40]    = {:02x?}", &sector[36..40]);
    println!("Sector[44..48]    = {:02x?}", &sector[44..48]);
    println!("Sector[54..62]    = {:02x?}", &sector[54..62]);
    let bs = unsafe { BootSector::from_bytes(&sector[..62])};

    println!("bytes/sector      = {}", bs.bytes_per_sector());
    println!("sectors/cluster   = {}", bs.sectors_per_cluster());
    println!("reserved_sectors  = {}", bs.reserved_sectors());
    println!("number_fat        = {}", bs.number_fat());
    println!("sectors_per_fat   = {}", bs.sector_per_fat());
    println!("root_cluster      = {}", bs.root_cluster());
    println!("fs_type           = {:?}",std::str::from_utf8(&bs.fs_type()).unwrap_or("?"));
    Ok(())
}
