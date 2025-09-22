use std::{fs::OpenOptions, io::Write};

use clap::Parser;

use rcgen::generate_simple_self_signed;

use crate::parse_args::Args;

mod parse_args;

fn main() {
    let args: Args = Args::parse();

    if args.is_emtpy() {
        println!("Autosigned Certificate creation Failed : no socket address found");
        return;
    }

    println!("Create auto-signed certificate for [{:?}]", args);

    let Ok(certified_keys) = generate_simple_self_signed(args.to_strings_vec()) else {
        println!("Failed To generate_simple_self_signed ");
        return;
    };

    let cert = certified_keys.cert.pem();
    let sign_key = certified_keys.signing_key.serialize_pem();

    let Ok(mut cert_file) = OpenOptions::new().create(true).write(true).open("cert.pem") else {
        println!("Failed to create cert_file");
        return;
    };

    if let Err(e) = cert_file.write_all(cert.as_bytes()) {
        println!("Failed To Write cert.pem : [{:?}]", e);
        return;
    }
    let Ok(mut sign_file) = OpenOptions::new().create(true).write(true).open("key.pem") else {
        println!("Failed to create cert_file");
        return;
    };

    if let Err(e) = sign_file.write_all(sign_key.as_bytes()) {
        println!("Failed To Write key.pem : [{:?}]", e);
        return;
    }

    println!("Certified keys written succesfully !  => cert.pem, key.pem ");
}
