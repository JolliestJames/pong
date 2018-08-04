use amethyst::assets::Loader;
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Event, KeyboardInput, Material, MaterialDefaults, MeshHandle,
                         PosTex, VirtualKeyCode, WindowEvent};

pub struct Pong;

impl<'a, 'b> State<GameData<'a, 'b>> for Pong {
    
    fn on_start(&mut self, game_state: StateData<GameData>) {
        let world = game_state.world;
        world.register::<Paddle>();
    }

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

#[derive(PartialEq, Eq)]
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

const PADDLE_HEIGHT: f32 = 0.30;
const PADDLE_WIDTH: f32 = 0.05;
const PADDLE_COLOR: [f32; 4] = [0.0, 1.0, 1.0, 1.0];

fn initialize_paddles(world: &mut World) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let mesh = create_mesh(
        world,
        generate_rectangle_vertices(0.0, 0.0, PADDLE_WIDTH, PADDLE_HEIGHT),
    );
    
    let material = create_color_material(world, PADDLE_COLOR);

    let y = -PADDLE_HEIGHT/2.0;
    left_transform.translation = Vector3::new(-1.0, y, 0.0);
    right_transform.translation = Vector3::new(1.0 - PADDLE_WIDTH, y, 0.0);

    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(Paddle::new(Side::Left))
        .with(GlobalTransform::default())
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(Paddle::new(Side::Right))
        .with(GlobalTransform::default())
        .with(right_transform)
        .build();

}

fn generate_rectangle_vertices(left: f32,
                               bottom: f32,
                               right: f32, 
                               top: f32) -> Vec<PosTex> {

    vec![
        PosTex {
            position: [left, bottom, 0.],
            tex_coord: [0.0, 0.0],
        },
        PosTex {
            position: [right, bottom, 0.],
            tex_coord: [0.0, 0.0],
        },
        PosTex {
            position: [left, top, 0.],
            tex_coord: [0.0, 0.0],
        },
        PosTex {
            position: [right, top, 0.],
            tex_coord: [0.0, 0.0],
        },
        PosTex {
            position: [left, top, 0.],
            tex_coord: [0.0, 0.0],
        },

        PosTex {
            position: [right, bottom, 0.],
            tex_coord: [0.0, 0.0],
        },
    ]

}

fn create_mesh(world: &World, vertices: Vec<PosTex>) -> MeshHandle {
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}

/// Creates a solid material of the specified color.
fn create_color_material(world: &World, color: [f32; 4]) -> Material {
    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load_from_data(color.into(),
                                       (),
                                       &world.read_resource());

    Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}