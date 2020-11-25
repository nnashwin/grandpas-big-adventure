use crate::{input, resources, util};

use log::*;
use specs::{self, world::Builder};
use warmy;

use std::path;

pub struct World {
    pub resources: resources::Store,
    pub input: input::State,
    pub specs_world: specs::World,
}

impl World {
    pub fn new(resource_dir: &path::Path) -> Self {
        info!("Setting up resource path: {:?}", resource_dir);
        let opt = warmy::StoreOpt::default().set_root(resource_dir);
        let store = warmy::Store::new(opt)
            .expect("Could not create asset store?  Does the directory exist?");

        let mut w = specs::World::new();

        let mut the_world = Self {
            resources: store,
            input: input::State::new(),
            specs_world: w,
        };

        // Make a test entity.
        the_world
            .specs_world
            .create_entity()
            .build();

        the_world
    }
}
