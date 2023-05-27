UDP_IP = "localhost"
UDP_PORT = 3636;

import socket
import struct

# Register this monitor with the GameController
send_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
message_register = b"RGTr\0"
print(message_register)
send_sock.sendto(message_register, (UDP_IP, UDP_PORT))

# Recieve all monitor messages
recv_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
recv_sock.bind(("localhost", 3838))

while True:
    data, addr = recv_sock.recvfrom(1024)
    print("received message: %s" % data)
