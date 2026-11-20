# SigmaOS-accelerate Component Wiki

## Overview
SigmaOS-accelerate is a SigmaOS component providing hardware acceleration framework with GPU/CPU/NPU detection, compute capability, and memory management. It serves as the foundation for hardware-accelerated computing within the SigmaOS ecosystem.

## Principles
- Single Responsibility
- Hardware Abstraction
- Fail-Fast on Unsupported Hardware
- Deterministic Detection
- Clear Abstraction Layer
- Composition over Inheritance for hardware backends

## Architecture
SigmaOS-accelerate follows a hardware abstraction layer pattern:
- **Detection Module**: Identifies available hardware (GPU, CPU, NPU)
- **Capability Module**: Reports compute capabilities and memory info
- **Management Module**: Handles resource allocation and scheduling
- **Backend Abstraction**: Composition of different hardware backend implementations

## API
The component provides the following public APIs:
- `hardware_acceleration::detect()` - Detect available acceleration hardware
- `hardware_acceleration::Capability` - Struct with compute capability info
- `gpu::Management` - GPU resource management
- `npu::Management` - NPU resource management
- Error types for unsupported hardware scenarios

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific hardware configuration
2. **Act**: Execute the code under test (detection, capability query)
3. **Assert**: Verify the expected hardware detection results

Example:
```rust
#[test]
fn test_hardware_detection_aaa() {
    // Arrange
    let expected_backends = vec!["cpu", "gpu"];
    
    // Act
    let available = hardware_acceleration::detect();
    
    // Assert
    assert!(available.contains(&"cpu".to_string()));
    // NPU may not be available in test environment
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Hardware detection is best-effort; gracefully degrades on unsupported hardware
- Comprehensive error types for all failure scenarios
- Unit tests provided for detection logic
- Integration tests for full hardware initialization

## TODO
- Add CUDA/OpenCL backend implementation
- Expand NPU detection capabilities
- Add memory management APIs
- Create integration tests with actual hardware
- Expand capability reporting