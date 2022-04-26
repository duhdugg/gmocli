use emocli::EmocliIndex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if &args.len() < &2 {
        print_help();
        std::process::exit(1);
    }
    let mut help_flag = args.contains(&"-h".to_string()) || args.contains(&"--help".to_string());
    let mut list_flag = args.contains(&"-l".to_string()) || args.contains(&"--list".to_string());
    let mut info_flag = args.contains(&"-i".to_string()) || args.contains(&"--info".to_string());
    let mut name_flag = args.contains(&"-n".to_string()) || args.contains(&"--name".to_string());
    let dash = "-".to_string();
    for arg in &args {
        if arg[0..1] == dash && arg[1..2] != dash {
            if arg.contains('h') {
                help_flag = true;
            }
            if arg.contains('l') {
                list_flag = true;
            }
            if arg.contains('i') {
                info_flag = true;
            }
            if arg.contains('n') {
                name_flag = true;
            }
        }
    }
    if help_flag {
        print_help();
    }
    let emocli_index = EmocliIndex::new();
    if list_flag {
        emocli_index.print_list(info_flag);
    }

    let mut search_keys: Vec<String> = vec![];
    for arg in &args[1..] {
        if !arg.starts_with('-') {
            search_keys.push(arg.to_string());
        }
    }

    if name_flag {
        let emoji_result = emocli_index.get_emoji_by_name(search_keys.join(" ").as_str());
        match emoji_result {
            Some(emoji) => {
                let emocli = emocli_index.map.get(&emoji).unwrap();
                emocli.print(info_flag);
            }
            None => {
                std::process::exit(1);
            }
        }
    } else {
        let emoji_matches = emocli_index.search_emoclis(search_keys);
        if emoji_matches.len() > 0 {
            for emoji in emoji_matches {
                let emocli = emocli_index.map.get(&emoji).unwrap();
                emocli.print(info_flag);
            }
        } else {
            if !list_flag {
                std::process::exit(1);
            }
        }
    }
}

fn print_help() {
    let lines = vec![
        "Usage:\temocli [OPTIONS] <search>",
        "",
        "OPTIONS:",
        "-h | --help \t print this help",
        "-l | --list \t list all emoji characters",
        "-i | --info \t include info",
        "-n | --name \t match name exactly",
        "",
        "📝 Online help: <https://github.com/duhdugg/emocli/>",
        "🐛 Issues: <https://github.com/duhdugg/emocli/issues>",
        "✨ Pull Requests: <https://github.com/duhdugg/emocli/pulls>",
    ];
    println!("{}", lines.join("\n"));
}