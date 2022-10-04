use sqlx;

use super::Repository;
use crate::{db, errors::core::Error, user::entities};

impl Repository {
    pub async fn update_user<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        user: &entities::User,
    ) -> Result<entities::User, Error> {
        const QUERY: &str = "update user_ set
              updated_at = $2,
              name = $3,
              full_name = COALESCE($4, full_name)
           where id = $1 returning *";

        match sqlx::query_as::<_, entities::User>(QUERY)
            .bind(user.id)
            .bind(user.updated_at)
            //
            .bind(&user.name)
            .bind(&user.full_name)
            .fetch_one(db)
            .await
        {
            Err(err) => {
                log::error!("updating user: {}", &err);
                Err(err.into())
            }
            Ok(user) => Ok(user),
        }
    }
}
