mod hfps;
mod loader;

use macroquad::prelude::*;
use hfps::HfpsAgent;

#[macroquad::main("HFPS Project: Neuro-Semantic Core")]
async fn main() {
    // 1. Setup Brain
    // Check if file exists, if not create a dummy one or handle error, for now assuming it exists as per previous code
    let profile_path = "data/ovelha.toml";
    let sheep_profile = if std::path::Path::new(profile_path).exists() {
        loader::load_profile(profile_path)
    } else {
        panic!("Profile not found at {}", profile_path);
    };
    
    let mut npc = HfpsAgent::new(sheep_profile.clone());
    
    println!("--- INICIANDO SISTEMA HFPS (AI CORE ONLY) ---");

    // Interface Variables
    let mut last_brain_state = "Neutral".to_string();
    let mut last_simulated_event = "None".to_string();

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0)); // Dark Modern Blue

        // --- DASHBOARD LATERAL (Brain State) ---
        // Centered or full screen since no chat anymore
        let start_x = 50.0;
        
        draw_text("HFPS KERNEL v7.0 (AI ONLY)", start_x, 40.0, 30.0, WHITE);
        
        // Visualização dos Canais de Pressão
        let channels = ["Vitality", "Security", "Dominance", "Engagement"];
        let mut y_pos = 100.0;
        
        for (i, name) in channels.iter().enumerate() {
            let val = npc.channels[i];
            let bar_width = (val.abs() / 100.0) * 300.0;
            let color = if val > 0.0 { GREEN } else { RED };
            
            draw_text(name, start_x, y_pos, 22.0, WHITE);
            draw_rectangle(start_x + 150.0, y_pos - 15.0, 300.0, 25.0, Color::new(0.0, 0.0, 0.0, 0.5)); // Bar Background
            draw_rectangle(start_x + 150.0, y_pos - 15.0, bar_width, 25.0, color);
            draw_text(&format!("{:.1}", val), start_x + 460.0, y_pos, 20.0, WHITE);
            
            y_pos += 50.0;
        }

        draw_line(start_x, 320.0, start_x + 500.0, 320.0, 2.0, GRAY);
        draw_text("BRAIN STATE:", start_x, 350.0, 20.0, GOLD);
        draw_text(&last_brain_state, start_x, 380.0, 40.0, WHITE);

        draw_text("LAST STIMULUS:", start_x, 430.0, 20.0, SKYBLUE);
        draw_text(&last_simulated_event, start_x, 460.0, 30.0, WHITE);

        // --- CONTROLS INFO ---
        let control_y = 550.0;
        draw_text("CONTROLS (Simulate Sensors):", start_x, control_y, 20.0, LIGHTGRAY);
        draw_text("[1] Danger (Wolf/Attack)", start_x, control_y + 30.0, 18.0, WHITE);
        draw_text("[2] Food (Grass/Eat)", start_x, control_y + 60.0, 18.0, WHITE);
        draw_text("[3] Interact (Play/Hello)", start_x, control_y + 90.0, 18.0, WHITE);

        // --- LOGIC ---
        // Input Handling - Direct Signal Simulation
        
        if is_key_pressed(KeyCode::Key1) {
            // Simulate 'Danger'
            npc.channels[1] -= 40.0; // Security Drop
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
            last_brain_state = "CONVERSING".to_string(); // Or Socializing
            last_simulated_event = "Social Interaction".to_string();
        }

        // Decay (Homeostase)
        for i in 0..4 {
            npc.channels[i] *= 0.998; // Recuperação lenta para o zero
        }

        next_frame().await
    }
}