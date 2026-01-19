//! ICC Profile handling

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ICC Profile type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileType {
    /// Input device profile (scanner, camera)
    Input,
    /// Display device profile (monitor)
    Display,
    /// Output device profile (printer)
    Output,
    /// Color space profile
    ColorSpace,
    /// Abstract profile
    Abstract,
    /// Named color profile
    NamedColor,
}

/// ICC Profile structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IccProfile {
    /// Unique identifier
    pub id: Uuid,
    /// Profile name
    pub name: String,
    /// Profile type
    pub profile_type: ProfileType,
    /// Profile description
    pub description: String,
    /// Copyright information
    pub copyright: String,
    /// Manufacturer
    pub manufacturer: String,
    /// Model
    pub model: String,
    /// Profile data (raw ICC data)
    #[serde(skip)]
    pub data: Vec<u8>,
    /// White point (X, Y, Z)
    pub white_point: (f32, f32, f32),
    /// Is embedded profile
    pub embedded: bool,
}

impl IccProfile {
    /// Create a new ICC profile
    pub fn new(name: impl Into<String>, profile_type: ProfileType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            profile_type,
            description: String::new(),
            copyright: String::new(),
            manufacturer: String::new(),
            model: String::new(),
            data: Vec::new(),
            white_point: (0.95047, 1.0, 1.08883), // D65
            embedded: false,
        }
    }

    /// Create sRGB profile
    pub fn srgb() -> Self {
        let mut profile = Self::new("sRGB IEC61966-2.1", ProfileType::ColorSpace);
        profile.description = "Standard RGB color space".into();
        profile.white_point = (0.95047, 1.0, 1.08883); // D65
        profile
    }

    /// Create Adobe RGB profile
    pub fn adobe_rgb() -> Self {
        let mut profile = Self::new("Adobe RGB (1998)", ProfileType::ColorSpace);
        profile.description = "Adobe RGB (1998) color space".into();
        profile.white_point = (0.95047, 1.0, 1.08883); // D65
        profile
    }

    /// Create Display P3 profile
    pub fn display_p3() -> Self {
        let mut profile = Self::new("Display P3", ProfileType::Display);
        profile.description = "Display P3 color space".into();
        profile.white_point = (0.95047, 1.0, 1.08883); // D65
        profile
    }

    /// Load profile from ICC data
    pub fn from_data(data: Vec<u8>) -> Option<Self> {
        if data.len() < 128 {
            return None;
        }

        // Basic ICC header parsing
        let size = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
        if data.len() < size {
            return None;
        }

        let profile_type = match &data[12..16] {
            b"scnr" => ProfileType::Input,
            b"mntr" => ProfileType::Display,
            b"prtr" => ProfileType::Output,
            b"spac" => ProfileType::ColorSpace,
            b"abst" => ProfileType::Abstract,
            b"nmcl" => ProfileType::NamedColor,
            _ => ProfileType::ColorSpace,
        };

        let mut profile = Self::new("Loaded Profile", profile_type);
        profile.data = data;

        Some(profile)
    }

    /// Check if profile is valid
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
    }

    /// Get profile signature
    pub fn signature(&self) -> Option<[u8; 4]> {
        if self.data.len() >= 8 {
            Some([self.data[4], self.data[5], self.data[6], self.data[7]])
        } else {
            None
        }
    }
}

impl Default for IccProfile {
    fn default() -> Self {
        Self::srgb()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_creation() {
        let profile = IccProfile::srgb();
        assert_eq!(profile.name, "sRGB IEC61966-2.1");
        assert_eq!(profile.profile_type, ProfileType::ColorSpace);
    }

    #[test]
    fn test_white_point() {
        let profile = IccProfile::srgb();
        assert!((profile.white_point.0 - 0.95047).abs() < 0.001);
    }
}
