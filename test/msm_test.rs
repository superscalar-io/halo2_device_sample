#![cfg(any(feature = "msm_cuda"))]

#[macro_use]
extern crate criterion;

use ark_std::{end_timer, start_timer};
use group::{ff::Field, Curve, Group};
use halo2_proofs::{
    arithmetic::*,
    poly::{commitment::ParamsProver, kzg::commitment::ParamsKZG},
};

use halo2_proofs::device_manager::*;

use halo2curves::bn256::{Bn256, Fr};
use rand_core::OsRng;
use std::ops::{Div, Sub};
use std::{println, time::Instant};

#[test]
fn msm_test_multi_params() {
    let k = 13u32;
    let agg_k: u32 = std::env::var("DEGREE")
        .unwrap_or_else(|_| "18".to_string())
        .parse()
        .expect("Cannot parse DEGREE env var as u32");

    let rng = OsRng;

    let coeffs1 = (0..1 << k).map(|_| Fr::random(OsRng)).collect::<Vec<_>>();
    let coeffs2 = (0..1 << agg_k)
        .map(|_| Fr::random(OsRng))
        .collect::<Vec<_>>();

    let params1 = ParamsKZG::<Bn256>::new(k);
    let params2 = ParamsKZG::<Bn256>::new(agg_k);

    let start_init = Instant::now();
    best_init_gpu(
        params1.id,
        &[
            &params1.get_g_lagrange().as_slice(),
            &params1.get_g().as_slice(),
        ],
    );
    best_init_gpu(
        params2.id,
        &[
            &params2.get_g_lagrange().as_slice(),
            &params2.get_g().as_slice(),
        ],
    );
    let init_cost = Instant::now().sub(start_init).as_secs_f64();

    let params1_g_lagrange = params1.get_g_lagrange().clone();
    let params1_g = params1.get_g().clone();
    let params2_g_lagrange = params2.get_g_lagrange().clone();
    let params2_g = params2.get_g().clone();

    let test_cases = vec![
        ("case1", &coeffs1, &params1_g_lagrange, params1.id, 0),
        ("case2", &coeffs1, &params1_g, params1.id, 1),
        ("case3", &coeffs2, &params2_g_lagrange, params2.id, 0),
        ("case4", &coeffs2, &params2_g, params2.id, 1),
    ];

    for (case_name, coeffs, bases, id, index) in test_cases.iter() {
        println!("\n========= {} =========", case_name);

        let start_cpu = Instant::now();
        let cpu_result = best_multiexp(coeffs, bases).to_affine();
        let cpu_cost = Instant::now().sub(start_cpu).as_secs_f64();

        let start_gpu = Instant::now();
        let gpu_result = best_multiexp_gpu(coeffs, bases, *id, *index as usize).to_affine();
        let gpu_cost = Instant::now().sub(start_gpu).as_secs_f64();

        assert_eq!(cpu_result, gpu_result);

        println!(
            "k:{:?} , timer: init: {:?}, gpu:{:?}, cpu:{:?}, speedup: {:?}x",
            k,
            init_cost,
            gpu_cost,
            cpu_cost,
            cpu_cost.div(gpu_cost)
        );

        println!("\n");
    }
}
