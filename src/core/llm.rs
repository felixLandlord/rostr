use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const GROQ_BASE_URL: &str = "https://api.groq.com/openai/v1";

#[derive(Debug, Clone)]
pub struct LlmClient {
    client: Client,
    api_key: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    stream: bool,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: i32,
    seed: i32,
    top_p: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    delta: Option<Delta>,
    message: Option<Message>,
}

#[derive(Debug, Deserialize)]
struct Delta {
    content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleStats {
    pub date: String,
    pub total_employees: usize,
    pub total_males: usize,
    pub total_females: usize,
    pub males_per_day: Vec<usize>,
    pub females_per_day: Vec<usize>,
    pub roles_per_day: Vec<HashMap<String, usize>>,
    pub day_most_attendance: String,
    pub day_least_attendance: String,
    pub daily_remote_percentage: Vec<f32>,
    pub daily_office_percentage: Vec<f32>,
    pub office_utilization_per_day: Vec<f32>,
    pub office_seat_capacity: usize,
}

impl LlmClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub fn stream_report(
        &self,
        stats: ScheduleStats,
    ) -> impl futures_util::Stream<Item = Result<String, String>> + '_ {
        let prompt = build_report_prompt(&stats);

        let request = ChatRequest {
            model: "openai/gpt-oss-120b".to_string(),
            stream: true,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.1,
            max_tokens: 4096,
            seed: 0,
            top_p: 1.0,
        };

        let client = self.client.clone();
        let url = format!("{}/chat/completions", GROQ_BASE_URL);

        async_stream::stream! {
            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&request)
                .send()
                .await
                .map_err(|e| format!("API request failed: {}", e))?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                yield Err(format!("API error {}: {}", status, body));
                return;
            }

            let mut stream = response.bytes_stream();
            let mut buffer = String::new();

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                            buffer.push_str(&text);

                            while let Some(newline_idx) = buffer.find('\n') {
                                let line = buffer.drain(..=newline_idx).collect::<String>();
                                if line.starts_with("data: ") {
                                    let data = line.strip_prefix("data: ").unwrap_or(&line);

                                    if data == "[DONE]" {
                                        return;
                                    }

                                    if let Ok(response) = serde_json::from_str::<ChatResponse>(data) {
                                        for choice in response.choices {
                                            if let Some(delta) = choice.delta {
                                                if let Some(content) = delta.content {
                                                    yield Ok(content);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        yield Err(format!("Stream error: {}", e));
                        return;
                    }
                }
            }
        }
    }

    pub async fn generate_report_completion(&self, stats: ScheduleStats) -> Result<String, String> {
        let prompt = build_report_prompt(&stats);

        let request = ChatRequest {
            model: "openai/gpt-oss-120b".to_string(),
            stream: false,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.1,
            max_tokens: 4096,
            seed: 0,
            top_p: 1.0,
        };

        let url = format!("{}/chat/completions", GROQ_BASE_URL);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("API request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("API error {}: {}", status, body));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if let Some(choice) = chat_response.choices.first() {
            if let Some(message) = &choice.message {
                return Ok(message.content.clone());
            }
        }

        Err("No response content".to_string())
    }
}

fn build_report_prompt(stats: &ScheduleStats) -> String {
    let mon_office = stats
        .office_utilization_per_day
        .get(0)
        .map(|&v| (v / 100.0 * stats.office_seat_capacity as f32) as usize)
        .unwrap_or(0);
    let mon_office_pct = format!(
        "{:.1}",
        stats.daily_office_percentage.get(0).copied().unwrap_or(0.0)
    );
    let mon_remote = stats
        .daily_remote_percentage
        .get(0)
        .map(|&v| (v / 100.0 * stats.total_employees as f32) as usize)
        .unwrap_or(0);
    let mon_remote_pct = format!(
        "{:.1}",
        stats.daily_remote_percentage.get(0).copied().unwrap_or(0.0)
    );
    let mon_util = format!(
        "{:.1}",
        stats
            .office_utilization_per_day
            .get(0)
            .copied()
            .unwrap_or(0.0)
    );

    let tue_office = stats
        .office_utilization_per_day
        .get(1)
        .map(|&v| (v / 100.0 * stats.office_seat_capacity as f32) as usize)
        .unwrap_or(0);
    let tue_office_pct = format!(
        "{:.1}",
        stats.daily_office_percentage.get(1).copied().unwrap_or(0.0)
    );
    let tue_remote = stats
        .daily_remote_percentage
        .get(1)
        .map(|&v| (v / 100.0 * stats.total_employees as f32) as usize)
        .unwrap_or(0);
    let tue_remote_pct = format!(
        "{:.1}",
        stats.daily_remote_percentage.get(1).copied().unwrap_or(0.0)
    );
    let tue_util = format!(
        "{:.1}",
        stats
            .office_utilization_per_day
            .get(1)
            .copied()
            .unwrap_or(0.0)
    );

    let wed_office = stats
        .office_utilization_per_day
        .get(2)
        .map(|&v| (v / 100.0 * stats.office_seat_capacity as f32) as usize)
        .unwrap_or(0);
    let wed_office_pct = format!(
        "{:.1}",
        stats.daily_office_percentage.get(2).copied().unwrap_or(0.0)
    );
    let wed_remote = stats
        .daily_remote_percentage
        .get(2)
        .map(|&v| (v / 100.0 * stats.total_employees as f32) as usize)
        .unwrap_or(0);
    let wed_remote_pct = format!(
        "{:.1}",
        stats.daily_remote_percentage.get(2).copied().unwrap_or(0.0)
    );
    let wed_util = format!(
        "{:.1}",
        stats
            .office_utilization_per_day
            .get(2)
            .copied()
            .unwrap_or(0.0)
    );

    let thu_office = stats
        .office_utilization_per_day
        .get(3)
        .map(|&v| (v / 100.0 * stats.office_seat_capacity as f32) as usize)
        .unwrap_or(0);
    let thu_office_pct = format!(
        "{:.1}",
        stats.daily_office_percentage.get(3).copied().unwrap_or(0.0)
    );
    let thu_remote = stats
        .daily_remote_percentage
        .get(3)
        .map(|&v| (v / 100.0 * stats.total_employees as f32) as usize)
        .unwrap_or(0);
    let thu_remote_pct = format!(
        "{:.1}",
        stats.daily_remote_percentage.get(3).copied().unwrap_or(0.0)
    );
    let thu_util = format!(
        "{:.1}",
        stats
            .office_utilization_per_day
            .get(3)
            .copied()
            .unwrap_or(0.0)
    );

    let fri_office = stats
        .office_utilization_per_day
        .get(4)
        .map(|&v| (v / 100.0 * stats.office_seat_capacity as f32) as usize)
        .unwrap_or(0);
    let fri_office_pct = format!(
        "{:.1}",
        stats.daily_office_percentage.get(4).copied().unwrap_or(0.0)
    );
    let fri_remote = stats
        .daily_remote_percentage
        .get(4)
        .map(|&v| (v / 100.0 * stats.total_employees as f32) as usize)
        .unwrap_or(0);
    let fri_remote_pct = format!(
        "{:.1}",
        stats.daily_remote_percentage.get(4).copied().unwrap_or(0.0)
    );
    let fri_util = format!(
        "{:.1}",
        stats
            .office_utilization_per_day
            .get(4)
            .copied()
            .unwrap_or(0.0)
    );

    let mon_m = stats.males_per_day.get(0).copied().unwrap_or(0);
    let mon_f = stats.females_per_day.get(0).copied().unwrap_or(0);
    let tue_m = stats.males_per_day.get(1).copied().unwrap_or(0);
    let tue_f = stats.females_per_day.get(1).copied().unwrap_or(0);
    let wed_m = stats.males_per_day.get(2).copied().unwrap_or(0);
    let wed_f = stats.females_per_day.get(2).copied().unwrap_or(0);
    let thu_m = stats.males_per_day.get(3).copied().unwrap_or(0);
    let thu_f = stats.females_per_day.get(3).copied().unwrap_or(0);
    let fri_m = stats.males_per_day.get(4).copied().unwrap_or(0);
    let fri_f = stats.females_per_day.get(4).copied().unwrap_or(0);

    let roles = stats
        .roles_per_day
        .iter()
        .enumerate()
        .map(|(i, map)| {
            let day = match i {
                0 => "Monday",
                1 => "Tuesday",
                2 => "Wednesday",
                3 => "Thursday",
                4 => "Friday",
                _ => "Unknown",
            };
            if map.is_empty() {
                format!("- {}: No data", day)
            } else {
                let roles_str = map
                    .iter()
                    .map(|(role, count)| format!("{}: {}", role, count))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("- {}: {}", day, roles_str)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"You are a professional HR analyst writing a detailed monthly attendance report. Generate a comprehensive report in GitHub-flavored Markdown for the schedule of {}.

Use the following data to generate insights and analysis:

## Schedule Data
- Total Employees: {}
- Gender Distribution: {} Males, {} Females
- Office Seat Capacity: {} seats
- Peak Attendance Day: {}
- Lowest Attendance Day: {}

## Daily Breakdown
### Monday
- Office: {} ({}%)
- Remote: {} ({}%)
- Utilization: {}%

### Tuesday
- Office: {} ({}%)
- Remote: {} ({}%)
- Utilization: {}%

### Wednesday
- Office: {} ({}%)
- Remote: {} ({}%)
- Utilization: {}%

### Thursday
- Office: {} ({}%)
- Remote: {} ({}%)
- Utilization: {}%

### Friday
- Office: {} ({}%)
- Remote: {} ({}%)
- Utilization: {}%

## Gender Distribution by Day
- Monday: {} Males, {} Females
- Tuesday: {} Males, {} Females
- Wednesday: {} Males, {} Females
- Thursday: {} Males, {} Females
- Friday: {} Males, {} Females

## Role Distribution
{}

Please write a comprehensive report that includes:
1. A title and executive summary
2. Key insights about attendance patterns
3. Analysis of office utilization trends
4. Gender distribution insights
5. Overall observations and conclusions

Use professional markdown formatting with headers, bullet points, and tables where appropriate. Make it detailed and data-driven."#,
        stats.date,
        stats.total_employees,
        stats.total_males,
        stats.total_females,
        stats.office_seat_capacity,
        stats.day_most_attendance,
        stats.day_least_attendance,
        mon_office,
        mon_office_pct,
        mon_remote,
        mon_remote_pct,
        mon_util,
        tue_office,
        tue_office_pct,
        tue_remote,
        tue_remote_pct,
        tue_util,
        wed_office,
        wed_office_pct,
        wed_remote,
        wed_remote_pct,
        wed_util,
        thu_office,
        thu_office_pct,
        thu_remote,
        thu_remote_pct,
        thu_util,
        fri_office,
        fri_office_pct,
        fri_remote,
        fri_remote_pct,
        fri_util,
        mon_m,
        mon_f,
        tue_m,
        tue_f,
        wed_m,
        wed_f,
        thu_m,
        thu_f,
        fri_m,
        fri_f,
        roles
    )
}
