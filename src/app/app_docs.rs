use super::app::App;

impl App {
    pub fn docs(query: &String) {
        dbg!("Docs for {}", query);
    }
}
