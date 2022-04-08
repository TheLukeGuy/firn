use firn_arch_x86::device::Cmos;
use firn_arch_x86::{Cpu, Feature};
use firn_core::cpu::Restrict;
use firn_core::mem::{BasicMem, Eeprom, Mem, MemMap};
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

    let mut cpu = Cpu::new();
    cpu.add_feature(Feature::Intel80186);

    let mut system = System::new(cpu, map);
    let cmos = Cmos::new_current_time();
    system.add_device(cmos);

    system.run();
}
