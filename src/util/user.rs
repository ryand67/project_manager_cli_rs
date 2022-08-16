pub struct User {
    pub email: String,
    pub role: String,
    pub name: String,
    pub id: i64,
}

impl User {
    pub fn new(email: String, name: String, role: String, id: i64) -> Self {
        Self {
            email,
            name,
            role,
            id,
        }
    }
}
