use nalgebra::Vector3;
use crate::game_node::*;
use crate::rotation::Rotation;
use crate::transform::Transform;

pub struct SceneGameNodeImpl{
    parent: GameNodeImpl,
    local_transform: Transform<f32>
}

impl GameNode for SceneGameNodeImpl {
    fn get_game_node_impl(&self) -> &GameNodeImpl {
        &self.parent
    }

    fn get_game_node_impl_mut(&mut self) -> &mut GameNodeImpl {
        &mut self.parent
    }

    fn new() -> Self
    where
        Self: Sized
    {
        SceneGameNodeImpl{
            parent: GameNodeImpl::new(),
            local_transform: Transform::new(Vector3::new(0.0, 0.0, 0.0),
                                            Vector3::new(1.0,1.0,1.0),
                                            Rotation::new(0.0,0.0,0.0))}
    }
}

impl SceneGameNode for SceneGameNodeImpl {

    fn get_scene_game_node_impl(&self) -> &SceneGameNodeImpl {
        self
    }
    fn get_scene_game_node_impl_mut(&mut self) -> &mut SceneGameNodeImpl {
        self
    }
}
pub trait SceneGameNode : GameNode{
    fn local_transform(&self) -> &Transform<f32>{
        &self.get_scene_game_node_impl().local_transform
    }
    fn local_transform_mut(&mut self) -> &mut Transform<f32>{
        &mut self.get_scene_game_node_impl_mut().local_transform
    }
    fn get_scene_game_node_impl(&self) -> &SceneGameNodeImpl;
    fn get_scene_game_node_impl_mut(&mut self) -> &mut SceneGameNodeImpl;
}
