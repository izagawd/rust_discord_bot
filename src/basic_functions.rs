use rand::Rng;

pub fn random_choice<T>(options: impl Iterator<Item = T> + Clone) -> Result<T,&'static str>{
    let chosen_index = rand::rng().random_range(0..options.clone().count());
    let mut index = 0;
    for i in options {
        if index == chosen_index {
            return Ok(i)
        }
        index += 1;
    }
    Err("Collection is empty")
}

