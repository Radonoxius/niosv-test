unsafe extern "C" {
    static _heap_size: u8;
}

pub fn heap_size() -> usize {
    &raw const _heap_size as usize
}