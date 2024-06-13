use chrono::NaiveDateTime;

pub enum Currency {
    Rub,
    Usd,
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

pub struct Expense {
    amount: f32,
    currency: Currency,
    category: ExpenseCategory,
    description: String,
    date: NaiveDateTime,
}

pub struct Income {
    amount: f32,
    currency: Currency,
    category: IncomeCategory,
    description: String,
    date: NaiveDateTime,
}
