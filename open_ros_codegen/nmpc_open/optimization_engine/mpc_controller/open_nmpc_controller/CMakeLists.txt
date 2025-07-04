cmake_minimum_required(VERSION 2.8.3)
add_compile_options(-std=c++11)
project(open_nmpc_controller)

find_package(catkin REQUIRED COMPONENTS
  roscpp
  std_msgs
  message_generation
)

add_message_files(
  FILES
  OptimizationResult.msg
  OptimizationParameters.msg
)

generate_messages(
 DEPENDENCIES std_msgs
)

catkin_package(
   CATKIN_DEPENDS message_runtime
)

include_directories(
  ${catkin_INCLUDE_DIRS}
  ${PROJECT_SOURCE_DIR}/include
)

set(NODE_NAME open_mpc_controller_node)
add_executable(${NODE_NAME} src/open_optimizer.cpp)
target_link_libraries(
    ${NODE_NAME}
    ${PROJECT_SOURCE_DIR}/extern_lib/libmpc_controller.a)
target_link_libraries(
    ${NODE_NAME}
    m dl
    ${catkin_LIBRARIES})
add_dependencies(
    ${NODE_NAME}
    ${${PROJECT_NAME}_EXPORTED_TARGETS}
    ${catkin_EXPORTED_TARGETS})