mod chip;
mod keyboard;
mod screen;
use bevy::{
    core::Timer,
    prelude::{App, Color, Commands, Msaa, OrthographicCameraBundle, Transform},
    DefaultPlugins,
};
use bevy_prototype_lyon::{
    plugin::ShapePlugin,
    prelude::{DrawMode, FillMode, GeometryBuilder, StrokeMode},
    shapes,
};
use std::ops;
pub(crate) fn new() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .insert_resource(chip::CpuTimer(Timer::from_seconds(0.1, true)))
        .insert_resource(chip::AppTimer(Timer::from_seconds(1. / 60., true)))
        .add_system(screen::pixels_change_color)
        .add_system(screen::pixels_disable)
        .add_system(keyboard::key_pressed)
        .add_system(keyboard::key_just_released)
        .add_system(chip::cpu_cycle)
        .run();
}
fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(5.),
        ..shapes::RegularPolygon::default()
    };

    let empty = commands.spawn().id();
    commands.entity(empty).despawn();
    let mut screen = [screen::Pixel(empty, true); screen::PIXELS];

    for i in 0..screen.len() {
        screen[i].0 = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::CYAN),
                    outline_mode: StrokeMode::new(Color::BLACK, 5.),
                },
                ops::Fn::call(&Transform::from_xyz, screen::i_to_xyz(i)),
            ))
            .id();
    }
    commands
        .spawn()
        .insert(chip::Cpu {
            registers: chip::default_registers(),
            stack: [0; usize::BITS as usize * 4],
            memory: chip::default_memory(),
            counter: 0,
            pointer: 0,
            i: 0,
            delay: 0,
            sound: 0,
            keys: [false; 16],
        })
        .insert(screen::Screen(screen));
}
