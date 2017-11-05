use libc::{c_int};

use super::create_st3::CreateSt3;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CreateSt {
    version: c_int,
    pub v3: CreateSt3,
}

unsafe impl Send for CreateSt {}
unsafe impl Sync for CreateSt {}

impl CreateSt {
    pub fn new() -> CreateSt {
        CreateSt {
            version: 3,
            v3: CreateSt3::new()
        }
    }
}

impl CreateSt {
    pub fn version(&self) -> c_int {
        self.version
    }
}
