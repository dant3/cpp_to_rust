//! Types for expressing properties of different target platforms and platform-based conditions

/// CPU architecture, as reported by `target_arch`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum Arch {
  X86,
  X86_64,
  Mips,
  PowerPC,
  PowerPC64,
  Arm,
  AArch64,
}

/// Operating system, as reported by `target_os`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum OS {
  Windows,
  MacOS,
  IOS,
  Linux,
  Android,
  FreeBSD,
  DragonFly,
  Bitrig,
  OpenBSD,
  NetBSD,
}

/// Operating system family, as reported by `target_family`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum Family {
  Windows,
  Unix,
}

/// Further disambiguates the target platform with information about the ABI/libc,
/// as reported by `target_env`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum Env {
  Gnu,
  Msvc,
  Musl,
  None,
}

/// Pointer width in bits,
/// as reported by `target_pointer_width`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum PointerWidth {
  P64,
  P32,
}

/// CPU endianness, as reported by `target_endian`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum Endian {
  Little,
  Big,
}

/// Combined information about a target, as reported by configuration
/// values of the Rust compiler.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Target {
  /// CPU architecture
  pub arch: Arch,
  /// Operating system
  pub os: OS,
  /// Operating system family
  pub family: Family,
  /// Further disambiguates the target platform with information about the ABI/libc,
  pub env: Env,
  /// Pointer width in bits,
  pub pointer_width: PointerWidth,
  /// Endianness of the target CPU
  pub endian: Endian,
}

/// Condition on properties of the target. Simple conditions
/// are considered true if the property of the current platform
/// is the same as the associated value of the enum. For
/// example, `Condition::OS(OS::Windows)` will be true on Windows
/// and false otherwise. `And`, `Or` and `Not` variants provide
/// logical operations on nested conditions. `True` and `False`
/// variants provide conditions which are always true and false,
/// respectively.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum Condition {
  Arch(Arch),
  OS(OS),
  Family(Family),
  Env(Env),
  PointerWidth(PointerWidth),
  // Vendor(Vendor),
  Endian(Endian),
  And(Vec<Condition>),
  Or(Vec<Condition>),
  Not(Box<Condition>),
  True,
  False,
}

#[cfg(target_arch = "x86")]
/// Returns current CPU architecture
pub fn current_arch() -> Arch {
  Arch::X86
}
#[cfg(target_arch = "x86_64")]
/// Returns current CPU architecture
pub fn current_arch() -> Arch {
  Arch::X86_64
}
#[cfg(target_arch = "mips")]
/// Returns current CPU architecture
pub fn current_arch() -> Arch {
  Arch::Mips
}
#[cfg(target_arch = "powerpc")]
/// Returns current CPU architecture
pub fn current_arch() -> Arch {
  Arch::PowerPC
}
#[cfg(target_arch = "powerpc64")]
/// Returns current CPU architecture
pub fn current_arch() -> Arch {
  Arch::PowerPC64
}
#[cfg(target_arch = "arm")]
/// Returns current CPU architecture
pub fn current_arch() -> Arch {
  Arch::Arm
}
#[cfg(target_arch = "aarch64")]
/// Returns current CPU architecture
pub fn current_arch() -> Arch {
  Arch::AArch64
}


#[cfg(target_os = "windows")]
/// Returns current operating system
pub fn current_os() -> OS {
  OS::Windows
}
#[cfg(target_os = "macos")]
/// Returns current operating system
pub fn current_os() -> OS {
  OS::MacOS
}
#[cfg(target_os = "ios")]
/// Returns current operating system
pub fn current_os() -> OS {
  OS::IOS
}
#[cfg(target_os = "linux")]
/// Returns current operating system
pub fn current_os() -> OS {
  OS::Linux
}
#[cfg(target_os = "android")]
/// Returns current operating system
pub fn current_os() -> OS {
  OS::Android
}
#[cfg(target_os = "freebsd")]
/// Returns current operating system
pub fn current_os() -> OS {
  OS::FreeBSD
}
#[cfg(target_os = "dragonfly")]
/// Returns current operating system
pub fn current_os() -> OS {
  OS::DragonFly
}
#[cfg(target_os = "bitrig")]
/// Returns current operating system
pub fn current_os() -> OS {
  OS::Bitrig
}
#[cfg(target_os = "openbsd")]
/// Returns current operating system
pub fn current_os() -> OS {
  OS::OpenBSD
}
#[cfg(target_os = "netbsd")]
/// Returns current operating system
pub fn current_os() -> OS {
  OS::NetBSD
}


#[cfg(target_family = "unix")]
/// Returns current operating system family
pub fn current_family() -> Family {
  Family::Unix
}
#[cfg(target_family = "windows")]
/// Returns current operating system family
pub fn current_family() -> Family {
  Family::Windows
}

#[cfg(target_env = "gnu")]
/// Returns current platform disambiguation
pub fn current_env() -> Env {
  Env::Gnu
}
#[cfg(target_env = "msvc")]
/// Returns current platform disambiguation
pub fn current_env() -> Env {
  Env::Msvc
}
#[cfg(target_env = "musl")]
/// Returns current platform disambiguation
pub fn current_env() -> Env {
  Env::Musl
}
#[cfg(target_env = "")]
/// Returns current platform disambiguation
pub fn current_env() -> Env {
  Env::None
}

#[cfg(target_pointer_width = "32")]
/// Returns current pointer width
pub fn current_pointer_width() -> PointerWidth {
  PointerWidth::P32
}
#[cfg(target_pointer_width = "64")]
/// Returns current pointer width
pub fn current_pointer_width() -> PointerWidth {
  PointerWidth::P64
}

#[cfg(target_endian = "little")]
/// Returns current CPU endianness
pub fn current_endian() -> Endian {
  Endian::Little
}
#[cfg(target_endian = "big")]
/// Returns current CPU endianness
pub fn current_endian() -> Endian {
  Endian::Big
}

/// Returns properties of the current target
pub fn current_target() -> Target {
  Target {
    arch: current_arch(),
    os: current_os(),
    family: current_family(),
    env: current_env(),
    pointer_width: current_pointer_width(),
    endian: current_endian(),
  }
}


impl Condition {
  /// Evaluate the condition for `target`. Returns true if
  /// `target` matches the condition.
  pub fn eval(&self, target: &Target) -> bool {
    use target::Condition::*;
    match *self {
      Arch(ref arch) => &target.arch == arch,
      OS(ref os) => &target.os == os,
      Family(ref family) => &target.family == family,
      Env(ref env) => &target.env == env,
      PointerWidth(ref pointer_width) => &target.pointer_width == pointer_width,
      Endian(ref endian) => &target.endian == endian,
      And(ref conditions) => conditions.iter().all(|c| c.eval(target)),
      Or(ref conditions) => conditions.iter().any(|c| c.eval(target)),
      Not(ref condition) => !condition.eval(target),
      True => true,
      False => false,
    }
  }
  /// Construct a condition opposite to `self`.
  pub fn negate(&self) -> Condition {
    Condition::Not(Box::new(self.clone()))
  }
}
