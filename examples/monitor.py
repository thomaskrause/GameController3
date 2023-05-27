GAME_CONTROLLER_IP = "localhost"
UDP_REGISTER_PORT = 3636;
UDP_MONITOR_PORT = 3838;

import socket
import struct

# Register this monitor with the GameController
send_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
message_register = b"RGTr\0"
send_sock.sendto(message_register, (GAME_CONTROLLER_IP, UDP_REGISTER_PORT))

# Recieve all monitor messages
recv_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
recv_sock.bind(("localhost", UDP_MONITOR_PORT))

while True:
    
    # Receive a single package with size of 118 bytes
    data, addr = recv_sock.recvfrom(118)
    # Unpack the data, this only uses some of the fields and ignores the team info completly
    (header, 
     version, 
     packet_number, _, _, _, _, 
     state, _, _, _, 
     secs_remaining, 
     secondary_time) = struct.unpack(">4sBBBBBBBBBBhh", data[0:18])
    # Check header, which is different for monitor messages and the version
    # Other than the header, this is the same package as normal game controller messages
    if header == b"RGTD" and version == 15:
        # Get a visible string for the game state
        if state == 0:
            game_state = "Initial"
        elif state == 1:
            game_state = "Ready"
        elif state == 2:
            game_state = "Set"
        elif state == 3:
            game_state = "Playing"
        elif state == 4:
            game_state = "Finished"
        else:
            "Unknown"
        print("[%s] %s secs_remaining=%d secondary_time=%d" % (packet_number, game_state, secs_remaining, secondary_time))
    else:
        print("Received invalid message")