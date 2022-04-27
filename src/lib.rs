use std::collections::HashMap;

const EMOJI_BYTES: &[u8] = include_bytes!("../data/emoji/emoji.json");
const GITMOJI_BYTES: &[u8] = include_bytes!("../data/gitmoji/gitmoji.json");
type EmocliMap = HashMap<String, Emocli>;

#[derive(Debug)]
pub struct Emocli {
    emoji: String,
    name: String,
    category_name: String,
    subcategory_name: String,
    en_keywords: Vec<String>,
    en_tts_description: String,
    gitmoji_description: Option<String>,
}

impl Emocli {
    pub fn print(&self, with_info: bool) {
        match with_info {
            true => {
                let gitmoji_description = match self.gitmoji_description.as_ref() {
                    None => "".to_string(),
                    Some(gitmoji_description) => format!(" # {}", gitmoji_description),
                };
                let keywords: String = self.en_keywords.join(",");
                println!(
                    "{} {} | {} / {} | {}{}",
                    self.emoji,
                    self.name,
                    self.category_name,
                    self.subcategory_name,
                    keywords,
                    gitmoji_description,
                )
            }
            false => {
                print!("{}", self.emoji);
            }
        }
    }
}

#[derive(Debug)]
pub struct EmocliIndex {
    ordering: Vec<String>,
    pub map: EmocliMap,
}

impl EmocliIndex {
    pub fn get_emoji_by_name(&self, name: &str) -> Option<String> {
        let mut ret = None;
        for emoji in self.ordering.iter() {
            let emocli = self.map.get(emoji).unwrap();
            if emocli.name == name {
                ret = Some(emoji.to_string());
                break;
            }
        }
        ret
    }

    pub fn new() -> EmocliIndex {
        let emojis_json = json::parse(std::str::from_utf8(EMOJI_BYTES).unwrap()).unwrap();
        let gitmojis_json = json::parse(std::str::from_utf8(GITMOJI_BYTES).unwrap()).unwrap();
        let mut ordering: Vec<String> = vec![];
        let mut map: EmocliMap = EmocliMap::new();

        if let json::JsonValue::Array(emojis) = emojis_json {
            for emoji in emojis {
                if let json::JsonValue::Object(e) = emoji {
                    let characters = e.get("characters").unwrap().to_string();
                    let name = e.get("name").unwrap().to_string();
                    let category_name = e.get("category_name").unwrap().to_string();
                    let subcategory_name = e.get("subcategory_name").unwrap().to_string();
                    let en_tts_description = e.get("en_tts_description").unwrap().to_string();
                    let en_keywords_value = e.get("en_keywords").unwrap();
                    let mut en_keywords: Vec<String> = vec![];
                    if let json::JsonValue::Array(en_keywords_value) = en_keywords_value {
                        for keyword in en_keywords_value {
                            en_keywords.push(keyword.to_string());
                        }
                    }
                    let gitmoji_description = None;
                    ordering.push(characters.to_string());
                    map.insert(
                        characters.to_string(),
                        Emocli {
                            emoji: characters,
                            name,
                            category_name,
                            subcategory_name,
                            en_keywords,
                            en_tts_description,
                            gitmoji_description,
                        },
                    );
                }
            }
        }

        for (emoji, emocli) in map.iter_mut() {
            emocli.gitmoji_description = get_gitmoji_description(&gitmojis_json, emoji.to_string());
        }
        EmocliIndex { ordering, map }
    }

    pub fn print_list(&self, with_info: bool) {
        for emoji in self.ordering.iter() {
            self.map.get(emoji).unwrap().print(with_info);
        }
    }

    pub fn search_emoclis(&self, search_keys: Vec<String>) -> Vec<String> {
        let mut matches: Vec<String> = vec![];
        if &search_keys.len() > &0 {
            for emoji in self.ordering.iter() {
                let emocli = self.map.get(emoji).unwrap();
                let search_string = format!(
                    "{} {} {} {} {} {} {}",
                    emocli.emoji,
                    emocli.name,
                    emocli.category_name,
                    emocli.subcategory_name,
                    emocli.en_keywords.join(","),
                    emocli.en_tts_description,
                    emocli
                        .gitmoji_description
                        .as_ref()
                        .unwrap_or(&"".to_string()),
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
                    matches.push(emoji.to_string());
                }
            }
        }
        matches
    }
}

pub fn get_gitmoji_description(gitmojis_json: &json::JsonValue, emoji: String) -> Option<String> {
    let mut gitmoji_description = None;
    if let json::JsonValue::Object(gitmojis_object) = &gitmojis_json {
        if let json::JsonValue::Array(gitmojis) = gitmojis_object.get("gitmojis").unwrap() {
            for gitmoji in gitmojis {
                if let json::JsonValue::Object(gitmoji) = gitmoji {
                    let mut gm_emoji = gitmoji.get("emoji").unwrap().to_string();
                    let vs16 = std::str::from_utf8(&[0xef, 0xb8, 0x8f]).unwrap();
                    if !emoji.contains(vs16) {
                        gm_emoji = gm_emoji.replace(vs16, "");
                    }
                    if emoji == gm_emoji {
                        let gmd = gitmoji.get("description").unwrap().to_string();
                        gitmoji_description = Some(gmd);
                    }
                }
            }
        }
    }
    gitmoji_description
}
