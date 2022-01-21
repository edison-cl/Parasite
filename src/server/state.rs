use std::sync::Mutex;
pub struct AppState{
    pub count:Mutex<u32>
}