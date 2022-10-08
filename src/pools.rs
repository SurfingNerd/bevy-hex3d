use bevy::{prelude::{Handle, StandardMaterial, Assets, ResMut, info}, utils::HashMap};



pub struct MaterialRegistry {
  current_handles: HashMap<usize, Handle<StandardMaterial>>
}

/// MaterialRegistry can store often used materials by indexing it by a `create` function that creates the materials one time
/// and returns the stored material on subsequent calls, see function get_or_create
impl MaterialRegistry {
  
  pub fn new() -> Self {
    MaterialRegistry{ current_handles: HashMap::new() }
  }


  /// creates material created by create function and returns stored Material Handle in subsequent calls.
  pub fn get_or_create(&mut self, assets: &mut ResMut<Assets<StandardMaterial>>, create: fn(&mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial>) -> Handle<StandardMaterial> {

    let address = create as usize;
    // info!("material at: {}", address);

    if let Some(result) = self.current_handles.get(&address) {
      return result.as_weak(); // not sure if i should return as_weak() here or just a clone ?
    } else {
      let mut handle = create(assets);
      handle.make_strong(assets);
      let result = handle.as_weak(); // not sure if i should return as_weak() here or just a clone ?
      self.current_handles.insert(address, handle);
      return result;
    }
  }
}

impl Default for MaterialRegistry {

  fn default() -> Self {
    MaterialRegistry { current_handles: HashMap::new() }
  }
}
