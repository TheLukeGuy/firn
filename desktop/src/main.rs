use firn_arch_x86::device::Cmos;
use firn_arch_x86::Cpu;
use firn_core::mem::{BasicMem, Eeprom, MemDump, MemMap};
use firn_core::System;

// TODO: This file is temporary for testing until a proper GUI is in place
fn main() {
    let mem = BasicMem::new(640 * 1024);
    let eeprom = Eeprom::new_with_size(256 * 1024, firn_arch_x86::DEFAULT_BIOS);

    let mut map = MemMap::new(1024 * 1024);
    map.map_full(mem);
    map.map_from(0xc0000, 0xfffff, eeprom);

    map.dump_to_file("mem.bin")
        .unwrap_or_else(|err| println!("Failed to dump memory: {}", err));

    let cpu = Cpu::new();
    let mut system = System::new(cpu, map);

    let cmos = Cmos::new_current_time();
    system.add_device(cmos);

    system.run();
}
