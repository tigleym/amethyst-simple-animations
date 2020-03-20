use amethyst::{
  assets::{AssetStorage, Loader, Handle},
  core::transform::Transform,
  ecs::prelude::{Component, DenseVecStorage},
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const CAMERA_HEIGHT: f32 = 100.0;
pub const CAMERA_WIDTH: f32 = 100.0;

pub struct Animation {
  pub frames: i32,
  pub frame_duration: u64,
}

impl Component for Animation {
  type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct MyState {
  sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for MyState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    // load the spritesheet necessary to render the graphics
    self.sprite_sheet_handle.replace(load_sprite_sheet(world));

    initialize_camera(world);
    initialize_sprite(world, self.sprite_sheet_handle.clone().unwrap());
  }
}

fn initialize_camera(world: &mut World) {
  // Setup camera in a way that our screen covers whole arena and (0,0) is in the bottom left.
  let mut transform = Transform::default();
  transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 1.0);

  world
    .create_entity()
    .with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
    .with(transform)
    .build();
}

// Initializes the sprite to animate.
fn initialize_sprite(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
  // Create the translation
  let mut local_transform = Transform::default();
  local_transform.set_translation_xyz(CAMERA_WIDTH / 2.0, CAMERA_HEIGHT / 2.0, 0.0);

  // Assign the sprite to display
  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet_handle,
    sprite_number: 0, // the first frame for this example is the first sprite.
  };

  world
    .create_entity()
    .with(sprite_render)
    .with(Animation {
      frames: 4,
      frame_duration: 10,
    })
    .with(local_transform)
    .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  // Load the sprite sheet necessary to render the graphics.
  // The texture is the pixel data.
  // `texture_handle` is a cloneable reference to the texture
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      "texture/sprite_sheet.png",
      ImageFormat::default(),
      (),
      &texture_storage,
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "texture/sprite_sheet.ron", // Here we load the associated ron file
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}
