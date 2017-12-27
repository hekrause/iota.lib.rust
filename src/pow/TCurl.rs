
pub enum Mode {
    CURLP27,
    CURLP81,
    KERL,
}

pub trait TCurl {
    fn new(mode: Mode) -> Self;
    fn absorb(&mut self, trits: &Vec<i8>, offset: usize, length: usize);
    fn squeeze(&mut self, trits: &mut Vec<i8>, offset: usize, length: usize);
    fn transform(&mut self);
    fn reset(&mut self);
    fn get_state(&self);
    fn set_state(&self);
    fn clone(&self);
}