use std::fs::File;
use std::io::{Read, Error};
use std::mem;

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
    let filename: &str = "/home/dijkstra/Documents/fat12.img";

    let mut file: File = File::open(filename).expect("Failed to open file");

    let mut buffer: [u8; 512] = [0u8; 512];
    file.read_exact(&mut buffer).expect("Failed to read bootloader data");

    let bootloader_instance: &Fat12Bootloader = unsafe { &*(buffer.as_ptr() as *const Fat12Bootloader) };
    println!("Bootloader has a size of {} bytes", mem::size_of<Fat12Bootloader>());
    println!("Bootloader data:");
    println!("Bytes per sector: {}", bootloader_instance.bytes_per_sector);
    println!("Sectors per cluster: {}", bootloader_instance.sector_per_clustor);
    println!("Number of reserved sectors: {}", bootloader_instance.num_reserved_sectors);
    println!("Number of FATs: {}", bootloader_instance.num_fats);
    println!("Maximum number of root directory entries: {}", bootloader_instance.max_num_root_entries);
    println!("Total sector count: {}", bootloader_instance.total_sector_count);
    println!("Sector per fat: {}", bootloader_instance.sector_per_fat);
    println!("Sector per track: {}", bootloader_instance.sector_per_track);
    println!("Number of heads: {}", bootloader_instance.num_of_heads);
    println!("Total sector count for FAT: {}", bootloader_instance.total_sector_count_for_fat32);
    println!("Bootloader signature: {}", bootloader_instance.boot_signature);
    println!("Volume ID: {}", bootloader_instance.volume_id);
    println!("Volume label: {}", String::from_utf8_lossy(&bootloader_instance.volume_label));
    println!("File system type: {}", String::from_utf8_lossy(&bootloader_instance.file_sys_type));
}
