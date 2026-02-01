use currency_rs::{Currency, CurrencyOpts};

#[derive(Debug, Clone)]
pub struct FmtCurrencyOpts {
    pub symbol: String,
    pub separator: String,
    pub decimal: String,
    pub precision: i64,
    pub pattern: String,
    // pub increment: f64,
    // pub use_vedict: bool,
}

impl Default for FmtCurrencyOpts {
    fn default() -> Self {
        Self {
            symbol: "$".into(),
            separator: ",".into(),
            decimal: ".".into(),
            precision: 2,
            pattern: "!#".into(),
            // increment: 0.5,             // Round to nearest increment. (0.55 becomes 0.50)
            // use_vedict: false,          // Use Indian numbering system. (10,00,000 (Lakhs))
        }
    }
}

/// Helper function to keep view clean
/// Input: Integer that represents the cents of a dollar amount. 
/// (e.g. If I want the output to be $10,000.00, then I would multiply 10,000.00 by 100, which would be 1_000_000).
/// Output: String "$10,000.00"
/// ---
/// symbol:    The currency symbol (default "$")
/// separator: The thousands separator (default ",")
/// decimal:   The decimal separator (default ".")
/// precision: The number of decimal places (default 2)
/// pattern:   The order of symbol vs number (default "!#"). 
///            ! = symbol, # = amount, a space between ! and # will put a space between the symbol and the amount.
///            "!#".into() = $100 (symbol first, amount last)
///            "# !".into() = 100 $ (amount first, space, symbol last) 
/// ---
/// Usage Examples:
/// 
/// Case 1: Use defaults
/// let (price, set_price) = signal(1_000_000);
/// format_currency(price.get(), None);
/// 
/// Case 2: Override specific fields
/// let (price, set_price) = signal(1_000_000);
/// format_currency(price, Some(FmtCurrencyOpts {
///     symbol: "€".into(),
///     decimal: ",".into(),
///     separator: ".".into(),
///     ..Default::default()  // Fills the rest with default values
/// }));
pub fn format_currency(cents: i64, options: Option<FmtCurrencyOpts>) -> String {
    let dollars = cents as f64 / 100.0;

    // If None is passed to the second parameter, then use the Default implementation.
    let opts_struct = options.unwrap_or_default();

    let opts = CurrencyOpts::new()
        .set_symbol(opts_struct.symbol)
        .set_separator(opts_struct.separator)
        .set_decimal(opts_struct.decimal)
        .set_precision(opts_struct.precision)
        .set_pattern(opts_struct.pattern);
    
    Currency::new_float(dollars, Some(opts)).format()
}
