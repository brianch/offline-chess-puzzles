use fluent_bundle::FluentResource;
use fluent_bundle::bundle::FluentBundle;
use unic_langid::langid;
use std::io::Read;
use once_cell::sync::Lazy;

use crate::config::TRANSLATIONS_DIRECTORY;

static BUNDLE_ENUS: Lazy<FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>> = Lazy::new(|| {
    let file = std::fs::File::open(TRANSLATIONS_DIRECTORY.to_owned() + "en-US/ocp.ftl").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut source = String::new();
    reader.read_to_string(&mut source).expect("Failed to read en-US translation file");
    let res = FluentResource::try_new(source).expect("Could not parse the FTL file.");
    let mut bundle = FluentBundle::new_concurrent(vec![langid!("en-US")]);
    bundle.add_resource(res).expect("Failed to add FTL resources to the bundle.");
    bundle
});

static BUNDLE_PTBR: Lazy<FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>> = Lazy::new(|| {
    let file = std::fs::File::open(TRANSLATIONS_DIRECTORY.to_owned() + "pt-BR/ocp.ftl").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut source = String::new();
    reader.read_to_string(&mut source).expect("Failed to read pt-BR translation file");
    let res = FluentResource::try_new(source).expect("Could not parse the FTL file.");
    let mut bundle = FluentBundle::new_concurrent(vec![langid!("pt-BR")]);
    bundle.add_resource(res).expect("Failed to add FTL resources to the bundle.");
    bundle
});

static BUNDLE_ES: Lazy<FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>> = Lazy::new(|| {
    let file = std::fs::File::open(TRANSLATIONS_DIRECTORY.to_owned() + "es/ocp.ftl").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut source = String::new();
    reader.read_to_string(&mut source).expect("Failed to read ES translation file");
    let res = FluentResource::try_new(source).expect("Could not parse the FTL file.");
    let mut bundle = FluentBundle::new_concurrent(vec![langid!("es")]);
    bundle.add_resource(res).expect("Failed to add FTL resources to the bundle.");
    bundle
});

static BUNDLE_FR: Lazy<FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>> = Lazy::new(|| {
    let file = std::fs::File::open(TRANSLATIONS_DIRECTORY.to_owned() + "fr/ocp.ftl").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut source = String::new();
    reader.read_to_string(&mut source).expect("Failed to read FR translation file");
    let res = FluentResource::try_new(source).expect("Could not parse the FTL file.");
    let mut bundle = FluentBundle::new_concurrent(vec![langid!("fr")]);
    bundle.add_resource(res).expect("Failed to add FTL resources to the bundle.");
    bundle
});

static BUNDLE_CN: Lazy<FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>> = Lazy::new(|| {
    let file = std::fs::File::open(TRANSLATIONS_DIRECTORY.to_owned() + "cn/ocp.ftl").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut source = String::new();
    reader.read_to_string(&mut source).expect("Failed to read CN translation file");
    let res = FluentResource::try_new(source).expect("Could not parse the FTL file.");
    let mut bundle = FluentBundle::new_concurrent(vec![langid!("fr")]);
    bundle.add_resource(res).expect("Failed to add FTL resources to the bundle.");
    bundle
});

pub fn tr(lang: &Language, key: &str) -> String {
    let bundle = match lang {
        Language::Portuguese => &BUNDLE_PTBR,
        Language::English => &BUNDLE_ENUS,
        Language::Spanish => &BUNDLE_ES,
        Language::French => &BUNDLE_FR,
        Language::Chinese => &BUNDLE_CN,
    };
    let msg = bundle.get_message(key).expect(&("Missing translation key ".to_owned() + key));
    let mut errors = vec![];
    let pattern = msg.value().expect("Missing Value.");
    bundle.format_pattern(pattern, None, &mut errors).to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English, Portuguese, Spanish, French, Chinese
}

impl Language {
    pub const ALL: [Language; 5] = [
        Language::English, Language::Portuguese, Language::Spanish, Language::French, Language::Chinese
    ];
}

impl DisplayTranslated for Language {
    fn to_str_tr(&self) -> &str {
        match self {
            Language::English => "english",
            Language::Portuguese => "portuguese",
            Language::Spanish => "spanish",
            Language::French => "french",
            Language::Chinese => "chinese",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PickListWrapper<D: DisplayTranslated> {
    pub lang: Language,
    pub item: D,
}

pub trait DisplayTranslated {
    fn to_str_tr(&self) -> &str;
}

impl<D: DisplayTranslated> std::fmt::Display for PickListWrapper<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&tr(&self.lang, self.item.to_str_tr()))
    }
}

impl<D: DisplayTranslated + std::cmp::PartialEq> PartialEq for PickListWrapper<D> {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl<D: DisplayTranslated + std::cmp::PartialEq> Eq for PickListWrapper<D> {}

impl PickListWrapper<Language> {
    pub fn get_langs(lang: Language) -> Vec<PickListWrapper<Language>> {
        let mut themes_wrapper = Vec::new();
        for item in Language::ALL {
            themes_wrapper.push(
                PickListWrapper::<Language> { lang, item }
            );
        }
        themes_wrapper
    }

    pub fn new_lang(lang: Language, item: Language) -> Self {
        Self { lang, item }
    }
}
