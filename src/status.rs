pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.split(" ").collect::<String>().to_lowercase();
        match value.as_str() {
            "todo" => Ok(Self::ToDo),
            "inprogress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            _ => Err(String::from("wrong status")),
        }
    }

    type Error = String;
}

impl Into<String> for Status {
    fn into(self) -> String {
        match self {
            Self::Done => "Done".to_string(),
            Self::InProgress => "In progress".to_string(),
            Self::ToDo => "To do".to_string(),
        }
    }
}
