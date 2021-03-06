use super::contiguous_buffer::ContiguousBuffer;
use super::kv_buffer_type::KvBufferType;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyBuffer {
    pub _type: KvBufferType,
    pub contig: ContiguousBuffer,
}
