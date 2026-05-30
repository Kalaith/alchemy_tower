use std::collections::HashMap;

use super::GameData;

impl GameData {
    pub(crate) fn build_indexes(&mut self) -> Result<(), String> {
        self.area_index = build_unique_index(
            self.areas
                .iter()
                .enumerate()
                .map(|(index, area)| (&area.id, index)),
            "area",
        )?;
        self.item_index = build_unique_index(
            self.items
                .iter()
                .enumerate()
                .map(|(index, item)| (&item.id, index)),
            "item",
        )?;
        self.route_index = build_unique_index(
            self.gathering_routes
                .iter()
                .enumerate()
                .map(|(index, route)| (&route.id, index)),
            "route",
        )?;
        self.npc_index = build_unique_index(
            self.npcs
                .iter()
                .enumerate()
                .map(|(index, npc)| (&npc.id, index)),
            "npc",
        )?;
        self.quest_index = build_unique_index(
            self.quests
                .iter()
                .enumerate()
                .map(|(index, quest)| (&quest.id, index)),
            "quest",
        )?;
        self.mutation_formula_index = build_mutation_formula_index(self);
        Ok(())
    }
}

fn build_mutation_formula_index(data: &GameData) -> HashMap<String, Vec<usize>> {
    let mut mutation_formula_index = HashMap::<String, Vec<usize>>::new();
    for (index, formula) in data.mutation_formulas.iter().enumerate() {
        mutation_formula_index
            .entry(formula.seed_item_id.clone())
            .or_default()
            .push(index);
    }
    mutation_formula_index
}

fn build_unique_index<'a, I>(entries: I, kind: &str) -> Result<HashMap<String, usize>, String>
where
    I: IntoIterator<Item = (&'a String, usize)>,
{
    let mut index = HashMap::new();
    let mut duplicates = Vec::new();
    for (id, value) in entries {
        if index.insert(id.clone(), value).is_some() {
            duplicates.push(id.clone());
        }
    }
    if duplicates.is_empty() {
        Ok(index)
    } else {
        duplicates.sort();
        duplicates.dedup();
        Err(format!("Duplicate {kind} id(s): {}", duplicates.join(", ")))
    }
}
