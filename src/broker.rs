use crate::hfps::{AgentSystem, calculate_hash};
use crate::spatial::SpatialGrid;

/// Represents a physical event occurring in the game world.
/// Events are "pulses" that propagate through the spatial grid and
/// apply pressure to agents that possess listeners for them.
pub struct WorldEvent {
    /// The string identifier of the event (e.g., "explosion", "scent_of_blood").
    pub name: String,
    
    /// The coordinates (x, y) where the event originated.
    pub origin: (f32, f32),
    
    /// The raw strength of the event at the source.
    /// This value is attenuated by distance before reaching the agent.
    pub base_intensity: f32,
}

/// A static service responsible for propagating events to agents.
/// It uses the SpatialGrid to find affected agents in O(1) time.
pub struct EventBroker;

impl EventBroker {
    /// Emits an event into the world, updating all affected agents.
    /// 
    /// # Arguments
    /// * `event` - The event to be propagated.
    /// * `system` - The mutable agent system (where agent channels are stored).
    /// * `grid` - The spatial grid for spatial partitioning lookups.
    pub fn emit(event: &WorldEvent, system: &mut AgentSystem, grid: &SpatialGrid) {
        let event_hash = calculate_hash(&event.name);
        
        // Find cells within the maximum possible influence radius.
        // Hardcoded to 500.0 for this demo, but should be dynamic based on event intensity.
        let nearby_cell_indices = grid.get_nearby_cells(event.origin, 500.0); 

        // Iterate over potentially affected cells (broad phase)
        for cell_idx in nearby_cell_indices {
            let mut agent_idx = grid.heads[cell_idx];
            
            // Traverse the linked list of agents in this cell (narrow phase)
            while agent_idx != -1 {
                let i = agent_idx as usize;
                
                // Calculate squared distance to avoid expensive sqrt() unless necessary
                let dx = system.x[i] - event.origin.0;
                let dy = system.y[i] - event.origin.1;
                let dist_sq = dx*dx + dy*dy;

                let profile = &system.profiles[i];
                
                // Check if the agent actually cares about this event type
                for stimulus in profile.listeners.iter() {
                    if stimulus.event_hash == event_hash {
                        let radius_sq = stimulus.radius * stimulus.radius;
                        if dist_sq <= radius_sq {
                            let distance = dist_sq.sqrt();
                            
                            // Linear attenuation: Pressure drops to 0 at max radius.
                            let falloff = 1.0 - (distance / stimulus.radius);
                            let final_pressure = event.base_intensity * stimulus.power * falloff;
                            
                            let chan_idx = stimulus.channel as usize;
                            
                            // --- APPLY PRESSURE (SoA) ---
                            // 1. Base Sensitivity (from DNA Profile)
                            // 2. Adaptation Modifiers (Habituation - Short Term)
                            // 3. DNA Modifiers (Epigenetics - Long Term)
                            let effective_sens = profile.sensitivity[chan_idx] 
                                * (1.0 + system.adaptation[chan_idx][i]) 
                                * system.dna_modifiers[chan_idx][i];
                                
                            system.channels[chan_idx][i] += final_pressure * effective_sens;
                            
                            // --- UPDATE ADAPTATION (Neuroplasticity) ---
                            // Applying pressure also "hardens" the agent against future pressure on this channel.
                            // This creates the "Callus Effect".
                            let adaptation_push = final_pressure.abs() * profile.adaptation_rates[chan_idx] * 0.01;
                            
                            // Clamp adaptation between -0.9 (Hypersensitive) and 2.0 (Numb).
                            system.adaptation[chan_idx][i] = (system.adaptation[chan_idx][i] - adaptation_push).clamp(-0.9, 2.0);
                        }
                    }
                }
                
                // Move to the next agent in the linked list
                agent_idx = grid.next[agent_idx as usize];
            }
        }
    }
}