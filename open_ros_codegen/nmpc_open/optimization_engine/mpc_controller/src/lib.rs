//
// Auto-generated file by OptimizationEngine
// See https://alphaville.github.io/optimization-engine/
//
// Generated at: 2024-07-02 15:22:35.239851
//

use icasadi_mpc_controller;
use libc::{c_double, c_ulong, c_ulonglong};

use optimization_engine::{constraints::*, panoc::*, alm::*, *};

// ---Private Constants----------------------------------------------------------------------------------

/// Tolerance of inner solver
const EPSILON_TOLERANCE: f64 = 1e-05;

/// Initial tolerance
const INITIAL_EPSILON_TOLERANCE: f64 = 0.0001;

/// Update factor for inner tolerance
const EPSILON_TOLERANCE_UPDATE_FACTOR: f64 = 0.1;

/// Delta tolerance
const DELTA_TOLERANCE: f64 = 0.0001;

/// LBFGS memory
const LBFGS_MEMORY: usize = 10;

/// Maximum number of iterations of the inner solver
const MAX_INNER_ITERATIONS: usize = 500;

/// Maximum number of outer iterations
const MAX_OUTER_ITERATIONS: usize = 10;

/// Maximum execution duration in microseconds
const MAX_DURATION_MICROS: u64 = 5000000;

/// Penalty update factor
const PENALTY_UPDATE_FACTOR: f64 = 5.0;

/// Initial penalty
const INITIAL_PENALTY_PARAMETER: f64 = 1.0;

/// Sufficient decrease coefficient
const SUFFICIENT_INFEASIBILITY_DECREASE_COEFFICIENT: f64 = 0.1;


// ---Public Constants-----------------------------------------------------------------------------------

/// Number of decision variables
pub const MPC_CONTROLLER_NUM_DECISION_VARIABLES: usize = 120;

/// Number of parameters
pub const MPC_CONTROLLER_NUM_PARAMETERS: usize = 6;

/// Number of parameters associated with augmented Lagrangian
pub const MPC_CONTROLLER_N1: usize = 0;

/// Number of penalty constraints
pub const MPC_CONTROLLER_N2: usize = 0;

// ---Export functionality from Rust to C/C++------------------------------------------------------------

/// Solver cache (structure `mpc_controllerCache`)
///
#[allow(non_camel_case_types)]
#[no_mangle]
pub struct mpc_controllerCache {
    cache: AlmCache,
}

impl mpc_controllerCache {
    pub fn new(cache: AlmCache) -> Self {
        mpc_controllerCache { cache }
    }
}

/// mpc_controller version of ExitStatus
/// Structure: `mpc_controllerExitStatus`
#[allow(non_camel_case_types)]
#[repr(C)]
#[no_mangle]
pub enum mpc_controllerExitStatus {
    /// The algorithm has converged
    ///
    /// All termination criteria are satisfied and the algorithm
    /// converged within the available time and number of iterations
    mpc_controllerConverged,
    /// Failed to converge because the maximum number of iterations was reached
    mpc_controllerNotConvergedIterations,
    /// Failed to converge because the maximum execution time was reached
    mpc_controllerNotConvergedOutOfTime,
    /// If the gradient or cost function cannot be evaluated internally
    mpc_controllerNotConvergedCost,
    /// Computation failed and NaN/Infinite value was obtained
    mpc_controllerNotConvergedNotFiniteComputation,
}

/// mpc_controller version of AlmOptimizerStatus
/// Structure: `mpc_controllerSolverStatus`
///
#[repr(C)]
#[no_mangle]
pub struct mpc_controllerSolverStatus {
    /// Exit status
    exit_status: mpc_controllerExitStatus,
    /// Number of outer iterations
    num_outer_iterations: c_ulong,
    /// Total number of inner iterations
    ///
    /// This is the sum of the numbers of iterations of
    /// inner solvers
    num_inner_iterations: c_ulong,
    /// Norm of the fixed-point residual of the the problem
    last_problem_norm_fpr: c_double,
    /// Total solve time
    solve_time_ns: c_ulonglong,
    /// Penalty value
    penalty: c_double,
    /// Norm of delta y divided by the penalty parameter
    delta_y_norm_over_c: c_double,
    /// Norm of F2(u)
    f2_norm: c_double,
    /// Value of cost function at solution
    cost: c_double,
    /// Lagrange multipliers
    lagrange: *const c_double
    }

/// Allocate memory and setup the solver
#[no_mangle]
pub extern "C" fn mpc_controller_new() -> *mut mpc_controllerCache {
    Box::into_raw(Box::new(mpc_controllerCache::new(initialize_solver())))
}

/// Solve the parametric optimization problem for a given parameter
/// .
/// .
/// Arguments:
/// - `instance`: re-useable instance of AlmCache, which should be created using
///   `mpc_controller_new` (and should be destroyed once it is not
///   needed using `mpc_controller_free`
/// - `u`: (on entry) initial guess of solution, (on exit) solution
///   (length: `MPC_CONTROLLER_NUM_DECISION_VARIABLES`)
/// - `params`:  static parameters of the optimizer
///   (length: `MPC_CONTROLLER_NUM_PARAMETERS`)
/// - `y0`: Initial guess of Lagrange multipliers (if `0`, the default will
///   be used; length: `MPC_CONTROLLER_N1`)
/// - `c0`: Initial penalty parameter (provide `0` to use the default initial
///   penalty parameter
/// .
/// .
/// Returns:
/// Instance of `mpc_controllerSolverStatus`, with the solver status
/// (e.g., number of inner/outer iterations, measures of accuracy, solver time,
/// and the array of Lagrange multipliers at the solution).
///
#[no_mangle]
pub extern "C" fn mpc_controller_solve(
    instance: *mut mpc_controllerCache,
    u: *mut c_double,
    params: *const c_double,
    y0: *const c_double,
    c0: *const c_double,
) -> mpc_controllerSolverStatus {

    // Convert all pointers into the required data structures
    let instance: &mut mpc_controllerCache = unsafe {
        assert!(!instance.is_null());
        &mut *instance
    };

    // "*mut c_double" to "&mut [f64]"
    let u : &mut [f64] = unsafe {
        assert!(!u.is_null());
        std::slice::from_raw_parts_mut(u as *mut f64, MPC_CONTROLLER_NUM_DECISION_VARIABLES)
    };

    // "*const c_double" to "&[f64]"
    let params : &[f64] = unsafe {
        assert!(!params.is_null());
        std::slice::from_raw_parts(params as *const f64, MPC_CONTROLLER_NUM_PARAMETERS)
    };

    let c0_option: Option<f64> = if c0.is_null() {
        None::<f64>
    } else {
        Some(unsafe { *c0 })
    };

    let y0_option: Option<Vec<f64>> = if y0.is_null() {
        None::<Vec<f64>>
    } else {
        Some(unsafe { std::slice::from_raw_parts(y0 as *mut f64, MPC_CONTROLLER_N1).to_vec() })
    };

    // Invoke `solve`
    let status = solve(params,&mut instance.cache, u, &y0_option, &c0_option);

    // Check solution status and cast it as `mpc_controllerSolverStatus`
    match status {
        Ok(status) => mpc_controllerSolverStatus {
            exit_status: match status.exit_status() {
                core::ExitStatus::Converged => mpc_controllerExitStatus::mpc_controllerConverged,
                core::ExitStatus::NotConvergedIterations => mpc_controllerExitStatus::mpc_controllerNotConvergedIterations,
                core::ExitStatus::NotConvergedOutOfTime => mpc_controllerExitStatus::mpc_controllerNotConvergedOutOfTime,
            },
            num_outer_iterations: status.num_outer_iterations() as c_ulong,
            num_inner_iterations: status.num_inner_iterations() as c_ulong,
            last_problem_norm_fpr: status.last_problem_norm_fpr(),
            solve_time_ns: status.solve_time().as_nanos() as c_ulonglong,
            penalty: status.penalty() as c_double,
            delta_y_norm_over_c: status.delta_y_norm_over_c() as c_double,
            f2_norm: status.f2_norm() as c_double,
            cost: status.cost() as c_double,
            lagrange: match status.lagrange_multipliers() {
                Some(_y) => {
                    0 as *const c_double
                
                },
                None => {
                    0 as *const c_double
                }
            }
        },
        Err(e) => mpc_controllerSolverStatus {
            exit_status: match e {
                SolverError::Cost => mpc_controllerExitStatus::mpc_controllerNotConvergedCost,
                SolverError::NotFiniteComputation => mpc_controllerExitStatus::mpc_controllerNotConvergedNotFiniteComputation,
            },
            num_outer_iterations: std::u64::MAX as c_ulong,
            num_inner_iterations: std::u64::MAX as c_ulong,
            last_problem_norm_fpr: std::f64::INFINITY,
            solve_time_ns: std::u64::MAX as c_ulonglong,
            penalty: std::f64::INFINITY as c_double,
            delta_y_norm_over_c: std::f64::INFINITY as c_double,
            f2_norm: std::f64::INFINITY as c_double,
            cost: std::f64::INFINITY as c_double,
            lagrange:0 as *const c_double
        },
    }
}

/// Deallocate the solver's memory, which has been previously allocated
/// using `mpc_controller_new`
#[no_mangle]
pub extern "C" fn mpc_controller_free(instance: *mut mpc_controllerCache) {
    // Add impl
    unsafe {
        assert!(!instance.is_null());
        Box::from_raw(instance);
    }
}


// ---Parameters of the constraints----------------------------------------------------------------------

const CONSTRAINTS_XMIN :Option<&[f64]> = Some(&[-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,-0.1,-0.25,]);
const CONSTRAINTS_XMAX :Option<&[f64]> = Some(&[0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,0.35,0.25,]);










// ---Internal private helper functions------------------------------------------------------------------

/// Make constraints U
fn make_constraints() -> impl Constraint {

    let bounds = Rectangle::new(CONSTRAINTS_XMIN, CONSTRAINTS_XMAX);
    bounds
}





// ---Main public API functions--------------------------------------------------------------------------


/// Initialisation of the solver
pub fn initialize_solver() -> AlmCache {
    let panoc_cache = PANOCCache::new(MPC_CONTROLLER_NUM_DECISION_VARIABLES, EPSILON_TOLERANCE, LBFGS_MEMORY);
    let alm_cache = AlmCache::new(panoc_cache, MPC_CONTROLLER_N1, MPC_CONTROLLER_N2);

    alm_cache
}


/// Solver interface
pub fn solve(
    p: &[f64],
    alm_cache: &mut AlmCache,
    u: &mut [f64],
    y0: &Option<Vec<f64>>,
    c0: &Option<f64>,
) -> Result<AlmOptimizerStatus, SolverError> {

    assert_eq!(p.len(), MPC_CONTROLLER_NUM_PARAMETERS, "Wrong number of parameters (p)");
    assert_eq!(u.len(), MPC_CONTROLLER_NUM_DECISION_VARIABLES, "Wrong number of decision variables (u)");

    let psi = |u: &[f64], xi: &[f64], cost: &mut f64| -> Result<(), SolverError> {
        icasadi_mpc_controller::cost(&u, &xi, &p, cost);
        Ok(())
    };
    let grad_psi = |u: &[f64], xi: &[f64], grad: &mut [f64]| -> Result<(), SolverError> {
        icasadi_mpc_controller::grad(&u, &xi, &p, grad);
        Ok(())
    };
    
    let bounds = make_constraints();

    let alm_problem = AlmProblem::new(
        bounds,
        NO_SET,
        NO_SET,
        psi,
        grad_psi,
        NO_MAPPING,
        NO_MAPPING,
        MPC_CONTROLLER_N1,
        MPC_CONTROLLER_N2,
    );

    let mut alm_optimizer = AlmOptimizer::new(alm_cache, alm_problem)
        .with_delta_tolerance(DELTA_TOLERANCE)
        .with_epsilon_tolerance(EPSILON_TOLERANCE)
        .with_initial_inner_tolerance(INITIAL_EPSILON_TOLERANCE)
        .with_inner_tolerance_update_factor(EPSILON_TOLERANCE_UPDATE_FACTOR)
        .with_max_duration(std::time::Duration::from_micros(MAX_DURATION_MICROS))
        .with_max_outer_iterations(MAX_OUTER_ITERATIONS)
        .with_max_inner_iterations(MAX_INNER_ITERATIONS)
        .with_initial_penalty(c0.unwrap_or(INITIAL_PENALTY_PARAMETER))
        .with_penalty_update_factor(PENALTY_UPDATE_FACTOR)
        .with_sufficient_decrease_coefficient(SUFFICIENT_INFEASIBILITY_DECREASE_COEFFICIENT);

    // solve the problem using `u` an the initial condition `u` and
    // initial vector of Lagrange multipliers, if provided;
    // returns the problem status (instance of `AlmOptimizerStatus`)
    if let Some(y0_) = y0 {
        let mut alm_optimizer = alm_optimizer.with_initial_lagrange_multipliers(&y0_);
        alm_optimizer.solve(u)
    } else {
        alm_optimizer.solve(u)
    }

}