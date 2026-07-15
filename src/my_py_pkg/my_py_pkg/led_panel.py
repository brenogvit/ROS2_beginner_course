#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from my_robot_interfaces.msg import LedPanelState
from my_robot_interfaces.srv import SetLed

class LedPanelNode(Node): 
    def __init__(self):
        super().__init__("led_panel")
        self.led_panel_state_pub_ = self.create_publisher(LedPanelState, "led_panel_state", 10)
        self.timer_ = self.create_timer(1.0, self.publish_led_panel_state)
        self.get_logger().info("LED panel node has been started.")

        self.create_ = self.create_service(SetLed, "set_led", self.callback_set_led)
        self.get_logger().info("Set LED Service has been started")
        
        self.declare_parameter("led_state", [False, False, False])
        self.led_status = self.get_parameter("led_state").value

    def publish_led_panel_state(self):
        msg = LedPanelState()
        msg.is_led1_on = self.led_status[0]
        msg.is_led2_on = self.led_status[1]
        msg.is_led3_on = self.led_status[2]
        self.led_panel_state_pub_.publish(msg)

    def callback_set_led(self,
                         request: SetLed.Request,
                         response: SetLed.Response):
        if 1<= request.led_number <= 3:
            index = request.led_number - 1
            self.led_status[index] = request.state
            response.message = (f"Turning LED {request.led_number} {'ON' if request.state else 'OFF'}")
        else:
            response.message = "Invalid LED number"
            response.success = False
            return response
        
        response.success = True
        return response
     
     
def main(args=None):
    rclpy.init(args=args)
    node = LedPanelNode()
    rclpy.spin(node)
    rclpy.shutdown()
     
     
if __name__ == "__main__":
    main()