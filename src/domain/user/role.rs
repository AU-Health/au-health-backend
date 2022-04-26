use async_graphql::{Enum, Error};

#[non_exhaustive]
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(type_name = "USER_ROLE", rename_all = "snake_case")]
pub enum Role {
    User,
    Admin,
}

impl Role {

    //checks to see if the user is authorized to do a certain action.
    //in most cases, only admin can do the action, or everyone can do the action
    pub fn authorized(self, authorized_roles: Vec<Role>) -> Result<(), Error> {
        for role in authorized_roles {
            if self == role {
                return Ok(());
            }
        }

        Err(Error::new("Unauthorized"))
    }
}
