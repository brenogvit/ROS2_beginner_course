#!/usr/bin/env python3
    
import math
import rclpy
from rclpy.node import Node
from turtlesim.msg import Pose
from geometry_msgs.msg import Twist
from my_robot_interfaces.msg import AliveTurtles
from my_robot_interfaces.srv import CatchTurtle



class TurtleControllerChooseNode(Node):
    def __init__(self):
        super().__init__("turtle_controller_choose")

        # DEFINE VARIABLES
        self.target_x = 5.0
        self.target_y = 5.0
        self.alive_turtle_name = []
        self.alive_turtle_x = []
        self.alive_turtle_y = []
        self.i = 0
        self.current_pose = None
        self.target_reached = False

        self.declare_parameter("catch_closest_turtle_first", True)
        self.catch_closest_turtle_first_ = self.get_parameter("catch_closest_turtle_first").value

        self.subscriber_ = self.create_subscription(
            Pose, "turtle1/pose", self.callback_turtle_pose, 10)
        
        self.publisher_ = self.create_publisher(
            Twist, "turtle1/cmd_vel", 10)
        
        self.timer = self.create_timer(
            0.1, self.timer_callback)
        self.get_logger().info("Turtle Controller has been started.")

        self.subscriber_ = self.create_subscription(
            AliveTurtles, "alive_turtles", self.callback_alive_turtles, 10)
        
        self.catch_turtle_service = self.create_service(
            CatchTurtle, "catch_turtle", self.callback_catch_turtle)

    def callback_turtle_pose(self, msg: Pose):
        self.current_pose = msg
        self.get_logger().debug(f"Received pose: ({msg.x:.2f}, {msg.y:.2f}, theta={msg.theta:.2f})")


    def callback_alive_turtles(self, msg: AliveTurtles):
        
        #self.alive_turtle_name = []
        #self.alive_turtle_x = []
        #self.alive_turtle_y = []

        self.alive_turtle_name.append(msg.name)
        self.alive_turtle_x.append(msg.x)
        self.alive_turtle_y.append(msg.y)

        if self.catch_closest_turtle_first_:
            self.select_closest_target()
        else:
            self.select_next_target()
        
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
    
    def select_next_target(self):
        if self.i < len(self.alive_turtle_name):
            self.target_reached = False
            self.get_logger().info(f"Target turtle: {self.alive_turtle_name[self.i]}")
            self.target_name = self.alive_turtle_name[self.i]
            self.target_x = self.alive_turtle_x[self.i]
            self.target_y = self.alive_turtle_y[self.i]

    def timer_callback(self):
        if self.current_pose is None:
            return

        pose = self.current_pose

        dx = self.target_x - pose.x
        dy = self.target_y - pose.y
        distance = math.hypot(dx, dy) #calcula a hipotenusa considerando a distancia em x e a distancia em y; ou seja, a distancia diagonal que deve ser percorrida

        if distance < 0.30:
            self.publisher_.publish(Twist())

            if not self.target_reached:
                self.target_reached = True
                self.get_logger().info("Target reached")
            return

        target_angle = math.atan2(dy, dx) #retorna o angulo em radianos da tangente considerando a distancia percorrida em y e em x
        angle_error = target_angle - pose.theta
        angle_error = (angle_error + math.pi) % (2.0 * math.pi) - math.pi #calcula o erro no intervalo [pi, -pi]

        vel = Twist()
        vel.linear.x = min(1.5, distance)
        vel.angular.z = max(-2.0, min(2.0, 4.0 * angle_error))
        self.publisher_.publish(vel)
    
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
                self.get_logger().info("No turtle caught.")
                return response

        else:
            dx = self.target_x - self.current_pose.x
            dy = self.target_y - self.current_pose.y
            distance = math.hypot(dx, dy)

            if self.target_reached or distance < 0.30:
                if self.catch_closest_turtle_first_:
                    response.success = True
                    response.name = self.target_name
                    self.get_logger().info(f"Turtle {response.name} caught!")
                    self.remove_current_target()
                    self.select_closest_target()
                    return response 
                else:
                    response.success = True
                    response.name = self.target_name if self.target_name is not None else self.alive_turtle_name[self.i]
                    self.get_logger().info(f"Turtle {response.name} caught!")

                    if self.i + 1 < len(self.alive_turtle_name):
                        self.i += 1
                        self.target_reached = False
                        self.target_name = self.alive_turtle_name[self.i]
                        self.target_x = self.alive_turtle_x[self.i]
                        self.target_y = self.alive_turtle_y[self.i]
                        self.get_logger().info(f"Next target turtle: {self.target_name}")
                    return response

        # If none of the above branches returned, explicitly return a negative response
        response.success = False
        response.name = ""
        self.get_logger().info("No turtle caught.")
        return response



def main(args=None):
    rclpy.init(args=args)
    node = TurtleControllerChooseNode() 
    rclpy.spin(node)
    rclpy.shutdown()
     
     
if __name__ == "__main__":
    main()