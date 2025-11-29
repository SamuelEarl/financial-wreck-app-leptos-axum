use std::fmt;

// The custom colors structure
#[derive(Clone, Debug)]
pub struct Colors {
    pub bg: String, // background color
    pub fg: String, // foreground color
    pub br: String, // border color
    pub ol: String, // outline color
}

// Button variants
#[derive(Clone, Debug)]
pub enum BtnVariant {
    Primary,
    Secondary,
    Tertiary,
    Alert,
}

// Helper to convert the enum to a lowercase string (e.g., "primary")
impl fmt::Display for BtnVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BtnVariant::Primary => write!(f, "primary"),
            BtnVariant::Secondary => write!(f, "secondary"),
            BtnVariant::Tertiary => write!(f, "tertiary"),
            BtnVariant::Alert => write!(f, "alert"),
        }
    }
}

pub fn get_btn_colors(
    colors: Option<Colors>, 
    variant: BtnVariant, 
    inverted: bool
) -> String {
    // Case A: Custom colors provided
    if let Some(c) = colors {
        return format!(
            "background-color: {}; color: {}; border-color: {}; outline-color: {};",
            c.bg, c.fg, c.br, c.ol
        );
    }
    
    // Case B: Use pre-defined CSS variables based on variant
    // Note: Rust's format! macro handles the Display trait implementation automatically
    if inverted {
        format!(
            "background-color: var(--{variant}-fg); \
             color: var(--{variant}-bg); \
             border-color: var(--{variant}-fg); \
             outline-color: var(--{variant}-fg);"
        )
    } else {
        format!(
            "background-color: var(--{variant}-bg); \
             color: var(--{variant}-fg); \
             border-color: var(--{variant}-bg); \
             outline-color: var(--{variant}-bg);"
        )
    }
}

// enum Colors {
//   bg: string, // background color
//   fg: string, // foreground color
//   br: string, // border color
//   ol: string, // outline color
// };

// pub fn get_btn_colors(colors: Colors | null = null, variant: string, inverted: boolean) {
//   // Return custom colors.
//   if (colors) {
//     return format!("background-color: {colors::bg}; color: {colors::fg}; border-color: {colors::br}; outline-color: {colors::ol};");
//   }
//   // Return pre-defined colors, which have already been defined in the theme.css file.
//   else {
//     if (inverted) {
//       return format!(" background-color: var(--{variant}-fg); color: var(--{variant}-bg); border-color: var(--{variant}-fg); outline-color: var(--{variant}-fg);");
//     }
//     else {
//       return format!(" background-color: var(--{variant}-bg); color: var(--{variant}-fg); border-color: var(--{variant}-bg); outline-color: var(--{variant}-bg);");
//     }
//   }
// }
