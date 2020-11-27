use sqlx::postgres::PgPool;

#[derive(Debug)]
pub enum UserPerm {
    Admin,
    DJ,
    User,
    None,
}

impl From<i16> for UserPerm {
    fn from(i: i16) -> Self {
        match i {
            0 => Self::None,
            1 => Self::User,
            2 => Self::DJ,
            3 => Self::Admin,
            _ => panic!("Can only be 0-3"),
        }
    }
}

impl Into<i16> for UserPerm {
    fn into(self) -> i16 {
        match self {
            Self::None => 0,
            Self::User => 1,
            Self::DJ => 2,
            Self::Admin => 3,
        }
    }
}

pub async fn get_user_perms(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
) -> anyhow::Result<Option<UserPerm>> {
    let rec = match sqlx::query!(
        r#"
        SELECT perm_level
        FROM perms
        WHERE guild_id = $1 AND user_id = $2"#,
        guild_id,
        user_id
    )
    .fetch_optional(pool)
    .await?
    {
        Some(row) => row,
        None => return Ok(None),
    };

    Ok(Some(rec.perm_level.into()))
}

pub async fn set_user_perms(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    perm_level: UserPerm,
) -> anyhow::Result<UserPerm> {
    let perm_level: i16 = perm_level.into();

    let rec = sqlx::query!(
        r#"
        INSERT INTO perms (guild_id, user_id, perm_level) VALUES ($1, $2, $3)
        ON CONFLICT (guild_id, user_id)
        DO UPDATE SET perm_level = $3
        RETURNING perm_level
        "#,
        guild_id,
        user_id,
        perm_level
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.perm_level.into())
}

#[derive(Debug)]
pub struct UserIdPermLevel {
    user_id: i64,
    perm_level: i16,
}

pub async fn get_all_users_with_perm(
    pool: &PgPool,
    guild_id: i64,
    perm_level: UserPerm,
) -> anyhow::Result<Vec<UserIdPermLevel>> {
    let perm_level: i16 = perm_level.into();

    let rec = sqlx::query_as!(
        UserIdPermLevel,
        r#"
        SELECT user_id, perm_level
        FROM perms
        WHERE guild_id = $1 AND perm_level = $2
        "#,
        guild_id,
        perm_level
    )
    .fetch_all(pool)
    .await?;

    Ok(rec)
}

pub async fn delete_user(pool: &PgPool, guild_id: i64, user_id: i64) -> anyhow::Result<i64> {
    let rec = sqlx::query!(
        r#"
        DELETE FROM perms
        WHERE user_id = $1 AND guild_id = $2
        RETURNING user_id"#,
        user_id,
        guild_id
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.user_id)
}

pub async fn delete_guild(pool: &PgPool, guild_id: i64) -> anyhow::Result<Option<i64>> {
    let rec = match sqlx::query!(
        r#"
        DELETE FROM perms
        WHERE guild_id = $1
        RETURNING guild_id"#,
        guild_id
    )
    .fetch_optional(pool)
    .await?
    {
        Some(row) => row,
        None => return Ok(None),
    };

    Ok(Some(rec.guild_id))
}
