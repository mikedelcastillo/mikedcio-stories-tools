use regex::Regex;

#[derive(Debug)]
pub struct PostText {
    pub title: String,
    pub tags: Vec<String>,
    pub link: Option<String>,
    pub caption: String,
}

#[derive(Debug)]
pub enum BotCommand {
    Text(String),

    Done,

    MakePostStream,
    MakePost(PostText),
}

pub static BOT_TRIGGER: &str = "/";

pub fn match_and_get_command(_source: &str, _command: &str) {}

pub fn parse_make_post(content: &String) -> PostText {
    let re_prop = Regex::new(r"(\S*):\s*([^\n]*)").unwrap();

    // Isolate caption
    let caption = re_prop.replace_all(content, "").trim().to_string();
    let mut title = String::new();
    let mut tags = vec![];
    let mut link = None;

    // Capture post properties
    for capture in re_prop.captures_iter(content) {
        let prop = capture.get(1).map_or("", |m| m.as_str()).to_lowercase();
        let val = capture.get(2).map_or("", |m| m.as_str());

        if prop == "title" {
            title.push_str(val);
        } else if prop == "tags" {
            let re_sanitize = Regex::new(r"[^a-z0-9- ]").unwrap();
            let sanitized = val.to_lowercase();
            let sanitized = re_sanitize.replace_all(sanitized.as_str(), "");
            for tag in sanitized.split_whitespace() {
                tags.push(tag.to_string());
            }
        } else if prop == "link" {
            link = Some(val.trim().to_string());
        }
    }

    PostText {
        title,
        tags,
        link,
        caption,
    }
}

pub fn parse_message(txt: &String) -> BotCommand {
    let txt = txt.trim();
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

        if command == "post" {
            return BotCommand::MakePost(parse_make_post(&content));
        } else if command == "post_stream" {
            return BotCommand::MakePostStream;
        } else if command == "done" {
            return BotCommand::Done;
        }
    }

    return BotCommand::Text(txt.to_string());
}
