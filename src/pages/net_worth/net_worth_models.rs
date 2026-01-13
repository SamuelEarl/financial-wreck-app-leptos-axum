use serde::{Deserialize, Serialize}; // Assuming you need this for DB/Frontend
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetCategory {
    BankAccount,
    RetirementInvestment,
    NonRetirementInvestment,
    CertificateOfDeposit,
    CashValueOfLifeInsurance,
    Annuity,
    Pension,
    HealthSavingsAccount,
    Cryptocurrency,
    CashOnHand,
    RealEstate,
    Vehicle,
    PersonalItem,
    Business,
    MoneyOwedToYou,
    OtherAsset,
}

impl AssetCategory {
    /// Returns the database/internal identifier for option_value.
    pub fn option_value(&self) -> &'static str {
        match self {
            Self::BankAccount => "bank_account",
            Self::RetirementInvestment => "retirement_investment",
            Self::NonRetirementInvestment => "non_retirement_investment",
            Self::CertificateOfDeposit => "cod",
            Self::CashValueOfLifeInsurance => "cash_value",
            Self::Annuity => "annuity",
            Self::Pension => "pension",
            Self::HealthSavingsAccount => "hsa",
            Self::Cryptocurrency => "cryptocurrency",
            Self::CashOnHand => "cash",
            Self::RealEstate => "real_estate",
            Self::Vehicle => "vehicle",
            Self::PersonalItem => "personal_item",
            Self::Business => "business",
            Self::MoneyOwedToYou => "money_owed_to_you",
            Self::OtherAsset => "other_asset",
        }
    }

    /// Returns the human-readable text for `option_text`.
    pub fn option_text(&self) -> &'static str {
        match self {
            Self::BankAccount => "Bank Account (checking, savings)",
            Self::RetirementInvestment => "Retirement Investment (401k, IRA)",
            Self::NonRetirementInvestment => "Non-Retirement Investment (mutual funds, stocks, bonds)",
            Self::CertificateOfDeposit => "Certificate of Deposit",
            Self::CashValueOfLifeInsurance => "Cash Value of Life Insurance",
            Self::Annuity => "Annuity",
            Self::Pension => "Pension",
            Self::HealthSavingsAccount => "Health Savings Account (HSA)",
            Self::Cryptocurrency => "Cryptocurrency",
            Self::CashOnHand => "Cash On-Hand",
            Self::RealEstate => "Real Estate",
            Self::Vehicle => "Vehicle",
            Self::PersonalItem => "Personal Item",
            Self::Business => "A Business (your portion only)",
            Self::MoneyOwedToYou => "Money Owed To You",
            Self::OtherAsset => "Other Asset",
        }
    }

    pub fn all() -> &'static [AssetCategory] {
        &[
            Self::BankAccount,
            Self::RetirementInvestment,
            Self::NonRetirementInvestment,
            Self::CertificateOfDeposit,
            Self::CashValueOfLifeInsurance,
            Self::Annuity,
            Self::Pension,
            Self::HealthSavingsAccount,
            Self::Cryptocurrency,
            Self::CashOnHand,
            Self::RealEstate,
            Self::Vehicle,
            Self::PersonalItem,
            Self::Business,
            Self::MoneyOwedToYou,
            Self::OtherAsset,
        ]
    }
}

/// Easy Conversion from String
/// If you are reading from a database and getting the string "bank_account", you need a way to turn that back into the Enum.
impl FromStr for AssetCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bank_account" => Ok(Self::BankAccount),
            "retirement_investment" => Ok(Self::RetirementInvestment),
            "non_retirement_investment" => Ok(Self::NonRetirementInvestment),
            "cod" => Ok(Self::CertificateOfDeposit),
            "cash_value" => Ok(Self::CashValueOfLifeInsurance),
            "annuity" => Ok(Self::Annuity),
            "pension" => Ok(Self::Pension),
            "hsa" => Ok(Self::HealthSavingsAccount),
            "cryptocurrency" => Ok(Self::Cryptocurrency),
            "cash" => Ok(Self::CashOnHand),
            "real_estate" => Ok(Self::RealEstate),
            "vehicle" => Ok(Self::Vehicle),
            "personal_item" => Ok(Self::PersonalItem),
            "business" => Ok(Self::Business),
            "money_owed_to_you" => Ok(Self::MoneyOwedToYou),
            "other_asset" => Ok(Self::OtherAsset),
            _ => Err(()),
        }
    }
}
