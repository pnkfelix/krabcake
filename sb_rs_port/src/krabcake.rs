const fn vg_userreq_tool_base(a: u32, b: u32) -> u32 {
    ((a)&0xff) << 24 | ((b)&0xff) << 16
}

#[allow(dead_code)]
#[repr(u32)]
pub enum VgKrabcakeClientRequest {
    BorrowMut = vg_userreq_tool_base('K' as u32, 'C' as u32),
    BorrowShr,
    AsRaw,
    AsBorrowMut,
    AsBorrowShr,
    RetagFnPrologue,
    RetagAssign,
    RetagRaw,
    KrabcakeRecordOverlapError = vg_userreq_tool_base('K' as u32, 'C' as u32) + 256,
}
