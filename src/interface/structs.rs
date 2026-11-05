//! C ABI Structs

/// Implementation of `ublksrv_ctrl_dev_info`
#[repr(C)]
#[derive(Default)]
pub(super) struct UblkSrvCtrlDevInfo {
    pub nr_hw_queues: u16,
    pub queue_depth: u16,
    pub state: u16,
    pub io_desc_size: u16,

    pub max_io_buf_bytes: u32,
    pub dev_id: u32,

    pub ublksrv_pid: i32,
    pub pad1: u32,

    pub flags: u64,

    // For ublksrv internal use, invisible to ublk driver
    pub ublksrv_flags: u64,

    pub owner_uid: u32, // store by kernel
    pub owner_gid: u32, // store by kernel
    pub reserved1: u64,
    pub reserved2: u64,
}

#[cfg(test)]
mod tests {
    use crate::interface::structs::UblkSrvCtrlDevInfo;

    fn size_and_alignment<T>(size: usize, align: usize) {
        assert_eq!(std::mem::size_of::<T>(), size);
        assert_eq!(std::mem::align_of::<T>(), align);
    }

    #[test]
    fn verify_size_and_alignment() {
        size_and_alignment::<UblkSrvCtrlDevInfo>(64, 8);
    }
}
