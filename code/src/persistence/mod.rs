use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rocket::{Build, Rocket};

use crate::superbabsys::SuperBabsy;

pub struct SuperBabsys {
    babsyMap: Arc<Mutex<HashMap<String, SuperBabsy>>>,
}

impl SuperBabsys {
    pub fn get_super_babsys(&self) -> Vec<SuperBabsy> {
        self.babsyMap.lock().unwrap().values().cloned().collect()
    }
}

pub fn manage(rocket: Rocket<Build>) -> Rocket<Build> {
    let babsy_map = Arc::new(Mutex::new(HashMap::new()));
    let in_memory_super_babsy = SuperBabsys {
        babsyMap: babsy_map,
    };
    rocket.manage(in_memory_super_babsy)
}
