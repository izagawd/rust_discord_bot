use std::sync::LazyLock;
use ab_glyph::{Font, FontRef};
use rand::Rng;


pub static GLOBAL_FONT : LazyLock<FontRef> = LazyLock::new(||{
     FontRef::try_from_slice(include_bytes!("C:/windows/fonts/arial.ttf"))
        .expect("Couldn't load font data")

});
pub fn random_choice<T>(options: impl Iterator<Item = T> + Clone) -> Result<T,&'static str>{
    let count =options.clone().count();
    if count == 0{
       return  Err("Collection is empty")
    }


    let chosen_index = rand::rng().random_range(0..count);
    let mut index = 0;
    for i in options {
        if index == chosen_index {
            return Ok(i)
        }
        index += 1;
    }
    Err("This usually shouldn't happen")

}

