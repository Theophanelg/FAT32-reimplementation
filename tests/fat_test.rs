use fat32_reimplementation::{FatVolume, BlockDevice, BootSector};

mod mock_devices;
use mock_devices::MockDevice;

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

#[test]
fn test_read_file_cluster() {
    let mock = MockDevice;
    let volume = FatVolume::new(mock).unwrap();
    let root_entry = volume.read_root().unwrap();
    let mut file_data = [0u8;512];
    volume.read_file_cluster(&root_entry, &mut file_data).unwrap();
    assert_eq!(file_data[0], 0xAA);
}

#[test]
fn test_read_root_entries() {
    let mock = MockDevice;
    let volume = FatVolume::new(mock).unwrap();
    
    let mut debug_cluster = [0u8; 512];
    volume.read_cluster(2, &mut debug_cluster).unwrap();
    println!("Cluster 2 data[0] = {:02x}", debug_cluster[0]);
    println!("Cluster 2 data[32] = {:02x}", debug_cluster[32]);
    
    let entry2 = volume.read_root_entries().unwrap();
    assert_eq!(entry2.first_cluster, 4);
}

#[test]
fn test_boot_sector_mock() {
    let mock = MockDevice;
    let mut sector = [0u8; 512];
    mock.read_sector(0, &mut sector).unwrap();
    
    let boot = unsafe { BootSector::from_bytes(&sector[..62]) };
    println!("Mock bytes_per_sector = {}", boot.bytes_per_sector());
    println!("root_cluster = {}", boot.root_cluster());
    
    assert_eq!(boot.bytes_per_sector(), 512);
}
