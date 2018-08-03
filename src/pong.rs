use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput,
                         VirtualKeyCode, WindowEvent};

pub struct Pong;

impl<'a, 'b> State<GameData<'a, 'b>> for Pong {
    
    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    fn update(&mut self, game_state: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        game_state.data.update(&game_state.world);
        Trans::None
    }

}