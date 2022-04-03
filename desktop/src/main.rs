use chrono::Utc;
use firn_arch_x86::device::Cmos;
use firn_core::cpu::Cpu;
use firn_core::mem::{BasicMem, Eeprom, MemDump, MemMap, MemRange};
use firn_core::System;

// TODO: This file is temporary for testing until a proper GUI is in place
fn main() {
    let mem = BasicMem::new(640 * 1024);
    let eeprom = Eeprom::new_with_size(256 * 1024, firn_arch_x86::DEFAULT_BIOS);

    let mut map = MemMap::new(1024 * 1024);
    map.map(MemRange::from_memory_full(&mem), mem);
    map.map(MemRange::new(0xc0000, 0xfffff), eeprom);
    map.dump_to_file("mem.bin")
        .unwrap_or_else(|err| println!("Failed to dump memory: {}", err));

    let mut system = System::new(map);
    let cmos = Cmos::new(Utc::now());
    system.add_device(cmos);

    let mut cpu = firn_arch_x86::Cpu::new(system);
    cpu.run();
}
