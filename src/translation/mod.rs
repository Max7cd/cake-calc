mod german;

pub fn translate(s: &str, lang: &Language) -> String {
    match lang {
        Language::English => s.to_owned(),
        Language::German => german::TRANSLATION_TABLE.get(s).unwrap_or(&s).to_owned().to_owned(),
    }
}

pub enum Language {
    English,
    German
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl From<&Language> for String {
    fn from(value: &Language) -> Self {
        match value {
            Language::English => "English".into(),
            Language::German => "German".into(),
        }
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match &value.to_lowercase()[..] {
            "english" => Language::English,
            "german" => Language::German,
            _ => panic!()
        }
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        String::from(self)
    }
}