pub enum Command {
    Insert { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Exit,

}