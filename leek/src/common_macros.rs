#[macro_export]
macro_rules! keydown {
    ($x:ident) => {
        Event::KeyDown { scancode: Some(Scancode::$x), .. }
    };
}

/// Returns a new world, with the specified components registered.
#[macro_export]
macro_rules! world_with {
    ( $( $x:ty ),+ ) => {
          {
            let mut world = World::new();
            $(
                world.register::<$x>();
            )*
            world
          }
    };
}

/// Generate an assets module with the keys for audio and texture resources.
/// (use it with a build.rs and leek_codegen)
#[macro_export]
macro_rules! assets_gen {
    () => {
        mod assets {
            use leek::resources::PathKey;

            include!(concat!(env!("OUT_DIR"), "/texture_file_keys.rs"));
            include!(concat!(env!("OUT_DIR"), "/audio_file_keys.rs"));
        }
    };
}
