pub struct SpawnRequest {
    pub x: i32,
    pub y: i32,
    pub spawn_name: String,
}

pub struct SpawnBuilder {
    pub requests: Vec<SpawnRequest>,
}

impl SpawnBuilder {
    pub fn new() -> SpawnBuilder {
        SpawnBuilder {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, x: i32, y: i32, spawn_name: String) {
        self.requests.push(SpawnRequest { x, y, spawn_name });
    }
}
