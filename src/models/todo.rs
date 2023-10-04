use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ToDo {
    pub id: i32,
    pub username: String,
    pub title: String,
    pub completed: bool,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub completed_date: Option<chrono::NaiveDate>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(primary_key(id))]
pub struct NewToDo<'a> {
    #[diesel(deserialize_as = i32)]
    pub id: i32,
    pub username: &'a str,
    pub title: &'a str,
    pub completed: bool,
    pub description: Option<&'a str>,
    pub due_date: Option<chrono::NaiveDate>,
}
