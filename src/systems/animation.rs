use amethyst::{
  core::timing::Time,
  derive::SystemDesc,
  ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
  renderer::{SpriteRender},
};

use crate::state::Animation;

#[derive(SystemDesc)]
pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
  type SystemData = (
    ReadStorage<'s, Animation>,
    WriteStorage<'s, SpriteRender>,
    Read<'s, Time>,
  );

  fn run(&mut self, (animations, mut sprite_renders, time): Self::SystemData) {
    // Get every animation with its associated SpriteRender component.
    for (animation, sprite) in (&animations, &mut sprite_renders).join() {
      let elapsed_time = time.frame_number();
      let frame = (elapsed_time / animation.frame_duration) as i32 % animation.frames;

      sprite.sprite_number = animation.first_sprite_index + frame as usize;
    }
  }
}
