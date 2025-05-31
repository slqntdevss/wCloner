use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use regex::Regex;
use tokio::time::Instant;

use crate::debug;
use crate::log;
use crate::utils::requests;

fn sanitize_path<P: AsRef<std::path::Path>>(path: P) -> PathBuf {
    let mut sanitized = PathBuf::new();

    for component in path.as_ref().components() {
        match component {
            Component::Normal(os_str) => {
                let segment = os_str.to_string_lossy();
                let cleaned: String = segment
                    .chars()
                    .map(|c| match c {
                        ':' | '*' | '?' | '"' | '<' | '>' | '|' | '#' | '%' | '&' => '_',
                        _ => c,
                    })
                    .collect();
                sanitized.push(cleaned);
            }
            _ => continue,
        }
    }

    sanitized
}

pub async fn clone(base_url: &str, path: &str) -> Result<bool, Error> {
    let base_html_text = requests::get(base_url.to_owned() + path).await.unwrap();
    let dir_name = sanitize_path(base_url.to_owned() + path);

    log!("{}", format!("Cloning {base_url} to cloned_site directory."));

    println!("{}", dir_name.to_str().unwrap());

    let dir = fs::create_dir_all(dir_name.to_str().unwrap())?;
    debug!("cloned_site directory made, continuing with the clone.");

    let mut file = File::create(Path::new(dir_name.to_str().unwrap()).join("index.html"))?;

    file.write_all(base_html_text.as_ref()).expect("Failed to write to HTML base file.");
    file.write("<!--wCloner by slqntdevss - https://github.com/slqntdevss/wCloner-->".as_ref()).expect("Failed to add tag.");

    debug!("index.html file created, starting crawling to find CSS and image files.");
    let file_path = File::open(Path::new(dir_name.to_str().unwrap()).join("index.html"))?;
    let reader = BufReader::new(file_path);

    for line in reader.lines() {
        let line_res = line?;
        if line_res.clone().contains("<link") && (line_res.clone().contains("rel=\"stylesheet\"") || line_res.clone().contains("rel='stylesheet'")) && line_res.clone().contains("stylesheet") {
            let css = line_res.split("href=").collect::<Vec<_>>()[1];
            let re = Regex::new(r#"["']([^"']+\.css[^"']*)["']"#).unwrap();

            if css.starts_with("'") || css.starts_with("\"") {
                let start_time = Instant::now();
                for cap in re.captures_iter(css) {
                    let href = &cap[1];
                    debug!("Found CSS href: {}", href);

                    if href.ends_with(".css") {
                        let full_css_url = if href.starts_with('/') {
                            format!("{base_url}{href}")
                        } else if href.starts_with("http") {
                            href.to_string()
                        } else {
                            continue;
                        };

                        let parts: Vec<PathBuf> = Path::new(href)
                            .components()
                            .map(|c| sanitize_path(c.as_os_str().to_string_lossy().to_string()))
                            .collect();

                        let sanitized_path = parts.iter().collect::<PathBuf>();
                        let target_path = Path::new(&dir_name).join(&sanitized_path);

                        if let Some(parent) = target_path.parent() {
                            fs::create_dir_all(parent)?;
                        }
                        let css_res = requests::get(full_css_url.clone()).await.unwrap();
                        let mut css_file = File::create(&target_path)?;
                        css_file.write_all(css_res.as_ref())?;

                        log!(
                "{}",
                format!(
                    "Successfully wrote to {} in {:5}s",
                    sanitized_path.display(),
                    start_time.elapsed().as_secs_f32()
                )
            );
                    }
                }
            }
        } else if line_res.clone().contains("<link") && (line_res.clone().contains("rel=\"icon\"") || line_res.clone().contains("rel='icon'")) && line_res.clone().contains("icon") {
            let favicon = line_res.split("href=").collect::<Vec<_>>()[1];
            let re = Regex::new(r#"["']([^"']+\.(png|jpg|jpeg|gif|bmp|svg)[^"']*)["']"#).unwrap();
            println!("{}", favicon);
            if favicon.starts_with("'") || favicon.starts_with("\"") || favicon.starts_with("\"") {
                let start_time = Instant::now();
                for cap in re.captures_iter(favicon) {
                    let href = &cap[1];
                    debug!("Found favicon href: {}", href);
                    if href.ends_with(".ico") || href.ends_with(".png") || href.ends_with(".jpg") || href.ends_with(".jpeg") {
                        let full_favi_url = if href.starts_with('/') {
                            format!("{base_url}{href}")
                        } else if href.starts_with("http") {
                            href.to_string()
                        } else {
                            continue;
                        };

                        let parts: Vec<PathBuf> = Path::new(href)
                            .components()
                            .map(|c| sanitize_path(c.as_os_str().to_string_lossy().to_string()))
                            .collect();

                        let sanitized_path = parts.iter().collect::<PathBuf>();
                        let target_path = Path::new(&dir_name).join(&sanitized_path);

                        let faviGet = requests::get_with_no_text(full_favi_url).await.unwrap();
                        let mut favi = File::create(&target_path)?;
                        let bytes = faviGet.bytes().await.unwrap();
                        let bytes_vec: Vec<u8> = bytes.into_iter().collect();
                        favi.write_all(&bytes_vec)?;
                        log!(
                "{}",
                format!(
                    "Successfully wrote to {} in {:5}s",
                    sanitized_path.display(),
                    start_time.elapsed().as_secs_f32()
                ))
                    }
                }
            }
        } else if line_res.clone().contains("<img") && (line_res.clone().contains("src=")) && line_res.clone().contains("src") {
            let img = line_res.split("src=").collect::<Vec<_>>()[1];
            let re = Regex::new(r#"["']([^"']+\.(png|jpg|jpeg|gif|bmp|svg)[^"']*)["']"#).unwrap();
            println!("{}", img);
            if img.starts_with("'") || img.starts_with("\"") || img.starts_with("\"") {
                let start_time = Instant::now();
                for cap in re.captures_iter(img) {
                    let src = &cap[1];
                    debug!("Found img src: {}", src);
                    if src.ends_with(".svg") || src.ends_with(".png") || src.ends_with(".jpg") || src.ends_with(".jpeg") {
                        let full_img_url = if src.starts_with('/') {
                            format!("{base_url}{src}")
                        } else if src.starts_with("http") {
                            src.to_string()
                        } else {
                            continue;
                        };

                        let parts: Vec<PathBuf> = Path::new(src)
                            .components()
                            .map(|c| sanitize_path(c.as_os_str().to_string_lossy().to_string()))
                            .collect();

                        let sanitized_path = parts.iter().collect::<PathBuf>();
                        let target_path = Path::new(&dir_name).join(&sanitized_path);
                        log!("rahh");
                        let imgGet = requests::get_with_no_text(full_img_url).await.unwrap();
                        let mut img = File::create(&target_path)?;
                        log!("rahh22222222");
                        let bytes = imgGet.bytes().await.unwrap();
                        let bytes_vec: Vec<u8> = bytes.into_iter().collect();
                        img.write_all(&bytes_vec)?;
                        log!(
                "{}",
                format!(
                    "Successfully wrote to {} in {:5}s",
                    sanitized_path.display(),
                    start_time.elapsed().as_secs_f32()
                ))
                    }
                }
            }
        }
    }

    Ok(true)
}