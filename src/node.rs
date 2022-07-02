pub trait Node {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn as_str(&self) -> String;
}