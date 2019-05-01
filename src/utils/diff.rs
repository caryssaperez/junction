extern crate git2;

use git2::Repository;

pub struct FileDiff {
    repo: Repository,
}

impl FileDiff {
    pub fn new(path: &str) -> FileDiff {
        let repo = match Repository::open(path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };

        FileDiff { repo }
    }

    pub fn get_diff(&self) {
        // return self.repo.stats().unwrap().files_changed();
    }
}
