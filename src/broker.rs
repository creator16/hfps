use crate::hfps::{AgentSystem, calculate_hash};
use crate::spatial::SpatialGrid;

pub struct WorldEvent {
    pub name: String,
    pub origin: (f32, f32),
    pub base_intensity: f32,
}

pub struct EventBroker;

impl EventBroker {
    pub fn emit(event: &WorldEvent, system: &mut AgentSystem, grid: &SpatialGrid) {
        let event_hash = calculate_hash(&event.name);
        let nearby_cell_indices = grid.get_nearby_cells(event.origin, 500.0); 

        for cell_idx in nearby_cell_indices {
            let mut agent_idx = grid.heads[cell_idx];
            
            while agent_idx != -1 {
                let i = agent_idx as usize;
                let dx = system.x[i] - event.origin.0;
                let dy = system.y[i] - event.origin.1;
                let dist_sq = dx*dx + dy*dy;

                let profile = &system.profiles[i];
                for stimulus in profile.listeners.iter() {
                    if stimulus.event_hash == event_hash {
                        let radius_sq = stimulus.radius * stimulus.radius;
                        if dist_sq <= radius_sq {
                            let distance = dist_sq.sqrt();
                            let falloff = 1.0 - (distance / stimulus.radius);
                            let final_pressure = event.base_intensity * stimulus.power * falloff;
                            
                            let chan_idx = stimulus.channel as usize;
                            
                            // --- PRESSÃO EM SoA ---
                            let effective_sens = profile.sensitivity[chan_idx] * (1.0 + system.adaptation[chan_idx][i]) * system.dna_modifiers[chan_idx][i];
                            system.channels[chan_idx][i] += final_pressure * effective_sens;
                            
                            // Atualiza habituação
                            let adaptation_push = final_pressure.abs() * profile.adaptation_rates[chan_idx] * 0.01;
                            system.adaptation[chan_idx][i] = (system.adaptation[chan_idx][i] - adaptation_push).clamp(-0.9, 2.0);
                        }
                    }
                }
                
                // Pular para o próximo agente na célula
                agent_idx = grid.next[agent_idx as usize];
            }
        }
    }
}