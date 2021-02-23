use crate::user::vo::name::Name;
use crate::user::vo::userid::UserId;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct User {
    id: UserId,
    name: Name,
}

impl User {
    pub fn new(id: UserId, name: Name) -> Result<Self> {
        Ok(Self { id, name })
    }

    pub fn id(&self) -> UserId {
        self.id.clone()
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }
}

//IDで同一性を評価する
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use crate::user::user::User;
    use crate::user::vo::name::tests::{name1, name2};
    use crate::user::vo::userid::tests::{userid1, userid2};

    fn user1() -> User {
        User::new(userid1(), name1()).unwrap()
    }

    fn user2() -> User {
        User::new(userid2(), name2()).unwrap()
    }

    #[test]
    fn user_equals() {
        //IDによって同一と見なされること
        let user1 = self::user1();
        let user2 = self::user1();
        assert_eq!(user1, user2);

        //IDによって同一と見なされないこと
        let user3 = self::user2();
        assert_ne!(user1, user3);
    }
}
