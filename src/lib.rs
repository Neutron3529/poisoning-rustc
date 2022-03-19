//! # A simple crate that poisoning crates.io
//! malicious build.rs that could switch rustc to any program you want (not the file write this comment, it is build.rs)
//! This build file works in Linux system which install rustc with rustup.
//! similar on Windows system, but I had performed `dd if=/dev/null` through that garbage partition in my computer. Thus I do not port the script to windows.
//! for crates with this script, all the crate depends it would be infected, and who use a infected crate would get involved.
//! Since we operate rustc on the disk, only an update of rustc could cure the infected rust compiler.
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
