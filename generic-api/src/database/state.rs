pub struct DatabaseState {
    connection: Vec<Box<dyn crate::database::connection::Connection>>,
}
