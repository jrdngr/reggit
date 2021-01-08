use git2::{Cred, Config, RemoteCallbacks};

use std::path::Path;

fn main() {
    let config = Config::open_default().unwrap();

    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        // Cred::credential_helper(
        //    &config,
        //    url,
        //    None,
        //)
        Cred::ssh_key_from_agent("picklenerd")
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    builder.clone(
        "git@github.com:picklenerd/reggit.git",
        Path::new("./reggit"),
    ).unwrap();
}
