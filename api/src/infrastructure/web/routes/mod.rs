pub mod auth;
pub mod profile;

#[cfg(test)]
pub async fn extract_body_response<T>(body: axum::body::Body) -> Result<T, anyhow::Error>
where
    for<'a> T: serde::Deserialize<'a>,
{
    let bytes = axum::body::to_bytes(body, usize::MAX).await?;
    Ok(serde_json::from_slice::<T>(&bytes)?)
}
