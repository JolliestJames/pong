extern crate amethyst;

use amethyst::Result;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Event, KeyboardInput,
                         Pipeline, PosTex, RenderBundle, Stage, 
                         VirtualKeyCode, WindowEvent};

struct Pong;