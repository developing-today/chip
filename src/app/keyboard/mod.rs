use bevy::{
    input::Input,
    prelude::{KeyCode, Query, Res},
};
pub(crate) fn key_pressed(keys: Res<Input<KeyCode>>, mut query: Query<&mut super::chiploxide::Cpu>) {
    for mut cpu in query.iter_mut() {
        for key in keys.get_pressed() {
            match key {
                KeyCode::Numpad1 => cpu.keys[1] = true,
                KeyCode::Numpad2 => cpu.keys[2] = true,
                KeyCode::Numpad3 => cpu.keys[3] = true,
                KeyCode::Numpad4 => cpu.keys[0xC] = true,
                KeyCode::Q => cpu.keys[4] = true,
                KeyCode::W => cpu.keys[5] = true,
                KeyCode::E => cpu.keys[6] = true,
                KeyCode::R => cpu.keys[0xD] = true,
                KeyCode::A => cpu.keys[7] = true,
                KeyCode::S => cpu.keys[8] = true,
                KeyCode::D => cpu.keys[9] = true,
                KeyCode::F => cpu.keys[0xE] = true,
                KeyCode::Z => cpu.keys[0xA] = true,
                KeyCode::X => cpu.keys[0] = true,
                KeyCode::C => cpu.keys[0xB] = true,
                KeyCode::V => cpu.keys[0xF] = true,
                KeyCode::Space => cpu.keys[0] = true,
                KeyCode::Return => cpu.keys[1] = true,
                KeyCode::Back => cpu.keys[2] = true,
                KeyCode::Left => cpu.keys[3] = true,
                KeyCode::Up => cpu.keys[4] = true,
                KeyCode::Right => cpu.keys[5] = true,
                KeyCode::Down => cpu.keys[6] = true,
                _ => (),
            }
        }
    }
}
pub(crate) fn key_just_released(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut super::chiploxide::Cpu>,
) {
    for mut cpu in query.iter_mut() {
        for key in keys.get_just_released() {
            match key {
                KeyCode::Numpad1 => cpu.keys[1] = false,
                KeyCode::Numpad2 => cpu.keys[2] = false,
                KeyCode::Numpad3 => cpu.keys[3] = false,
                KeyCode::Numpad4 => cpu.keys[0xC] = false,
                KeyCode::Q => cpu.keys[4] = false,
                KeyCode::W => cpu.keys[5] = false,
                KeyCode::E => cpu.keys[6] = false,
                KeyCode::R => cpu.keys[0xD] = false,
                KeyCode::A => cpu.keys[7] = false,
                KeyCode::S => cpu.keys[8] = false,
                KeyCode::D => cpu.keys[9] = false,
                KeyCode::F => cpu.keys[0xE] = false,
                KeyCode::Z => cpu.keys[0xA] = false,
                KeyCode::X => cpu.keys[0] = false,
                KeyCode::C => cpu.keys[0xB] = false,
                KeyCode::V => cpu.keys[0xF] = false,
                KeyCode::Space => cpu.keys[0] = false,
                KeyCode::Return => cpu.keys[1] = false,
                KeyCode::Back => cpu.keys[2] = false,
                KeyCode::Left => cpu.keys[3] = false,
                KeyCode::Up => cpu.keys[4] = false,
                KeyCode::Right => cpu.keys[5] = false,
                KeyCode::Down => cpu.keys[6] = false,
                _ => (),
            }
        }
    }
}
