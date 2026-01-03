pub struct SpatialGrid {
    pub cell_size: f32,
    pub heads: [i32; 400],    // 20x20 células
    pub next: [i32; 2000],    // 2000 agentes
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            heads: [-1; 400],
            next: [-1; 2000],
        }
    }

    pub fn clear(&mut self) {
        for h in self.heads.iter_mut() { *h = -1; }
        // O array 'next' não precisa ser limpo completamente, 
        // pois será sobrescrito durante as inserções baseadas nos 'heads'.
    }

    pub fn insert(&mut self, agent_id: usize, pos: (f32, f32)) {
        let gx = ((pos.0 / self.cell_size).floor() as i32).clamp(0, 19);
        let gy = ((pos.1 / self.cell_size).floor() as i32).clamp(0, 19);
        let cell_idx = (gy * 20 + gx) as usize;

        // Inserção no início da lista (O(1))
        self.next[agent_id] = self.heads[cell_idx];
        self.heads[cell_idx] = agent_id as i32;
    }

    pub fn get_nearby_cells(&self, pos: (f32, f32), radius: f32) -> Vec<usize> {
        let min_gx = (((pos.0 - radius) / self.cell_size).floor() as i32).clamp(0, 19);
        let max_gx = (((pos.0 + radius) / self.cell_size).floor() as i32).clamp(0, 19);
        let min_gy = (((pos.1 - radius) / self.cell_size).floor() as i32).clamp(0, 19);
        let max_gy = (((pos.1 + radius) / self.cell_size).floor() as i32).clamp(0, 19);

        let mut nearby = Vec::with_capacity(25);
        for y in min_gy..=max_gy {
            for x in min_gx..=max_gx {
                nearby.push((y * 20 + x) as usize);
            }
        }
        nearby
    }
}
