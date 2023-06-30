pub enum MessageIO<T> {
    Break,
    Continue,
    Ok(T),
}
