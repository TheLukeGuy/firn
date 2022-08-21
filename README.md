> **Warning**
> This project is archived (and very incomplete)! It will eventually be replaced with individual projects for some systems.

# :snowflake: Firn

Firn is a generic system emulator, emulating full systems of numerous architectures (currently only x86).

## :dart: Goals

Firn is still very work-in-progress and unusable in any real-world scenarios. For the most part these goals are not yet met.

- Support for many guest CPU architectures and system types
- Highly configurable, usable in any scenario
- Highly portable, usable anywhere that Rust compiles to
- Nice and well-documented API for embedding Firn into other projects
- API bindings for many programming languages

## :world_map: Roadmap

Firn is still very early in development, so this roadmap will likely change in the future. It's currently a rough plan in chronological order.

### :floppy_disk: 16-bit and 32-bit x86

- [x] Basic x86 instruction decoding
- [ ] Full 8086 instruction set
- [ ] Disk input and output
- [ ] Audio and video support
- [ ] Keyboard and mouse support
- [ ] Fancy GUI for general use and debugging
- [ ] <=i386 instructions and features
- [ ] <=Pentium II instructions and features

### :joystick: Retro Consoles

- [ ] Game Boy (Color) support
- [ ] NES support
- [ ] Super NES support
- [ ] Bindings for Java and other languages

### :desktop_computer: Far Future

- [ ] Modern x86-64 instructions and features
- [ ] RISC-V support
- [ ] ARM support
- [ ] Other old consoles
- [ ] Just-in-time compilation

## :scroll: License

Firn is licensed under either of:

- Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
- MIT License (https://opensource.org/licenses/MIT)

at your option.
