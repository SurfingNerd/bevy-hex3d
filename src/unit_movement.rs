

pub trait UnitMovement {
    fn get_movement_tick_cost(&mut self, unit: &mut Unit, direction: Direction);

    fn continue_movement(&mut self, unit: &mut Unit, direction: Direction);
}

pub fn move_entites_system(
  mut query: Query<(&mut PositionComponent, &mut MoveComponent, &mut Transform)>,
  mut game: ResMut<Game>,
) {
  for (mut position, mut movement, mut transform) in query.iter_mut() {
      movement.ticks_passed += 1;
      let old_x = position.x;
      let old_y = position.y;
      if movement.ticks_passed >= movement.ticks_to_move {
          if position.x < position.y && game.get_entity(position.x + 1, position.y) == None {
              position.x += 1;
          } else if game.get_entity(position.x, position.y + 1) == None {
              position.y += 1;
          } else {
            info!("nowhere to go ?!");
          }

          // if the entity would move outside of the game, 
          // we do not move and it just stays where it is.
          if position.x >= game.height || position.y >= game.width {
              // info!("reached the edge of the world.");
              continue;
          }
          
          let entity = game.delete_entity(old_x, old_y);
          game.set_entity(position.x, position.y, entity);

          movement.ticks_passed = 0;

          // update the UI Pos.
          let c = hex2d::Coordinate::new(position.x, position.y);
          let (x_pixel, y_pixel) = c.to_pixel(game.hex_spacing);
          transform.translation.x = x_pixel;
          transform.translation.z = y_pixel;
          transform.translation.y = (game.get_height(position.x, position.y) as f32 / 1000.0) + 0.2;
          // transform.translation = Vec3:: { x_pixel, 0.01, y_pixel };

          // info!(
          //     "Updated Position to {:?} to {:?}",
          //     position, transform.translation
          // );
      }
  }
}
