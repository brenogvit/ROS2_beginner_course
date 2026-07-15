#!/usr/bin/env python3
    
import rclpy
from rclpy.node import Node
from example_interfaces.msg import Int64
from example_interfaces.srv import SetBool

     
     
class NumberCounterNode(Node): 
    def __init__(self):
        super().__init__("number_counter") 
        self.subscriptions_ = self.create_subscription(Int64, "number", self.callback_number, 10)
        self.get_logger().info("Number Counter has been started.")
        self.publisher_ = self.create_publisher(Int64, "number_count", 10)
        self.timer = self.create_timer(1.0, self.publish_number)

        self.counter = 0

        self.create_ = self.create_service(
            SetBool, "reset_counter", self.callback_set_bool)
        self.get_logger().info("Reset Counter Server has been started.")

        
    def callback_number(self,
                         msg: Int64):
        self.received_number = msg.data
        #self.get_logger().info(str(self.received_number))

    def publish_number(self):
        msg = Int64()
        msg.data = self.counter
        self.publisher_.publish(msg)
        self.counter += 1 

    def callback_set_bool(self,
                          request: SetBool.Request,
                          response: SetBool.Response):
        if request.data:
            response.success = True
            response.message = "Received True"
            self.counter = 0

        else:
            response.success = False
            response.message = "Received False"
        self.get_logger().info(response.message)
        return response    

def main(args=None):
    rclpy.init(args=args)
    node = NumberCounterNode() 
    rclpy.spin(node)
    rclpy.shutdown()
     
     
if __name__ == "__main__":
    main()