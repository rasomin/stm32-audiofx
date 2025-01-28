try:
    import serial
    import serial.tools.list_ports
except ImportError:
    import os
    os.system("pip3 install pyserial")
    
    import serial
    import serial.tools.list_ports
    

def find_uart_device(target=None):
    ports = serial.tools.list_ports.comports()
    
    for port in ports:
        if target in port.description:
            return port.device
    return None

def main():
    target = "STM32"
    port = find_uart_device(target)
    
    if port is None:
        print("No device found")
        return False
    
    try:
        with serial.Serial(port, baudrate = 115200, timeout = 1) as ser:
            print(f"Connected to {port}")
            
            while True:
                if ser.in_waiting > 0:
                    data = ser.readline().decode().strip()
                    print(data)
                    
    except serial.SerialException as e:
        print(e)
        
    except KeyboardInterrupt:
        return True
        
        
        
if __name__ == "__main__":
    main()
    
    