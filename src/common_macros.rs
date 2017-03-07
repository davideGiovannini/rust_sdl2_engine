

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
