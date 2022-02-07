use bevy::{
    core::Time,
    prelude::{Added, Color, Commands, Component, Entity, Query, Res, Without},
};
use bevy_prototype_lyon::prelude::DrawMode;
use std::ops;
pub(crate) const SCREEN_X: usize = 64;
pub(crate) const SCREEN_Y: usize = 32;
pub(crate) const PIXELS: usize = SCREEN_X * SCREEN_Y;
const HALF_X: usize = SCREEN_X / 2;
const HALF_Y: usize = SCREEN_Y / 2;
const PIXEL_X: usize = 10;
const PIXEL_Y: usize = 10;
#[derive(Debug, Clone, Copy)]
pub(crate) struct Pixel(pub(crate) Entity, pub(crate) bool);
#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Screen(pub(crate) [Pixel; PIXELS]);
#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Disabled();
#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Off();
pub(crate) fn pixels_disable(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DrawMode), (Without<Off>, Added<Disabled>)>,
) {
    // println!("disable:{:?}", query.iter().count());
    for (id, mut draw_mode) in query.iter_mut() {
        match *draw_mode {
            DrawMode::Outlined {
                ref mut fill_mode,
                ref mut outline_mode,
            } => {
                fill_mode.color = Color::BLACK;
                outline_mode.color = Color::BLACK;
            }
            _ => (),
        }
        commands.entity(id).insert(Off());
    }
}
pub(crate) fn pixels_change_color(time: Res<Time>, mut query: Query<&mut DrawMode, Without<Off>>) {
    let hue = ((time.time_since_startup().as_millis() / 450) % 360) as f32;
    for mut draw_mode in query.iter_mut() {
        // println!("hue:{:?}", hue);
        if let DrawMode::Outlined {
            ref mut fill_mode,
            ref mut outline_mode,
        } = *draw_mode
        {
            fill_mode.color = Color::hsl(hue, 1., 0.5);
            outline_mode.color = Color::BLACK;
        }
    }
}
pub(crate) fn i_to_xyz(i: usize) -> (f32, f32, f32) {
    let xyz: Xyz = i.into();
    xyz.into()
}
#[derive(Debug, Clone, Copy)]
struct Xyz {
    x: f32,
    y: f32,
    z: f32,
}
#[derive(Debug, Clone, Copy)]
struct Iz {
    i: usize,
    z: isize,
}
impl From<usize> for Iz {
    fn from(i: usize) -> Self {
        (i, 0isize).into()
    }
}
impl From<(usize, isize)> for Iz {
    fn from((i, z): (usize, isize)) -> Self {
        Iz { i, z }
    }
}
impl From<Iz> for (usize, isize) {
    fn from(iz: Iz) -> Self {
        (iz.i, iz.z)
    }
}
impl From<usize> for Xyz {
    fn from(i: usize) -> Self {
        let iz: Iz = i.into();
        iz.into()
    }
}
fn div_rem<T: ops::Div<Output = T> + ops::Rem<Output = T> + Copy>(x: T, y: T) -> (T, T) {
    (x / y, x % y)
}
impl From<Iz> for Xyz {
    fn from(iz: Iz) -> Self {
        let (i, z) = iz.into();
        let (y, x) = div_rem(i as isize, SCREEN_X as isize);
        Self {
            x: ((x - HALF_X as isize) * PIXEL_X as isize) as f32,
            y: ((-y + HALF_Y as isize) * PIXEL_Y as isize) as f32,
            z: z as f32,
        }
    }
}
impl From<Xyz> for (f32, f32, f32) {
    fn from(xyz: Xyz) -> Self {
        (xyz.x, xyz.y, xyz.z)
    }
}
