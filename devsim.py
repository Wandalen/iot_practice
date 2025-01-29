import random
import time
import json
from azure.iot.device import IoTHubDeviceClient, Message
import os

# IoT Hub connection string
CONNECTION_STRING = os.getenv("PY_CONNECTION_STRING")

# Initialize IoT Hub client
def initialize_client(connection_string):
    return IoTHubDeviceClient.create_from_connection_string(connection_string)

# Initial sensor data
current_data = {
    "temperature": random.uniform(10.0, 20.0),  # Initial temperature between 10 and 20
    "humidity": random.uniform(30.0, 50.0),     # Initial humidity between 30 and 50
    "weight": random.uniform(500.0, 5000.0)     # Initial weight between 500 and 5000
}

# Generate realistic sensor data
def generate_sensor_data():
    global current_data

    # Define realistic step limits for changes
    max_temperature_step = 0.5  # Maximum change in temperature
    max_humidity_step = 1.0     # Maximum change in humidity
    max_weight_step = 50.0      # Maximum change in weight

    # Generate new values with minimal step changes
    current_data["temperature"] += random.uniform(-max_temperature_step, max_temperature_step)
    current_data["humidity"] += random.uniform(-max_humidity_step, max_humidity_step)
    current_data["weight"] += random.uniform(-max_weight_step, max_weight_step)

    # Ensure the values remain within their realistic ranges
    current_data["temperature"] = max(10.0, min(20.0, current_data["temperature"]))
    current_data["humidity"] = max(30.0, min(50.0, current_data["humidity"]))
    current_data["weight"] = max(500.0, min(5000.0, current_data["weight"]))

    return {
        "deviceId": "warehouse-01-sensor-01",  # Realistic device ID
        "temperature": round(current_data["temperature"], 3),  # Smoothed temperature
        "humidity": round(current_data["humidity"], 3),        # Smoothed humidity
        "weight": round(current_data["weight"], 3)             # Smoothed weight
    }

# Send data to IoT Hub
def send_sensor_data(client):
    while True:
        try:
            # Generate smoothed data
            sensor_data = generate_sensor_data()
            message = Message(json.dumps(sensor_data))
            
            # Send message
            print(f"Sending message: {sensor_data}")
            client.send_message(message)
            print("Message successfully sent to Azure IoT Hub.")
            
            # Wait before sending next message
            time.sleep(12)  # Send data every 12 seconds
        except Exception as e:
            print(f"Error while sending message: {e}")
            time.sleep(5)  # Wait 5 seconds before retrying

# Main function
if __name__ == "__main__":
    # Initialize IoT Hub client
    client = initialize_client(CONNECTION_STRING)
    print("IoT Hub client initialized. Starting to send messages...")
    
    # Send data
    send_sensor_data(client)
