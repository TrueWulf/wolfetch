# JSON Output

JSON output is schema version `1`, exposed as the numeric `schema_version`
property.

```json
{
  "schema_version": 1,
  "Distro": "Arch Linux",
  "CPU": "AMD Ryzen 7 5700X 8-Core Processor (6%)",
  "GPU": "NVIDIA GeForce RTX 2060 SUPER (N/A)",
  "startup_ms": 0.25,
  "process_memory_mib": 1.57
}
```

Field keys use the same names as terminal labels and are emitted only when
selected by `show=`. `startup_ms` is emitted when runtime reporting is enabled;
`process_memory_mib` is emitted when process-memory reporting is enabled and is
`null` when unavailable. Truncated text values end with `...`.
