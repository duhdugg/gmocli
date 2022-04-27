use std::collections::HashMap;

const EMOCLI_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/data/emocli.tsv"));

type EmocliMap<'a> = HashMap<&'a str, Emocli<'a>>;

#[derive(Debug)]
pub struct Emocli<'a> {
    pub emoji: &'a str,
    pub name: &'a str,
    pub category_name: &'a str,
    pub subcategory_name: &'a str,
    pub en_keywords: &'a str,
    pub en_tts_description: &'a str,
    pub gitmoji_description: &'a str,
}

impl<'a> Emocli<'a> {
    pub fn print(&self, with_info: bool) {
        match with_info {
            true => {
                let mut gitmoji_description = String::new();
                if self.gitmoji_description.len() > 0 {
                    gitmoji_description = format!(" # {}", self.gitmoji_description);
                }
                println!(
                    "{} {} | {} / {} | {}{}",
                    self.emoji,
                    self.name,
                    self.category_name,
                    self.subcategory_name,
                    self.en_keywords,
                    &gitmoji_description[..],
                )
            }
            false => {
                print!("{}", self.emoji);
            }
        }
    }
}

#[derive(Debug)]
pub struct EmocliIndex<'a> {
    pub ordering: Vec<&'a str>,
    pub map: EmocliMap<'a>,
}

impl<'a> EmocliIndex<'a> {
    pub fn get_emoji_by_name(&self, name: &str) -> Option<&'a str> {
        let mut ret = None;
        for emoji in self.ordering.iter() {
            let emocli = self.map.get(emoji).unwrap();
            if emocli.name == name {
                ret = Some(&emoji[..]);
                break;
            }
        }
        ret
    }

    pub fn new() -> EmocliIndex<'a> {
        let mut ordering: Vec<&'a str> = vec![];
        let mut map: EmocliMap = EmocliMap::new();
        let emocli_data = std::str::from_utf8(EMOCLI_DATA).unwrap();

        for line in emocli_data.lines() {
            let split_tabs: Vec<&str> = line.split("\t").collect();
            let emoji = split_tabs[0];
            let name = split_tabs[1];
            let category_name = split_tabs[2];
            let subcategory_name = split_tabs[3];
            let en_tts_description = split_tabs[4];
            let en_keywords = split_tabs[5];
            let gitmoji_description = split_tabs[6];
            ordering.push(emoji);
            map.insert(
                emoji,
                Emocli {
                    emoji,
                    name,
                    category_name,
                    subcategory_name,
                    en_keywords,
                    en_tts_description,
                    gitmoji_description,
                },
            );
        }

        EmocliIndex { ordering, map }
    }

    pub fn print_list(&self, with_info: bool) {
        for emoji in self.ordering.iter() {
            self.map.get(emoji).unwrap().print(with_info);
        }
    }

    pub fn search_emoclis(&self, search_keys: Vec<&str>) -> Vec<&'a str> {
        let mut matches: Vec<&'a str> = vec![];
        if &search_keys.len() > &0 {
            for emoji in self.ordering.iter() {
                let emocli = self.map.get(emoji).unwrap();
                let search_string = format!(
                    "{} {} {} {} {} {} {}",
                    emocli.emoji,
                    emocli.name,
                    emocli.category_name,
                    emocli.subcategory_name,
                    emocli.en_keywords,
                    emocli.en_tts_description,
                    emocli.gitmoji_description,
                );
                let mut has_match = false;
                for search_key in &search_keys {
                    if search_string
                        .to_lowercase()
                        .contains(search_key.to_lowercase().as_str())
                    {
                        has_match = true;
                        break;
                    }
                }
                if has_match {
                    matches.push(emoji);
                }
            }
        }
        matches
    }
}
