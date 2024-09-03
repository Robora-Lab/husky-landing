Ingenuity Labs, Robora Labs, Cooperative UAV-USV Landing Project

Author: William Stewart

Date: 2024/06/21

Email: 21wss3@queensu.ca

Emergency Email: willst3wart@gmail.com

Github:
https://github.com/Robora-Lab/husky_crazyflie_landing

System:
  ROS 1 Noetic
  Ubuntu 20.04.06
  Raspberry Pi 4B
  Arduino Uno

Raspberry Pi "RoboraPi" Login:
  Username: will
  Password: 321

Husky User Manual:
https://docs.clearpathrobotics.com/docs/robots/outdoor_robots/husky/user_manual_husky/
If you get a yellow light when trying to drive the Husky, check: lockout key, e-stop, and change the battery.

Once laptop is on "Lab" wifi, to ssh into Pi:
  Shell: `ssh -l will 172.20.42.##`               last two digits (##) change. To see what they are, plug Pi into monitor and keyboard (before booting) and run `$ ip addr show`.


To drive the robot using logitech controller with blue tape on back:
  `$ screen -S s1`                                to open a screen named s1 - keeps program running even if disconnection occurs.
  `$ roslaunch wavemodel_pkg wavemodel.launch`    to run the program that lets you drive the landing pad and have the waves respond.

  Hold down LB for slow or RB for fast and use left joystick to drive husky. Be safe.
  Orange switch on deck controls power to landing pad. It performs start-up animation.
  Vicon system has to be powered on with its computer connected to lab wifi and WillHusky selected in the vicon tracker software for the platform to respond to the wave model.
  If you can't see the Husky driving around in the Vicon software then it won't work. If you've never used the Vicon system before, ask to get help.
  The red "B" button on the Logitech controller functions as an e-stop. This will also work while using MPC.

To drive the robot using MPC:
  Tutorial used:
  https://alphaville.github.io/optimization-engine/docs/example_navigation_ros_codegen#code-generation-for-the-mpc-controller.

  To run the MPC program, first do `$ screen -S s2` then `$ roslaunch open_nmpc_controller open_optimizer.launch`.
  If you get a Master error it's because wavemodel.launch should be running too.
  The Husky will not move without a goal position. You can publish to the goalPose topic using using:

  rostopic pub /goalPose geometry_msgs/PoseStamped "header:
  seq: 0
  stamp:
    secs: 0
    nsecs: 0
  frame_id: ''
pose:
  position:
    x: 1.0
    y: 1.0
    z: 0.0
  orientation:
    x: 0.0
    y: 0.0
    z: 0.0
    w: 3.14"
    NOTE: Only pose x, y and orientation w matter. W is being used to represent the yaw angle in radians, with 0 being aligned with the positive x axis.

    The green "A" button on the Logitech controller is used to pause and un-pause the MPC.
    In the paused state, computation still occurs, but no commands will be sent to the motors.

Making Changes:

  If you wish to change the code for tilting the landing pad, wave_node.cpp is located in `~/landingpad_ws/src/wavemodel_pkg/scripts`
  You will need to execute `$ cd ~/landingpad_ws/` then `$ catkin_make` to compile the code after any changes are made.

  If you wish to change the Arduino code, which you really shouldn't need to do, it can be found on the github:
  https://github.com/Robora-Lab/husky_crazyflie_landing

  The MPC uses code-gen as specified in this tutorial: https://alphaville.github.io/optimization-engine/docs/example_navigation_ros_codegen#code-generation-for-the-mpc-controller.

  If you wish to change JUST the MPC parameters, such as Q and R matrices, Umin and Umax: 
  * Make those changes by doing `$ nano ~/husky_crazyflie_landing/open_ros_codegen/nmpc_open/create_open_solver.py`.
  * After you change the MPC parameters, you will need to `$ cd ~/husky_crazyflie_landing/open_ros_codegen/nmpc_open/` then `$ python create_open_solver.py`.
  * `cd ~/husky_crazyflie_landing/mpc_ws/src/open_nmpc_controller/extern_lib` and delete the file `libmpc_controller.a` using the command `rm libmpc_controller`
  * `cd ~/husky_crazyflie_landing/open_ros_codegen/nmpc_open/open_nmpc_controller/extern_lib` and copy the new `libmpc_controller.a` file to the directory where we just deleted the deprecated version using the command `cp libmpc_controller.a /husky_crazyflie_landing/mpc_ws/src/open_nmpc_controller/extern_lib`
  * Then you MUST compile the C++ node that implements the MPC.
  You can do this with `$ cd ~/husky_crazyflie_landing/mpc_ws` and `$ catkin_make`.

  If you wish to change more about the MPC like the prediction horizon or the vehicle dynamics you can make those changes by doing `$ nano ~/husky_crazyflie_landing/open_ros_codegen/nmpc_open/create_open_solver.py`.
  You must then run the python file, like before, by doing `$ cd ~/husky_crazyflie_landing/open_ros_codegen/nmpc_open/` then `$ python create_open_solver.py`.
  However, before compiling the C++ node that implements the MPC, you must do a few steps. Read all the way through before starting:
  * First, copy the C++ file "open_optimizer.cpp" in `~/husky_crazyflie_landing/mpc_ws/src/open_nmpc_controller/src` to the directory named "backup" in the home directory.
  * Next, `$ cd ~/husky_crazyflie_landing/mpc_ws/src` and recursively remove the directory named "open_nmpc_controller". Make sure you saved the C++ file to a location outside of this directory!
  * Then, `$ cd ~/husky_crazyflie_landing/open_ros_codegen/nmpc_open/optimization_engine/mpc_controller` and copy the directory "open_nmpc_controller" into `~/husky_crazyflie_landing/mpc_ws/src`.
  * This will have overwritten the C++ file in `~/husky_crazyflie_landing/mpc_ws/src/open_nmpc_controller/src` with code that looks similar but is bad, so you'll need to replace it with the backup you made earlier.
  * Lastly, after doing the python code gen, copying the directory, then replacing the bad code file, you should be ready to `$ cd ~/husky_crazyflie_landing/mpc_ws` and `$ catkin_make` like before.
