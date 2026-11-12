use clap::Parser;
use futures::future::join_all;
use reqwest::Client;
use std::error::Error;
use std::path::PathBuf;
use tokio::fs;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    max_threads: Option<usize>,

    file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Args::parse();

    let num_threads = args.max_threads.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    });

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_threads)
        .enable_all()
        .build()?;

    runtime.block_on(async_main(args.file))
}


async fn async_main(file_path: PathBuf) -> Result<(), Box<dyn Error + Send + Sync>> {
    let content = fs::read_to_string(&file_path).await?;

    let client = Client::new();

    let tasks = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|url| {
            let client = client.clone();
            let url = url.to_string();
            tokio::spawn(async move { download_and_save(&client, &url).await })
        });

    let results = join_all(tasks).await;

    for res in results {
        match res {
            Ok(Ok(())) => {}
            Ok(Err(e)) => eprintln!("{}", e),
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}

async fn download_and_save(client: &Client, url: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let response = client
        .get(url)
        .send()
        .await?
        .error_for_status()?;

    let body = response.text().await?;

    let filename = sanitize_filename(url);
    fs::write(&filename, body).await?;

    println!("saved {} to {}", url, filename);
    Ok(())
}

fn sanitize_filename(url: &str) -> String {
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    let clean_url: String = without_scheme
        .trim_end_matches('/')
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect();

    if clean_url.ends_with(".html") {
        clean_url
    } else {
        format!("{}.html", clean_url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("https://example.com/page"), "example.com_page.html");
        assert_eq!(sanitize_filename("example.com/index.html"), "example.com_index.html");
    }
}
