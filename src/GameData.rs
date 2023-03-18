use druid::{Data, Lens};


#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub label: String,
    pub board_size : i32,
    pub turn : i32,
    #[data(eq)]
    pub board : Vec<Vec<i32>>,
}