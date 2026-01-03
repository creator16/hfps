/// A high-performance static spatial partitioning system.
/// Uses a Linked-List approach embedded in arrays to achieve O(1) insertions and lookups
/// with ZERO heap allocations during the simulation step.
/// 
/// This is critical for scaling to 10,000+ agents.
pub struct SpatialGrid {
    /// Dimension of each square cell in world units.
    pub cell_size: f32,
    
    /// Array of "Head" pointers for each cell.
    /// heads[cell_index] stores the ID of the *first* agent in that cell.
    /// -1 indicates an empty cell.
    pub heads: [i32; 400],    // 20x20 grid = 400 cells
    
    /// Array of "Next" pointers for each agent.
    /// next[agent_id] stores the ID of the *next* agent in the same cell.
    /// -1 indicates the end of the list for that cell.
    pub next: [i32; 2000],    // Capacity for 2000 agents
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            heads: [-1; 400],
            next: [-1; 2000],
        }
    }

    /// Resets the grid headers.
    /// Must be called at the start of every frame before re-inserting agents.
    /// Optimization: We don't need to zero out the `next` array, only the heads.
    pub fn clear(&mut self) {
        for h in self.heads.iter_mut() { *h = -1; }
    }

    /// Inserts an agent into the grid.
    /// This is an O(1) operation (prepend to linked list).
    pub fn insert(&mut self, agent_id: usize, pos: (f32, f32)) {
        // Calculate grid coordinates with clamping to avoid out-of-bounds panics
        let gx = ((pos.0 / self.cell_size).floor() as i32).clamp(0, 19);
        let gy = ((pos.1 / self.cell_size).floor() as i32).clamp(0, 19);
        let cell_idx = (gy * 20 + gx) as usize;

        // Standard Linked-List Prepend:
        // 1. Point this agent's "next" to the current head of the cell.
        self.next[agent_id] = self.heads[cell_idx];
        
        // 2. Make this agent the new head of the cell.
        self.heads[cell_idx] = agent_id as i32;
    }

    /// Returns a list of cell indices that overlap with the query radius.
    /// Used for Broad Phase collision detection.
    pub fn get_nearby_cells(&self, pos: (f32, f32), radius: f32) -> Vec<usize> {
        // Determine the bounding box of the query in grid coordinates
        let min_gx = (((pos.0 - radius) / self.cell_size).floor() as i32).clamp(0, 19);
        let max_gx = (((pos.0 + radius) / self.cell_size).floor() as i32).clamp(0, 19);
        let min_gy = (((pos.1 - radius) / self.cell_size).floor() as i32).clamp(0, 19);
        let max_gy = (((pos.1 + radius) / self.cell_size).floor() as i32).clamp(0, 19);

        // Pre-allocate to avoid small re-allocations
        let mut nearby = Vec::with_capacity(25);
        
        for y in min_gy..=max_gy {
            for x in min_gx..=max_gx {
                nearby.push((y * 20 + x) as usize);
            }
        }
        nearby
    }
}
