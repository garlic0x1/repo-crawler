pub mod github;
pub mod svn;

#[cfg(test)]
mod tests {
    use super::github::*;
    use super::svn::*;

    // #[tokio::test]
    // async fn github() {
    //     let mut repo = GithubRepo::new("garlic0x1", "gar-crawl").await.unwrap();
    //     match repo.contents(vec![".rs, .md"]).await {
    //         Err(err) => {
    //             println!("{:?}", err);
    //             panic!();
    //         }
    //         _ => (),
    //     }
    //     println!("{:?}", repo);
    //     panic!();
    // }

    #[tokio::test]
    async fn svn() {
        let repo_url = "http://plugins.svn.wordpress.org/qiwi-button/trunk/";
        let repo = SvnRepo::new(repo_url, vec![".php"]).await.unwrap();
        println!("{:?}", repo);
        panic!();
    }
}
