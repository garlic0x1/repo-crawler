use anyhow::Result;
use gar_crawl::crawler::*;
use serde::Deserialize;
// use serde_json::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GithubRepo {
    user: String,
    repo: String,
    files: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct File {
    path: String,
    download_url: Option<String>,
}

impl GithubRepo {
    /// example url: http(s)://github.com/user/repo
    pub async fn from_url(url: &str, filetypes: Vec<&str>) -> Result<Self> {
        let split = url.split("/");
        let split_vec: Vec<&str> = split.collect();

        if split_vec.len() == 5 {
            let user = split_vec.get(3).unwrap().to_string();
            let repo = split_vec.get(4).unwrap().to_string();

            let mut result = Self {
                user,
                repo,
                files: HashMap::new(),
            };
            result.contents(filetypes).await?;
            return Ok(result);
        }

        anyhow::bail!("invalid url, try http(s)://github.com/user/repo");
    }

    pub async fn new(user: &str, repo: &str, filetypes: Vec<&str>) -> Result<Self> {
        let mut result = Self {
            user: user.into(),
            repo: repo.into(),
            files: HashMap::new(),
        };
        result.contents(filetypes).await?;
        return Ok(result);
    }

    async fn contents(&mut self, filetypes: Vec<&str>) -> Result<()> {
        let base_path = format!(
            "https://api.github.com/repos/{}/{}/contents",
            self.user, self.repo
        );

        Crawler::builder()
            .on_page_propagator(|args| {
                let mut links = vec![];
                let res: Result<Vec<File>, serde_json::Error> =
                    serde_json::from_str(&args.page.text);

                if let Ok(files) = res {
                    for file in files.iter() {
                        if let Some(dl_url) = &file.download_url {
                            // if downloadable file
                            if let Ok(url) = reqwest::Url::parse(&dl_url) {
                                links.push(url);
                            }
                        } else {
                            let link = format!(
                                "https://api.github.com/repos/{}/{}/contents/{}",
                                self.user, self.repo, file.path
                            );

                            if let Ok(url) = reqwest::Url::parse(&link) {
                                links.push(url);
                            }
                        }
                    }
                }
                links
            })
            .on_page(|args| {
                for filetype in filetypes.iter() {
                    if args.page.url.to_string().ends_with(filetype) {
                        self.files
                            .insert(args.page.url.to_string(), args.page.text.clone());
                    }
                }
            })
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.79 Safari/537.36".into())
            .depth(100)
            .build()?
            .crawl(&base_path)
            .await?;

        Ok(())
    }
}
//diff
