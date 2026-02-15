use crate::core::models::Weekday;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DayCombination {
    pub days: Vec<Weekday>,
}

pub struct ScheduleGenerator {
    pub combinations: HashMap<i32, Vec<DayCombination>>,
}

impl ScheduleGenerator {
    pub fn new() -> Self {
        let mut generator = Self {
            combinations: HashMap::new(),
        };
        generator.initialize_combinations();
        generator
    }

    fn initialize_combinations(&mut self) {
        // 1-day combinations
        self.combinations.insert(1, vec![
            DayCombination { days: vec![Weekday::Monday] },
            DayCombination { days: vec![Weekday::Tuesday] },
            DayCombination { days: vec![Weekday::Wednesday] },
            DayCombination { days: vec![Weekday::Thursday] },
            DayCombination { days: vec![Weekday::Friday] },
        ]);

        // 2-day combinations
        self.combinations.insert(2, vec![
            DayCombination { days: vec![Weekday::Monday, Weekday::Wednesday] },
            DayCombination { days: vec![Weekday::Monday, Weekday::Thursday] },
            DayCombination { days: vec![Weekday::Monday, Weekday::Friday] },
            DayCombination { days: vec![Weekday::Tuesday, Weekday::Thursday] },
            DayCombination { days: vec![Weekday::Tuesday, Weekday::Friday] },
            DayCombination { days: vec![Weekday::Wednesday, Weekday::Friday] },
        ]);

        // 3-day combinations
        self.combinations.insert(3, vec![
            DayCombination { days: vec![Weekday::Monday, Weekday::Wednesday, Weekday::Friday] },
        ]);

        // 5-day combinations
        self.combinations.insert(5, vec![
            DayCombination { days: vec![
                Weekday::Monday,
                Weekday::Tuesday,
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Friday,
            ] },
        ]);
    }

    pub fn get_combinations(&self, days: i32) -> Option<&Vec<DayCombination>> {
        self.combinations.get(&days)
    }
}

pub fn shuffle_combinations(combos: &[DayCombination]) -> Vec<DayCombination> {
    let mut shuffled = combos.to_vec();
    let mut rng = thread_rng();
    shuffled.shuffle(&mut rng);
    shuffled
}
