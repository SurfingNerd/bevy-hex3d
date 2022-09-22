use bevy::prelude::{Handle, StandardMaterial, Assets, ResMut};




#[derive(Debug)]
pub struct MaterialPool {

  current_handle: Option<Handle<StandardMaterial>>

}

impl MaterialPool {
  
  pub fn new() -> Self {
    MaterialPool{ current_handle: None }
  }

  // pub fn set(&mut self, assets: &mut Assets<StandardMaterial>, mut value: Handle<StandardMaterial>) {
  //   value.make_strong(assets);
  //   self.current_handle = Some(value);
  // }

  // pub fn get(&mut self) -> &Option<Handle<StandardMaterial>> {
  //   return &self.current_handle;
  // }

  pub fn get_or_create(&mut self, assets: &mut ResMut<Assets<StandardMaterial>>, create: fn(&mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial>) -> Handle<StandardMaterial> {
    if let Some(result) = &self.current_handle {
      return result.as_weak();
    } else {
      let mut handle = create(assets);
      handle.make_strong(assets);
      self.current_handle = Some(handle);
      if let Some(handle) = &self.current_handle {
        return handle.clone();
      }
      panic!();
    }
  }
}

impl Default for MaterialPool {

  fn default() -> Self {
    MaterialPool { current_handle: None }
  }
}
