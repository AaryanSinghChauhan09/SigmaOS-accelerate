//! Memory information for acceleration devices.

/// Memory type
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryType {
    /// VRAM memory
    VRAM,
    /// Shared memory
    Shared,
    /// HBM memory
    HBM,
    /// LPDDR memory
    LPDDR,
}

/// Memory info
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    /// Total memory in MB
    pub total_mb: u64,
    /// Free memory in MB
    pub free_mb: u64,
    /// Allocated memory in MB
    pub allocated_mb: u64,
    /// Memory type
    pub memory_type: MemoryType,
    /// Memory bandwidth in GB/s
    pub bandwidth_gbps: f64,
    /// Memory latency in nanoseconds
    pub latency_ns: u64,
    /// Created timestamp
    pub created_at: u64,
}

impl MemoryInfo {
    /// Create new memory info
    pub fn new(total_mb: u64, memory_type: MemoryType) -> Self {
        Self {
            total_mb,
            free_mb: total_mb,
            allocated_mb: 0,
            memory_type,
            bandwidth_gbps: 0.0,
            latency_ns: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Set bandwidth
    pub fn set_bandwidth(&mut self, bandwidth_gbps: f64) {
        self.bandwidth_gbps = bandwidth_gbps;
    }
    
    /// Set latency
    pub fn set_latency(&mut self, latency_ns: u64) {
        self.latency_ns = latency_ns;
    }
    
    /// Allocate memory
    pub fn allocate(&mut self, mb: u64) -> bool {
        if self.free_mb >= mb {
            self.free_mb -= mb;
            self.allocated_mb += mb;
            return true;
        }
        false
    }
    
    /// Free memory
    pub fn free(&mut self, mb: u64) -> bool {
        if self.allocated_mb >= mb {
            self.allocated_mb -= mb;
            self.free_mb += mb;
            return true;
        }
        false
    }
    
    /// Get memory usage percentage
    pub fn get_usage_percent(&self) -> f64 {
        if self.total_mb == 0 {
            0.0
        } else {
            (self.allocated_mb as f64 / self.total_mb as f64) * 100.0
        }
    }
    
    /// Get free memory percentage
    pub fn get_free_percent(&self) -> f64 {
        if self.total_mb == 0 {
            0.0
        } else {
            (self.free_mb as f64 / self.total_mb as f64) * 100.0
        }
    }
    
    /// Check if has free memory
    pub fn has_free_memory(&self) -> bool {
        self.free_mb > 0
    }
    
    /// Check if memory is full
    pub fn is_full(&self) -> bool {
        self.free_mb == 0
    }
    
    /// Get memory info age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
}

/// Memory stats
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Total allocated in MB
    pub total_allocated_mb: u64,
    /// Current allocated in MB
    pub current_allocated_mb: u64,
    /// Total freed in MB
    pub total_freed_mb: u64,
    /// Allocation count
    pub alloc_count: u64,
    /// Free count
    pub free_count: u64,
    /// Peak allocated in MB
    pub peak_allocated_mb: u64,
    /// Created timestamp
    pub created_at: u64,
    /// Updated timestamp
    pub updated_at: u64,
}

impl MemoryStats {
    /// Create new memory stats
    pub fn new() -> Self {
        Self {
            total_allocated_mb: 0,
            current_allocated_mb: 0,
            total_freed_mb: 0,
            alloc_count: 0,
            free_count: 0,
            peak_allocated_mb: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Record allocation
    pub fn record_alloc(&mut self, mb: u64) {
        self.total_allocated_mb += mb;
        self.current_allocated_mb += mb;
        self.alloc_count += 1;
        
        if self.current_allocated_mb > self.peak_allocated_mb {
            self.peak_allocated_mb = self.current_allocated_mb;
        }
        
        self.update_timestamp();
    }
    
    /// Record free
    pub fn record_free(&mut self, mb: u64) {
        self.total_freed_mb += mb;
        self.current_allocated_mb = self.current_allocated_mb.saturating_sub(mb);
        self.free_count += 1;
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Get allocation success rate
    pub fn get_alloc_success_rate(&self) -> f64 {
        let total_ops = self.alloc_count + self.free_count;
        if total_ops == 0 {
            0.0
        } else {
            (self.alloc_count as f64 / total_ops as f64) * 100.0
        }
    }
    
    /// Get memory efficiency
    pub fn get_memory_efficiency(&self) -> f64 {
        if self.total_allocated_mb == 0 {
            0.0
        } else {
            (self.current_allocated_mb as f64 / self.total_allocated_mb as f64) * 100.0
        }
    }
    
    /// Reset stats
    pub fn reset(&mut self) {
        *self = Self::new();
    }
    
    /// Get stats age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Check if stats have allocations
    pub fn has_allocations(&self) -> bool {
        self.alloc_count > 0
    }
    
    /// Check if stats have frees
    pub fn has_frees(&self) -> bool {
        self.free_count > 0
    }
    
    /// Check if stats have allocated memory
    pub fn has_allocated_memory(&self) -> bool {
        self.current_allocated_mb > 0
    }
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self::new()
    }
}
