use super::GameplayState;
use crate::data::GameData;
use std::collections::BTreeMap;

impl GameplayState {
    pub(super) fn area_path(
        &self,
        data: &GameData,
        start_area_id: &str,
        target_area_id: &str,
    ) -> Option<Vec<String>> {
        let mut frontier = vec![start_area_id.to_owned()];
        let mut came_from = BTreeMap::<String, (String, String)>::new();
        let mut index = 0;

        while index < frontier.len() {
            let area_id = frontier[index].clone();
            index += 1;
            if area_id == target_area_id {
                break;
            }
            let area = data.area(&area_id)?;
            for warp in &area.warps {
                if !came_from.contains_key(&warp.target_area) && warp.target_area != start_area_id {
                    came_from.insert(warp.target_area.clone(), (area_id.clone(), warp.id.clone()));
                    frontier.push(warp.target_area.clone());
                }
            }
        }

        if start_area_id != target_area_id && !came_from.contains_key(target_area_id) {
            return None;
        }

        let mut path = Vec::<String>::new();
        let mut current = target_area_id.to_owned();
        while current != start_area_id {
            let (previous_area, warp_id) = came_from.get(&current)?.clone();
            path.push(warp_id);
            current = previous_area;
        }
        path.reverse();
        Some(path)
    }
}
