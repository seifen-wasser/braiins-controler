# Braiins Controller

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

# THIS IS STILL IN DEVELOPMENT
# AND IM STILL LEARNING RUST SO DONT EXPECT GOOD CODE

**Braiins Controller** is a software tool that dynamically controls a BraiinsOS‑based miner based on real‑time power readings from a compatible energy monitoring device.  
By continuously adjusting the miner’s operating parameters, it helps you optimise energy usage, reduce costs, or maintain a power budget – perfect for solar setups, demand‑response scenarios, or simply avoiding circuit overloads.

## How It Works

1. The software periodically polls your power monitoring device (e.g., a Shelly EM) to obtain current power consumption.
2. It compares the reading with your configured target power budget.
3. If the power exceeds the budget, it reduces the miner’s hash rate (by lowering frequency or turning off one or more miners).
4. If power is below the budget, it gradually increases the miner’s performance, up to the maximum allowed.
5. All actions are logged, and you can monitor the system via HTTP endpoints (if enabled).

## Supported Hardware

- **Miners**: Any ASIC miner running **BraiinsOS** (tested on Antminer S19.  
- **Power monitors**: Currently supports **Shelly EM / Shelly 3EM** devices. Other devices can be added by extending the code.
