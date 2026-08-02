## Architecture
![architecture](./photo/arch.svg)
<br>
<br>
<br>
## illustrate
### Source : 
#### 

### Lexer : 
#### 

### Parser : 
#### 

### Eval :
#### 

### Result :
#### 
<br>
<br>
<br>

## Telemetry Ingestion & Processing Pipeline
Experimental computer :
####    OS: Ubuntu 24.04.4 LTS (Noble Numbat) x86_64
####    CPU: AMD Ryzen 5 2600 (12) @ 3.40 GHz
####    GPU: NVIDIA GeForce GTX 1660 [Discrete]
####    Memory: 4.05 GiB / 15.56 GiB
####    Disk (/): 66.96 GiB / 456.35 GiB (15%) - ext4

     
Empirical micro-benchmarks executed using `Criterion` (100 samples per module). The measurements capture the complete end-to-end execution cost, including Linux kernel sys-calls, C-FFI overhead, and in-memory delta calculations:

| Benchmark Target | Latency | Primary Operation & Bottleneck |
| :--- | :--- | :--- |
| **`bench_cpu_module`** | **`101.06 µs`** | Parsing `/proc/stat` & `/proc/cpuinfo` for 12 threads |
| **`bench_gpu_module`** | **`47.795 µs`** | NVML C-FFI driver query via C boundary |
| **`bench_mem_module`** | **`12.335 µs µs`** | Byte-scanning `/proc/meminfo` metrics |
| **`bench_disk_module`** | **`1.3708 µs µs`** | Real `statvfs` Linux kernel syscall & buffer mapping |
| **`Full Telemetry Cycle`** | **`162.5608 µs`** | **End-to-end telemetry ingestion + delta logic** |

---
