extern crate yaml_rust;

use std::fs::File;
use std::io::prelude::*;
use std::process;

use yaml_rust::YamlLoader;

use self::yaml_rust::Yaml;

#[derive(Debug, PartialEq, Clone)]
pub struct TeamMember {
    pub short: String,
    pub name: String,
    pub email: String,
}

impl TeamMember {
    pub fn co_authored_by(p: &TeamMember) -> String {
        return format!("Co-authored-by: {} <{}>", p.name, p.email);
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub regex: String,
    pub separator: String,
    pub drop_shorts: bool,
    pub me: String,
    pub team: Vec<TeamMember>,
}

pub fn read_config(path: &str) -> Config {
    let mut team_memebrs: Vec<TeamMember> = vec![];
    let contents = load_yaml_file(path);
    let regex = contents[0]["regex"].as_str().unwrap();
    let me = contents[0]["me"].as_str().unwrap();
    let separator = contents[0]["separator"].as_str().unwrap();
    let drop_shorts_opt = contents[0]["drop-shorts"].as_bool();
    let drop_shorts  = if drop_shorts_opt.is_some() {
        drop_shorts_opt.unwrap()
    } else {
        false
    };

    let team = contents[0]["team"].as_vec().unwrap();
    for item in team.iter() {
        let m = item.as_hash().unwrap();
        let short = m.get(&Yaml::String("short".to_string())).unwrap().as_str().unwrap();
        let name = m.get(&Yaml::String("name".to_string())).unwrap().as_str().unwrap();
        let email = m.get(&Yaml::String("email".to_string())).unwrap().as_str().unwrap();
        team_memebrs.push(TeamMember {
            short: short.to_string(),
            name: name.to_string(),
            email: email.to_string(),
        });
    }
    return Config {
        regex: regex.to_string(),
        separator: separator.to_string(),
        drop_shorts: drop_shorts,
        me: me.to_string(),
        team: team_memebrs,
    };
}


fn load_yaml_file(path: &str) -> Vec<Yaml> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    let result = file.read_to_string(&mut contents);
    if result.is_err() {
        println!("Could not read YAML configuration.");
        process::exit(exitcode::CONFIG)
    }
    let docs = YamlLoader::load_from_str(&contents).unwrap();

    return docs;
}

#[cfg(test)]
mod read_config {
    use super::*;

    #[test]
    fn test_read_test_config() {
        let config = Config {
            regex: "\\[.+?\\]\\s(.*?)\\s.*".to_string(),
            separator: "|".to_string(),
            drop_shorts: true,
            me: "fli".to_string(),
            team: vec![TeamMember {
                short: "hug".to_string(),
                name: "Hugo Heimlich".to_string(),
                email: "hugo.heimlich@domain.com".to_string(),
            }, TeamMember {
                short: "fli".to_string(),
                name: "Fliedbelt Igel".to_string(),
                email: "fliedbelt.igel@domain.com".to_string(),
            }],
        };

        let path = "test-resources/test-config.yaml";
        assert_eq!(read_config(path), config);
    }

    #[test]
    fn test_read_test_config_without_drop_shorts() {
        let config = Config {
            regex: "\\[.+?\\]\\s(.*?)\\s.*".to_string(),
            separator: "|".to_string(),
            drop_shorts: false,
            me: "fli".to_string(),
            team: vec![TeamMember {
                short: "hug".to_string(),
                name: "Hugo Heimlich".to_string(),
                email: "hugo.heimlich@domain.com".to_string(),
            }, TeamMember {
                short: "fli".to_string(),
                name: "Fliedbelt Igel".to_string(),
                email: "fliedbelt.igel@domain.com".to_string(),
            }],
        };

        let path = "test-resources/test-config-without-drop-shorts.yaml";
        assert_eq!(read_config(path), config);
    }
}
