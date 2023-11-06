use std::mem::size_of;
// The total size of fat bootloader must be 512 bytes
struct Fat12Bootloader {
    // Ignore the first 11 bytes
    _ignore1: [i8; 11],
    bytes_per_sector: u16,
    sector_per_clustor: u8,
    num_reserved_sectors: u16,
    num_fats: u8,
    max_num_root_entries: u16,
    total_sector_count: u16,
    _ignore2: i8,
    sector_per_fat: u16,
    sector_per_track: u16,
    num_of_heads: u16,
    _ignore3: [i8; 4],
    total_sector_count_for_fat32: u32,
    _ignore4: i16,
    boot_signature: i8,
    volume_id: u32,
    volume_label: [u8; 11],
    file_sys_type: [u8; 8],
    _ignore5: [i8; 450],
}
fn main() {
    const PATH_TO_FAT: &str = "/home/dijkstra/Documents/fat12.img";

    println!("The size of FAT12 Bootloader is {}", size_of::<Fat12Bootloader>())
}