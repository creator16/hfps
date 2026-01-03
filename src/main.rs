mod hfps;
mod loader;

use macroquad::prelude::*;
use hfps::HfpsAgent;

#[macroquad::main("HFPS Project: Neuro-Homeostatic Core")]
async fn main() {
    // 1. Setup Brain
    // Check if the profile exists. If not, the application will panic with a helpful message.
    let profile_path = "data/ovelha.toml";
    let sheep_profile = if std::path::Path::new(profile_path).exists() {
        loader::load_profile(profile_path)
    } else {
        panic!("❌ CRITICAL ERROR: Profile not found at '{}'. Please check your 'data' directory.", profile_path);
    };
    
    // Initialize a single agent for visualization.
    // In a real game scenario, you would use AgentSystem::new(2000) instead.
    let mut npc = HfpsAgent::new(sheep_profile.clone());
    
    println!("--- HFPS SYSTEM INITIALIZED (AI CORE ONLY) ---");
    println!("> Press [1] to simulate Predator Attack (Security Drop)");
    println!("> Press [2] to simulate Food Found (Vitality Boost)");
    println!("> Press [3] to simulate Social Check (Engagement Boost)");

    // Interface Variables for Visualization
    let mut last_brain_state = "Neutral".to_string();
    let mut last_simulated_event = "None".to_string();

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0)); // Dark Modern Blue Background

        // --- DASHBOARD LATERAL (Brain State) ---
        let start_x = 50.0;
        
        draw_text("HFPS KERNEL v7.0 (VISUALIZER)", start_x, 40.0, 30.0, WHITE);
        
        // --- VISUALIZATION OF PRESSURE CHANNELS ---
        let channels = ["Vitality", "Security", "Dominance", "Engagement"];
        let mut y_pos = 100.0;
        
        for (i, name) in channels.iter().enumerate() {
            let val = npc.channels[i];
            
            // Calculate bar width relative to max value (100.0)
            let bar_width = (val.abs() / 100.0) * 300.0;
            
            // Green for positive pressure, Red for negative pressure (e.g. Fear)
            let color = if val > 0.0 { GREEN } else { RED };
            
            draw_text(name, start_x, y_pos, 22.0, WHITE);
            
            // Draw Bar Background
            draw_rectangle(start_x + 150.0, y_pos - 15.0, 300.0, 25.0, Color::new(0.0, 0.0, 0.0, 0.5)); 
            
            // Draw Pressure Bar
            draw_rectangle(start_x + 150.0, y_pos - 15.0, bar_width, 25.0, color);
            
            // Draw numeric value
            draw_text(&format!("{:.1}", val), start_x + 460.0, y_pos, 20.0, WHITE);
            
            y_pos += 50.0;
        }

        draw_line(start_x, 320.0, start_x + 500.0, 320.0, 2.0, GRAY);
        
        draw_text("BRAIN STATE:", start_x, 350.0, 20.0, GOLD);
        draw_text(&last_brain_state, start_x, 380.0, 40.0, WHITE);

        draw_text("LAST STIMULUS:", start_x, 430.0, 20.0, SKYBLUE);
        draw_text(&last_simulated_event, start_x, 460.0, 30.0, WHITE);

        // --- CONTROLS INSTRUCTIONS ---
        let control_y = 550.0;
        draw_text("CONTROLS (Sensor Simulation):", start_x, control_y, 20.0, LIGHTGRAY);
        draw_text("[1] Danger (Wolf/Attack)", start_x, control_y + 30.0, 18.0, WHITE);
        draw_text("[2] Food (Grass/Eat)", start_x, control_y + 60.0, 18.0, WHITE);
        draw_text("[3] Interact (Play/Hello)", start_x, control_y + 90.0, 18.0, WHITE);

        // --- LOGIC LOOP ---
        // Input Handling - Simulating 'EventBroker' signals via Keyboard
        
        if is_key_pressed(KeyCode::Key1) {
            // Simulate 'Danger'
            npc.channels[1] -= 40.0; // Security Drop (Panic)
            last_brain_state = "PANIC".to_string();
            last_simulated_event = "Danger Signal Detected".to_string();
        }
        
        if is_key_pressed(KeyCode::Key2) {
             // Simulate 'Food'
            npc.channels[0] += 20.0; // Vitality Boost
            last_brain_state = "FEEDING".to_string();
            last_simulated_event = "Food Source Located".to_string();
        }

        if is_key_pressed(KeyCode::Key3) {
            // Simulate 'Social'
            npc.channels[3] += 10.0; // Engagement Boost
            last_brain_state = "CONVERSING".to_string();
            last_simulated_event = "Social Interaction".to_string();
        }

        // Decay (Homeostasis)
        // In the real engine, this is handled by System::tick()
        for i in 0..4 {
            npc.channels[i] *= 0.998; // Slow recovery to zero
        }

        next_frame().await
    }
}