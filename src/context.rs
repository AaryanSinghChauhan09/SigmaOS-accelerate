//! Acceleration context definitions.

/// Context state
#[derive(Debug, Clone, PartialEq)]
pub enum ContextState {
    /// Created context
    Created,
    /// Active context
    Active,
    /// Paused context
    Paused,
    /// Destroyed context
    Destroyed,
}

/// Context config
#[derive(Debug, Clone)]
pub struct ContextConfig {
    /// Max memory in MB
    pub max_memory_mb: u64,
    /// Enable profiling
    pub enable_profiling: bool,
    /// Enable debugging
    pub enable_debugging: bool,
    /// Compute mode
    pub compute_mode: ComputeMode,
    /// Stream priority
    pub stream_priority: StreamPriority,
}

/// Compute mode
#[derive(Debug, Clone, PartialEq)]
pub enum ComputeMode {
    /// Default mode
    Default,
    /// High performance mode
    HighPerformance,
    /// Power saving mode
    PowerSaving,
}

/// Stream priority
#[derive(Debug, Clone, PartialEq)]
pub enum StreamPriority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 4096,
            enable_profiling: false,
            enable_debugging: false,
            compute_mode: ComputeMode::Default,
            stream_priority: StreamPriority::Medium,
        }
    }
}

/// Acceleration context
#[derive(Debug, Clone)]
pub struct AccelContext {
    /// Context ID
    pub id: String,
    /// Context state
    pub state: ContextState,
    /// Context config
    pub config: ContextConfig,
    /// Context device ID
    pub device_id: String,
    /// Context memory allocated in MB
    pub memory_allocated_mb: u64,
    /// Context streams count
    pub stream_count: u32,
    /// Context created timestamp
    pub created_at: u64,
    /// Context activated timestamp
    pub activated_at: Option<u64>,
    /// Context destroyed timestamp
    pub destroyed_at: Option<u64>,
}

impl AccelContext {
    /// Create new acceleration context
    pub fn new(id: String) -> Self {
        Self {
            id,
            state: ContextState::Created,
            config: ContextConfig::default(),
            device_id: String::new(),
            memory_allocated_mb: 0,
            stream_count: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            activated_at: None,
            destroyed_at: None,
        }
    }
    
    /// Set device ID
    pub fn set_device_id(&mut self, device_id: String) {
        self.device_id = device_id;
    }
    
    /// Set config
    pub fn set_config(&mut self, config: ContextConfig) {
        self.config = config;
    }
    
    /// Activate context
    pub fn activate(&mut self) -> bool {
        if self.state == ContextState::Created || self.state == ContextState::Paused {
            self.state = ContextState::Active;
            self.activated_at = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
            return true;
        }
        false
    }
    
    /// Pause context
    pub fn pause(&mut self) -> bool {
        if self.state == ContextState::Active {
            self.state = ContextState::Paused;
            return true;
        }
        false
    }
    
    /// Destroy context
    pub fn destroy(&mut self) -> bool {
        if self.state != ContextState::Destroyed {
            self.state = ContextState::Destroyed;
            self.destroyed_at = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
            return true;
        }
        false
    }
    
    /// Allocate memory
    pub fn allocate_memory(&mut self, mb: u64) -> bool {
        if self.memory_allocated_mb + mb <= self.config.max_memory_mb {
            self.memory_allocated_mb += mb;
            return true;
        }
        false
    }
    
    /// Free memory
    pub fn free_memory(&mut self, mb: u64) -> bool {
        if self.memory_allocated_mb >= mb {
            self.memory_allocated_mb -= mb;
            return true;
        }
        false
    }
    
    /// Add stream
    pub fn add_stream(&mut self) {
        self.stream_count += 1;
    }
    
    /// Remove stream
    pub fn remove_stream(&mut self) {
        if self.stream_count > 0 {
            self.stream_count -= 1;
        }
    }
    
    /// Check if context is active
    pub fn is_active(&self) -> bool {
        self.state == ContextState::Active
    }
    
    /// Check if context is destroyed
    pub fn is_destroyed(&self) -> bool {
        self.state == ContextState::Destroyed
    }
    
    /// Get context age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get context lifetime in seconds
    pub fn get_lifetime(&self) -> Option<u64> {
        self.destroyed_at.map(|destroyed| destroyed.saturating_sub(self.created_at))
    }
    
    /// Check if context has memory allocated
    pub fn has_memory_allocated(&self) -> bool {
        self.memory_allocated_mb > 0
    }
    
    /// Check if context has streams
    pub fn has_streams(&self) -> bool {
        self.stream_count > 0
    }
    
    /// Get memory usage percentage
    pub fn get_memory_usage_percent(&self) -> f64 {
        if self.config.max_memory_mb == 0 {
            0.0
        } else {
            (self.memory_allocated_mb as f64 / self.config.max_memory_mb as f64) * 100.0
        }
    }
}
