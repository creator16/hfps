use std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

pub type Flag = u32;

pub fn calculate_hash(t: &String) -> u64 {
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Channel {
    Vitality = 0, Security = 1, Dominance = 2, Engagement = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorProfile {
    pub name: String,
    pub sensitivity: [f32; 4],
    pub decay_rates: [f32; 4],
    pub adaptation_rates: [f32; 4],
    pub thresholds: Vec<Threshold>,
    pub listeners: Vec<Stimulus>,
    pub emissions: Vec<Emission>,
}

/// Entidade individual para testes isolados (NLP/Protótipos)
pub struct HfpsAgent {
    pub channels: [f32; 4],
    pub adaptation: [f32; 4],
    pub dna_modifiers: [f32; 4],
    pub active_flags: Flag,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stimulus {
    pub event_name: String,
    #[serde(skip)]
    pub event_hash: u64,
    pub channel: Channel,
    pub power: f32,
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threshold {
    pub channel: Channel,
    pub value: f32,
    pub flag: Flag,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emission {
    pub flag: Flag,
    pub event_name: String,
    #[serde(skip)]
    pub event_hash: u64,
    pub power: f32,
    pub radius: f32,
}

/// --- FASE 5: STRUCTURE OF ARRAYS (SoA) SYSTEM ---
/// Em vez de Vec<Agent>, temos um sistema que gerencia arrays paralelos.
/// Isso maximiza o Cache-L1 e permite auto-vetorização SIMD.
pub struct AgentSystem {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub channels: [Vec<f32>; 4],
    pub adaptation: [Vec<f32>; 4],
    pub dna_modifiers: [Vec<f32>; 4],
    pub active_flags: Vec<Flag>,
    pub profiles: Vec<Arc<BehaviorProfile>>,
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

    /// --- SIMD HOT PATH (Auto-Vetorizado) ---
    /// Este é o coração da Fase 5. O loop é simples para que o LLVM use SSE/AVX.
    pub fn tick(&mut self) {
        let dt = 0.016;
        
        // Loop por canal para garantir acesso linear à memória (Cache friendly)
        for c in 0..4 {
            let channels = &mut self.channels[c];
            let adaptation = &mut self.adaptation[c];
            let dna = &mut self.dna_modifiers[c];
            
            // O uso de chunks/iteradores simples ajuda na auto-vetorização
            for i in 0..self.count {
                // 1. Decaimento (Usa o decay_rate do perfil do agente correspondente)
                let decay = self.profiles[i].decay_rates[c] * dt;
                let val = channels[i];
                if val > 0.0 { channels[i] = (val - decay).max(0.0); }
                else if val < 0.0 { channels[i] = (val + decay).min(0.0); }

                // 2. Homeostase (Recuperação da adaptação)
                adaptation[i] *= 0.9995;

                // 3. DNA Drift (Mutação permanente)
                let learning_step = adaptation[i] * 0.00001;
                dna[i] = (dna[i] + learning_step).clamp(0.1, 5.0);
            }
        }

        // 4. Update Flags (Não vetorizável facilmente devido à lógica de threshold, mas rápida em SoA)
        for i in 0..self.count {
            let mut flags = 1; // IDLE
            let profile = &self.profiles[i];
            for t in &profile.thresholds {
                let current_val = self.channels[t.channel as usize][i];
                if (t.value >= 0.0 && current_val >= t.value) || (t.value < 0.0 && current_val <= t.value) {
                    flags |= t.flag;
                }
            }
            self.active_flags[i] = flags;
        }
    }
}

/// Snapshot de DNA para exportação no novo sistema
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnaKernel {
    pub modifiers: [f32; 4],
}