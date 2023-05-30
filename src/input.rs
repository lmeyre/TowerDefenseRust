use bevy::prelude::*;

pub fn handle_input(
    mouse_button_input: ResMut<Input<MouseButton>>,
    mut left_click_event: EventWriter<LeftClickEvent>,
    mut right_click_event: EventWriter<RightClickEvent>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        left_click_event.send(LeftClickEvent);
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        right_click_event.send(RightClickEvent);
    }
}

pub struct LeftClickEvent;
pub struct RightClickEvent;
