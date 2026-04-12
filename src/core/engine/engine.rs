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

        // Group flexible employees by required days
        let mut grouped = self.group_by_required_days(&mut flexible_emps);

        // 1. Process Mentors first
        // We need to extract mentors from grouped map to process them first
        let mut mentors_grouped = HashMap::new();
        for (days, emps) in grouped.iter_mut() {
            let (mentors, others): (Vec<Employee>, Vec<Employee>) = emps.drain(..).partition(|e| e.is_mentor);
            if !mentors.is_empty() {
                mentors_grouped.insert(*days, mentors);
            }
            *emps = others;
        }

        let mentor_schedules = HashMap::new(); // Initial empty map for mentors (they don't depend on anyone usually)
        
        // Process Mentors
        self.process_flexible_employees(
            mentors_grouped,
            &mut schedule,
            &mut day_counts,
            past_schedules,
            &mentor_schedules,
            employees,
        );

        // Re-build mentor schedules map (now including flexible days we just assigned)
        let mut full_mentor_schedules = self.build_mentor_schedules(&fixed_emps, &schedule);
        // Also add flexible mentors we just processed
        // We can just scan the schedule for all mentors
        for day in Weekday::all() {
            let emps = schedule.get_employees_for_day(day);
            for emp in emps {
                if emp.is_mentor {
                    full_mentor_schedules.entry(emp.id).or_default().push(day);
                }
            }
        }
        
        // 2. Process Mentees (who depend on mentors)
        let mut mentees_grouped = HashMap::new();
        for (days, emps) in grouped.iter_mut() {
            let (mentees, others): (Vec<Employee>, Vec<Employee>) = emps.drain(..).partition(|e| e.is_mentee);
            if !mentees.is_empty() {
                mentees_grouped.insert(*days, mentees);
            }
            *emps = others;
        }

        self.process_flexible_employees(
            mentees_grouped,
            &mut schedule,
            &mut day_counts,
            past_schedules,
            &full_mentor_schedules,
            employees,
        );

        // 3. Process remaining employees
        self.process_flexible_employees(
            grouped,
            &mut schedule,
            &mut day_counts,
            past_schedules,
            &full_mentor_schedules, // Doesn't matter for them
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
                        let mut mentor_days = Vec::new();
                        if emp.is_mentee {
                            if let Some(mentor_id) = emp.mentor_id {
                                // Find mentor in all_employees to check their ID
                                // Wait, we have mentor_schedules which is map<ID, Days>.
                                if let Some(days) = mentor_schedules.get(&mentor_id) {
                                    mentor_days = days.clone();
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
                            &mentor_days,
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
        mentor_days: &[Weekday],
    ) -> DayCombination {
        // Filter out combinations that don't match required days length if strict? 
        // No, `combinations` already have correct length (num_days).
        
        let shuffled = shuffle_combinations(combinations);
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
            
            // Mentorship overlap score
            // We want to MINIMIZE score.
            // If we match a mentor day, we subtract from score (reward).
            let mut mentorship_score = 0.0;
            if !mentor_days.is_empty() {
                let matches = combo.days.iter().filter(|d| mentor_days.contains(d)).count();
                // Reward for each match. 
                // Weight should be significant to prioritize overlap.
                mentorship_score = -(matches as f64 * 10.0); 
            }

            let total_score = variance + (3.0 * repetition_score) + mentorship_score;

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
