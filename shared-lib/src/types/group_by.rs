use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GroupBy {
    Day,
    Month,
    Year,
}

impl GroupBy {
    pub const fn as_sql_select_str(&self) -> &str {
        match self {
            Self::Day => {
                "CAST(YEAR(tt.timestamp) AS UNSIGNED INTEGER) AS year,
            MONTH(tt.timestamp) AS month,
            DAYOFMONTH(tt.timestamp) AS day"
            }

            Self::Month => {
                "CAST(YEAR(tt.timestamp) AS UNSIGNED INTEGER) AS year,
            MONTH(tt.timestamp) AS month,
            1 AS day"
            }
            Self::Year => {
                "CAST(YEAR(tt.timestamp) AS UNSIGNED INTEGER) AS year,
            1 AS month,
            1 AS day"
            }
        }
    }

    pub const fn as_sql_group_by_str(&self) -> &str {
        match self {
            Self::Day => "year, month, day",
            Self::Month => "year, month",
            Self::Year => "year",
        }
    }
}
