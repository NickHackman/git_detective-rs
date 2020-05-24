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
use git::GitReference;
pub use git::{Branch, Commit, FileStatus, Signature, Tag};
use git2::{Repository, StatusOptions, StatusShow};
pub use git2::{RepositoryState, Status};

pub(crate) mod error;
pub use error::Error;

pub(crate) mod stats;
pub use stats::Stats;

pub(crate) mod project_stats;
pub use project_stats::ProjectStats;

pub(crate) mod diff_stats;
pub use diff_stats::DiffStats;

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
    repository: Repository,
    excluded_files: HashSet<String>,
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
        Ok(Self {
            repository: Repository::discover(path)?,
            excluded_files: HashSet::new(),
        })
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
        let repository = if recursive {
            Repository::clone_recurse(url.as_ref(), path)?
        } else {
            Repository::clone(valid_url.as_ref(), path)?
        };

        Ok(Self {
            repository,
            excluded_files: HashSet::new(),
        })
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
        let mut rev_walk = self.repository.revwalk()?;
        rev_walk.push_head()?;
        Ok(rev_walk
            .flatten()
            .filter_map(|id| self.repository.find_commit(id).ok())
            .fold(HashSet::new(), |mut set, commit| {
                if let Some(name) = commit.author().name() {
                    set.insert(name.to_string());
                }
                set
            }))
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
        let names = self.repository.tag_names(None)?;
        Ok(names
            .iter()
            .filter_map(|name| name)
            .filter_map(move |name| match self.repository.revparse_single(name) {
                Ok(obj) => obj.into_tag().map(Tag::from).ok(),
                Err(_) => None,
            })
            .collect())
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
        Ok(self
            .repository
            .branches(None)?
            .flatten()
            .map(|(branch, _)| Branch::from(branch)))
    }

    /// All commits for Repository that are parents of `HEAD` in **reverse** order
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
        let mut rev_walk = self.repository.revwalk()?;
        rev_walk.push_head()?;
        Ok(rev_walk
            .flatten()
            .filter_map(move |id| self.repository.find_commit(id).map(Commit::from).ok()))
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
        self.repository.state()
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
        let state = self.state();
        if state != RepositoryState::Clean {
            return Err(Error::UncleanState(state));
        }
        let oid = git_ref.id();
        self.repository
            .checkout_tree(&git_ref.into_object()?, None)?;
        self.repository.set_head_detached(oid)?;
        Ok(())
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
        let mut base_options = StatusOptions::new();
        let options = base_options
            .show(StatusShow::IndexAndWorkdir)
            .include_unmodified(true);
        Ok(self
            .repository
            .statuses(Some(options))?
            .iter()
            .map(FileStatus::from)
            .filter(|file_stat| !self.excluded_files.contains(&file_stat.path))
            .collect())
    }

    /// Get workdir
    fn workdir(&self) -> PathBuf {
        // Safe to unwrap because we don't allow bare repositories
        self.repository.workdir().unwrap().into()
    }

    /// Get the blame for a file
    fn blame_file<P: AsRef<Path>>(&self, path: P) -> Result<git2::Blame<'_>, Error> {
        Ok(self.repository.blame_file(&path.as_ref(), None)?)
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
        let files = self.ls()?;
        let workdir = self.workdir();
        let repo = std::sync::Mutex::new(self);
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
        let blame = self.blame_file(path)?;
        let workdir = self.workdir();
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
    /// let mut gd = GitDetective::open(".")?;
    /// let before_files = gd.ls()?;
    ///
    /// gd.exclude_file("README.md");
    /// gd.exclude_file("Cargo.toml");
    ///
    /// let mut after_files = gd.ls()?;
    /// assert!(after_files.iter().all(|file| &file.path != "README.md" && &file.path != "Cargo.toml"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn exclude_file<S: Into<String>>(&mut self, file: S) {
        self.excluded_files.insert(file.into());
    }

    /// Get insertion/deletion statistics
    ///
    /// The same `+` and `-` deltas that Github shows in the [contributors](https://github.com/NickHackman/Git-Detective/graphs/contributors) page
    ///
    /// # Example
    ///
    /// ```
    /// # use git_detective::Error;
    /// use git_detective::GitDetective;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let mut gd = GitDetective::open(".")?;
    /// let diff_stats = gd.diff_stats()?;
    /// for (author, diff_stat) in diff_stats {
    ///   println!("{}: +{} -{}", author, diff_stat.insertions, diff_stat.deletions);
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// - Unable to walk commits
    /// - Unable to get [`git2::Tree`](https://docs.rs/git2/latest/git2/struct.Tree.html) for a [`git2::Commit`](https://docs.rs/git2/latest/git2/struct.Commit.html)
    /// - Unable to get the stats for a [`git2::Diff`](https://docs.rs/git2/latest/git2/struct.Diff.html)
    pub fn diff_stats(&self) -> Result<HashMap<String, DiffStats>, Error> {
        let mut rev_walk = self.repository.revwalk()?;
        rev_walk.push_head()?;
        Ok(rev_walk
            .flatten()
            .filter_map(|id| self.repository.find_commit(id).ok())
            .try_fold(HashMap::new(), |mut contribs, commit| -> Result<_, Error> {
                let old_tree = commit
                    .parent(0)
                    .map_or(None, |parent| parent.tree().map_or(None, |tree| Some(tree)));
                let new_tree = commit.tree()?;
                let diff =
                    self.repository
                        .diff_tree_to_tree(old_tree.as_ref(), Some(&new_tree), None)?;
                if let Some(author) = commit.author().name() {
                    let author = author.into();
                    let entry = contribs.entry(author).or_insert_with(DiffStats::default);
                    *entry += diff.stats()?;
                }
                Ok(contribs)
            })?)
    }

    /// Get files contributed to by all Contributors in commits that are parents of `HEAD`
    ///
    /// All file paths are given relatively to the Git working directory
    ///
    /// # Example
    ///
    /// ```
    /// # use git_detective::Error;
    /// use git_detective::GitDetective;
    ///
    /// # fn main() -> Result<(), Error> {
    /// let mut gd = GitDetective::open(".")?;
    /// let contrib_files = gd.files_contributed_to()?;
    /// for (author, files) in contrib_files {
    ///   println!("{}: {:?}", author, files);
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// - Unable to walk commits
    /// - Unable to get [`git2::Tree`](https://docs.rs/git2/latest/git2/struct.Tree.html) for a [`git2::Commit`](https://docs.rs/git2/latest/git2/struct.Commit.html)
    /// - Unable to get the stats for a [`git2::Diff`](https://docs.rs/git2/latest/git2/struct.Diff.html)
    pub fn files_contributed_to(&self) -> Result<HashMap<String, HashSet<PathBuf>>, Error> {
        let mut rev_walk = self.repository.revwalk()?;
        rev_walk.push_head()?;
        Ok(rev_walk
            .flatten()
            .filter_map(|id| self.repository.find_commit(id).ok())
            .try_fold(HashMap::new(), |mut contribs, commit| -> Result<_, Error> {
                let old_tree = commit
                    .parent(0)
                    .map_or(None, |parent| parent.tree().map_or(None, Some));
                let new_tree = commit.tree()?;
                let diff =
                    self.repository
                        .diff_tree_to_tree(old_tree.as_ref(), Some(&new_tree), None)?;

                if let Some(author) = commit.author().name() {
                    let author = author.into();
                    let files = diff.deltas().fold(HashSet::new(), |mut files, delta| {
                        if let Some(path) = delta.new_file().path() {
                            files.insert(path.to_path_buf());
                        }
                        files
                    });
                    let prev_files = contribs.entry(author).or_insert_with(HashSet::default);
                    *prev_files = files.union(prev_files).cloned().collect();
                }
                Ok(contribs)
            })?)
    }
}
