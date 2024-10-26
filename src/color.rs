

#[derive(Copy)]
#[derive(Clone)]
struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8

}
impl Color{
    fn new() -> Self{
        Self{r:0,g:0,b:0,a:225}
    }

}
impl Default for Color{
    fn default() -> Self {
        Self::new()
    }
}