# halo2_device_sample

## Description
This Rust project serves as a reference code only, demonstrating how to accelerate Halo2 using hardware.
It primarily focuses on GPU acceleration, with computation units such as MSM and NTT.
Furthermore, it can be compatible with other hardware devices, such as FPGA, and can also accelerate other computing modules, such as Evaluation.

## How to use
* This project just provides reference code for reference purposes only and cannot be run directly.
* The "src/" path corresponds to the path in "halo2/halo2_proof/src".
* The "test/" path corresponds to the path in "halo2/halo2_proof/test".
* In the "src/device" path, we provide the Device Manager module, which allows computations like MSM and NTT to utilize hardware resources by accessing the Device Manager.
* The Device Manager is compatible with various hardware types, including GPUs and FPGAs.The Device Manager module manages various GPUs and FPGAs, helping users abstract unnecessary hardware details, allowing them to focus on the application layer. It also provides APIs including initialization, execution, deinitialization, and querying, among others. 

## Introduction to Device Manager
[Introduction.md](doc/Introduction.md)
