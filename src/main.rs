use git2::{Cred, Config, RemoteCallbacks};

use std::path::Path;

fn main() {
  // Prepare callbacks.
  let mut callbacks = RemoteCallbacks::new();
  callbacks.credentials(|url, _username_from_url, _allowed_types| {
    Cred::credential_helper(
      &Config::open_default()?,
      url,
      None,
    )
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
