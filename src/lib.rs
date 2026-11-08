//! SigmaOS-accelerate — hardware acceleration framework component.
//!
//! Models GPU/CPU/NPU acceleration detection, device enumeration,
//! compute capability, memory info, and acceleration context management.

pub mod device;
pub mod capability;
pub mod context;
pub mod memory;
pub mod manager;
pub mod options;

pub use device::{AccelDevice, DeviceType, DeviceState, DeviceInfo};
pub use capability::{ComputeCapability, FeatureSet, ClockInfo};
pub use context::{AccelContext, ContextConfig, ContextState};
pub use memory::{MemoryInfo, MemoryType, MemoryStats};
pub use manager::{AccelManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_device_type() {
        let types = vec![
            DeviceType::GPU,
            DeviceType::NPU,
            DeviceType::DSP,
            DeviceType::VPU,
            DeviceType::FPGA,
        ];
        assert_eq!(types.len(), 5);
    }
    
    #[test]
    fn test_device_state() {
        let states = vec![
            DeviceState::Available,
            DeviceState::Busy,
            DeviceState::Error,
            DeviceState::Disabled,
        ];
        assert_eq!(states.len(), 4);
    }
    
    fn test_memory_type() {
        let types = vec![
            MemoryType::VRAM,
            MemoryType::Shared,
            MemoryType::HBM,
            MemoryType::LPDDR,
        ];
        assert_eq!(types.len(), 4);
    }
    
    #[test]
    fn test_context_state() {
        let states = vec![
            ContextState::Created,
            ContextState::Active,
            ContextState::Paused,
            ContextState::Destroyed,
        ];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_device_creation() {
        let device = AccelDevice::new("gpu-0".into(), DeviceType::GPU);
        assert_eq!(device.id, "gpu-0");
        assert_eq!(device.device_type, DeviceType::GPU);
        assert_eq!(device.state, DeviceState::Available);
    }
    
    #[test]
    fn test_compute_capability() {
        let cap = ComputeCapability::new(8, 0);
        assert_eq!(cap.major, 8);
        assert_eq!(cap.minor, 0);
        assert_eq!(cap.version_string(), "8.0");
    }
    
    #[test]
    fn test_feature_set() {
        let mut features = FeatureSet::new();
        features.add("cuda");
        features.add("tensor");
        assert!(features.has("cuda"));
        assert!(features.has("tensor"));
        assert!(!features.has("opencl"));
    }
    
    #[test]
    fn test_clock_info() {
        let clock = ClockInfo::new(1500, 2000, 1750);
        assert_eq!(clock.base_mhz, 1500);
        assert_eq!(clock.boost_mhz, 2000);
        assert_eq!(clock.current_mhz, 1750);
    }
    
    #[test]
    fn test_memory_info() {
        let info = MemoryInfo::new(8192, MemoryType::VRAM);
        assert_eq!(info.total_mb, 8192);
        assert_eq!(info.memory_type, MemoryType::VRAM);
    }
    
    #[test]
    fn test_memory_stats() {
        let mut stats = MemoryStats::new();
        stats.record_alloc(1024);
        stats.record_alloc(2048);
        stats.record_free(512);
        
        assert_eq!(stats.total_allocated_mb, 3072);
        assert_eq!(stats.current_allocated_mb, 2560);
    }
    
    #[test]
    fn test_context_creation() {
        let ctx = AccelContext::new("ctx-1".into());
        assert_eq!(ctx.id, "ctx-1");
        assert_eq!(ctx.state, ContextState::Created);
    }
    
    #[test]
    fn test_context_config() {
        let mut config = ContextConfig::default();
        config.max_memory_mb = 4096;
        config.enable_profiling = true;
        assert_eq!(config.max_memory_mb, 4096);
        assert!(config.enable_profiling);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = AccelManager::new();
        assert!(manager.list_devices().is_empty());
    }
    
    #[test]
    fn test_add_device() {
        let mut manager = AccelManager::new();
        let device_id = manager.add_device("gpu-0".into(), DeviceType::GPU);
        
        assert!(manager.get_device(&device_id).is_some());
        assert_eq!(manager.list_devices().len(), 1);
    }
    
    #[test]
    fn test_device_info_setting() {
        let mut manager = AccelManager::new();
        let device_id = manager.add_device("gpu-0".into(), DeviceType::GPU);
        
        let mut info = DeviceInfo::default();
        info.name = "NVIDIA RTX 4090".into();
        info.vendor = "NVIDIA".into();
        
        assert!(manager.set_device_info(&device_id, info));
        assert_eq!(manager.get_device(&device_id).unwrap().info.name, "NVIDIA RTX 4090");
    }
    
    #[test]
    fn test_create_context() {
        let mut manager = AccelManager::new();
        let device_id = manager.add_device("gpu-0".into(), DeviceType::GPU);
        
        let ctx_id = manager.create_context(&device_id);
        assert!(ctx_id.is_some());
        assert!(manager.get_context(ctx_id.as_ref().unwrap()).is_some());
    }
    
    #[test]
    fn test_activate_context() {
        let mut manager = AccelManager::new();
        let device_id = manager.add_device("gpu-0".into(), DeviceType::GPU);
        let ctx_id = manager.create_context(&device_id).unwrap();
        
        assert!(manager.activate_context(&ctx_id));
        assert_eq!(manager.get_context(&ctx_id).unwrap().state, ContextState::Active);
    }
    
    #[test]
    fn test_memory_allocation() {
        let mut manager = AccelManager::new();
        let device_id = manager.add_device("gpu-0".into(), DeviceType::GPU);
        manager.set_device_memory_info(&device_id, MemoryInfo::new(8192, MemoryType::VRAM));
        
        assert!(manager.allocate_memory(&device_id, 4096));
        assert_eq!(manager.get_device_memory(&device_id).unwrap().allocated_mb, 4096);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = AccelManager::new();
        manager.add_device("gpu-0".into(), DeviceType::GPU);
        
        let export = manager.export_data();
        
        let mut manager2 = AccelManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_devices().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = AccelManager::new();
        manager.add_device("gpu-0".into(), DeviceType::GPU);
        manager.add_device("npu-0".into(), DeviceType::NPU);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_devices, 2);
        assert_eq!(stats.total_gpus, 1);
        assert_eq!(stats.total_npus, 1);
    }
}
