use ctor::ctor;
use cpuinfo_sys::*;
use std::fmt::{self, Debug};
use std::ffi::CStr;

#[ctor]
fn init() {
    unsafe { cpuinfo_initialize() };
}

macro_rules! iter {
    ($Name:ident, $Type:ident, $fn:tt) => {
        pub struct $Name {
            cur: u32,
            total: u32,
        }

        impl $Name {
            fn new(start: u32, count: u32) -> $Name {
                $Name {
                    cur: start, total: count + start,
                }
            }
        }

        impl Iterator for $Name {
            type Item = $Type;

            fn next(&mut self) -> Option<$Type> {
                if self.cur >= self.total {
                    return None;
                }

                let x = $Type(unsafe { $fn(self.cur) });
                self.cur += 1;
                Some(x)
            }
        }
    }
}

pub use cpuinfo_sys::{cpuinfo_uarch, cpuinfo_vendor};

#[repr(transparent)]
pub struct Core(*const cpuinfo_core);

impl Core {
    pub fn vendor(&self) -> cpuinfo_vendor {
        unsafe { *self.0 }.vendor
    }

    pub fn microarch(&self) -> cpuinfo_uarch {
        unsafe { *self.0 }.uarch
    }
    
    pub fn processors(&self) -> Processors {
        let start = unsafe { *self.0 }.processor_start;
        let count = unsafe { *self.0 }.processor_count;
        Processors::new(start, count)
    }

    pub fn core_id(&self) -> u32 {
        unsafe { *self.0 }.core_id
    }

    pub fn cluster(&self) -> Cluster {
        Cluster(unsafe { *self.0 }.cluster)
    }

    pub fn cpuid(&self) -> u32 {
        unsafe { *self.0 }.cpuid
    }

    pub fn frequency(&self) -> u64 {
        unsafe { *self.0 }.frequency
    }
}

#[repr(transparent)]
pub struct Cluster(*const cpuinfo_cluster);

impl Cluster {
    pub fn cores(&self) -> Cores {
        let start = unsafe { *self.0 }.core_start;
        let count = unsafe { *self.0 }.core_count;
        Cores::new(start, count)
    }
}

#[repr(transparent)]
pub struct Package(*const cpuinfo_package);

impl Package {
    pub fn name(&self) -> String {
        let ptr = unsafe { *self.0 }.name;
        let c_str = unsafe { CStr::from_ptr(ptr.as_ptr()) };
        c_str.to_string_lossy().to_string()
    }

    pub fn processors(&self) -> Processors {
        let start = unsafe { *self.0 }.processor_start;
        let count = unsafe { *self.0 }.processor_count;
        Processors::new(start, count)
    }

    pub fn cores(&self) -> Cores {
        let start = unsafe { *self.0 }.core_start;
        let count = unsafe { *self.0 }.core_count;
        Cores::new(start, count)
    }

    pub fn clusters(&self) -> Clusters {
        let start = unsafe { *self.0 }.cluster_start;
        let count = unsafe { *self.0 }.cluster_count;
        Clusters::new(start, count)
    }
}

#[repr(transparent)]
pub struct Processor(*const cpuinfo_processor);

impl Processor {
    pub fn smt_id(&self) -> u32 {
        unsafe { *self.0 }.smt_id
    }

    pub fn core(&self) -> Core {
        Core(unsafe { *self.0 }.core)
    }

    pub fn cluster(&self) -> Cluster {
        Cluster(unsafe { *self.0 }.cluster)
    }

    pub fn package(&self) -> Package {
        Package(unsafe { *self.0 }.package)
    }
}

impl Debug for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Package")
            .field("name", &self.name())
            .field("clusters", &self.clusters().collect::<Vec<_>>())
            .finish()
    }
}

impl Debug for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cluster")
            .field("cores", &self.cores().collect::<Vec<_>>())
            .finish()
    }
}

impl Debug for Processor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Processor")
            .field("smt_id", &self.smt_id())
            .finish()
    }
}

impl Debug for Core {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Core")
            .field("core_id", &self.core_id())
            .field("processors", &self.processors().collect::<Vec<_>>())
            .finish()
    }
}

pub fn processors() -> Processors {
    Processors::new(0, unsafe { cpuinfo_get_processors_count() })
}

pub fn packages() -> Packages {
    Packages::new(0, unsafe { cpuinfo_get_packages_count() })
}

pub fn cores() -> Cores {
    Cores::new(0, unsafe { cpuinfo_get_cores_count() })
}

pub fn clusters() -> Clusters {
    Clusters::new(0, unsafe { cpuinfo_get_clusters_count() })
}

#[cfg(target_os = "linux")]
pub fn current_core() -> Core {
    Core(unsafe { cpuinfo_get_current_core() })
}

#[cfg(target_os = "linux")]
pub fn current_processor() -> Processor {
    Processor(unsafe { cpuinfo_get_current_processor() })
}

iter!(Processors, Processor, cpuinfo_get_processor);
iter!(Packages, Package, cpuinfo_get_package);
iter!(Cores, Core, cpuinfo_get_core);
iter!(Clusters, Cluster, cpuinfo_get_cluster);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let core = processors().next().unwrap().core();
        println!("{:?} {:?}", core.vendor(), core.microarch());
    }
}