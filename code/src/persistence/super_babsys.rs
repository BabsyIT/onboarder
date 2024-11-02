use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rocket::{Build, Rocket};

use crate::superbabsys::SuperBabsy;

use super::fake_data;

pub struct SuperBabsys {
    babsy_map: Arc<Mutex<HashMap<String, SuperBabsy>>>,
}

impl SuperBabsys {
    pub fn get_super_babsys(&self) -> Vec<SuperBabsy> {
        self.babsy_map.lock().unwrap().values().cloned().collect()
    }

    pub fn get_super_babsy(&self, id: &str) -> Option<SuperBabsy> {
        self.babsy_map.lock().unwrap().get(id).cloned()
    }
}

pub fn manage(rocket: Rocket<Build>) -> Rocket<Build> {
    let babsy_map = Arc::new(Mutex::new(fake_data::fake_data()));
    let in_memory_super_babsy = SuperBabsys { babsy_map };
    rocket.manage(in_memory_super_babsy)
}
