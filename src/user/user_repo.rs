use crate::establish_connection;
use crate::schema::users as schema;
use crate::user::user_dao::UserDao;
use diesel::dsl::count;
use diesel::prelude::*;

pub struct UserRepo {}

impl UserRepo {
    fn count() -> i64 {
        let connection: SqliteConnection = establish_connection();
        let count = schema::dsl::users
            .select(count(schema::id))
            .first(&connection)
            .expect("ERROR");
        count
    }

    fn create(user: &UserDao) {
        let connection: SqliteConnection = establish_connection();
        diesel::insert_into(schema::dsl::users)
            .values(user)
            .execute(&connection)
            .expect("ERROR");
    }

    fn delete(id: i32) {
        let connection: SqliteConnection = establish_connection();
        diesel::delete(schema::dsl::users.find(id))
            .execute(&connection)
            .expect("ERROR");
    }

    fn first() -> UserDao {
        let connection: SqliteConnection = establish_connection();
        let user = schema::dsl::users
            .first::<UserDao>(&connection)
            .expect("ERROR");
        user
    }
}

#[cfg(test)]
mod tests {
    use crate::user::user_dao;
    use crate::user::user_repo::UserRepo;

    #[test]
    fn count() {
        let count = UserRepo::count();
        assert!(count >= 0);
    }

    #[test]
    fn create() {
        //1件もレコードが登録されていないこと
        assert_eq!(UserRepo::count(), 0);

        //ユーザ登録
        let user = user_dao::tests::user1();
        UserRepo::create(&user);

        //1件追加されること
        assert_eq!(UserRepo::count(), 1);
    }

    #[test]
    fn delete() {
        //1件追加されていること
        assert_eq!(UserRepo::count(), 1);

        //ユーザ削除
        let user = user_dao::tests::user1();
        UserRepo::delete(user.id);

        //1件もレコードが登録されていないこと
        assert_eq!(UserRepo::count(), 0);
    }

    #[test]
    fn first() {
        //ユーザ登録
        self::create();

        //最初のレコードを取得
        let user = UserRepo::first();
        println!("#### {:?}", user);

        //ユーザ削除
        self::delete();
    }
}
