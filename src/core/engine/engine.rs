use crate::core::models::{Employee, MonthlySchedule, PastSchedules, PastSchedulesExt, Weekday};
use crate::core::engine::generator::{ScheduleGenerator, shuffle_combinations, DayCombination};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

pub struct Engine {
    generator: ScheduleGenerator,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            generator: ScheduleGenerator::new(),
        }
    }

    pub fn generate_schedule(
        &self,
        employees: &[Employee],
        past_schedules: &PastSchedules,
        year: i32,
        month: u32,
    ) -> MonthlySchedule {
        let mut schedule = MonthlySchedule::new(year, month);
        
        // Track included employees
        schedule.included_employees = employees.iter().map(|e| e.id).collect();
        
        let mut day_counts = HashMap::new();
        
        for day in Weekday::all() {
            day_counts.insert(day, 0);
        }

        // Process fixed schedules
        let (mut flexible_emps, fixed_emps) = self.process_fixed_schedules(employees, &mut schedule, &mut day_counts);

        // Build mentor schedules map
        let mentor_schedules = self.build_mentor_schedules(&fixed_emps, &schedule);

        // Group flexible employees by required days
        let grouped = self.group_by_required_days(&mut flexible_emps);

        // Process flexible employees
        self.process_flexible_employees(
            grouped,
            &mut schedule,
            &mut day_counts,
            past_schedules,
            &mentor_schedules,
            employees,
        );

        schedule
    }

    fn build_mentor_schedules(&self, employees: &[Employee], schedule: &MonthlySchedule) -> HashMap<i32, Vec<Weekday>> {
        let mut mentor_schedules = HashMap::new();
        for emp in employees {
            if emp.is_mentor {
                let days = schedule.get_days_for_employee(emp.id);
                mentor_schedules.insert(emp.id, days);
            }
        }
        mentor_schedules
    }

    fn process_fixed_schedules(
        &self,
        employees: &[Employee],
        schedule: &mut MonthlySchedule,
        day_counts: &mut HashMap<Weekday, usize>,
    ) -> (Vec<Employee>, Vec<Employee>) {
        let mut flexible = Vec::new();
        let mut fully_scheduled = Vec::new();

        for emp in employees {
            // Apply fixed days regardless of whether they need more days
            if !emp.fixed_days.is_empty() {
                for &day in &emp.fixed_days {
                    schedule.add_employee(day, emp.clone());
                    *day_counts.entry(day).or_default() += 1;
                }
            }

            // If they need more days than fixed, they are flexible for the remainder
            if emp.required_days as usize > emp.fixed_days.len() {
                flexible.push(emp.clone());
            } else {
                fully_scheduled.push(emp.clone());
            }
        }

        (flexible, fully_scheduled)
    }

    fn group_by_required_days(&self, employees: &mut [Employee]) -> HashMap<i32, Vec<Employee>> {
        let mut grouped: HashMap<i32, Vec<Employee>> = HashMap::new();

        for emp in employees.iter() {
            let remaining = emp.required_days - emp.fixed_days.len() as i32;
            if remaining > 0 {
                grouped.entry(remaining).or_default().push(emp.clone());
            }
        }

        let mut rng = thread_rng();
        for emps in grouped.values_mut() {
            emps.shuffle(&mut rng);
        }

        grouped
    }

    fn process_flexible_employees(
        &self,
        grouped: HashMap<i32, Vec<Employee>>,
        schedule: &mut MonthlySchedule,
        day_counts: &mut HashMap<Weekday, usize>,
        past_schedules: &PastSchedules,
        mentor_schedules: &HashMap<i32, Vec<Weekday>>,
        all_employees: &[Employee],
    ) {
        let mut keys: Vec<i32> = grouped.keys().cloned().collect();
        keys.sort_by(|a, b| b.cmp(a)); // Reverse sort

        for num_days in keys {
            if let Some(employees) = grouped.get(&num_days) {
                if let Some(combinations) = self.generator.get_combinations(num_days) {
                    for emp in employees {
                        let mut required_days = Vec::new();
                        if emp.is_mentee {
                            if let Some(mentor_id) = emp.mentor_id {
                                if let Some(mentor) = all_employees.iter().find(|e| e.id == mentor_id) {
                                    if mentor.required_days > 0 {
                                        if let Some(mentor_days) = mentor_schedules.get(&mentor_id) {
                                            required_days = mentor_days.clone();
                                        }
                                    }
                                }
                            }
                        }

                        // Filter out combinations that overlap with fixed days
                        let valid_combinations: Vec<DayCombination> = combinations
                            .iter()
                            .filter(|combo| !combo.days.iter().any(|d| emp.fixed_days.contains(d)))
                            .cloned()
                            .collect();

                        if valid_combinations.is_empty() {
                            continue;
                        }

                        let best_combo = self.find_best_combination(
                            &valid_combinations,
                            day_counts,
                            emp,
                            past_schedules,
                            &required_days,
                        );

                        for &day in &best_combo.days {
                            schedule.add_employee(day, emp.clone());
                            *day_counts.entry(day).or_default() += 1;
                        }
                    }
                }
            }
        }
    }

    fn find_best_combination(
        &self,
        combinations: &[DayCombination],
        day_counts: &HashMap<Weekday, usize>,
        employee: &Employee,
        past_schedules: &PastSchedules,
        required_days: &[Weekday],
    ) -> DayCombination {
        let mut valid_combos: Vec<DayCombination> = if !required_days.is_empty() {
            let filtered = self.filter_combinations_by_required_days(combinations, required_days);
            if filtered.is_empty() {
                combinations.to_vec()
            } else {
                filtered
            }
        } else {
            combinations.to_vec()
        };

        let shuffled = shuffle_combinations(&valid_combos);
        let mut best_combo = shuffled[0].clone();
        let mut min_score = f64::MAX;

        let past_day_freq = self.calculate_past_day_frequencies(employee.id, past_schedules);

        for combo in shuffled {
            let mut temp_counts = day_counts.clone();
            for &day in &combo.days {
                *temp_counts.entry(day).or_default() += 1;
            }

            let variance = self.calculate_variance(&temp_counts);

            let mut repetition_score = 0.0;
            for &day in &combo.days {
                repetition_score += past_day_freq.get(&day).cloned().unwrap_or(0.0);
            }

            let total_score = variance + (3.0 * repetition_score);

            if total_score < min_score {
                min_score = total_score;
                best_combo = combo;
            }
        }

        best_combo
    }

    fn filter_combinations_by_required_days(
        &self,
        combinations: &[DayCombination],
        required_days: &[Weekday],
    ) -> Vec<DayCombination> {
        combinations
            .iter()
            .filter(|combo| {
                required_days.iter().all(|req_day| combo.days.contains(req_day))
            })
            .cloned()
            .collect()
    }

    fn calculate_past_day_frequencies(
        &self,
        employee_id: i32,
        past_schedules: &PastSchedules,
    ) -> HashMap<Weekday, f64> {
        let mut frequencies = HashMap::new();
        for day in Weekday::all() {
            frequencies.insert(day, 0.0);
        }

        let schedules = past_schedules.get_employee_past_schedules(employee_id, 2);

        for (i, schedule) in schedules.iter().enumerate() {
            let weight = 1.0 - (i as f64 / schedules.len() as f64 * 0.75);

            for (day, worked) in schedule {
                if *worked {
                    *frequencies.entry(*day).or_default() += weight;
                }
            }
        }

        frequencies
    }

    fn calculate_variance(&self, counts: &HashMap<Weekday, usize>) -> f64 {
        let values: Vec<usize> = counts.values().cloned().collect();
        if values.is_empty() {
            return 0.0;
        }

        let sum: usize = values.iter().sum();
        let mean = sum as f64 / values.len() as f64;

        let mut variance = 0.0;
        for &v in &values {
            let diff = v as f64 - mean;
            variance += diff * diff;
        }

        variance
    }
}
