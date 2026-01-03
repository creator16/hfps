# HFPS: Homeostatic Fluidic Pulse System
### The Anatomical Alternative to Neural Networks (v7.0)

![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Performance](https://img.shields.io/badge/Performance-High-green.svg)

> *"The complexity of life does not require complex hardware. It requires efficient flow."*

HFPS is a high-performance, data-oriented AI engine designed to simulate complex emergent behaviors through **fluidic pressure propagation** rather than traditional decision trees or neural networks. It is designed to model massive crowods and organic ecosystems with $O(1)$ complexity.

---

## 🔬 Core Philosophy: Anatomical Intelligence

The world of AI is obsessed with statistics (Deep Learning). HFPS takes a different approach: **Biology & Physics**.
Instead of training a neural network to *guess* what an agent should do, HFPS gives the agent an *anatomy* and lets the laws of pressure dictate behavior.

| Feature | Deep Learning (LLMs/RL) | HFPS (Fluidic Core) |
| :--- | :--- | :--- |
| **Logic** | Matrix Multiplication | Fluid Pressure |
| **Training** | Offline (GPU Heavy) | Real-time Adaptation (CPU) |
| **Memory** | Weights & Biases | Neuroplasticity & DNA |
| **Hardware** | Heavy (Needs GPU) | Light (Runs on Watch) |
| **Throughput** | ~Heavy | **2000+ Agents @ 60FPS** |

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

## 🔮 Roadmap: The Future

*   **Swarm Intelligence:** Advanced group coordination tactics.
*   **Spatial Awareness:** Raycasting integrated into pressure channels (Sight).
*   **Tool Usage:** Agents learning to use objects to relieve pressure.
*   **Industrialization:** C FFI bindings for integration with Unity/Unreal.

---

## 📄 License

This project is open-source under the **MIT License**.
*Created by Edward & Antigravity.*
