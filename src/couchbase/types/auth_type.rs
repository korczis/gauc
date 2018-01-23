#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum AuthType {
    Classic = 0,
    Rbac = 1,
    Dynamic = 2
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum AuthFlags {
    Cluster = 1,
    Bucket = 2,
    Both = 3
}
