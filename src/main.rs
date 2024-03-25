use std::env;
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use std::thread;

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
    let args: Vec<String> = env::args().collect();
    let target: Vec<u8> = args
        .get(1)
        .map(|s| {
            s.chars()
                .map(|c| {
                    u8::from_str_radix(&c.to_string(), 16)
                        .expect(&format!("Wrong input {} in {}", c, s))
                })
                .collect()
        })
        .unwrap_or(vec![0; 7]);

    let repo = Repository::open(".").unwrap();
    let head_commit = repo.head().unwrap().peel_to_commit().unwrap();

    let mut buffer = String::new();
    show_commit(&head_commit, &mut buffer).unwrap();

    let answer = Arc::new(Mutex::new(None));
    let t_num = num_cpus::get().min(16);

    let mut handles = vec![];
    for tid in 0..t_num {
        let answer = answer.clone();
        let orig = buffer.clone();
        let target = target.clone();
        let h = thread::spawn(move || {
            let [part1, part2] = orig
                .split("committer ")
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            for i in 0..usize::MAX {
                if answer.lock().unwrap().is_some() {
                    break;
                }

                let name_pre = format!("{}{:x}_", (tid as u8 + 65) as char, i);
                // println!("{}", name_pre);

                let full = format!(
                    "commit {}\0{}committer {}{}",
                    orig.len() + name_pre.len(),
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
                    .zip(&target)
                    .all(|(a, &b)| a == b)
                {
                    *answer.lock().unwrap() = Some(name_pre);
                }
            }
        });
        handles.push(h);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let name_pre = Arc::try_unwrap(answer)
        .unwrap()
        .into_inner()
        .unwrap()
        .expect("Can't found");
    let committer = head_commit.committer();
    let new_name = format!("{}{}", name_pre, committer.name().unwrap());
    let new_committer =
        Signature::new(&new_name, committer.email().unwrap(), &committer.when()).unwrap();
    let oid = head_commit
        .amend(Some("HEAD"), None, Some(&new_committer), None, None, None)
        .unwrap();
    println!("Done, new commit hash: {}", oid);
}
