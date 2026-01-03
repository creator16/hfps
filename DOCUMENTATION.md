# 📖 HFPS Official Technical Documentation
## v4.1.0 - The Anatomical Intelligence Engine

HFPS (Homeostatic Fluidic Pulse System) is a high-performance, data-oriented AI engine designed to simulate complex emergent behaviors through fluidic pressure propagation rather than traditional decision trees or neural networks.

---

## 1. Core Concepts

### A. Pressure Channels
Every agent in HFPS possesses 4 floating-point channels that represent their internal state:
1.  **Vitality:** Physical health or energy.
2.  **Security:** Safety/Fear level. Negative values indicate panic/danger.
3.  **Dominance:** Charisma or aggression level. Used for leadership or hunting.
4.  **Engagement:** Interest or social connection.

### B. The Pulse (World Events)
Information travels through the world as **Events**. An event has:
- **Origin:** (x, y) coordinates.
- **Base Intensity:** The raw power of the stimulus.
- **Falloff:** Pressure decreases linearly over distance based on the listener's radius.

### C. Adaptation (Phase 4)
HFPS agents exhibit **Neuroplasticity**. If a channel receives sustained pressure, its sensitivity decreases (Habituación). 
- **Effect:** Agents stop responding to repetitive stimuli (e.g., getting used to loud noises).
- **Recovery:** When pressure ceases, sensitivity slowly returns to the baseline.

### D. Epigenetics (Phase 5: DNA Drift)
Beyond temporary habituation, HFPS implements **Mutable AI Models** through DNA modifiers:
- **Plasticity Leak:** If an agent remains habituated for prolonged periods, a portion of that experience "leaks" into its `dna_modifiers`.
- **Permanent Evolution:** This creates a permanent shift in the agent's base sensitivity. Over time, an agent's "personality" literally mutates based on its life history.
- **Auditable Learning:** Unlike the opaque weights of a Neural Network, DNA Drift is transparent, deterministic, and can be tracked in real-time.

---

## 2. Profile Configuration (TOML)

Profiles define the "biology" of an agent. Saved in `data/*.toml`.

```toml
name = "Example Agent"

[sensitivity]
Security = 2.0   # How much pressure affects the channel (multiplier)

[decay_rates]
Security = 1.0   # How fast the channel returns to 0.0 per second

[adaptation_rates]
Security = 0.5   # Speed of habituation (callusing)

[[thresholds]]
channel = "Security"
value = -50.0
flag = 4          # Activate FLAG_FLEEING if Security < -50.0

[[listeners]]
event_name = "loud_noise"
channel = "Security"
power = -10.0     # Negative = Fear
radius = 400.0    # Range of hearing

[[emissions]]
flag = 4          # If agent is fleeing...
event_name = "panic_scream"
power = 20.0      # ...emit this pulse to neighbors
radius = 200.0
```

---

## 3. Engine Architecture: Genetic SoA
HFPS uses a **Structure of Arrays (SoA)** architecture. Unlike traditional Object-Oriented AI, data is stored in parallel contiguous vectors.

### A. The AgentSystem
The `AgentSystem` manages all entities. Vectors for `x`, `y`, `channels`, and `dna` are stored separately to maximize CPU Cache L1 hits.

### B. Spatial Grid O(1)
HFPS uses a static linked-list spatial grid integrated with the SoA system.
- **Performance:** $O(1)$ insertions and lookups.
- **Memory:** Zero heap allocations during simulation.

---

## 4. Integration Guide (Rust API)

### Step 1: Initialize System
```rust
let mut system = AgentSystem::new(2000); // Pre-allocate for 2000 agents
let human_profile = loader::load_profile("data/human.toml");

// Add agents
system.add_agent((500.0, 500.0), human_profile.clone());
```

### Step 2: The Main Loop (The Pulse)
```rust
loop {
    // 1. Update Spatial Grid (continous O(1))
    grid.clear();
    for i in 0..system.count { grid.insert(i, (system.x[i], system.y[i])); }

    // 2. Multi-threaded / Vectorized Tick
    system.tick(); // Process decay, adaptation and DNA mutation for all agents

    // 3. Emit World Events via Broker
    EventBroker::emit(&WorldEvent {
        name: "explosion".into(),
        origin: (500.0, 500.0),
        base_intensity: 50.0,
    }, &mut system, &grid);
}
```

### Step 3: Handle Flags (Accessing Data)
Since agents are indices, you access their state via the system's arrays:
```rust
for i in 0..system.count {
    let flags = system.active_flags[i];
    if flags & FLAG_FLEEING != 0 {
        // Render fleeing behavior
    }
}
```

---

## 5. Performance Tips
- **Release Mode:** Always run with `cargo run --release`. The Spatial Grid and Event Broker rely heavily on LLVM optimizations.
- **Avoid Clones:** Pass profiles via `Arc` or references.
- **Batch Processing:** Process emissions using the `cell_emissions` buffer to avoid $O(N^2)$ social interactions.

---

## 6. Global Model Persistence (DNA Kernels)
HFPS models are persistent and shareable through **DNA Kernels**. Unlike Neural Network weights, these kernels are compact (64 bytes) and human-readable.

### A. The .dna File Format
A trained agent's experience is saved as a JSON kernel containing its DNA modifiers.
```json
{
  "modifiers": [1.0, 0.85, 1.0, 1.0]
}
```
*Index 1 (Security) at 0.85 means this agent is permanently 15% more resistant to fear.*

### B. Model Operations
- **Export (Snapshot):** Captures the current `dna_modifiers` of an agent and writes them to a `.dna` file.
- **Injection:** Loads a `.dna` file and applies its modifiers to a new agent at runtime. This allows for "Pre-Trained" NPCs that inherit traits from previous simulations.

### C. Comparison with Deep Learning
| Feature | Deep Learning (onnx/pt) | HFPS DNA Kernel |
| :--- | :--- | :--- |
| **Size** | 50MB - 5GB | **64 Bytes** |
| **Training** | Offline (GPU Heavy) | **Real-time (CPU < 1ms)** |
| **Transparency** | Black Box | **Fully Auditable** |
| **Persistence** | Weights/Biases | **Anatomical Modifiers** |

---

## 7. NLP Engine: Semantic Galaxy v7.7
O subsistema de NLP do HFPS não utiliza bases estatísticas massivas (LLMs), mas sim um modelo de **Pressão e Atração Semântica** baseado na anatomia do agente.

### A. Lexical Atoms (Átomos Lexicais)
A menor unidade de pensamento. Cada palavra carregada possui:
- **Coord:** Posição em um mapa 2D (Social/Individual vs. Paz/Conflito).
- **Charge:** Impacto emocional (pressão) que a palavra causa nos Canais do NPC.
- **Kind:** Classificação gramatical-anatômica (Subject, Action, Quality, Object).

### B. Weighted Associative Chains (Ponderação de Fluxo)
O motor aprende sequências de palavras a partir de um corpus de texto (via `ingest_text`). Ele grava a frequência absoluta de transição entre palavras, criando "Caminhos de Menor Resistência" que definem o estilo de fala do NPC.

### C. Stochastic Articulation (Articulação sob Pressão)
A geração de frases é um processo de navegação estocástica:
1.  **Emotional Gravity:** O estado interno dos 4 Canais internos do Agente distorce o mapa semântico, "puxando" as escolhas para territórios que combinam com seu humor atual.
2.  **Stochastic Noise (Temperature):** Um fator de ruído controlado que permite ao NPC explorar caminhos semânticos menos prováveis, gerando variedade linguística.
3.  **Boredom Penalty:** Penalidade de score para palavras já utilizadas na mesma frase, impedindo loops infinitos e redundância gramatical.

| Recurso | NLP HFPS | LLMs Tradicionais |
| :--- | :--- | :--- |
| **Geração** | Pressionada por Emoção | Probabilidade Estatística |
| **Memória** | Persistência Galáctica (TOML) | Pesos de Parâmetros (Binário) |
| **Aprendizado** | Ingestão Local em Tempo Real | Treinamento Offline (GPU) |
| **Consistência** | Determinada pela Anatomia | Determinada pelo Contexto (Prompt) |

### D. Filosofia Evolutiva: O Salto Além dos LLMs
Diferente dos LLMs tradicionais que buscam a perfeição estatística, o HFPS foca na **Sinceridade Biológica**.

1.  **Comunicação como Força Física:** No HFPS, a fala não é apenas texto; é uma emissão de pressão. Palavras de pânico ou agressividade emitem pulsos reais na `Spatial Grid`, afetando fisicamente os NPCs ao redor. A palavra é uma extensão do corpo.
2.  **Cicatrizes Semânticas (Memória de Longo Prazo):** Em vez de uma janela de contexto que esquece, o HFPS utiliza o deslocamento de coordenadas na Galáxia. Eventos traumáticos ou sociais "puxam" palavras permanentemente para novos territórios semânticos. O NPC não lembra o que foi dito, ele lembra o que aquele conceito se tornou para ele.
3.  **Imperfeição como Conexão:** LLMs são treinados para serem polidos. O HFPS abraça o ruído, a gagueira e a incoerência sob pressão. Essa imperfeição biológica é o que gera o **Ponto de Ruptura do Teste de Turing**: o jogador sente que está falando com um ser vivo sob estresse, não com uma calculadora de palavras.

---
*HFPS: Simulando o fluxo da vida e do pensamento através da lógica de pressão.* 🏁🔥🧪🧬🗣️🌌🔭
