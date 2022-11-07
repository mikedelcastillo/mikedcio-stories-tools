use regex::Regex;

#[derive(Debug)]
pub enum BotMessage {
    Nothing,
    JustText(String),
    MakePost {
        title: String,
        tags: Vec<String>,
        caption: String,
    },
}

#[derive(Debug)]
pub enum BotMessageError {
    MakePostIncomplete,
    CommandNotFound(String),
}

pub static BOT_TRIGGER: &str = "/";

pub fn match_and_get_command(_source: &str, _command: &str) {}

pub fn parse_make_post(content: &String) -> Result<BotMessage, BotMessageError> {
    let re = Regex::new(r"(\S*):\s*([^\n]*)").unwrap();

    // Isolate caption
    let caption = re.replace_all(content, "").trim().to_string();
    let mut title = String::new();
    let mut tags = vec![];

    // Capture post properties
    for capture in re.captures_iter(content) {
        let prop = capture.get(1).map_or("", |m| m.as_str()).to_lowercase();
        let val = capture.get(2).map_or("", |m| m.as_str());

        if prop == "title" {
            title.push_str(val);
        } else if prop == "tags" {
            for tag in val.split_whitespace() {
                tags.push(tag.to_string());
            }
        }
    }

    if title.len() == 0 && tags.len() == 0 && caption.len() == 0 {
        return Err(BotMessageError::MakePostIncomplete)
    }

    Ok(BotMessage::MakePost {
        title,
        tags,
        caption,
    })
}

pub fn parse_bot_message(txt: &str) -> Result<BotMessage, BotMessageError> {
    let txt = txt.trim();
    println!("PARSING_BOT_MESSAGE: {}", txt);

    let txt_lwc = txt.to_lowercase();

    if txt_lwc.starts_with(BOT_TRIGGER) {
        let txt_com_offset = &txt_lwc[BOT_TRIGGER.len()..];

        let re = Regex::new(r"^\S*").unwrap();
        let content = re.replace(txt, "");
        let content = content.trim().to_string();

        // post
        if txt_com_offset.starts_with("post") {
            return parse_make_post(&content);
        }

        return Err(BotMessageError::CommandNotFound(txt.to_string()));
    }

    if txt.len() > 0 {
        return Ok(BotMessage::JustText(txt.to_string()));
    }

    Ok(BotMessage::Nothing)
}
