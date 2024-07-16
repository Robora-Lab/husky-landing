import casadi as cs
import opengen as og
import numpy as np
import math

N = 60  # The MPC horizon length
NX = 3  # The number of elements in the state vector
NU = 2  # The number of elements in the control vector
sampling_time = 0.1
NSim = 600

Q = cs.DM.eye(NX) * [5.0, 5.0, 0]     #state cost
R = cs.DM.eye(NU) * [0.2, 2]            #contol cost
QN = cs.DM.eye(NX) * [50.0, 50.0, 0.0]

D=cs.DM.eye(NX) * [1.0,1.0,1.0]     #coop cost

def dynamics_ct(_x, _u):
    return cs.vcat([_u[0] * cs.cos(_x[2]),
                    _u[0] * cs.sin(_x[2]),
                    _u[1]])


def dynamics_dt(x, u):
    dx = dynamics_ct(x, u)
    return cs.vcat([x[i] + sampling_time * dx[i] for i in range(NX)])


# The stage cost for x and u
def stage_cost(_x, _u, _x_ref=None, _u_ref=None):
    if _x_ref is None:
        _x_ref = cs.DM.zeros(_x.shape)
    if _u_ref is None:
        _u_ref = cs.DM.zeros(_u.shape)

    dx = _x - _x_ref

    #dx[2] = 0;

    #cs.if_else(dx[2] > math.pi, 2*math.pi - dx[2], dx[2]) # fix to wraparound issue
    #dx[2] = cs.if_else(dx[2] < math.pi, 2*math.pi - dx[2], dx[2]) #

    du = _u - _u_ref
    return cs.mtimes([dx.T, Q, dx]) + cs.mtimes([du.T, R, du])


# The terminal cost for x
def terminal_cost(_x, _x_ref=None):
    if _x_ref is None:
        _x_ref = cs.DM.zeros(_x.shape)
    dx = _x - _x_ref
    return cs.mtimes([dx.T, QN, dx])

# cooperation cost
def coop_cost(_x,_x_ref=None,_cf_ref=None):
    if _x_ref is None:
        _x_ref = cs.DM.zeros(_x.shape)
    if _cf_ref is None:
        _cf_ref = cs.DM.zeros(_x.shape)

    dd = _x_ref - _cf_ref                     #ref pos (aka husky cf_ref) - drone cf_ref
    return cs.mtimes([dd.T, D, dd])


x_0 = cs.MX.sym("x_0", NX)
x_ref = cs.MX.sym("x_ref", NX)
u_k = [cs.MX.sym('u_' + str(i), NU) for i in range(N)]
cf_ref = cs.MX.sym("cf_ref", NX)

# Create the cost function
x_t = x_0
total_cost = 0

for t in range(0, N):
    total_cost += stage_cost(x_t, u_k[t], x_ref)  # update cost
    total_cost += coop_cost(x_t, x_ref,cf_ref)        # add coop cost
    x_t = dynamics_dt(x_t, u_k[t])  # update state

total_cost += terminal_cost(x_t, x_ref)  # terminal cost

optimization_variables = []
optimization_parameters = []

# optim variable = optimizer will change to minimize cost function
# optim parameter = fixed value that influences optimizer

optimization_variables += u_k
optimization_variables.append(x_ref)     #reference is now able to change

optimization_parameters += [x_0]
optimization_parameters.append(x_ref)
optimization_parameters += [cf_ref]   #drone cf_ref received is fixed

optimization_variables = cs.vertcat(*optimization_variables)
optimization_parameters = cs.vertcat(*optimization_parameters)

umin = [-0.10, -0.25] * N  # - cs.DM.ones(NU * N) * cs.inf
umax = [0.35, 0.25] * N  # cs.DM.ones(NU * N) * cs.inf

bounds = og.constraints.Rectangle(umin, umax)

problem = og.builder.Problem(optimization_variables,
                             optimization_parameters,
                             total_cost) \
    .with_constraints(bounds)

ros_config = og.config.RosConfiguration() \
    .with_package_name("open_nmpc_controller") \
    .with_node_name("open_mpc_controller_node") \
    .with_rate((int)(1.0/sampling_time)) \
    .with_description("Cool ROS node.")

build_config = og.config.BuildConfiguration()\
    .with_build_directory("optimization_engine")\
    .with_build_mode("release")\
    .with_build_c_bindings() \
    .with_ros(ros_config)

meta = og.config.OptimizerMeta()\
    .with_optimizer_name("mpc_controller")

solver_config = og.config.SolverConfiguration()\
    .with_tolerance(1e-5)

builder = og.builder.OpEnOptimizerBuilder(problem,
                                          meta,
                                          build_config,
                                          solver_config)
builder.build()
