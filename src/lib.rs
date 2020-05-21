//! A Library to aide in investigating the work done in a Git Repository
//!
//! # Example
//!
//! ```
//! # use git_detective::Error;
//! use git_detective::GitDetective;
//!
//! # fn main() -> Result<(), Error> {
//! let mut gd = GitDetective::open(".")?;
//!
//! let project_stats = gd.final_contributions()?;
//!
//! let nh_contributions = match project_stats.contribs_by_name("Nick Hackman") {
//!   Some(contribs) => contribs,
//!   None => panic!("Nick Hackman didn't contribute to this repository"),
//! };
//!
//! println!("Nick Hackman's Contributions = {:?}", nh_contributions);
//! # Ok(())
//! # }
//! ```
//!
#![deny(
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms,
    unused_must_use
)]

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use tokei::{Config, LanguageType};
use url::Url;

pub(crate) mod git;
pub use git::{Branch, Commit, FileStatus, Signature, Tag};
use git::{GitReference, Repo};
pub use git2::{RepositoryState, Status};

pub(crate) mod error;
pub use error::Error;

pub(crate) mod stats;
pub use stats::Stats;

pub(crate) mod project_stats;
pub use project_stats::ProjectStats;

/// Enables more in-depth investigating of Git Repositories
///
/// # Examples
/// Open a local repository, uses `git discover` which traverses
/// up all directories until it finds a git repository or root.
/// ```
/// use git_detective::GitDetective;
///
/// let repo = GitDetective::open(".");
/// assert!(repo.is_ok());
/// ```
///
/// Clone a Git repository, providing a path to clone it to and whether
/// or not to clone recusrively
///
/// ```
/// # use std::fs::remove_dir_all;
/// use git_detective::GitDetective;
///
/// let path = "toml-rs";
///
/// let repo = GitDetective::clone("https://github.com/alexcrichton/toml-rs", path, true);
/// assert!(repo.is_ok());
/// # remove_dir_all(path);
/// ```
pub struct GitDetective {
    repo: Repo,
}

impl GitDetective {
    /// Open a Local Git Repository
    ///
    /// # Example
    ///
    /// ```
    /// use git_detective::GitDetective;
    ///
    /// let repo = GitDetective::open(".");
    /// assert!(repo.is_ok());
    /// ```
    ///
    /// # Errors
    /// - Couldn't find a Git Repository
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let repo = Repo::open(path)?;
        Ok(Self { repo })
    }

    /// Clone a remote Git Repository
    ///
    /// # Example
    ///
    /// ```
    /// # use std::fs::remove_dir_all;
    /// use git_detective::GitDetective;
    ///
    /// let path = "git_detective_cloned";
    ///
    /// let repo = GitDetective::clone("https://github.com/NickHackman/Git-Detective.git", path, true);
    /// assert!(repo.is_ok());
    /// # remove_dir_all(path);
    /// ```
    ///
    /// # Errors
    /// - URL isn't valid
    /// - Path provided isn't writable
    /// - URL isn't a Git Repository
    pub fn clone<S: AsRef<str>, P: AsRef<Path>>(
        url: S,
        path: P,
        recursive: bool,
    ) -> Result<Self, Error> {
        let valid_url = Url::parse(url.as_ref())?;
        let repo = Repo::clone(valid_url.as_str(), path.as_ref(), recursive)?;
        Ok(Self { repo })
    }

    /// `HashSet` of all contributors of Repository
    ///
    /// # Example
    ///
    /// ```
    /// # use git_detective::Error;
    /// use git_detective::GitDetective;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let repo = GitDetective::open(".")?;
    /// let contributors = repo.contributors()?;
    ///
    /// assert!(contributors.contains("Nick Hackman"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn contributors(&self) -> Result<HashSet<String>, Error> {
        self.repo.contributors()
    }

    /// All tags of Repository
    ///
    /// # Example
    ///
    /// ```
    /// # use git_detective::Error;
    /// use git_detective::GitDetective;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let repo = GitDetective::open(".")?;
    /// let tags = repo.tags()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tags(&self) -> Result<Vec<Tag<'_>>, Error> {
        self.repo.tags(None)
    }

    /// All branches for Repository
    ///
    /// # Example
    ///
    /// ```
    /// # use git_detective::Error;
    /// use git_detective::GitDetective;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let repo = GitDetective::open(".")?;
    /// let mut branches = repo.branches()?;
    ///
    /// for branch in branches {
    ///   println!("{}", branch.name()?);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn branches(&self) -> Result<impl Iterator<Item = Branch<'_>>, Error> {
        self.repo.branches(None)
    }

    /// All commits for Repository
    ///
    /// # Example
    ///
    /// ```
    /// # use git_detective::Error;
    /// use git_detective::GitDetective;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let repo = GitDetective::open(".")?;
    /// let commits = repo.commits()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn commits(&self) -> Result<impl Iterator<Item = Commit<'_>>, Error> {
        self.repo.commits()
    }

    /// Current state of Repository
    ///
    /// # Example
    ///
    /// ```
    /// # use git_detective::Error;
    /// use git_detective::GitDetective;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let repo = GitDetective::open(".")?;
    /// let state = repo.state();
    ///
    /// println!("{:?}", state);
    /// # Ok(())
    /// # }
    /// ```
    pub fn state(&self) -> RepositoryState {
        self.repo.state()
    }

    /// Checkout a [`Tag`](struct.Tag.html), [`Branch`](struct.Branch.html), or [`Commit`](struct.Commit.html)
    ///
    /// # Caution
    /// This detaches the `HEAD` of the repository,
    /// and the repository should **NOT** be edited unless `HEAD` is reattached.
    ///
    /// # Errors
    /// - [`RepositoryState`]() is not `RepositoryState::Clean`
    /// - `GitReference` doesn't exist in the repository
    pub fn checkout<'repo, GitRef: GitReference<'repo>>(
        &self,
        git_ref: GitRef,
    ) -> Result<(), Error> {
        self.repo.checkout(git_ref)
    }

    /// List files in the Index
    ///
    /// # Example
    ///
    /// ```
    /// # use git_detective::Error;
    /// use git_detective::GitDetective;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let gd = GitDetective::open(".")?;
    /// let files = gd.ls()?;
    ///
    /// for file in files {
    ///   println!("{}", file.path);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// - Couldn't read Git Repository
    pub fn ls(&self) -> Result<Vec<FileStatus>, Error> {
        self.repo.ls()
    }

    /// Count the final contibutions for an entire git repository
    ///
    /// Final contributions takes the last commit, and completely
    /// ignores current untracked changes in the git repository.
    ///
    /// # Example
    ///
    /// ```
    /// # use git_detective::Error;
    /// use git_detective::GitDetective;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let mut gd = GitDetective::open(".")?;
    /// let project_stats = gd.final_contributions()?;
    ///
    /// for contributor in project_stats.contributors() {
    ///   println!("{}", contributor);
    /// }
    ///
    /// println!("{}", project_stats.total_lines());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # All of my `code` is "Plain Text"
    ///
    /// `Git-Detective` fallsback to interpreting files as "Plain Text" when no file type can be determined.
    /// This is common for files like `Cargo.lock` and `LICENSE`.
    ///
    /// # Errors
    /// - Failed to read file [`IOError`](enum.Error.html#variant.IOError)
    /// - Failed to git blame [`GitError`](enum.Error.html#variant.GitError)
    pub fn final_contributions(&mut self) -> Result<ProjectStats, Error> {
        let files = self.repo.ls()?;
        let workdir = self.repo.workdir();
        let repo = std::sync::Mutex::new(&mut self.repo);
        Ok(files
            .par_iter()
            .filter_map(|file| {
                if let Ok(repo) = repo.lock() {
                    if let Ok(blame) = repo.blame_file(&file.path) {
                        return GitDetective::_final_contributions_file(
                            &workdir, &file.path, blame,
                        )
                        .map(ProjectStats::from)
                        .ok();
                    }
                }
                None
            })
            .reduce(ProjectStats::default, |mut stats_lhs, stats_rhs| {
                stats_lhs += stats_rhs;
                stats_lhs
            }))
    }

    /// Count the final contibutions for a file
    ///
    /// Final contributions takes the last commit, and completely
    /// ignores current untracked changes in the git repository.
    ///
    /// # Example
    ///
    /// ```
    /// # use git_detective::Error;
    /// use git_detective::GitDetective;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let gd = GitDetective::open(".")?;
    /// let (lang, final_contribs) = gd.final_contributions_file(file!())?;
    ///
    /// println!("Language = {}", lang);
    /// println!("final contributions = {:?}", final_contribs);
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # All of my `code` is "Plain Text"
    ///
    /// `Git-Detective` fallsback to interpreting files as "Plain Text" when no file type can be determined.
    /// This is common for files like `Cargo.lock` and `LICENSE`.
    ///
    /// # Errors
    /// - Failed to read file [`IOError`](enum.Error.html#variant.IOError)
    /// - Failed to git blame [`GitError`](enum.Error.html#variant.GitError)
    pub fn final_contributions_file<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<(&'static str, HashMap<String, Stats>), Error> {
        let path = path.as_ref();
        let blame = self.repo.blame_file(path)?;
        let workdir = self.repo.workdir();
        GitDetective::_final_contributions_file(&workdir, path, blame)
    }

    /// Internal Function
    ///
    /// Performs final contributions counting for a file
    fn _final_contributions_file<Dir: Into<PathBuf>, P: AsRef<Path>>(
        workdir: Dir,
        path: P,
        blame: git2::Blame<'_>,
    ) -> Result<(&'static str, HashMap<String, Stats>), Error> {
        let workdir = workdir.into();
        let path = path.as_ref();
        let full_path = workdir.join(path);
        let config = Config::default();

        let lang_type = LanguageType::from_path(&full_path, &config).unwrap_or(LanguageType::Text);
        let annotations = lang_type
            .annotate_file(full_path, &config)
            .map_err(|(err, path)| Error::IOError(err, path))?;

        let contributions = blame
            .iter()
            .fold(HashMap::new(), |mut contributions, hunk| {
                let final_sig = hunk.final_signature();
                let final_author = match final_sig.name() {
                    Some(name) => name.to_string(),
                    // TODO: Log Non-UTF8 name, instead of silently ignoring
                    None => return contributions,
                };

                let end = hunk.final_start_line() + hunk.lines_in_hunk();
                for line_num in hunk.final_start_line()..end {
                    let line_type = match annotations.get(&line_num) {
                        Some(line_type) => line_type,
                        None => continue,
                    };
                    let stats = contributions
                        .entry(final_author.clone())
                        .or_insert_with(Stats::default);
                    *stats += line_type;
                }
                contributions
            });

        Ok((lang_type.name(), contributions))
    }

    /// Exclude a file from all further [`ls`](struct.GitDetective.html#method.ls) and
    /// [`final_contributions`](struct.GitDetective.html#method.final_contributions)
    ///
    /// # Example
    ///
    /// ```
    /// # use git_detective::Error;
    /// use git_detective::GitDetective;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let mut repo = GitDetective::open(".")?;
    /// let before_files = repo.ls()?;
    ///
    /// repo.exclude_file("README.md");
    /// repo.exclude_file("Cargo.toml");
    ///
    /// let mut after_files = repo.ls()?;
    /// assert!(after_files.iter().all(|file| &file.path != "README.md" && &file.path != "Cargo.toml"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn exclude_file<S: Into<String>>(&mut self, file: S) {
        self.repo.exclude_file(file);
    }
}
