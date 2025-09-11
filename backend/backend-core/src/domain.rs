use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resource {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assignment {
    pub resource_id: Uuid,
    pub project_id: Uuid,
    pub year: i32,
    pub month: i32, // 1..=12
}
