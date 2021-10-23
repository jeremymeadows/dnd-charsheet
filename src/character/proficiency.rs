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

impl std::ops::Add for Proficiency {
    type Output = Self;

    fn add(self, other: Proficiency) -> Self {
        use Proficiency::*;

        match self {
            None => other,
            Proficient => match other {
                None | Proficient => Proficient,
                Expert => Expert,
            },
            Expert => Expert,
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
        use Proficiency::*;

        match self {
            None => 0,
            Proficient => 1,
            Expert => 2,
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Proficiency::None => "○",
            Proficiency::Proficient => "⏺",
            Proficiency::Expert => "★",
        }
    }

    pub fn next(&self) -> Self {
        use Proficiency::*;

        match self {
            None => Proficient,
            Proficient => Expert,
            Expert => None,
        }
    }

    pub fn learn(&mut self) {
        *self = self.next();
    }
}
