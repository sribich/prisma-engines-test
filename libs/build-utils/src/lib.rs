/// Sets the current git commit hash as an environment variable in rustc 
/// available for use during compilation.
/// 
/// It is stored in the `GIT_HASH` environment variable. If it is already
/// set when this function is called, it will do nothing.
pub fn store_git_commit_hash_in_env() -> String {
    if let Ok(hash) = std::env::var("GIT_HASH") {
        return hash;
    }

    let repo = gix::discover(".").unwrap();
    let hash = repo.rev_parse_single("HEAD").unwrap().to_string();

    println!("cargo:rustc-env=GIT_HASH={hash}");

    hash
}
