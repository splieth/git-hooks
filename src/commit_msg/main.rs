extern crate exitcode;

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{prelude::*, Seek, SeekFrom};
use std::process;

mod config;
mod returns;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename_opt = args.get(1);
    if filename_opt.is_none() {
        println!("Don't run this command yourself. Link it with your git-hooks and it will be called automatically.");
        process::exit(exitcode::DATAERR)
    }

    let contents = fs::read_to_string(filename_opt.unwrap())
        .expect("Something went wrong reading the file");

    let config_path = env::var("COMMIT_TEAM_CONFIG");
    let config: config::Config;
    match config_path {
        Ok(path) => config = config::read_config(&path),
        Err(_e) => {
            println!("Provide a commit_msg.config path via $COMMIT_TEAM_CONFIG");
            process::exit(exitcode::CONFIG)
        }
    }

    let new_msg = build_commit_msg(&contents, config);
    match new_msg {
        Ok(msg) => {
            write_to_file(filename_opt.unwrap(), &msg);
            std::process::exit(exitcode::OK)
        }
        Err(e) => {
            println!("{:?}", e.message);
            process::exit(exitcode::DATAERR)
        }
    }
}

fn write_to_file(filename: &str, msg: &str) -> () {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filename)
        .unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(msg.as_bytes()).unwrap();
}


fn build_msg(old_msg: &str, ps: Vec<config::TeamMember>) -> String {
    let mut message = format!("{}\n", old_msg);
    for x in ps.iter().skip(1) {
        let n = config::TeamMember::co_authored_by(&x);
        message = format!("{}\n{}", message, n)
    }

    return message;
}

fn extract_shorts<'a>(input: &'a str, regex: &str, separator: &str) -> returns::Result<Vec<&'a str>> {
    use regex::Regex;

    let re = Regex::new(regex).unwrap();

    match re.captures(input) {
        Some(groups) => {
            let first = groups.get(1).unwrap().as_str();
            if first.starts_with(separator) || first.ends_with(separator) {
                return Err(returns::Error { message: "Match starts or ends with the separator which is not allowed.".to_string() });
            }

            let parts: Vec<&str> = first.split(separator).collect();

            return Ok(parts);
        }
        None => return Err(returns::Error { message: "Regex does not match.".to_string() }),
    }
}

fn shorts_to_members(shorts: Vec<&str>, members: Vec<config::TeamMember>) -> returns::Result<Vec<config::TeamMember>> {
    let mut tm: Vec<config::TeamMember> = Vec::new();
    for x in shorts {
        let m = members.iter().find(|y| &y.short == x);
        if m.is_some() {
            tm.push(config::TeamMember {
                short: m.unwrap().short.to_string(),
                name: m.unwrap().name.to_string(),
                email: m.unwrap().email.to_string(),
            })
        } else {
            return Err(returns::Error { message: format!("Could not find: {}", x) });
        }
    }

    return Ok(tm);
}


fn build_commit_msg(input: &str, config: config::Config) -> returns::Result<String> {
    let shorts_result = extract_shorts(input, &config.regex, &config.separator);
    if shorts_result.is_err() {
        return Err(shorts_result.err().unwrap());
    }
    let members_result = shorts_to_members(shorts_result.unwrap(), config.team);
    if members_result.is_err() {
        return Err(members_result.err().unwrap());
    }
    let msg = build_msg(input, members_result.unwrap());

    return Ok(msg);
}


#[cfg(test)]
mod build_commit_msg {
    use super::*;

    #[test]
    fn test_pairing() {
        let config = config::Config {
            regex: "\\[.+?\\]\\s(.*?)\\s.*".to_string(),
            separator: "|".to_string(),
            team: vec![config::TeamMember {
                short: "hug".to_string(),
                name: "Hugo Heli".to_string(),
                email: "hugo.heli@domain.com".to_string(),
            }, config::TeamMember {
                short: "lup".to_string(),
                name: "Lud Lopi".to_string(),
                email: "lud.lopi@domain.com".to_string(),
            }],
        };
        let input = "[12] hug|lup some commit message";
        let expected = "[12] hug|lup some commit message\n\nCo-authored-by: Lud Lopi <lud.lopi@domain.com>";

        assert_eq!(build_commit_msg(input, config).unwrap(), expected.to_string());
    }

    #[test]
    fn test_team_member_missing() {
        let config = config::Config {
            regex: "\\[.+?\\]\\s(.*?)\\s.*".to_string(),
            separator: ",".to_string(),
            team: vec![config::TeamMember {
                short: "hug".to_string(),
                name: "Hugo Heli".to_string(),
                email: "hugo.heli@domain.com".to_string(),
            }],
        };
        let input = "[12] hug,lup some commit message";

        assert!(build_commit_msg(input, config).is_err());
    }
}
