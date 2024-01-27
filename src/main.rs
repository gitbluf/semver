mod models;

use git2::Repository;

use models::semver::SemVer;


fn main() {
    let repo = Repository::open(".")
        .expect("Failed to open repository");

    // Get the HEAD commit
    let head = repo.head()
        .expect("Failed to get HEAD");

    // Ensure the HEAD is pointing to a commit
    assert!(head.is_branch());

    // Get the commit pointed by the HEAD
    let commit = head.peel_to_commit()
        .expect("Failed to peel HEAD to commit");

    // Get all tags in the repository
    let tags = repo.tag_names(None).unwrap();

    // Sort the tags by creation date and take the latest one
    let latest_tag = tags.iter().max().unwrap();
    // Open the repository

    // Parse the latest tag into a SemVer struct
    let mut semver = SemVer::from_tag(&latest_tag.unwrap());

    // Update the semver based on the commit message
    semver.update_based_on_commit_msg(&commit.message().unwrap());

    // Print the new semver
    println!("New semver: {} \n", semver);

    // Print the commit details
    println!("Commit ID: {}", commit.id());
    println!("Author: {}", commit.author().name().unwrap());
    println!("Message: {}", commit.message().unwrap());
}
