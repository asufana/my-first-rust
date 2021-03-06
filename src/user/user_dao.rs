use crate::schema::users;

#[derive(Clone, Debug, Insertable, Queryable)]
#[table_name = "users"]
pub struct UserDao {
    pub id: i32,
    pub name: String,
}

#[cfg(test)]
pub mod tests {
    use crate::user::user_dao::UserDao;

    pub fn user1() -> UserDao {
        UserDao {
            id: 1,
            name: String::from("hana"),
        }
    }

    // pub fn user2() -> UserDao {
    //     UserDao {
    //         id: 2,
    //         name: String::from("mako"),
    //     }
    // }
}
