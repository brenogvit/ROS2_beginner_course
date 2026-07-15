#!/usr/bin/env python3
    
import rclpy
import math
from rclpy.node import Node
from turtlesim.srv import Spawn, Kill
import random
from my_robot_interfaces.msg import AliveTurtles
from my_robot_interfaces.srv import CatchTurtle

     
     
class TurtleSpawnerNode(Node): 
    def __init__(self):
        super().__init__("turtle_spawner") 
        self.client_ = self.create_client(Spawn, "spawn")
        self.publisher_ = self.create_publisher(AliveTurtles, "alive_turtles", 10)
        self.number_of_turtles = 2
        self.client2_ = self.create_client(CatchTurtle, "catch_turtle")
        self.client3_ = self.create_client(Kill, "kill")
        self.pending_catch_timer = None

        self.declare_parameter("spawn_frequency", 3.0)
        self.spawn_frequency_ = self.get_parameter("spawn_frequency").value

        self.declare_parameter("turtle_name_prefix", "turtle_")
        self.turtle_name_prefix_ = self.get_parameter("turtle_name_prefix").value

        

    def call_spawn_turtle(self):
        while not self.client_.wait_for_service(1.0):
            self.get_logger().warn("Waiting for the service to be available...")

        request = Spawn.Request()
        request.name = self.turtle_name_prefix_ + str(self.number_of_turtles)
        request.x = random.uniform(0.0, 11.0)
        request.y = random.uniform(0.0, 11.0)
        request.theta = random.uniform(0.0, 2.0 * math.pi)
        future = self.client_.call_async(request)

        msg = AliveTurtles()
        msg.name = request.name
        msg.x = request.x
        msg.y = request.y
        self.publisher_.publish(msg)
        self.number_of_turtles += 1

        if self.pending_catch_timer is not None:
            self.destroy_timer(self.pending_catch_timer)
        self.pending_catch_timer = self.create_timer(2.0, self.delayed_call_catch_turtle)

    def delayed_call_catch_turtle(self):
        if self.pending_catch_timer is not None:
            self.destroy_timer(self.pending_catch_timer)
            self.pending_catch_timer = None
        self.call_CatchTurtle()


    def call_CatchTurtle(self):
        while not self.client2_.wait_for_service(1.0):
            self.get_logger().warn("Waiting for the service to be available...")

        request = CatchTurtle.Request()
        future = self.client2_.call_async(request)
        future.add_done_callback(self.callback_catch_turtle)

    def callback_catch_turtle(self, future):
        response = future.result()
        if response.success:
            self.get_logger().info(f"Turtle {response.name} caught!")
            self.call_kill_turtle(response.name)
        else:
            self.get_logger().info("No turtle caught.")

    def call_kill_turtle(self, turtle_name):
        while not self.client3_.wait_for_service(1.0):
            self.get_logger().warn("Waiting for the service to be available...")

        request = Kill.Request()
        request.name = turtle_name
        future = self.client3_.call_async(request)
        #future.add_done_callback(self.callback_kill_turtle)
    
     
     
def main(args=None):
    rclpy.init(args=args)
    node = TurtleSpawnerNode()
    node.timer = node.create_timer(node.spawn_frequency_, node.call_spawn_turtle)

    rclpy.spin(node)
    rclpy.shutdown()
     
     
if __name__ == "__main__":
    main()