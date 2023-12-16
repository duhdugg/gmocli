use std::collections::HashMap;
use std::fs;
use std::path;

const EMOJI_BYTES: &[u8] = include_bytes!("./data/emoji/emoji.json");
const GITMOJI_BYTES: &[u8] = include_bytes!("./data/gitmoji/gitmoji.json");
type GmocliMap = HashMap<String, HashMap<String, String>>;

fn main() {
    println!("rerun-if-changed=./data/emoji/emoji.json");
    println!("rerun-if-changed=./data/gitmoji/gitmoji.json");
    let gmocli_index = GmocliIndex::new();
    gmocli_index.generate_data();
}

#[derive(Debug)]
struct GmocliIndex {
    ordering: Vec<String>,
    map: GmocliMap,
}

impl GmocliIndex {
    fn new() -> GmocliIndex {
        let emojis_json = json::parse(std::str::from_utf8(EMOJI_BYTES).unwrap()).unwrap();
        let gitmojis_json = json::parse(std::str::from_utf8(GITMOJI_BYTES).unwrap()).unwrap();
        let mut ordering: Vec<String> = vec![];
        let mut map: GmocliMap = GmocliMap::new();

        if let json::JsonValue::Array(emojis) = emojis_json {
            for emoji in emojis {
                if let json::JsonValue::Object(e) = emoji {
                    let characters = e.get("characters").unwrap().to_string();
                    ordering.push(characters.to_string());
                    let mut gmocli: HashMap<String, String> = HashMap::new();
                    let en_keywords_value = e.get("en_keywords").unwrap();
                    let mut en_keywords: Vec<String> = vec![];
                    if let json::JsonValue::Array(en_keywords_value) = en_keywords_value {
                        for keyword in en_keywords_value {
                            en_keywords.push(keyword.to_string());
                        }
                    }
                    gmocli.insert("emoji".to_string(), characters.to_string());
                    gmocli.insert("name".to_string(), e.get("name").unwrap().to_string());
                    gmocli.insert(
                        "category_name".to_string(),
                        e.get("category_name").unwrap().to_string(),
                    );
                    gmocli.insert(
                        "subcategory_name".to_string(),
                        e.get("subcategory_name").unwrap().to_string(),
                    );
                    gmocli.insert("en_keywords".to_string(), en_keywords.join(","));
                    gmocli.insert(
                        "en_tts_description".to_string(),
                        e.get("en_tts_description").unwrap().to_string(),
                    );
                    map.insert(characters.to_string(), gmocli);
                }
            }
        }

        for (emoji, gmocli) in map.iter_mut() {
            gmocli.insert(
                "gitmoji_description".to_string(),
                get_gitmoji_description(&gitmojis_json, emoji.to_string()),
            );
        }
        GmocliIndex { ordering, map }
    }

    fn generate_data(&self) {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let out_file = format!("{}/data/gmocli.tsv", out_dir);
        fs::create_dir_all(&format!("{}/data", out_dir)).expect("Could not create data directory.");
        let empty = "".to_string();
        let mut lines: Vec<String> = vec![];
        for emoji in self.ordering.iter() {
            let gmocli = self.map.get(emoji).unwrap();
            lines.push(format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                gmocli.get("emoji").unwrap(),
                gmocli.get("name").unwrap(),
                gmocli.get("category_name").unwrap_or(&empty),
                gmocli.get("subcategory_name").unwrap_or(&empty),
                gmocli.get("en_tts_description").unwrap_or(&empty),
                gmocli.get("en_keywords").unwrap_or(&empty),
                gmocli.get("gitmoji_description").unwrap_or(&empty),
            ));
        }
        let data_string = lines.join("\n");
        let data = data_string.as_bytes();
        let data_path = path::Path::new(&out_file);
        fs::write(data_path, data).unwrap();
    }
}

fn get_gitmoji_description(gitmojis_json: &json::JsonValue, emoji: String) -> String {
    let mut gitmoji_description = "".to_string();
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
                        gitmoji_description = gmd
                    }
                }
            }
        }
    }
    gitmoji_description
}
