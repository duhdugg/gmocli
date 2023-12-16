use std::collections::HashMap;

const GMOCLI_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/data/gmocli.tsv"));

type GmocliMap<'a> = HashMap<&'a str, Gmocli<'a>>;

#[derive(Debug)]
pub struct Gmocli<'a> {
    pub emoji: &'a str,
    pub name: &'a str,
    pub category_name: &'a str,
    pub subcategory_name: &'a str,
    pub en_keywords: &'a str,
    pub en_tts_description: &'a str,
    pub gitmoji_description: &'a str,
}

impl<'a> Gmocli<'a> {
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
pub struct GmocliIndex<'a> {
    pub ordering: Vec<&'a str>,
    pub map: GmocliMap<'a>,
}

impl<'a> GmocliIndex<'a> {
    pub fn get_emoji_by_name(&self, name: &str) -> Option<&'a str> {
        let mut ret = None;
        for emoji in self.ordering.iter() {
            let gmocli = self.map.get(emoji).unwrap();
            if gmocli.name == name {
                ret = Some(&emoji[..]);
                break;
            }
        }
        ret
    }

    pub fn new() -> GmocliIndex<'a> {
        let mut ordering: Vec<&'a str> = vec![];
        let mut map: GmocliMap = GmocliMap::new();
        let gmocli_data = std::str::from_utf8(GMOCLI_DATA).unwrap();

        for line in gmocli_data.lines() {
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
                Gmocli {
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

        GmocliIndex { ordering, map }
    }

    pub fn print_list(&self, with_info: bool) {
        for emoji in self.ordering.iter() {
            self.map.get(emoji).unwrap().print(with_info);
        }
    }

    pub fn search_gmoclis(&self, search_keys: Vec<&str>) -> Vec<&'a str> {
        let mut matches: Vec<&'a str> = vec![];
        if &search_keys.len() > &0 {
            for emoji in self.ordering.iter() {
                let gmocli = self.map.get(emoji).unwrap();
                let search_string = format!(
                    "{} {} {} {} {} {} {}",
                    gmocli.emoji,
                    gmocli.name,
                    gmocli.category_name,
                    gmocli.subcategory_name,
                    gmocli.en_keywords,
                    gmocli.en_tts_description,
                    gmocli.gitmoji_description,
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
