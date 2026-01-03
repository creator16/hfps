use std::fs;
use std::collections::HashMap;
use std::sync::Arc;
use crate::hfps::{BehaviorProfile, Threshold, Stimulus, Emission};

/// Loads a behavior profile from a TOML file.
/// 
/// This function handles the conversion from the human-readable TOML format
/// (where channels are strings like "Security") to the internal indexed format
/// (where channels are usize indices 0-3).
///
/// # Panics
/// Panics if the file cannot be read or if the TOML syntax is invalid.
pub fn load_profile(path: &str) -> Arc<BehaviorProfile> {
    let content = fs::read_to_string(path)
        .expect(&format!("Error opening profile file: {}", path));
    
    // Temporary struct to deserialize textual keys from TOML.
    #[derive(serde::Deserialize)]
    struct RawProfile {
        name: String,
        sensitivity: HashMap<String, f32>,
        decay_rates: HashMap<String, f32>,
        adaptation_rates: HashMap<String, f32>,
        thresholds: Vec<Threshold>,
        listeners: Vec<Stimulus>,
        emissions: Vec<Emission>,
    }

    let raw: RawProfile = toml::from_str(&content).expect("Error parsing TOML syntax");

    let mut sensitivity = [1.0; 4];
    let mut decay_rates = [0.1; 4];
    let mut adaptation_rates = [0.0; 4]; // Default: No adaptation

    let chan_map = [
        ("Vitality", 0), ("Security", 1), ("Dominance", 2), ("Engagement", 3)
    ];

    for (name, idx) in &chan_map {
        if let Some(&val) = raw.sensitivity.get(*name) { sensitivity[*idx] = val; }
        if let Some(&val) = raw.decay_rates.get(*name) { decay_rates[*idx] = val; }
        if let Some(&val) = raw.adaptation_rates.get(*name) { adaptation_rates[*idx] = val; }
    }

    let mut profile = BehaviorProfile {
        name: raw.name,
        sensitivity,
        decay_rates,
        adaptation_rates,
        thresholds: raw.thresholds,
        listeners: raw.listeners,
        emissions: raw.emissions,
    };

    // Pre-calculate hashes for faster runtime comparison.
    for stimulus in &mut profile.listeners {
        stimulus.event_hash = crate::hfps::calculate_hash(&stimulus.event_name);
    }
    for emission in &mut profile.emissions {
        emission.event_hash = crate::hfps::calculate_hash(&emission.event_name);
    }
    
    Arc::new(profile)
}