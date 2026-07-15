#!/usr/bin/env python3
    
import rclpy
from rclpy.node import Node
from example_interfaces.srv import SetBool
from functools import partial
     
     
class ResetCounterNode(Node): #
    def __init__(self):
        super().__init__("reset_counter")
        self.client_ = self.create_client(SetBool, "reset_counter")

    def call_reset_counter(self, data: bool):
        while not self.client_.wait_for_service(1.0):
            self.get_logger().warn("Waiting for the service to be available...")

        request = SetBool.Request()
        request.data = data

        future = self.client_.call_async(request)
        future.add_done_callback(partial(self.callback_call_reset_counter, request=request))

    def callback_call_reset_counter(self, future, request):
        response = future.result()
        if response.success == True:
            response.message = "Received True"
            self.counter = 0
            
        else:
            response.success = False
            response.message = "Received False"
        self.get_logger().info(response.message) 
        return response
     
def main(args=None):
    rclpy.init(args=args)
    node = ResetCounterNode()
    node.call_reset_counter(True)
    rclpy.spin(node)
    rclpy.shutdown()
     
     
if __name__ == "__main__":
    main()