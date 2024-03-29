pub use firn_core::*;

pub mod arch {
    #[cfg(feature = "arch-x86")]
    pub mod x86 {
        pub use firn_arch_x86::*;
    }
}
