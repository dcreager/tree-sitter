use lazy_static::lazy_static;
use std::fs;
use std::path::{Path, PathBuf};
use tree_sitter::Language;
use tree_sitter_highlight::{HighlightConfiguration, HighlightConfigurationBuilder};
use tree_sitter_loader::Loader;

include!("./dirs.rs");

lazy_static! {
    static ref TEST_LOADER: Loader = Loader::with_parser_lib_path(SCRATCH_DIR.clone());
}

pub fn test_loader<'a>() -> &'a Loader {
    &*TEST_LOADER
}

pub fn fixtures_dir<'a>() -> &'static Path {
    &FIXTURES_DIR
}

pub fn get_language(name: &str) -> Language {
    TEST_LOADER
        .load_language_at_path(&GRAMMARS_DIR.join(name).join("src"), &HEADER_DIR)
        .unwrap()
}

pub fn get_language_queries_path(language_name: &str) -> PathBuf {
    GRAMMARS_DIR.join(language_name).join("queries")
}

pub fn get_highlight_config(
    language_name: &str,
    injection_query_filename: Option<&str>,
    highlight_names: Vec<String>,
) -> HighlightConfiguration {
    let language = get_language(language_name);
    let queries_path = get_language_queries_path(language_name);
    let mut builder = HighlightConfigurationBuilder::new();
    let highlights_query = fs::read_to_string(queries_path.join("highlights.scm")).unwrap();
    builder.add_highlights_query_part(&highlights_query);
    if let Some(injection_query_filename) = injection_query_filename {
        let injections_query =
            fs::read_to_string(queries_path.join(injection_query_filename)).unwrap();
        builder.add_injections_query_part(&injections_query);
    }
    let locals_query = fs::read_to_string(queries_path.join("locals.scm")).unwrap_or(String::new());
    builder.add_locals_query_part(&locals_query);
    builder.set_recognized_names(highlight_names);
    builder.build(language).unwrap()
}

pub fn get_test_language(name: &str, parser_code: &str, path: Option<&Path>) -> Language {
    let parser_c_path = SCRATCH_DIR.join(&format!("{}-parser.c", name));
    if !fs::read_to_string(&parser_c_path)
        .map(|content| content == parser_code)
        .unwrap_or(false)
    {
        fs::write(&parser_c_path, parser_code).unwrap();
    }
    let scanner_path = path.and_then(|p| {
        let result = p.join("scanner.c");
        if result.exists() {
            Some(result)
        } else {
            None
        }
    });
    TEST_LOADER
        .load_language_from_sources(name, &HEADER_DIR, &parser_c_path, &scanner_path)
        .unwrap()
}

pub fn get_test_grammar(name: &str) -> (String, Option<PathBuf>) {
    let dir = fixtures_dir().join("test_grammars").join(name);
    let grammar = fs::read_to_string(&dir.join("grammar.json")).unwrap();
    (grammar, Some(dir))
}
