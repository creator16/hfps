use std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// A simple type alias for behavior flags (bitmask).
/// used to identify states like FLEEING, SLEEPING, etc.
pub type Flag = u32;

/// Calculates a stable hash for a given string.
/// This is used to map event names (string) to event IDs (u64)
/// for faster comparison during the simulation loop.
pub fn calculate_hash(t: &String) -> u64 {
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// The 4 Primary Pressure Channels of the HFPS system.
/// Unlike standard RPG stats, these are fluid containers that accept "Pressure".
/// Pressure can be positive or negative.
/// 
/// - Vitality: Health, Energy, Hunger (Inverse).
/// - Security: Safety vs Fear. Negative pressure = Panic/Danger.
/// - Dominance: Aggression, Confidence, Social Hierarchy.
/// - Engagement: Curiosity, Social Interest, Boredom (Inverse).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Channel {
    Vitality = 0, Security = 1, Dominance = 2, Engagement = 3,
}

/// Defines the "Biology" of a species.
/// This struct is shared via Arc<T> across all agents of the same type.
/// It acts as a static configuration or "DNA Blueprint".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorProfile {
    pub name: String,
    
    /// Multiplier for incoming pressure per channel.
    /// Example: A high Security sensitivity means the agent gets scared easily.
    pub sensitivity: [f32; 4],
    
    /// How fast the pressure returns to zero (Homeostasis).
    /// value is subtracted per second.
    pub decay_rates: [f32; 4],
    
    /// How fast the agent "gets used to" a stimulus (Habituation).
    /// Used in Phase 4: Neuroplasticity.
    pub adaptation_rates: [f32; 4],
    
    /// Triggers that activate FLAGS when a channel reaches a certain value.
    pub thresholds: Vec<Threshold>,
    
    /// List of event names this agent listens for (e.g., "loud_noise").
    pub listeners: Vec<Stimulus>,
    
    /// List of events this agent emits when a flag is active (e.g., "scream").
    pub emissions: Vec<Emission>,
}

/// Individual Entity structure for isolated testing (e.g., single unit visualizer).
/// NOT used in the main simulation loop (AgentSystem uses SoA instead).
pub struct HfpsAgent {
    pub channels: [f32; 4],
    
    /// Neuroplasticity buffer.
    /// Stores how much the agent has "habituated" to a specific channel.
    /// Negative values mean the agent is "numb" to that pressure.
    pub adaptation: [f32; 4],
    
    /// Epigenetic Modifiers (DNA Drift).
    /// Permanent multipliers acquired through life experiences (long-term adaptation).
    /// 1.0 = Standard.
    pub dna_modifiers: [f32; 4],
    
    /// Bitmask of current active states.
    pub active_flags: Flag,
    
    /// Reference to the shared species configuration.
    pub profile: Arc<BehaviorProfile>,
}

impl HfpsAgent {
    pub fn new(profile: Arc<BehaviorProfile>) -> Self {
        Self {
            channels: [0.0; 4],
            adaptation: [0.0; 4],
            dna_modifiers: [1.0; 4],
            active_flags: 1,
            profile,
        }
    }
}

/// Defines a reaction to an external WorldEvent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stimulus {
    pub event_name: String,
    #[serde(skip)]
    pub event_hash: u64,
    pub channel: Channel,
    /// Multiplier relative to the event's base intensity.
    pub power: f32,
    /// Interaction radius.
    pub radius: f32,
}

/// Defines a state change trigger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threshold {
    pub channel: Channel,
    /// The pressure level required to trigger the flag.
    pub value: f32,
    /// The flag bit to set active.
    pub flag: Flag,
}

/// Defines an event emitted by the agent itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emission {
    pub flag: Flag,
    pub event_name: String,
    #[serde(skip)]
    pub event_hash: u64,
    pub power: f32,
    pub radius: f32,
}

/// --- PHASE 5: STRUCTURE OF ARRAYS (SoA) SYSTEM ---
/// HFPS uses a Data-Oriented Design approach.
/// Instead of having a `Vec<Agent>` where each agent is a struct, we have
/// parallel vectors for each property (x, y, channels[0], etc.).
/// 
/// **Why?**
/// 1. **Cache Locality:** When processing "Vitality" decay for 2000 agents, the CPU 
///    loads a continuous chunk of floats into the L1 Cache.
/// 2. **SIMD (Auto-Vectorization):** The compiler (LLVM) can easily optimize loops
///    over simple arrays into AVX instructions, processing 8 or 16 agents at once.
/// 3. **Memory Layout:** Eliminates padding and pointer chasing.
pub struct AgentSystem {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    /// [ChannelIndex][AgentIndex]
    pub channels: [Vec<f32>; 4],
    pub adaptation: [Vec<f32>; 4],
    pub dna_modifiers: [Vec<f32>; 4],
    pub active_flags: Vec<Flag>,
    /// Pointer to the unchanging species data for each agent
    pub profiles: Vec<Arc<BehaviorProfile>>,
    /// Total active agents
    pub count: usize,
}

impl AgentSystem {
    pub fn new(capacity: usize) -> Self {
        Self {
            x: Vec::with_capacity(capacity),
            y: Vec::with_capacity(capacity),
            channels: [Vec::with_capacity(capacity), Vec::with_capacity(capacity), Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
            adaptation: [Vec::with_capacity(capacity), Vec::with_capacity(capacity), Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
            dna_modifiers: [Vec::with_capacity(capacity), Vec::with_capacity(capacity), Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
            active_flags: Vec::with_capacity(capacity),
            profiles: Vec::with_capacity(capacity),
            count: 0,
        }
    }

    pub fn add_agent(&mut self, pos: (f32, f32), profile: Arc<BehaviorProfile>) {
        self.x.push(pos.0);
        self.y.push(pos.1);
        for i in 0..4 {
            self.channels[i].push(0.0);
            self.adaptation[i].push(0.0);
            self.dna_modifiers[i].push(1.0);
        }
        self.active_flags.push(0);
        self.profiles.push(profile);
        self.count += 1;
    }

    /// --- SIMD HOT PATH (Auto-Vectorized) ---
    /// This is the heart of Phase 5.
    /// The loop structure is kept intentionally simple to allow LLVM to vectorize it.
    /// It handles Decay (Homeostasis), Habituation recovery, and DNA Drift.
    pub fn tick(&mut self) {
        let dt = 0.016; // Fixed timestep for simulation stability
        
        // Loop by channel to ensure linear memory access (Cache friendly)
        for c in 0..4 {
            let channels = &mut self.channels[c];
            let adaptation = &mut self.adaptation[c];
            let dna = &mut self.dna_modifiers[c];
            
            // Using simple indexing helps the compiler prove safety for vectorization
            for i in 0..self.count {
                // 1. Decay (Homeostasis)
                // Uses the decay_rate from the specific agent's profile profile
                // Optimization Note: accessing profile[i] here causes a cache miss (indirect access).
                // In a future "Phase 8", decay_rates should also be flattened into an SoA vector.
                let decay = self.profiles[i].decay_rates[c] * dt;
                let val = channels[i];
                if val > 0.0 { channels[i] = (val - decay).max(0.0); }
                else if val < 0.0 { channels[i] = (val + decay).min(0.0); }

                // 2. Adaptation Recovery (Plasticity)
                // Slowly returns the habituation buffer to 0.0
                adaptation[i] *= 0.9995;

                // 3. DNA Drift (Permanent Mutation)
                // If adaptation (stress) is high, it slowly "leaks" into the permanent DNA modifiers.
                // This simulates long-term evolutionary changes in the individual.
                let learning_step = adaptation[i] * 0.00001;
                dna[i] = (dna[i] + learning_step).clamp(0.1, 5.0);
            }
        }

        // 4. Update Flags
        // This part is "branchy" (ifs/elses) so it won't vectorize well,
        // but it is still fast due to the contiguous memory layout.
        for i in 0..self.count {
            let mut flags = 1; // Default state (e.g., IDLE)
            let profile = &self.profiles[i];
            
            // Check state thresholds
            for t in &profile.thresholds {
                let current_val = self.channels[t.channel as usize][i];
                // Check if value exceeds threshold (handling both positive and negative thresholds)
                if (t.value >= 0.0 && current_val >= t.value) || (t.value < 0.0 && current_val <= t.value) {
                    flags |= t.flag;
                }
            }
            self.active_flags[i] = flags;
        }
    }
}

/// A compact "Save File" for an agent's learned experiences.
/// Can be exported and injected into other agents.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnaKernel {
    pub modifiers: [f32; 4],
}