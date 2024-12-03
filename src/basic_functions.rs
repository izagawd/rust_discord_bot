use rand::Rng;

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

