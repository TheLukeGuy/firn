use crate::mem::MemMap;

pub struct System {
    pub mem: MemMap,
}

impl System {
    pub fn new(mem: MemMap) -> Self {
        Self { mem }
    }
}
