use firn::arch::x86;
use firn::cpu::Cpu;
use firn::mem::{BasicMem, Eeprom, MemDump, MemMap, MemRange};
use firn::{mem, System};

fn main() {
    let mem = BasicMem::new(640 * 1024);
    let eeprom = Eeprom::new_with_size(256 * 1024, mem::DEFAULT_BIOS);

    let mut map = MemMap::new(1024 * 1024);
    map.map(MemRange::from_memory_full(&mem), mem);
    map.map(MemRange::new(0xc0000, 0xfffff), eeprom);
    map.dump_to_file("mem.bin")
        .unwrap_or_else(|err| println!("Failed to dump memory: {}", err));

    let system = System::new(map);
    let mut cpu = x86::Cpu::new(system);
    cpu.run();
}
