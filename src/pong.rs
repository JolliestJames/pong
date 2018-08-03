use amethyst::ecs::prelude::{Component, DenseVecStorage};
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

enum Side {
    Left,
    Right,
}

struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side: side,
            width: 1.0,
            height: 1.0
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}