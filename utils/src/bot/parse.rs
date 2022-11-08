use regex::Regex;

#[derive(Debug)]
pub enum BotMessage {
    Nothing,
    JustText(String),
    Done,

    MakePostStream,
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
    let re_prop = Regex::new(r"(\S*):\s*([^\n]*)").unwrap();

    // Isolate caption
    let caption = re_prop.replace_all(content, "").trim().to_string();
    let mut title = String::new();
    let mut tags = vec![];

    // Capture post properties
    for capture in re_prop.captures_iter(content) {
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
        return Err(BotMessageError::MakePostIncomplete);
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
        let re_command = Regex::new(r"([a-z0-9_]*)").unwrap();
        let content = re_command.replace(&txt[BOT_TRIGGER.len()..], "");
        let content = content.trim().to_string();

        let command = re_command.captures(&txt_lwc[BOT_TRIGGER.len()..]);
        let command = match command {
            Some(captures) => match captures.get(1) {
                Some(s) => s.as_str(),
                _ => "",
            },
            _ => "",
        };

        // post
        println!("command: {}", command);
        if command == "post" {
            return parse_make_post(&content);
        } else if command == "post_stream" {
            return Ok(BotMessage::MakePostStream);
        } else if command == "done" {
            return Ok(BotMessage::Done);
        }

        return Err(BotMessageError::CommandNotFound(txt.to_string()));
    }

    if txt.len() > 0 {
        return Ok(BotMessage::JustText(txt.to_string()));
    }

    Ok(BotMessage::Nothing)
}
