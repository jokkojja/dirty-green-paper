use chrono::NaiveDateTime;

enum Currency {
    Rub,
    Usd,
}

enum IncomeCategory {
    Salary,
    Bonuses,
    Investments,
    PassiveIncome,
    Gifts,
    SaleOfAssets,
    Other,
}

enum ExpenseCategory {
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

struct Expense {
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
