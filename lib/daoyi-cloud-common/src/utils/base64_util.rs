pub async fn encode_password(password: &str) -> anyhow::Result<String> {
    Ok(bcrypt::hash(password, bcrypt::DEFAULT_COST)?)
}

pub async fn check_password(password: &str, hashed_password: &str) -> anyhow::Result<bool> {
    Ok(bcrypt::verify(password, hashed_password)?)
}
