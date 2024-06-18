use chrono::NaiveDateTime;

// TODO: Looks like fucking shit, need research and think is it ok way
pub enum Currency {
    Rub,
    Usd,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::Rub => "Rub",
            Currency::Usd => "Usd",
        }
    }
}

pub enum IncomeCategory {
    Salary,
    Bonuses,
    Investments,
    PassiveIncome,
    Gifts,
    SaleOfAssets,
    Other,
}

impl IncomeCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            IncomeCategory::Salary => "Salary",
            IncomeCategory::Bonuses => "Bonuses",
            IncomeCategory::Investments => "Investments",
            IncomeCategory::PassiveIncome => "PassiveIncome",
            IncomeCategory::Gifts => "Gifts",
            IncomeCategory::SaleOfAssets => "SaleOfAssets",
            IncomeCategory::Other => "Other",
        }
    }
}

pub enum ExpenseCategory {
    GroceriesAndFood,
    Housing,
    Utilities,
    Transportation,
    HealthAndMedical,
    ClothingAndFootwear,
    EntertainmentAndLeisure,
    Education,
    PersonalExpenses,
    Travel,
    GiftsAndDonations,
    Pets,
    DebtsAndLoans,
    ElectronicsAndGadgets,
    Other,
}

impl ExpenseCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExpenseCategory::GroceriesAndFood => "GroceriesAndFood",
            ExpenseCategory::Housing => "Housing",
            ExpenseCategory::Utilities => "Utilities",
            ExpenseCategory::Transportation => "Transportation",
            ExpenseCategory::HealthAndMedical => "HealthAndMedical",
            ExpenseCategory::ClothingAndFootwear => "ClothingAndFootwear",
            ExpenseCategory::EntertainmentAndLeisure => "EntertainmentAndLeisure",
            ExpenseCategory::Education => "Education",
            ExpenseCategory::PersonalExpenses => "PersonalExpenses",
            ExpenseCategory::Travel => "Travel",
            ExpenseCategory::GiftsAndDonations => "GiftsAndDonations",
            ExpenseCategory::Pets => "Pets",
            ExpenseCategory::DebtsAndLoans => "DebtsAndLoans",
            ExpenseCategory::ElectronicsAndGadgets => "ElectronicsAndGadgets",
            ExpenseCategory::Other => "Other",
        }
    }
}

pub struct Expense {
    pub amount: f32,
    pub currency: Currency,
    pub category: ExpenseCategory,
    pub description: String,
    pub date: NaiveDateTime,
}

pub struct Income {
    pub amount: f32,
    pub currency: Currency,
    pub category: IncomeCategory,
    pub description: String,
    pub date: NaiveDateTime,
}
