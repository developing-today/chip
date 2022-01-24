 struct Msaa {
     samples: u32,
 }

 impl Default for Msaa {
     fn default() -> Self {
         Self { samples: 4 }
     }
 }

 fn greet_pixels(query: Query<&Pixels>) {
         for px in query.iter() {
             let counter = 0;
             for i in px.0.iter() {
                 println!("hello px:{}!", i);
                 counter += 1;
             }
         }
     }
 }
/*
Input<KeyCode>: Keyboard key state, as a binary Input value
Input<MouseButton>: Mouse button state, as a binary Input value
Touches: The state of all fingers currently touching the touchscreen
MouseButtonInput: Changes in the state of mouse buttons
MouseWheel:
KeyboardInput: Changes in the state of keyboard keys (keypresses, not text)
ReceivedCharacter: Unicode text input from the OS (correct handling of the user's language and layout)
TouchInput: Changx
AppExit: Tell Bevy to shut down
CloseWindow: Tell Bevy to close a window
CreateWindow: Tell Bevy to open a new window
Transform: Local transform (relative to parent, if any)
Timer: Track if a time interval has elapsed
Stopwatch: Track how much time has passed
chiprs
chippers
bevy
lox
bevyclapchipnomloxrs
chip8lors
llvm? verilog? lol

*/

  Create an orthographic projection camera to render 2D content.
//
  The projection creates a camera space where X points to the right of the screen,
  Y points to the top of the screen, and Z points out of the screen (backward),
  forming a right-handed coordinate system. The center of the screen is at `X=0` and
  `Y=0`.
//
  The default scaling mode is [`ScalingMode::WindowSize`], resulting in a resolution
  where 1 unit in X and Y in camera space corresponds to 1 logical pixel on the screen.
  That is, for a screen of 1920 pixels in width, the X coordinates visible on screen go
  from `X=-960` to `X=+960` in world space, left to right. This can be changed by changing
  the [`OrthographicProjection::scaling_mode`] field.
//
  The camera is placed at `Z=+1000-0.1`, looking toward the world origin `(0,0,0)`.
  Its orthographic projection extends from `0.0` to `-1000.0` in camera view space,
  corresponding to `Z=+999.9` (closest to camera) to `Z=-0.1` (furthest away from
  camera) in world space.
  we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
  the camera's translation by far and use a right handed coordinate system

 let far = 1000.0;

 let orthographic_projection = OrthographicProjection {
     far,
     scaling_mode: ScalingMode::None,
     depth_calculation: DepthCalculation::ZDifference,
     left: 32.0 * 40.0,
     right: 32.0 * 40.0,
     top: 16.0 * 40.0,
     bottom: 16.0 * 40.0,
     ..Default::default()
 };
 let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);
 let view_projection = CameraProjection::get_projection_matrix(&orthographic_projection)
     * transform.compute_matrix().inverse();
 let frustum = Frustum::from_view_projection( * 40.0
     &view_projection,
     &transform.translation,
     &transform.back(),
     orthographic_projection.far(),
 );

 commands.spawn_bundle(OrthographicCameraBundle {
     camera: Camera {
         name: Some(CameraPlugin::CAMERA_2D.to_string()),
         near: orthographic_projection.near,
         far: orthographic_projection.far,
         ..Default::default()
     },
     orthographic_projection,
     visible_entities: VisibleEntities::default(),
     frustum,
     transform,
     global_transform: Default::default(),
 });
 commands.spawn_bundle(GeometryBuilder::build_as(
     &shape,
     DrawMode::Outlined {
         fill_mode: FillMode::color(Color::CYAN),
         outline_mode: StrokeMode::new(Color::BLACK, 1.0),
     },
     std::ops::Fn::call(&Transform::from_xyz, (-0.0, -0.0, 0.0)),
 ));
 commands.spawn_bundle(GeometryBuilder::build_as(
     &shape,
     DrawMode::Outlined {
         fill_mode: FillMode::color(Color::CYAN),
         outline_mode: StrokeMode::new(Color::BLACK, 5.0),
     },
     std::ops::Fn::call(&Transform::from_xyz, (00.0, 00.0, 0.0)),
 ));
 commands.spawn_bundle(GeometryBuilder::build_as(
     &shape,
     DrawMode::Outlined {
         fill_mode: FillMode::color(Color::CYAN),
         outline_mode: StrokeMode::new(Color::BLACK, 5.0),
     },
     std::ops::Fn::call(&Transform::from_xyz, (750.0, 750.0, 0.0)),
 ));
 commands.spawn_bundle(GeometryBuilder::build_as(
     &shape,
     DrawMode::Outlined {
         fill_mode: FillMode::color(Color::CYAN),
         outline_mode: StrokeMode::new(Color::BLACK, 5.0),
     },
     std::ops::Fn::call(&Transform::from_xyz, (750.0, -750.0, 0.0)),
 ));
 commands.spawn_bundle(GeometryBuilder::build_as(
     &shape,
     DrawMode::Outlined {
         fill_mode: FillMode::color(Color::CYAN),
         outline_mode: StrokeMode::new(Color::BLACK, 5.0),
     },
     std::ops::Fn::call(&Transform::from_xyz, (-750.0, 750.0, 0.0)),
 ));
 }
 fn setup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {

 let mut contributor_selection = ContributorSelection {
     order: vec![],
     idx: 0,
 };

 let mut rnd = rand::thread_rng();

 for name in contribs {
     let pos = (rnd.gen_range(-400.0..400.0), rnd.gen_range(0.0..400.0));
     let dir = rnd.gen_range(-1.0..1.0);
     let velocity = Vec3::new(dir * 500.0, 0.0, 0.0);
     let hue = rnd.gen_range(0.0..=360.0);

      some sprites should be flipped
     let flipped = rnd.gen_bool(0.5);

     let transform = Transform::from_xyz(pos.0, pos.1, 0.0);

     let entity = commands
         .spawn()
         .insert_bundle((
             Contributor { hue },
             Velocity {
                 translation: velocity,
                 rotation: -dir * 5.0,
             },
         ))
         .insert_bundle(SpriteBundle {
             sprite: Sprite {
                 custom_size: Some(Vec2::new(1.0, 1.0) * SPRITE_SIZE),
                 color: Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA),
                 flip_x: flipped,
                 ..Default::default()
             },
             texture: texture_handle.clone(),
             transform,
             ..Default::default()
         })
         .id();

     contributor_selection.order.push((name, entity));
 }

 contributor_selection.order.shuffle(&mut rnd);

 commands.insert_resource(contributor_selection);
 }

 fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
     commands.spawn_bundle(UiCameraBundle::default());

     commands.spawn_bundle((SelectTimer, Timer::from_seconds(SHOWCASE_TIMER_SECS, true)));

     commands
         .spawn()
         .insert(ContributorDisplay)
         .insert_bundle(TextBundle {
             style: Style {
                 align_self: AlignSelf::FlexEnd,
                 ..Default::default()
             },
             text: Text {
                 sections: vec![
                     TextSection {
                         value: "Contributor showcase".to_string(),
                         style: TextStyle {
                             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                             font_size: 60.0,
                             color: Color::WHITE,
                         },
                     },
                     TextSection {
                         value: "".to_string(),
                         style: TextStyle {
                             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                             font_size: 60.0,
                             color: Color::WHITE,
                         },
                     },
                 ],
                 ..Default::default()
             },
             ..Default::default()
         });
 }
//         if disabled.is_none() {
//             commands.remove_component::<Disabled>(pixel.0);
//             commands.insert_component(pixel.0, Disabled());
//         }
//         commands.insert_component(
//             pixel.0,
//             Transform::from_xyz(xyz_from_i(pixel.1, cpu.counter), 0.0, 0.0),
//         );
//     }
//     for i in 0..PIXELS {
//         if cpu.screen[i] {
//             println!("hello px:{:?}!", pixels.0[i]);
//         } else {
//             commands.entity(pixel.0[i]).insert(Disabled());
//         }
//     }
// }
