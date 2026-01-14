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


// The Sizes input structure
#[derive(Clone, Debug)]
pub struct Sizes {
    pub fs: Option<i32>,    // Font size
    pub fw: Option<String>, // Font weight
    pub pv: Option<i32>,    // Padding vertical
    pub ph: Option<i32>,    // Padding horizontal
    // pub gp: Option<i32>,    // Gap (flexbox)
}

impl Default for Sizes {
    fn default() -> Self {
        Self {
            fs: Some(4),
            fw: Some("normal".to_string()),
            pv: Some(2),
            ph: Some(3),
        }
    }
}

// /// A function that returns the default Sizes struct.
// /// This should be used in components to provide the default size values for applicable components.
// /// UPDATE: Maybe this is not necessary because of the "impl Default for Sizes".
// pub fn default_sizes() -> Sizes {
//     Sizes {
//         fs: Some(4),
//         fw: Some("normal".to_string()),
//         pv: Some(2),
//         ph: Some(3),
//         // gp: Some(4),
//     }
// }

// The ElementSizes return structure
#[derive(Clone, Debug)]
pub struct ElementSizes {
    pub all: String,
    pub fs: String,
    pub fw: String,
    pub pv: String,
    pub ph: String,
}

pub fn get_element_sizes(sizes: Option<Sizes>, is_btn: bool) -> ElementSizes {
    // Helper to safely access options without deep nesting.
    // If sizes is None, this acts as if all fields are None.
    let s = sizes.as_ref();

    // -- Font Size Logic --
    // Extract fs, defaulting to -1 if None
    let fs_val = s.and_then(|x| x.fs).unwrap_or(-1);
    
    let font_size = if fs_val > -1 {
        format!("var(--size-{});", fs_val)
    } else {
        "var(--size-base);".to_string()
    };

    // -- Font Weight Logic --
    // Logic: If fw exists, use it. Else if is_btn is true, "bold". Else "normal".
    let fw_val = s.and_then(|x| x.fw.clone());
    
    // let font_weight = if let Some(fw) = fw_val {
    //     fw
    // } else if is_btn {
    //     "bold".to_string()
    // } else {
    //     "normal".to_string()
    // };
    let font_weight = if let Some(fw) = fw_val {
        fw
    } else {
        "normal".to_string()
    };

    // -- Padding Logic --
    let pv_val = s.and_then(|x| x.pv).unwrap_or(-1);
    let padding_v = if pv_val > -1 {
        format!("var(--size-{})", pv_val)
    } else {
        "var(--padding-v-default)".to_string()
    };

    let ph_val = s.and_then(|x| x.ph).unwrap_or(-1);
    let padding_h = if ph_val > -1 {
        format!("var(--size-{})", ph_val)
    } else {
        "var(--padding-h-default)".to_string()
    };

    // -- Construct the 'all' string --
    let mut all = format!(
        "font-size: {} font-weight: {}; padding: {} {};",
        font_size, font_weight, padding_v, padding_h
    );

    // -- Gap Logic --
    let gap = if fs_val > -1 {
        format!("var(--size-{})", fs_val)
    } else {
        "var(--size-base)".to_string()
    };

    // If the element that is retrieving the size styles is a button, then add a `gap` (flexbox) style.
    if is_btn {
        all.push_str(&format!(" gap: {}", gap));
    }

    // -- Return the struct --
    ElementSizes {
        all,
        fs: font_size,
        fw: font_weight,
        pv: padding_v,
        ph: padding_h,
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum ElementWidths {
    Auto,
    Full,
}

pub fn get_element_width(width: ElementWidths) -> &'static str {
    match width {
        ElementWidths::Full => "width: 100%",
        ElementWidths::Auto => "",
    }
}
