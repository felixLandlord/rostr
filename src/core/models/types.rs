use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
}

impl Sex {
    pub fn all() -> Vec<Sex> {
        vec![Sex::Male, Sex::Female]
    }
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sex::Male => write!(f, "Male"),
            Sex::Female => write!(f, "Female"),
        }
    }
}

impl FromStr for Sex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "male" => Ok(Sex::Male),
            "female" => Ok(Sex::Female),
            _ => Err(format!("invalid sex: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    HR,
    AiLlmEngineer,
    SocialMediaMarketing,
    ITSupport,
    MLEngineer,
    DataScientist,
    DataAnalyst,
    FullStackEngineer,
    BackendEngineer,
    FrontendEngineer,
    BlockchainEngineer,
    QaEngineer,
    ProjectManager,
    UiUxDesigner,
    MobileEngineer,
    DevOpsEngineer,
    OperationsManager,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Role::HR => "Human Resource Manager",
            Role::AiLlmEngineer => "AI-LLM Engineer",
            Role::SocialMediaMarketing => "Social Media Marketing",
            Role::ITSupport => "IT Support",
            Role::MLEngineer => "Machine Learning Engineer",
            Role::DataScientist => "Data Scientist",
            Role::DataAnalyst => "Data Analyst",
            Role::FullStackEngineer => "Full-stack Engineer",
            Role::BackendEngineer => "Backend Engineer",
            Role::FrontendEngineer => "Frontend Engineer",
            Role::BlockchainEngineer => "Blockchain Engineer",
            Role::QaEngineer => "QA Engineer",
            Role::ProjectManager => "Project Manager",
            Role::UiUxDesigner => "UI/UX Designer",
            Role::MobileEngineer => "Mobile Engineer",
            Role::DevOpsEngineer => "DevOps Engineer",
            Role::OperationsManager => "Operations Manager",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Human Resource Manager" => Ok(Role::HR),
            "AI-LLM Engineer" => Ok(Role::AiLlmEngineer),
            "Social Media Marketing" => Ok(Role::SocialMediaMarketing),
            "IT Support" => Ok(Role::ITSupport),
            "Machine Learning Engineer" => Ok(Role::MLEngineer),
            "Data Scientist" => Ok(Role::DataScientist),
            "Data Analyst" => Ok(Role::DataAnalyst),
            "Full-stack Engineer" => Ok(Role::FullStackEngineer),
            "Backend Engineer" => Ok(Role::BackendEngineer),
            "Frontend Engineer" => Ok(Role::FrontendEngineer),
            "Blockchain Engineer" => Ok(Role::BlockchainEngineer),
            "QA Engineer" => Ok(Role::QaEngineer),
            "Project Manager" => Ok(Role::ProjectManager),
            "UI/UX Designer" => Ok(Role::UiUxDesigner),
            "Mobile Engineer" => Ok(Role::MobileEngineer),
            "DevOps Engineer" => Ok(Role::DevOpsEngineer),
            "Operations Manager" => Ok(Role::OperationsManager),
            _ => Err(format!("invalid role: {}", s)),
        }
    }
}

impl Role {
    pub fn all() -> Vec<Role> {
        vec![
            Role::HR,
            Role::AiLlmEngineer,
            Role::SocialMediaMarketing,
            Role::ITSupport,
            Role::MLEngineer,
            Role::DataScientist,
            Role::DataAnalyst,
            Role::FullStackEngineer,
            Role::BackendEngineer,
            Role::FrontendEngineer,
            Role::BlockchainEngineer,
            Role::QaEngineer,
            Role::ProjectManager,
            Role::UiUxDesigner,
            Role::MobileEngineer,
            Role::DevOpsEngineer,
            Role::OperationsManager,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Weekday {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "monday" => Ok(Weekday::Monday),
            "tuesday" => Ok(Weekday::Tuesday),
            "wednesday" => Ok(Weekday::Wednesday),
            "thursday" => Ok(Weekday::Thursday),
            "friday" => Ok(Weekday::Friday),
            _ => Err(format!("invalid weekday: {}", s)),
        }
    }
}

impl Weekday {
    pub fn all() -> Vec<Weekday> {
        vec![
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
        ]
    }
}

pub const DAYS_OPTIONS: &[u8] = &[1, 2, 3, 4];
