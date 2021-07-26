# 🪐 Gravity

Realtime Guitar DSP Effects with Rust

## Serial communication protocol Arduino -> Raspberry

| Byte | Description  |
|------|--------------|
| 0    | 71 (= G)     |
| 1    | 86 (= R)     |
| 2    | 84 (= V)     |
| 3    | 89 (= Y)     |
| 4    | Pot 1 Byte 1 |
| 5    | Pot 1 Byte 2 |
| 6    | Pot 2 Byte 1 |
| 7    | Pot 2 Byte 2 |