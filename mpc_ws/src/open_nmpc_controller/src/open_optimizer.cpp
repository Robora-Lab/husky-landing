/**
 * This is an auto-generated file by Optimization Engine (OpEn)
 * OpEn is a free open-source software - see doc.optimization-engine.xyz
 * dually licensed under the MIT and Apache v2 licences.
 *
 * Generated at 2024-06-19 15:28:27.622716.
 */
#include "ros/ros.h"
#include "open_nmpc_controller/OptimizationResult.h"
#include "open_nmpc_controller/OptimizationParameters.h"
#include "mpc_controller_bindings.hpp"
#include "open_optimizer.hpp"

#include <std_msgs/Bool.h>
#include <husky_msgs/HuskyStatus.h>
#include <geometry_msgs/Twist.h>
#include <geometry_msgs/PoseStamped.h>

#include <sensor_msgs/Joy.h>

#include "tf/transform_datatypes.h"
#include "tf/LinearMath/Matrix3x3.h"

ros::Time lastCommTime;
double elapsedTime = 0;

namespace open_nmpc_controller {
/**
 * Class open_nmpc_controller::OptimizationEngineManager manages the
 * exchange of data between the input and output topics
 * of this node
 */
class OptimizationEngineManager {

private:
    open_nmpc_controller::OptimizationParameters params;
    open_nmpc_controller::OptimizationResult results;
    double p[MPC_CONTROLLER_NUM_PARAMETERS] = { 0 };
    double u[MPC_CONTROLLER_NUM_DECISION_VARIABLES] = { 0 };
    double *y = NULL;

    static const int NX = 3;
    static const int NU = 2;

    double current_pos[NX] = {0};
    double current_ref[NX] = {0};

    bool trackingGoal = false;
    bool e_stop = false;
    int enabled = 0;
    int conflict = 0;
    int buttonPrev = 0;

    double elapsedSec;

    mpc_controllerCache* cache;
    double init_penalty = ROS_NODE_MPC_CONTROLLER_DEFAULT_INITIAL_PENALTY;

    /**
     * Publish obtained results to output topic
     */
    void publishToTopic(ros::Publisher& publisher)
    {
        publisher.publish(results);
    }

    /**
     * Updates the input data based on the data that are posted
     * on /mpc/open_parameters (copies value from topic data to
     * local variables). This method is responsible for parsing
     * the data announced on the input topic.
     */
    void updateInputData()
    {
        init_penalty = (params.initial_penalty > 1.0)
            ? params.initial_penalty
            : ROS_NODE_MPC_CONTROLLER_DEFAULT_INITIAL_PENALTY;

        if (params.parameter.size() > 0) {
            for (size_t i = 0; i < MPC_CONTROLLER_NUM_PARAMETERS; ++i)
                p[i] = params.parameter[i];
        }

        if (params.initial_guess.size() == MPC_CONTROLLER_NUM_DECISION_VARIABLES) {
            for (size_t i = 0; i < MPC_CONTROLLER_NUM_DECISION_VARIABLES; ++i)
                u[i] = params.initial_guess[i];
        }

	if (params.initial_y.size() == MPC_CONTROLLER_N1) {
            for (size_t i = 0; i < MPC_CONTROLLER_N1; ++i)
                y[i] = params.initial_y[i];
	}
    }

    /**
     * Call OpEn to solve the problem
     */
    mpc_controllerSolverStatus solve()
    {
        return mpc_controller_solve(cache, u, p, y, &init_penalty);
    }
/**
 * Public fields and methods
 */
public:
    /**
     * Constructor of OptimizationEngineManager
     */
    OptimizationEngineManager()
    {
	    y = new double[MPC_CONTROLLER_N1];
        cache = mpc_controller_new();
    }

    /**
     * Destructor of OptimizationEngineManager
     */
    ~OptimizationEngineManager()
    {
	if (y!=NULL) delete[] y;
        mpc_controller_free(cache);
    }

    /**
     * Copies results from `status` to the local field `results`
     */
    void updateResults(mpc_controllerSolverStatus& status)
    {
        std::vector<double> sol(u, u + MPC_CONTROLLER_NUM_DECISION_VARIABLES);
        results.solution = sol;
        std::vector<double> y(status.lagrange, status.lagrange + MPC_CONTROLLER_N1);
        results.lagrange_multipliers = y;
        results.inner_iterations = status.num_inner_iterations;
        results.outer_iterations = status.num_outer_iterations;
        results.norm_fpr = status.last_problem_norm_fpr;
        results.cost = status.cost;
        results.penalty = status.penalty;
        results.status = (int)status.exit_status;
        results.solve_time_ms = (double)status.solve_time_ns / 1000000.0;
        results.infeasibility_f2 = status.f2_norm;
        results.infeasibility_f1 = status.delta_y_norm_over_c;
    }

    /**
     * Callback that obtains data from topic `/open_nmpc_controller/open_params`
     */
    void mpcReceiveRequestCallback(
        const open_nmpc_controller::OptimizationParameters::ConstPtr& msg)
    {
        params = *msg;
    }

    void solveAndPublish(ros::Publisher& publisher)
    {
        updateInputData(); /* get input data */
        mpc_controllerSolverStatus status = solve(); /* solve!  */
        updateResults(status); /* pack results into `results` */
        publishToTopic(publisher);
    }

    void goalPoseCallback(const geometry_msgs::PoseStampedConstPtr& msg)
    {
        trackingGoal = true;

        current_ref[0] = msg->pose.position.x;
        current_ref[1] = msg->pose.position.y;
        current_ref[2] = msg->pose.orientation.w;  /// THIS IS RADIANS WE ARE BEING SNEAKY
    }

    void joyCallback(const sensor_msgs::Joy::ConstPtr& msg){
        int lb = msg->buttons[4];
        int rb = msg->buttons[5];
        if (conflict == 1) ros::shutdown();
        if ((rb == 1 || lb == 1) && enabled == 1){
            conflict = 1;
            enabled = 0;
            ROS_ERROR("CONFLICTING CONTROL INPUTS! - TERMINATING");
        }
        int buttonVal = msg->buttons[1];
        int stopVal = msg->buttons[2];
        if(stopVal == 1) enabled = 0;
        if(buttonVal == 1 && buttonPrev == 0){
            enabled = (enabled + 1)%2;  // toggles enabled
            if(enabled == 1) ROS_WARN("MPC Enabled!");
            else ROS_WARN("MPC Disabled!");
        }
        buttonPrev = buttonVal;
    }

    void poseCallback(const geometry_msgs::PoseStampedConstPtr& msg) {
        double roll, pitch, yaw;

        tf::Quaternion quat(msg->pose.orientation.x,
                            msg->pose.orientation.y,
                            msg->pose.orientation.z,
                            msg->pose.orientation.w);
        tf::Matrix3x3(quat).getRPY(roll, pitch, yaw);

        current_pos[0] = msg->pose.position.x;
        current_pos[1] = msg->pose.position.y;
        current_pos[2] = yaw;
    }

    void e_stopCallback(const husky_msgs::HuskyStatus::ConstPtr& msg){
        e_stop = msg->e_stop;
        if(e_stop){
            ROS_ERROR("E-STOP PRESSED! - TERMINATING");
            ROS_ERROR("STOP PUBLISHING TO GOAL POSE BEFORE RESTARTING");
            ros::shutdown();
        }
        if(e_stop == false){
            lastCommTime = ros::Time::now();
        }
    }

    void solveAndPublishCmdVel(ros::Publisher& publisher, ros::Publisher& hgoal_publisher)
    {
        double current_par [MPC_CONTROLLER_NUM_PARAMETERS] = {0};
        double current_var [MPC_CONTROLLER_NUM_DECISION_VARIABLES] = {0};
        double lin_vel_cmd = 0, ang_vel_cmd = 0;
        double hgoal_x = 0, hgoal_y = 0;

        for (int i=0; i<NX; i++) {
            current_par[i] = current_pos[i];
            current_par[i+NX] = current_ref[i];
        }

        /* solve                  */
        mpc_controllerSolverStatus status
            = mpc_controller_solve(cache, current_var, current_par, 0, &init_penalty);

        if(trackingGoal){
          lin_vel_cmd = current_var[0];
          ang_vel_cmd = current_var[1];
        } else {
	  lin_vel_cmd = 0.0;
	  ang_vel_cmd = 0.0;
	}

        if(lin_vel_cmd > 0.4 || lin_vel_cmd < -0.4 || ang_vel_cmd > 0.4 || ang_vel_cmd < -0.4){
          lin_vel_cmd = 0;
          ang_vel_cmd = 0;
          ROS_ERROR("DANGER: SAFETY THRUST EXCEEDED");
        }

        if(enabled == 1 && trackingGoal){
            geometry_msgs::Twist twist;
            twist.linear.x = lin_vel_cmd;
            twist.linear.y = 0.0;
            twist.linear.z = 0.0;
            twist.angular.x = 0.0;
            twist.angular.y = 0.0;
            twist.angular.z = ang_vel_cmd;
            publisher.publish(twist);

            hgoal_x = current_var[2];
            hgoal_y = current_var[3];

            geometry_msgs::PoseStamped hgoal;

            hgoal.pose.position.x = hgoal_x;
            hgoal.pose.position.y = hgoal_y;
            hgoal.pose.position.z = 1.5;
            hgoal.pose.orientation.x = 0;
            hgoal.pose.orientation.y = 0;
            hgoal.pose.orientation.z = 0;
            hgoal.pose.orientation.w = 0;

            hgoal_publisher.publish(hgoal);

            ROS_INFO("Goal x: %.1f, y: %.1f, yaw: %.1f", current_ref[0], current_ref[1], current_ref[2]);
            ROS_INFO("Sending (lin_vel, ang_vel): (%f, %f) \n", lin_vel_cmd, ang_vel_cmd);
            //ROS_INFO("Solve time: %f ms. \n", (double)status.solve_time_ns / 1000000.0);
            ROS_INFO("husky x goal: %f, husky y goal: %f", hgoal_x, hgoal_y);
        }
    }





}; /* end of class OptimizationEngineManager */
} /* end of namespace open_nmpc_controller */

/**
 * Main method
 *
 * This advertises a new (private) topic to which the optimizer
 * announces its solution and solution status and details. The
 * publisher topic is 'open_nmpc_controller/result'.
 *
 * It obtains inputs from 'open_nmpc_controller/parameters'.
 *
 */
int main(int argc, char** argv)
{
    std::string result_topic, params_topic;  /* parameter and result topics */
    double rate; /* rate of node (specified by parameter) */

    open_nmpc_controller::OptimizationEngineManager mng;
    ros::init(argc, argv, ROS_NODE_MPC_CONTROLLER_NODE_NAME);
    ros::NodeHandle nh, private_nh("~");

    /* obtain parameters from config/open_params.yaml file */
    private_nh.param("result_topic", result_topic,
                     std::string(ROS_NODE_MPC_CONTROLLER_RESULT_TOPIC));
    private_nh.param("params_topic", params_topic,
                     std::string(ROS_NODE_MPC_CONTROLLER_PARAMS_TOPIC));
    private_nh.param("rate", rate,
                     double(ROS_NODE_MPC_CONTROLLER_RATE));

    ros::Publisher mpc_pub
        = private_nh.advertise<open_nmpc_controller::OptimizationResult>(
            ROS_NODE_MPC_CONTROLLER_RESULT_TOPIC,
            ROS_NODE_MPC_CONTROLLER_RESULT_TOPIC_QUEUE_SIZE);
    ros::Subscriber sub
        = private_nh.subscribe(
            ROS_NODE_MPC_CONTROLLER_PARAMS_TOPIC,
            ROS_NODE_MPC_CONTROLLER_PARAMS_TOPIC_QUEUE_SIZE,
            &open_nmpc_controller::OptimizationEngineManager::mpcReceiveRequestCallback,
            &mng);

    ros::Subscriber pos_sub = nh.subscribe("vrpn_client_node/WillHusky/pose", 1, &open_nmpc_controller::OptimizationEngineManager::poseCallback, &mng);
    ros::Subscriber e_sub = nh.subscribe("/status", 1, &open_nmpc_controller::OptimizationEngineManager::e_stopCallback, &mng);

    ros::Subscriber joySub = nh.subscribe("/joy_teleop/joy", 1, &open_nmpc_controller::OptimizationEngineManager::joyCallback, &mng);

    ros::Subscriber command_trajectory_subscriber = private_nh.subscribe("/goalPoseCF", 1, &open_nmpc_controller::OptimizationEngineManager::goalPoseCallback, &mng);

    ros::Publisher pub_twist_cmd = nh.advertise<geometry_msgs::Twist>("/husky_velocity_controller/cmd_vel", 1);
    ros::Publisher pub_husky_goal = nh.advertise<geometry_msgs::PoseStamped>("/goalPose",1);

    ros::Rate loop_rate(ROS_NODE_MPC_CONTROLLER_RATE);

    lastCommTime = ros::Time::now();

    ROS_WARN("MPC is Disabled! Press 'A' to Enable");
    ROS_WARN("Publish a goal to start tracking");
    while (ros::ok()) {
        elapsedTime = ros::Time::now().toSec() - lastCommTime.toSec();
        if (elapsedTime > 1.5){
            ROS_ERROR("NO COMMUNICATION WITH HUSKY STATUS - TERMINATING");
            ROS_INFO("Use `roslaunch wavemodel_pkg wavemodel.launch` prior to running");
            ros::shutdown();
        }
        mng.solveAndPublishCmdVel(pub_twist_cmd, pub_husky_goal);
        ros::spinOnce();
        loop_rate.sleep();
    }

    return 0;
}
