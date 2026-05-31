use crate::content::ui_format;
use std::collections::BTreeSet;

pub(super) fn known_conditions(
    seasons: BTreeSet<String>,
    weathers: BTreeSet<String>,
    times: BTreeSet<String>,
) -> String {
    let option_separator = ui_format("gather_known_condition_option_separator", &[]);
    let mut parts = Vec::new();
    if !seasons.is_empty() {
        let values = join_gathering_options(seasons, &option_separator);
        parts.push(ui_format(
            "gather_known_condition_season",
            &[("values", &values)],
        ));
    }
    if !weathers.is_empty() {
        let values = join_gathering_options(weathers, &option_separator);
        parts.push(ui_format(
            "gather_known_condition_weather",
            &[("values", &values)],
        ));
    }
    if !times.is_empty() {
        let values = join_gathering_options(times, &option_separator);
        parts.push(ui_format(
            "gather_known_condition_time",
            &[("values", &values)],
        ));
    }
    if parts.is_empty() {
        ui_format("gather_known_conditions_none", &[])
    } else {
        let condition_separator = ui_format("gather_known_condition_separator", &[]);
        ui_format(
            "gather_known_conditions",
            &[("conditions", &parts.join(&condition_separator))],
        )
    }
}

fn join_gathering_options(values: BTreeSet<String>, separator: &str) -> String {
    values.into_iter().collect::<Vec<_>>().join(separator)
}
