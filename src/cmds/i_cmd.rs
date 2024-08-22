pub trait ICommand {
    fn generate_url(&self) -> String;
    fn generate_mthod(&self) -> String;
    fn get_file(&self) -> String;
    fn print_curl(&self, notion_api_key: String, notion_version: String);
}