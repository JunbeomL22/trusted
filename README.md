# tRusted

## Risk and Trading Engine

## Overview

This Rust project consists of two main components:
1. `quantlib` - A library for quantitative finance computations. For now, only plain products can be priced. Development for pricing structured products is ongoing. see [engine_example.rs](./examples/toymodel/src/bin/engine_example.rs) for quantlib example.
2. `trading-engine` - (under development) A library for simulating trading activities and order management.

## Crate structure
### quantlib
| Module | Description |
| ------ | ----------- |
| [data](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/data) | Raw market observatiions, which are not directly used for calculation <br>  Data is shared by Engine object in multi-thread environment|
| [parameters](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/parameters) | Objects generated from data objects for actual calculation |
| [isntruments](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/instruments) | ex) Futures, FxFutures, FxForward, FxSwap, VanillaOption, IRS, CCS, Bond, KtbFutures|
| [time](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/time) | Calendars, conventions, handling holiday |
| [pricing_engines](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/pricing_engines) | Engine, EngineGenerator, and Pricer |

| Struct \& Enum | Description |
|------- | ----------- |
|[CalculationConfiguration](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/pricing_engines/calculation_configuration.rs) | All information for pricing: delta bump ratio, gap days for theta calculation, etc
| [Pricer](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/pricing_engines/pricer.rs) | Enum containing pricers for each [Instrument](./trusted/quantlib/src/instrument.rs) |
| [Engine](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/pricing_engines/engine.rs) | An Engine takes data as Arc objects and creates parameters such as [ZeroCurve](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/parameters/zero_curve.rs), [DiscreteDividendRatio](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/parameters/discrete_dividend_ratio.rs), etc. The parameters, as Rc<RefCell<..>> objects, are shared only inside the Engine. Then the Engine excutes Pricers repeatedly for calculating risks, e.g., delta, gamma, theta, rho, etc|
| [CalculationResult](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/pricing_engines/calculation_result.rs)| price, greeks, cashflows |
| [EngineGenerator](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/pricing_engines/engine_generator.rs) | EngineGnerator groups instruments according to [InstrumentCategory](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/pricing_engines/engine_generator.rs), then [Engine](https://github.com/JunbeomL22/trusted/tree/main/quantlib/src/pricing_engines/engine.rs)s are created for each group of instruments. The purpose of separation is mmainly for compuation performance. This is especially useful for Monte-Carlo simulation (not yet developed) since the most of the computation cost in MC simulation is caused by path generation. |


```mermaid
---
title: quantlib structure
---
stateDiagram-v2
    EngineGenerator --> Engine1: instruments <br> data <br> calc config
    EngineGenerator --> Engine2: instruments <br> data <br> calc config
    EngineGenerator --> Engine3: instruments <br> data <br> calc config
    Engine1 --> CalculationResult: merge
    Engine2 --> CalculationResult: merge
    Engine3 --> CalculationResult: merge
```