
use std::{env, fs};
use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;
use std::path::PathBuf;
use regex::Regex;


#[derive(Debug, PartialEq, Eq)]
struct JavaVersion {
    path: PathBuf,
    name: String,
    locations: Vec<PathBuf>,
    major: i32,
    minor: i32,
    build: i32
}


impl JavaVersion {
    fn new(path: PathBuf, name: String, locations: Vec<PathBuf>, major: i32, minor: i32, build: i32) -> JavaVersion {

        JavaVersion {
            path,
            name,
            locations,
            major,
            minor,
            build
        }
    }

    pub fn full_path(&self) -> PathBuf {
        self.path.join(&self.name)
    }
}


impl PartialOrd for JavaVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for JavaVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major.cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.build.cmp(&other.build))
    }
}


fn parse_oracle_path(path: PathBuf, name: &str) -> Result<JavaVersion, Box<dyn Error>> {
    let temp = name.trim_start_matches("jdk");

    let (major, minor, build) = if temp.starts_with("-") {
        let temp = temp.trim_start_matches('-');
        let elems: Vec<_> = temp.split(".").collect();
        (i32::from_str(elems[0])?, i32::from_str(elems[1])?, i32::from_str(elems[2])?)
    } else {
        let split_on: &[_] = &['.', '_'];
        let elems: Vec<_> = temp.split(split_on).collect();
        (i32::from_str(elems[1])?, i32::from_str(elems[2])?, i32::from_str(elems[3])?)
    };

    Ok(JavaVersion::new(path, name.into(), vec![PathBuf::from("lib/"), PathBuf::from("jre/bin/server/")], major, minor, build))
}


fn parse_openjdk_path(path: PathBuf, name: &str) -> Result<JavaVersion, Box<dyn Error>> {

    let (major, minor, build) = {
        let release_path = path.join(name).join("release");

        let release_info = fs::read_to_string(release_path)?;
        let regex = Regex::new(r#"JAVA_VERSION="(\d+)\.(\d+)\.(\d+)""#)?;
        let elems = regex.captures(&release_info)
            .ok_or::<Box<dyn Error>>("Couldn't find JAVA_VERSION in openjdk release file".into())?;

        (i32::from_str(&elems[0])?, i32::from_str(&elems[1])?, i32::from_str(&elems[2])?)
    };

    Ok(JavaVersion::new(path, name.into(), vec![PathBuf::from("lib/"), PathBuf::from("lib/server/")], major, minor, build))
}


#[cfg(windows)]
fn __find_start_locs() -> Vec<PathBuf> {
    vec![
        PathBuf::from("C:\\Program Files\\Java"),
        PathBuf::from("C:\\Program Files (x86)\\Java")
    ]
}


#[cfg(unix)]
fn __find_start_locs() -> Vec<PathBuf> {
    vec![
        PathBuf::from("/usr/lib/jvm"),
        PathBuf::from("/lib/jvm")
    ]
}


#[cfg(not(any(windows, unix)))]
fn __find_start_locs() -> ! {
    panic!("No implementation to find JDK on this platform, use JAVA_HOME to set location")
}


fn process_name(path: PathBuf, name: &str) -> Result<JavaVersion, Box<dyn Error>> {
    if name.starts_with("jdk") {
        parse_oracle_path(path, name)
    } else if name.contains("openjdk") && name.contains(".") {
        parse_openjdk_path(path, name)
    } else {
        Err(format!("Couldn't parse name {} into JavaVersion", name).into())
    }
}


fn iter_directory(path: PathBuf, iter: fs::ReadDir) -> Vec<JavaVersion> {
    let mut versions = Vec::new();

    for d in iter {

        let d = match d {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Got error \"{}\" while iterating directory, ignoring", e);
                continue;
            }
        };

        let name = match d.file_name().into_string() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Couldn't convert OsString to String for dir result {:?}, ignoring", e);
                continue;
            }
        };

        match process_name(path.clone(), &name) {
            Ok(res) => {
                versions.push(res)
            }
            Err(e) => {
                eprintln!("Got error \"{}\" processing name {}, ignoring", e, name);
                continue;
            }
        }
    }

    versions
}

/// Implementation for finding possible Java versions on the system. Iterates possible locations,
/// collects found versions, and returns the highest one.
fn find_impl(_: env::VarError) -> Result<JavaVersion, Box<dyn Error>> {
    // Get the starting points of the search
    let start_paths = __find_start_locs();

    // Create list of JVM versions in the given directories
    let mut versions = Vec::new();

    for start_path in start_paths {
        let read_result = fs::read_dir(start_path.clone());

        let iter = match read_result {
            Ok(iter) => iter,
            Err(e) => {
                eprintln!("Got error \"{}\" while reading directory, ignoring", e);
                continue;
            }
        };

        versions.extend(iter_directory(start_path, iter));
    }

    if versions.len() == 0 {
        return Err("Couldn't find valid JVM versions in Java directory".into());
    }

    eprintln!("Found Versions: {:?}", versions);

    let out = versions.into_iter().fold(None, |mut acc, val| {
        match acc {
            None => {
                acc = Some(val);
            }
            Some(v) => {
                acc = Some(JavaVersion::max(v, val));
            }
        }
        acc
    }).ok_or::<Box<dyn Error>>("No java versions found".into())?;

    Ok(out)
}


/// Locate the JVM on this system. If JAVA_HOME is defined, it uses that. Otherwise we search
/// a set of platform-specific locations, and attempt to find the highest java version among them
fn find_jvm() -> Result<JavaVersion, Box<dyn Error>> {
    env::var("JAVA_HOME")
        .map_or_else(&find_impl, |loc| {
            let loc = PathBuf::from(loc);

            let path = loc.parent()
                .ok_or::<Box<dyn Error>>("Couldn't get JAVA_HOME parent dir".into())?;

            let name = loc.file_name()
                .ok_or::<Box<dyn Error>>("Couldn't get JAVA_HOME dir name".into())?
                .to_str()
                .ok_or::<Box<dyn Error>>("Couldn't convert JAVA_HOME dir name to unicode string".into())?;

            process_name(path.into(), name)
        })
}


fn main() {
    let version = find_jvm()
        .expect("Couldn't determine jvm location");  // TODO: Pretty error handling

    let jvm_loc = version.full_path();

    // Link to the JVM library
    println!("cargo:rustc-link-lib=jvm");

    // Add paths to search for JVM library in
    for i in version.locations {
        let link_path = jvm_loc.join(i);

        let link_path = link_path.to_str()
            .expect("Couldn't convert JVM link path to String");

        println!("cargo:rustc-link-search={}", link_path)
    }
}
