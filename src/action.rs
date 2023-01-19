#![allow(unused_imports, dead_code, unused_mut, unused_variables)]

use bevy::{app::*, prelude::*};

pub struct Action {
    priority: i32,
    contents: Vec<Content>,
}

pub enum Content {
    Talk(String),
    Message(String),
    Selection, // TODO::Eventsを使って実装
    Event,     // TODO::Eventsを使って実装
}
