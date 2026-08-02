//! Checks over the shape of the content files themselves, rather than what is
//! in them.
//!
//! This project's most repeated failure by a wide margin is **a key in a data
//! file that no struct claims**. Serde drops it in silence, so the file reads
//! as configured and the game ignores it, and nothing anywhere says so. It has
//! happened at least four times: the Southern Pass's
//! `required_completed_quest`, which meant the gate the whole southern half of
//! the map sits behind did not exist at runtime; `alchemy.heat` and
//! `alchemy.fill_slots` in the input bindings; `toast_icons` and
//! `default_toast_icon` in `ui_art.json`, which is why six generated icons were
//! never loaded; and three duplicate entries in the narrative milestone block.
//!
//! Each time the fix was `deny_unknown_fields` on **that one struct**. This is
//! the rest of the sweep — the project's own method note says that when a bug
//! class repeats you should go and find the others — plus the guard that keeps
//! the next struct from shipping without it.

#[cfg(test)]
mod tests {
    /// Every struct that content is authored into must reject a key it does not
    /// read. Scanning the source is the only way to ask this: derive attributes
    /// are gone by runtime, and there is nothing to reflect over.
    ///
    /// The exceptions are deliberate and narrow. **Save-side structs stay
    /// lenient**, because they parse files written by *older builds* rather
    /// than files written by hand — `HabitatStateEntry.placed_day` was deleted
    /// as dead in an earlier pass, and a save from before that still carries
    /// it. Strictness is right for content an author controls and wrong for a
    /// record the player already has on disk.
    #[test]
    fn every_content_schema_rejects_a_key_it_does_not_read() {
        /// Whole files that parse what *older builds wrote*, not what an author
        /// typed. `HabitatStateEntry.placed_day` was deleted as dead in an
        /// earlier pass, and every save from before that still carries it.
        const SAVE_SIDE_FILES: [&str; 3] = [
            "src/data/save_models.rs",
            "src/data/save_memory_models.rs",
            "src/data/schema_progression.rs",
        ];
        /// Read from both sides: the journal's own beat record is content in
        /// `narrative_text.json` and state in a save file.
        const SAVE_SIDE_STRUCTS: [&str; 1] = ["JournalMilestoneEntry"];

        let mut lenient = Vec::new();
        let mut checked = 0usize;
        for path in rust_sources() {
            if SAVE_SIDE_FILES.contains(&path.as_str()) {
                continue;
            }
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{path} should be readable: {error}"));
            let lines = source.lines().collect::<Vec<_>>();
            for (index, line) in lines.iter().enumerate() {
                let Some(name) = struct_name(line) else {
                    continue;
                };
                if SAVE_SIDE_STRUCTS.contains(&name) {
                    continue;
                }
                // The attributes and doc comment immediately above the
                // declaration, read upwards until something else starts.
                let attributes = lines[..index]
                    .iter()
                    .rev()
                    .take_while(|earlier| {
                        let earlier = earlier.trim();
                        earlier.starts_with("#[") || earlier.starts_with("///")
                    })
                    .collect::<Vec<_>>();
                // Only structs content is actually parsed into. A plain runtime
                // struct has nothing to be strict about.
                if !attributes.iter().any(|line| line.contains("Deserialize")) {
                    continue;
                }
                checked += 1;
                if !attributes
                    .iter()
                    .any(|line| line.contains("deny_unknown_fields"))
                {
                    lenient.push(format!("{name} in {path}"));
                }
            }
        }

        lenient.sort();
        // A source-scanning guard fails open: rename a file or change how a
        // struct is declared and it quietly checks nothing while still passing.
        // There were forty-odd content structs when this was written.
        assert!(
            checked >= 35,
            "the sweep only found {checked} content structs, so it is not looking where it thinks"
        );
        assert!(
            lenient.is_empty(),
            "content schemas that would silently swallow a key nobody reads:
{lenient:#?}"
        );
    }

    /// And the proof that the attribute does what the sweep assumes. A guard
    /// over source text is only as good as its belief about what the text
    /// means, so this drives a real content file through the real loader with
    /// one extra key in it and asserts the loader refuses it.
    #[test]
    fn a_key_nothing_reads_is_now_a_load_failure() {
        use crate::data::ItemDefinition;

        let good = r#"{
            "id": "test_herb",
            "name": "Test Herb",
            "category": "ingredient",
            "base_value": 5,
            "color": [1, 2, 3, 4],
            "description": "A herb that exists only in this test.",
            "quality": 20
        }"#;
        assert!(
            serde_json::from_str::<ItemDefinition>(good).is_ok(),
            "the control case has to parse, or this proves nothing"
        );

        let typo = good.replace("\"quality\": 20", "\"qualtiy\": 20");
        let error = serde_json::from_str::<ItemDefinition>(&typo)
            .err()
            .map(|error| error.to_string())
            .unwrap_or_default();
        assert!(
            error.contains("qualtiy"),
            "a misspelled key should name itself in the failure, got: {error:?}"
        );
    }

    /// Every content file the game embeds has to parse *strictly*, here, rather
    /// than at runtime. Two of these load through `parse_json_or_else`, which
    /// falls back to placeholder copy and carries on — so without this the
    /// strictness above would turn a typo into a game where every line of text
    /// reads "[missing ...]" instead of into a red test.
    #[test]
    fn every_embedded_content_file_still_parses() {
        crate::data::load_embedded().expect("the game data should load");
        // Touching these forces the lazy parses, which panic or fall back.
        let ui = crate::content::ui_copy("hud_vitality_label");
        assert!(
            !ui.starts_with("[missing"),
            "ui_text.json failed to parse and fell back to placeholders"
        );
        let narrative = crate::content::narrative_text();
        assert!(
            !narrative.epilogue_beats.is_empty(),
            "narrative_text.json failed to parse"
        );
        assert!(
            !narrative.reactions.is_empty(),
            "the narrative reaction files failed to parse"
        );
    }

    /// Every `.rs` file under `src`, so the sweep cannot miss a schema by
    /// living somewhere nobody thought of. The struct that started all this —
    /// `UiArtCatalog` — is in `src/art`, not in `src/data`, and a hand-written
    /// list of "the schema files" would have skipped exactly it.
    fn rust_sources() -> Vec<String> {
        fn walk(directory: &std::path::Path, found: &mut Vec<String>) {
            let Ok(entries) = std::fs::read_dir(directory) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, found);
                } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                    found.push(path.to_string_lossy().replace('\\', "/"));
                }
            }
        }
        let mut found = Vec::new();
        walk(std::path::Path::new("src"), &mut found);
        found.sort();
        found
    }

    /// The name on a `struct Foo {` line, ignoring visibility.
    fn struct_name(line: &str) -> Option<&str> {
        let line = line.trim();
        let rest = line
            .strip_prefix("pub(crate) struct ")
            .or_else(|| line.strip_prefix("pub(super) struct "))
            .or_else(|| line.strip_prefix("struct "))?;
        let name = rest
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .next()?;
        (!name.is_empty()).then_some(name)
    }
}
