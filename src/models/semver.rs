use std::fmt;
use regex::Regex;

#[derive(Debug)]
pub struct SemVer {
    major: i32,
    minor: i32,
    patch: i32,
}

impl SemVer {
    pub fn new() -> Self {
        SemVer {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }

    pub fn from_tag(tag: &str) -> Self {
        let re = Regex::new(r"\bv?(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)").unwrap();
        if let Some(cap) = re.captures(tag) {
            SemVer {
                major: cap["major"].parse::<i32>().unwrap(),
                minor: cap["minor"].parse::<i32>().unwrap(),
                patch: cap["patch"].parse::<i32>().unwrap(),
            }
        } else {
            SemVer::new()
        }
    }

    pub fn update_based_on_commit_msg(&mut self, commit_msg: &str) {
        match commit_msg {
            msg if msg.contains("major") || msg.contains("MAJOR") => self.bump_major(),
            msg if msg.contains("minor") || msg.contains("MINOR") => self.bump_minor(),
            msg if msg.contains("patch") || msg.contains("PATCH") => self.bump_patch(),
            _ => {},
        };
    }

    fn bump_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
    }

    fn bump_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
    }

    fn bump_patch(&mut self) {
        self.patch += 1;
    }
}

impl fmt::Display for SemVer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}


#[cfg(test)]
mod tests {
    use crate::SemVer;
#[test]
    fn test_new() {
        let semver = SemVer::new();
        assert_eq!(semver.major, 0);
        assert_eq!(semver.minor, 0);
        assert_eq!(semver.patch, 0);
    }

    #[test]
    fn test_from_tag() {
        let semver = SemVer::from_tag("v1.2.3");
        assert_eq!(semver.major, 1);
        assert_eq!(semver.minor, 2);
        assert_eq!(semver.patch, 3);

        let semver = SemVer::from_tag("invalid tag");
        assert_eq!(semver.major, 0);
        assert_eq!(semver.minor, 0);
        assert_eq!(semver.patch, 0);
    }

    #[test]
    fn test_update_based_on_commit_msg() {
        let mut semver = SemVer::new();
        semver.update_based_on_commit_msg("major: This is a major change");
        assert_eq!(semver.major, 1);
        assert_eq!(semver.minor, 0);
        assert_eq!(semver.patch, 0);
        assert_eq!(format!("{}", semver), "1.0.0");

        semver.update_based_on_commit_msg("minor: This is a minor change");
        assert_eq!(semver.major, 1);
        assert_eq!(semver.minor, 1);
        assert_eq!(semver.patch, 0);
        assert_eq!(format!("{}", semver), "1.1.0");

        semver.update_based_on_commit_msg("patch: This is a patch change");
        assert_eq!(semver.major, 1);
        assert_eq!(semver.minor, 1);
        assert_eq!(semver.patch, 1);
        assert_eq!(format!("{}", semver), "1.1.1");
    }
}