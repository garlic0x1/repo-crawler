use anyhow::Result;
use gar_crawl::crawler::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SvnRepo {
    repo_url: String,
    files: HashMap<String, String>,
}

impl SvnRepo {
    pub async fn from_url(url: &str, filetypes: Vec<&str>) -> Result<Self> {
        // filenames and contents
        let mut result = Self {
            repo_url: url.into(),
            files: HashMap::new(),
        };

        Crawler::builder()
        .add_default_propagators()
        .whitelist(url)
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.79 Safari/537.36".into())
        .on_page(|args| {
                for filetype in filetypes.iter() {
                    if args.page.url.to_string().ends_with(filetype) {
                        result.files
                            .insert(args.page.url.to_string(), args.page.text.clone());
                    }
                }
        })
        .depth(1)
        .build()?
        .crawl(url)
        .await?;

        Ok(result)
    }
}
