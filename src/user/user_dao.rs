#[derive(Clone, Debug, Queryable)]
pub struct UserDao {
    id: i32,
    name: String,
}

#[cfg(test)]
mod tests {
    use crate::establish_connection;
    use crate::user::user_dao::UserDao;
    use diesel::prelude::*;

    #[test]
    fn user_database() {
        let connection: SqliteConnection = establish_connection();
        let user = crate::schema::users::dsl::users
            .first::<UserDao>(&connection)
            .expect("ERROR");

        println!("#### {:?}", user);
    }
}
