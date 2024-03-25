use std::fmt::Write;

use git2::{Commit, Repository, Signature};
use sha1::{Digest, Sha1};

fn show_sig(header: &str, sig: Signature, buffer: &mut String) -> std::fmt::Result {
    let offset = sig.when().offset_minutes();
    let (sign, offset) = if offset < 0 {
        ('-', -offset)
    } else {
        ('+', offset)
    };
    let (hours, minutes) = (offset / 60, offset % 60);
    write!(
        buffer,
        "{} {} {} {}{:02}{:02}\n",
        header,
        sig,
        sig.when().seconds(),
        sign,
        hours,
        minutes
    )
}

fn show_commit(commit: &Commit, buffer: &mut String) -> std::fmt::Result {
    write!(buffer, "tree {}\n", commit.tree_id())?;
    for parent in commit.parent_ids() {
        write!(buffer, "parent {}\n", parent)?;
    }
    show_sig("author", commit.author(), buffer)?;
    show_sig("committer", commit.committer(), buffer)?;
    if let Some(msg) = commit.message() {
        write!(buffer, "\n{}", msg)?;
    }
    Ok(())
}

fn main() {
    let repo = Repository::open(".").unwrap();
    let head_commit = repo.head().unwrap().peel_to_commit().unwrap();

    let mut buffer = String::new();
    show_commit(&head_commit, &mut buffer).unwrap();

    let [part1, part2] = buffer
        .split("committer ")
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    for i in 0..usize::MAX {
        let name_pre = format!("{:x}_", i);
        let full = format!(
            "commit {}\0{}committer {}{}",
            buffer.len() + name_pre.len(),
            part1,
            name_pre,
            part2
        );

        // println!("{}", full);

        let mut hasher = Sha1::new();
        hasher.update(full.as_bytes());
        let result = hasher.finalize();
        if result
            .into_iter()
            .map(|v| [v / 16, v % 16])
            .flatten()
            .take(7)
            .all(|v| v == 0)
        {
            let committer = head_commit.committer();
            let new_name = format!("{}{}", name_pre, committer.name().unwrap_or_default());
            let new_committer = Signature::new(
                &new_name,
                committer.email().unwrap_or_default(),
                &committer.when(),
            )
            .unwrap();
            let oid = head_commit
                .amend(Some("HEAD"), None, Some(&new_committer), None, None, None)
                .unwrap();
            println!("Done, new commit hash {}", oid);
            break;
        }
    }
}
