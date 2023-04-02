use fluent_bundle::FluentResource;
use fluent_bundle::bundle::FluentBundle;
use unic_langid::langid;
use std::io::Read;
use once_cell::sync::Lazy;

use crate::search_tab::DisplayTranslated;

static BUNDLE_ENUS: Lazy<FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>> = Lazy::new(|| {
    let file = std::fs::File::open("./translations/en-US/ocp.ftl").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut source = String::new();
    reader.read_to_string(&mut source).expect("Failed to read en-US translation file");
    let res = FluentResource::try_new(source).expect("Could not parse the FTL file.");
    let mut bundle = FluentBundle::new_concurrent(vec![langid!("en-US")]);
    bundle.add_resource(res).expect("Failed to add FTL resources to the bundle.");
    bundle
});

static BUNDLE_PTBR: Lazy<FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>> = Lazy::new(|| {
    let file = std::fs::File::open("./translations/pt-BR/ocp.ftl").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut source = String::new();
    reader.read_to_string(&mut source).expect("Failed to read pt-BR translation file");
    let res = FluentResource::try_new(source).expect("Could not parse the FTL file.");
    let mut bundle = FluentBundle::new_concurrent(vec![langid!("pt-BR")]);
    bundle.add_resource(res).expect("Failed to add FTL resources to the bundle.");
    bundle
});

static BUNDLE_ES: Lazy<FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>> = Lazy::new(|| {
    let file = std::fs::File::open("./translations/es/ocp.ftl").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut source = String::new();
    reader.read_to_string(&mut source).expect("Failed to read ES translation file");
    let res = FluentResource::try_new(source).expect("Could not parse the FTL file.");
    let mut bundle = FluentBundle::new_concurrent(vec![langid!("es")]);
    bundle.add_resource(res).expect("Failed to add FTL resources to the bundle.");
    bundle
});

pub fn tr(lang: &Language, key: &str) -> String {
    let bundle = match lang {
        Language::Portuguese => &BUNDLE_PTBR,
        Language::English => &BUNDLE_ENUS,
        Language::Spanish => &BUNDLE_ES,
    };
    let msg = bundle.get_message(key).expect("Missing translation key.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Missing Value.");
    bundle.format_pattern(&pattern, None, &mut errors).to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English, Portuguese, Spanish
}

impl Language {
    pub const ALL: [Language; 3] = [
        Language::English, Language::Portuguese, Language::Spanish
    ];
}

impl DisplayTranslated for Language {
    fn to_str_tr(&self) -> &str {
        match self {
            Language::English => "english",
            Language::Portuguese => "portuguese",
            Language::Spanish => "spanish",
        }
    }
}
