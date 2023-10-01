#![cfg(any(feature = "fft_cuda"))]

#[macro_use]
extern crate criterion;

use crate::arithmetic::*;
use group::ff::Field;
use halo2_proofs::*;
use halo2curves::bn256::Fr;
use std::ops::{Div, Sub};
use std::time::Instant;

use rand_core::OsRng;

use halo2_proofs::device_manager::*;
use std::fs::File;
use std::io::*;

#[test]
fn fft_test() {
    const MIN_K: u32 = 19;
    const MAX_K: u32 = 24;
    for k in MIN_K..=MAX_K {
        // gen data
        let mut a_gpu = (0..(1 << k)).map(|_| Fr::random(OsRng)).collect::<Vec<_>>();
        let omega = Fr::random(OsRng);

        let mut a_cpu = a_gpu.clone();

        // compute
        let start_cpu = Instant::now();
        best_fft_cpu(&mut a_cpu, omega, k as u32);
        let cpu_cost = Instant::now().sub(start_cpu).as_secs_f64();

        let start_gpu = Instant::now();
        best_fft(&mut a_gpu, omega, k as u32);
        let gpu_cost = Instant::now().sub(start_gpu).as_secs_f64();

        assert_eq!(a_cpu, a_gpu);
        println!(
            "k:{:?} , timer: gpu:{:?}, cpu:{:?}, speedup: {:?}x",
            k,
            gpu_cost,
            cpu_cost,
            cpu_cost.div(gpu_cost)
        );
    }
}

#[test]
fn fft_panda_test() {
    const MIN_K: u32 = 16;
    const MAX_K: u32 = 23;

    let omega = Fr::random(OsRng);

    // compute omega
    let start = Instant::now();
    best_fft_init_gpu(omega.clone());
    let cost = Instant::now().sub(start).as_secs_f64();

    for k in MIN_K..=MAX_K {
        // gen data
        let mut a_gpu = (0..(1 << k)).map(|_| Fr::random(OsRng)).collect::<Vec<_>>();
        let mut a_cpu = a_gpu.clone();
        let mut a_gpu_ec = a_gpu.clone();
        // compute
        let start_cpu = Instant::now();
        best_fft_cpu(&mut a_cpu, omega, k as u32);
        let cpu_cost = Instant::now().sub(start_cpu).as_secs_f64();

        let start_gpu = Instant::now();
        best_fft_gpu(&mut a_gpu, omega, k as u32);
        let panda_gpu_cost = Instant::now().sub(start_gpu).as_secs_f64();
        assert_eq!(a_cpu, a_gpu);

        println!(
            "k:{:?} , timer: init: {:?}, cpu:{:?}, panda-gpu:{:?}, panda-speedup: {:?}x",
            k,
            cost,
            cpu_cost,
            panda_gpu_cost,
            cpu_cost.div(panda_gpu_cost),
        );
    }
}
