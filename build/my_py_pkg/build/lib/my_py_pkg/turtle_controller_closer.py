#!/usr/bin/env python3

import math
import rclpy
from rclpy.node import Node
from turtlesim.msg import Pose
from geometry_msgs.msg import Twist
from my_robot_interfaces.msg import AliveTurtles
from my_robot_interfaces.srv import CatchTurtle


class TurtleCloserNode(Node):
    def __init__(self):
        super().__init__("turtle_controller_closer")
        self.target_x = None
        self.target_y = None
        self.target_name = None
        self.alive_turtle_name = []
        self.alive_turtle_x = []
        self.alive_turtle_y = []
        self.current_pose = None
        self.target_reached = False

        self.subscriber_ = self.create_subscription(
            Pose, "turtle1/pose", self.callback_turtle_pose, 10)
        self.publisher_ = self.create_publisher(Twist, "turtle1/cmd_vel", 10)
        self.timer = self.create_timer(0.1, self.timer_callback)
        self.subscriber_alive_ = self.create_subscription(
            AliveTurtles, "alive_turtles", self.callback_alive_turtles, 10)
        self.catch_turtle_service = self.create_service(
            CatchTurtle, "catch_turtle", self.callback_catch_turtle)

        self.get_logger().info("Turtle Closer Controller has been started.")

    def callback_turtle_pose(self, msg: Pose):
        self.current_pose = msg
        self.get_logger().debug(
            f"Received pose: ({msg.x:.2f}, {msg.y:.2f}, theta={msg.theta:.2f})")

    def select_closest_target(self):
        if not self.alive_turtle_name:
            self.target_name = None
            self.target_x = None
            self.target_y = None
            self.target_reached = False
            return

        best_index = None
        best_distance = float("inf")

        for index, (x, y) in enumerate(zip(self.alive_turtle_x, self.alive_turtle_y)):
            if self.current_pose is None:
                distance = 0.0
            else:
                distance = math.hypot(x - self.current_pose.x, y - self.current_pose.y)
            if distance < best_distance:
                best_distance = distance
                best_index = index

        if best_index is not None:
            self.target_name = self.alive_turtle_name[best_index]
            self.target_x = self.alive_turtle_x[best_index]
            self.target_y = self.alive_turtle_y[best_index]
            self.target_reached = False
            self.get_logger().info(f"Target turtle: {self.target_name}")

    def timer_callback(self):
        if self.current_pose is None:
            return
        if self.target_x is None or self.target_y is None:
            return

        dx = self.target_x - self.current_pose.x
        dy = self.target_y - self.current_pose.y
        distance = math.hypot(dx, dy)

        if distance < 0.30:
            self.publisher_.publish(Twist())
            if not self.target_reached:
                self.target_reached = True
                self.get_logger().info("Target reached")
            return

        target_angle = math.atan2(dy, dx)
        angle_error = target_angle - self.current_pose.theta
        angle_error = (angle_error + math.pi) % (2.0 * math.pi) - math.pi

        vel = Twist()
        vel.linear.x = min(1.5, distance)
        vel.angular.z = max(-2.0, min(2.0, 4.0 * angle_error))
        self.publisher_.publish(vel)

    def callback_alive_turtles(self, msg: AliveTurtles):
        self.alive_turtle_name.append(msg.name)
        self.alive_turtle_x.append(msg.x)
        self.alive_turtle_y.append(msg.y)
        self.select_closest_target()

    def remove_current_target(self):
        if self.target_name is None:
            return

        if self.target_name in self.alive_turtle_name:
            index = self.alive_turtle_name.index(self.target_name)
            self.alive_turtle_name.pop(index)
            self.alive_turtle_x.pop(index)
            self.alive_turtle_y.pop(index)

    def callback_catch_turtle(self,
                              request: CatchTurtle.Request,
                              response: CatchTurtle.Response):
        if self.current_pose is None or self.target_x is None or self.target_y is None:
            response.success = False
            response.name = ""
            return response

        dx = self.target_x - self.current_pose.x
        dy = self.target_y - self.current_pose.y
        distance = math.hypot(dx, dy)

        if self.target_reached or distance < 0.30:
            response.success = True
            response.name = self.target_name
            self.get_logger().info(f"Turtle {response.name} caught!")
            self.remove_current_target()
            self.select_closest_target()
            return response

        response.success = False
        response.name = ""
        return response


def main(args=None):
    rclpy.init(args=args)
    node = TurtleCloserNode()
    rclpy.spin(node)
    rclpy.shutdown()


if __name__ == "__main__":
    main()
