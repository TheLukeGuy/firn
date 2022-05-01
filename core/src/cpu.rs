use crate::System;

/// A trait for CPUs.
///
/// A CPU handles the actual execution of the system. Their primary task is to determine the next
/// instruction in memory, decode it, and execute it. They must implement `Cpu` to be usable inside
/// of a [`System`] instance.
///
/// CPUs should not rely on any [`Device`]s being present. If a specific device is absolutely
/// necessary for the CPU to function, make it part of the CPU.
///
/// [`System`]: crate::System
/// [`Device`]: crate::device::Device
pub trait Cpu: Sized {
    /// Initializes the CPU.
    ///
    /// This is called in [`System::init`] (and therefore [`System::run`]) after all [`Device`]s are
    /// initialized. This should be implemented with caution since it's only ever run once. For
    /// tasks that need to be run every time the CPU is reset, implement [`reset`] instead.
    ///
    /// If this method isn't implemented, the `Cpu` will do nothing during initialization.
    ///
    /// [`System::init`]: crate::System::init
    /// [`System::run`]: crate::System::run
    /// [`Device`]: crate::device::Device
    /// [`reset`]: Cpu::reset
    fn init(&mut self) {}

    /// Resets the CPU.
    ///
    /// This is called in [`System::start`] (and therefore [`System::run`]) right before the
    /// execution loop begins. The CPU will always be initialized before it's reset. For tasks that
    /// need to be run only once, implement [`init`] instead.
    ///
    /// If this method isn't implemented, the `Cpu` will do nothing when it's reset.
    ///
    /// [`System::start`]: crate::System::start
    /// [`System::run`]: crate::System::run
    /// [`init`]: Cpu::init
    fn reset(&mut self) {}

    /// Executes the next iteration of the CPU.
    ///
    /// This is called constantly while the [`System`] is running, after all [`Device`]s are
    /// stepped. You probably want to decode and execute a single instruction in this method.
    ///
    /// [`System`]: crate::System
    /// [`Device`]: crate::device::Device
    fn step(sys: &mut System<Self>);
}

/// A trait for CPUs that support being restricted via "features".
///
/// When a CPU implements `Restrict`, users must use [`add_feature`] to explicitly add features to
/// the CPU for it to have the functionality that feature provides. If `add_feature` is never
/// called, the CPU has limited functionality. If `add_feature` is called once for every possible
/// feature, the CPU has full functionality.
///
/// It's recommended that CPUs implement `Restrict` whenever possible since it gives applications
/// more control over emulation. Features should be additional capabilities like extra instruction
/// sets and extensions rather than things like improved speed. They should be modeled after
/// real-world features whenever possible (for example, an x86 CPU could have features for Intel
/// 80186 instructions, protected mode, MMX, AMD64, etc.)
///
/// [`add_feature`]: Restrict::add_feature
///
/// # Examples
///
/// Suppose we're implementing a real CPU which has two versions: one which only supports
/// instructions `0x00` to `0x7f`, and another which supports the full range of instructions from
/// `0x00` to `0xff`. We could implement `Restrict` with a single feature called `AllInstructions`
/// to represent this.
///
/// ```rust
/// use firn_core::cpu::Restrict;
///
/// #[derive(PartialEq)]
/// enum Feature {
///     AllInstructions,
/// }
///
/// struct RestrictedCpu {
///     features: Vec<Feature>,
/// }
///
/// impl RestrictedCpu {
///     fn new() -> Self {
///         Self {
///             features: Vec::new(),
///         }
///     }
///
///     fn is_valid_instruction(&self, opcode: u8) -> bool {
///         match opcode {
///             0x00..=0x7f => true,
///             0x80..=0xff if self.has_feature(Feature::AllInstructions) => true,
///             _ => false,
///         }
///     }
/// }
///
/// // For simplicity, we don't implement `Cpu` for our CPU.
///
/// impl Restrict for RestrictedCpu {
///     type Feature = Feature;
///
///     fn add_feature(&mut self, feature: Self::Feature) {
///         self.features.push(feature);
///     }
///
///     fn has_feature(&self, feature: Self::Feature) -> bool {
///         self.features.contains(&feature)
///     }
/// }
///
/// let cpu_v1 = RestrictedCpu::new();
/// assert!(!cpu_v1.is_valid_instruction(0x98));
///
/// let mut cpu_v2 = RestrictedCpu::new();
/// cpu_v2.add_feature(Feature::AllInstructions);
/// assert!(cpu_v2.is_valid_instruction(0x98));
/// ```
pub trait Restrict {
    /// The feature type, usually an `enum`.
    type Feature: PartialEq;

    /// Adds a feature to the CPU.
    ///
    /// For more information and examples, see [`Restrict`].
    ///
    /// [`Restrict`]: Restrict
    fn add_feature(&mut self, feature: Self::Feature);

    /// Determines whether or not the CPU has the specified feature.
    ///
    /// For more information and examples, see [`Restrict`].
    ///
    /// [`Restrict`]: Restrict
    fn has_feature(&self, feature: Self::Feature) -> bool;
}
