use std::fs;
use crate::utils::requests;
use crate::utils::errors::CloneFailure;

pub async fn clone(url: &str) -> Result<bool, Err(CloneFailure)> {
    let base_html_text = requests::get(url).await?;

    log!

    let dir = fs::create_dir("cloned_site")?;

    Ok(true)
}