pub fn detect_marker(buffer: &[u8]) -> bool {
    for i in 0..buffer.len() {
        for j in (i + 1)..buffer.len() {
            if buffer[i] == buffer[j] {
                return false;
            }
        }
    }
    return true;
}
