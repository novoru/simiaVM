pub enum Object {
    Integer {
        value: i64,
    },

    Boolean {
        value: bool,
    },

    Null,
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer { value }   => format!("{}", value),
            Object::Boolean { value }   => format!("{}", value),
            Object::Null                => format!("null"),
            _                           => panic!()
        }
    }

    pub fn kind(&self) -> String {
        match self {
            Object::Integer { value: _ }   => "Integer".to_string(),
            Object::Boolean { value: _ }   => "Boolean".to_string(),
            Object::Null                => "Null".to_string(),
            _                           => panic!()
        }
    }
}