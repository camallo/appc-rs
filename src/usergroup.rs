use errors;

use std::{fs, io, path};
use nix::sys::stat;
use parsswd;

use std::io::BufRead;

/// Resolve the freeform `user` string for an application rooted at `rootfs`.
///
/// On success, it returns the corresponding numeric user ID (uid).
pub fn resolve_uid(user: &str, rootfs: &path::Path) -> errors::Result<u32> {
    let passwd = path::PathBuf::from(rootfs).join("etc").join("passwd");
    if let Ok(pfile) = fs::File::open(passwd) {
        for line in io::BufReader::new(pfile).lines() {
            let l = try!(line);
            if let Some(entry) = parsswd::PwEnt::from_str(&l) {
                if entry.name == user {
                    return Ok(entry.uid);
                }
            }
        }
    }

    if let Ok(uid) = user.parse::<u32>() {
        return Ok(uid);
    }

    let tf = path::PathBuf::from(user);
    if tf.is_absolute() {
        let rel_path = try!(tf.strip_prefix("/"));
        let abs_path = path::PathBuf::from(rootfs).join(rel_path);
        let filestat = try!(stat::lstat(&abs_path));
        return Ok(filestat.st_uid);
    }

    bail!("unable to determine uid");
}

/// Resolve the freeform `group` string for an application rooted at `rootfs`.
///
/// On success, it returns the corresponding numeric group ID (gid).
pub fn resolve_gid(group: &str, rootfs: &path::Path) -> errors::Result<u32> {
    let groups = path::PathBuf::from(rootfs).join("etc").join("group");
    if let Ok(gfile) = fs::File::open(groups) {
        for line in io::BufReader::new(gfile).lines() {
            let l = try!(line);
            if let Some(entry) = parsswd::GrpEnt::from_str(&l) {
                if entry.name == group {
                    return Ok(entry.gid);
                }
            }
        }
    }

    if let Ok(gid) = group.parse::<u32>() {
        return Ok(gid);
    }

    let tf = path::PathBuf::from(group);
    if tf.is_absolute() {
        let rel_path = try!(tf.strip_prefix("/"));
        let abs_path = path::PathBuf::from(rootfs).join(rel_path);
        let filestat = try!(stat::lstat(&abs_path));
        return Ok(filestat.st_gid);
    }

    bail!("unable to determine gid");
}
