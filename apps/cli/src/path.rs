use std::path::{Component, Path, PathBuf};


/// Construct a relative path from a provided base directory path to the provided path.
///
/// ```rust
/// use pathdiff::diff_paths;
/// use std::path::*;
///
/// assert_eq!(diff_paths("/foo/bar",      "/foo/bar/baz"),  Some("../".into()));
/// assert_eq!(diff_paths("/foo/bar/baz",  "/foo/bar"),      Some("baz".into()));
/// assert_eq!(diff_paths("/foo/bar/quux", "/foo/bar/baz"),  Some("../quux".into()));
/// assert_eq!(diff_paths("/foo/bar/baz",  "/foo/bar/quux"), Some("../baz".into()));
/// assert_eq!(diff_paths("/foo/bar",      "/foo/bar/quux"), Some("../".into()));
///
/// assert_eq!(diff_paths("/foo/bar",      "baz"),           Some("/foo/bar".into()));
/// assert_eq!(diff_paths("/foo/bar",      "/baz"),          Some("../foo/bar".into()));
/// assert_eq!(diff_paths("foo",           "bar"),           Some("../foo".into()));
///
/// assert_eq!(
///     diff_paths(&"/foo/bar/baz", "/foo/bar".to_string()),
///     Some("baz".into())
/// );
/// assert_eq!(
///     diff_paths(Path::new("/foo/bar/baz"), Path::new("/foo/bar").to_path_buf()),
///     Some("baz".into())
/// );
/// ```
pub fn diff_paths(path: &Path, base: &Path) -> Option<PathBuf> {
    if path.is_absolute() != base.is_absolute() {
        if path.is_absolute() {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else {
        let mut ita = path.components();
        let mut itb = base.components();

        let mut comps: Vec<Component> = vec![];

        // ./foo and foo are the same
        if let Some(Component::CurDir) = ita.clone().next() {
            ita.next();
        }
        
        if let Some(Component::CurDir) = itb.clone().next() {
            itb.next();
        }

        loop {
            match (ita.next(), itb.next()) {
                (None, None) => break,
                (Some(a), None) => {
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
                (None, _) => comps.push(Component::ParentDir),
                (Some(a), Some(b)) if comps.is_empty() && a == b => (),
                (Some(a), Some(b)) if b == Component::CurDir => comps.push(a),
                (Some(_), Some(b)) if b == Component::ParentDir => return None,
                (Some(a), Some(_)) => {
                    comps.push(Component::ParentDir);
                    for _ in itb {
                        comps.push(Component::ParentDir);
                    }
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
            }
        }
        Some(comps.iter().map(|c| c.as_os_str()).collect())
    }
}
