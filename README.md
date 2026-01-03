# HFPS: Homeostatic Fluidic Pulse System
### A High-Performance Anatomical Intelligence Machine (v7.0)

![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Performance](https://img.shields.io/badge/Performance-High-green.svg)

> *"The complexity of life does not require complex hardware. It requires efficient flow."*

HFPS is a high-performance, data-oriented AI engine designed to simulate complex emergent behaviors through **fluidic pressure propagation**. Unlike traditional AI approaches, HFPS models agents as physiological entities that respond to environmental pulses, allowing for the simulation of massive crowds and organic ecosystems with $O(1)$ complexity.

---

## 🔬 Core Philosophy: Anatomical Intelligence

HFPS operates on a fundamental belief: **Behavior is a sub-product of anatomy and physics.** 

Instead of treating AI as a black box of probabilistic weights, HFPS treats it as a biological system. We don't "train" a model to decide what to do; we build an anatomy with pressure channels and let the laws of physics dictate the emergent response. This creates a type of intelligence that is fundamentally deterministic, auditable, and extremely fast.

### Why Anatomical AI?
- **Determinism:** If you know the pressure levels and the stimulus, you know the result. No "hallucinations".
- **Authenticity:** Behavior feels organic because it's governed by homeostasis (recovery and decay).
- **Efficiency:** Arithmetic over matrices. It runs on everything from microcontrollers to high-end servers.

---

## ⚙️ Technical Architecture

### 1. Pressure Channels (The "Brain")
Every agent possesses 4 floating-point channels representing their internal state. Events in the world apply pressure to these channels:
*   **Vitality:** Physical health or energy.
*   **Security:** Fear/Safety level. (Negative = Panic).
*   **Dominance:** Aggression or social hierarchy.
*   **Engagement:** Social interest and curiosity.

### 2. Spatial Grid O(1)
HFPS utilizes a **Static Linked-List Grid** for spatial partitioning:
*   **Fixed Memory:** No heap allocations during the simulation loop.
*   **Cache Locality:** Optimized for L1/L2 cache hits (Structure of Arrays).
*   **Performance:** Check thousands of neighbors in microseconds.

### 3. Neuroplasticity (Adaptation)
Agents exhibit real-time biological adaptation:
*   **Habituation:** Agents develop "calluses" to repeated stimuli. Sustained pressure reduces sensitivity.
*   **Recovery:** When stimulus ceases, homeostasis returns sensitivity to baseline.
*   **DNA Drift:** Prolonged exposure to stress permanently mutates the agent's genetic modifiers (Epigenetics), creating unique personalities over time.

---

## 🚀 Getting Started

### Prerequisites
*   [Rust Toolchain](https://www.rust-lang.org/tools/install) (latest stable)

### Installation
```bash
git clone https://github.com/creator16/hfps.git
cd hfps
```

### Running the Simulation
The current build (`main.rs`) is a **Visual Kernel Visualization**. It demonstrates the internal state of a single agent responding to sensor inputs.

```bash
cargo run --release
```

**Controls (Sensor Simulation):**
*   `[1]` **Simulate Danger:** Triggers a "Predator/Attack" signal (Panic).
*   `[2]` **Simulate Food:** Triggers a "Resource/Grass" signal (Feeding).
*   `[3]` **Simulate Social:** Triggers a "Interaction" signal (Engagement).

Observe how the **Brain State** changes and the bars fluctuate based on pressure and decay (homeostasis).

---

## 🛠️ Configuration (Modding)

You can define the "species" of your agents using simple TOML files in `data/`.

**Example: `data/ovelha.toml`**
```toml
name = "Sheep"

[sensitivity]
Security = 2.0   # Highly paranoid (2x multiplier)
Vitality = 1.0

[decay_rates]
Security = 0.5   # Calms down slowly

[thresholds]
channel = "Security"
value = -50.0
flag = 4         # FLAG_FLEEING
```

---

## 🧪 Work In Progress & Contributions

**HFPS is an evolving research project.**
We strongly encourage forks, modifications, and pull requests.

### Current Limitations
1.  **Serialization:** The `.dna` format is simple JSON but likely needs a more robust binary format for large swarms.
2.  **Multithreading:** While `SoA` is ready for SIMD, we haven't implemented `Rayon` for parallel agent updates yet.
3.  **Bindings:** Currently only runs in Rust. We need C-bindings to export `hfps.dll` for Unity/Unreal/Godot.

### How to Contribute
1.  Fork the repo.
2.  Create a branch for your feature (`git checkout -b feature/swarm-logic`).
3.  Implement your changes (e.g., adding a `Reproduction` system).
4.  Submit a Pull Request.

> *"We are building the anatomy, not the script."*

---

## 📄 License

This project is open-source under the **MIT License**.

---

*Created with ❤️ for the Open Source Community.*
