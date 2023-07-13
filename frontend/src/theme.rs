use dyno_core::{serde, PlotColor};
#[repr(u8)]
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(crate = "serde")]
pub enum Theme {
    #[default]
    Dark = 0,
    Light = 1,
}

impl std::ops::Not for Theme {
    type Output = Self;
    fn not(self) -> Self::Output {
        self.swap()
    }
}

impl Theme {
    #[inline]
    pub const fn to_str(self) -> &'static str {
        match self {
            Theme::Dark => "business",
            Theme::Light => "pastel",
        }
    }
    #[inline]
    pub const fn swap(self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }
    #[inline]
    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }
    #[inline]
    pub fn is_light(self) -> bool {
        matches!(self, Self::Light)
    }

    #[inline]
    pub const fn plot_color(self) -> PlotColor {
        match self {
            Theme::Dark => PlotColor::dark(),
            Theme::Light => PlotColor::light(),
        }
    }
}
