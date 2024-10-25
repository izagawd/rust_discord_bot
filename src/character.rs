

pub trait Character{

    fn health(&self) ->  &f32;
    fn health_mut(&mut self) -> &mut f32;
    fn damage(&mut self, damage : f32) {
        *self.health_mut() -= damage;
    }
}

