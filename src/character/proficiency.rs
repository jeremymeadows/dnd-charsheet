#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum Proficiency {
    None,
    Proficient,
    Expert,
}

impl Default for Proficiency {
    fn default() -> Self {
        Self::None
    }
}

impl std::str::FromStr for Proficiency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "proficient" => Ok(Self::Proficient),
            "expert" => Ok(Self::Expert),
            _ => Err(format!("Invalid proficiency: {}", s)),
        }
    }
}

impl std::ops::Add for Proficiency {
    type Output = Self;

    fn add(self, other: Proficiency) -> Self {
        match self {
            Self::None => other,
            Self::Proficient => match other {
                Self::None | Self::Proficient => Self::Proficient,
                Self::Expert => Self::Expert,
            },
            Self::Expert => Self::Expert,
        }
    }
}

impl std::ops::AddAssign for Proficiency {
    fn add_assign(&mut self, other: Proficiency) {
        *self = *self + other;
    }
}

impl Proficiency {
    pub fn value(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Proficient => 1,
            Self::Expert => 2,
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::None => "○",
            Self::Proficient => "⏺",
            Self::Expert => "★",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::None => Self::Proficient,
            Self::Proficient => Self::Expert,
            Self::Expert => Self::None,
        }
    }

    pub fn learn(&mut self) {
        *self = self.next();
    }
}
