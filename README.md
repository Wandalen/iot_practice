# Azure and IOT Practice

### Useful Links

- [Practice](https://github.com/lsawicki-cdv/course-iot-cloud-computing)

### Examples

- [Smart-City-Infrastructure-Monitoring](https://github.com/vsakhnooo/Smart-City-Infrastructure-Monitoring)

# Requirements

- ✅ [Business context description](#business-context-description)
- ✅ [C4 - Architecture Diagram - Context Level](/architecture/Context.png)
- ✅ [C4 - Architecture Diagram - Container Level](/architecture/Container.png)
- ✅ [C4 - Architecture Diagram - Component Level](/architecture/Component.png)
- ❌ Azure/AWS Cost Calculator
- ✅ [Credentials Instructions](./secret/README.md)
- ✅ [IoT Device(or simulator) code](https://azure-samples.github.io/raspberry-pi-web-simulator/)
- ✅ [MQTT broker service](https://portal.azure.com/#@levchenkoden20gmail.onmicrosoft.com/resource/subscriptions/bcf62ce6-f30a-4cb1-9485-85a712cc619d/resourceGroups/SIMS/providers/Microsoft.Devices/IotHubs/iot-practicehub/Overview)
- ✅ [Data storage](https://portal.azure.com/#@levchenkoden20gmail.onmicrosoft.com/resource/subscriptions/bcf62ce6-f30a-4cb1-9485-85a712cc619d/resourceGroups/SIMS/providers/Microsoft.DBforPostgreSQL/serverGroupsv2/iot-practice-db/overview)
- ✅ [REST API](./src/routes/mod.rs)
- ✅ [Deployment instructions](./deploy/)
- ✅ [Test instructions](#try-it-out)
- ⚙️ [Are all business cases covered?](./src/routes/mod.rs) - As much as it reasonable for educational project
- ✅ [Does it work? - Yes!](https://iot-grafana-e3ewgph5cabrc6at.weu.grafana.azure.com/d/cebgcjrdq7togc/iot-dashboard?orgId=1&from=now-2d&to=now)
- ✅ [Infrastructure as Code](./deploy/main.tf)
- ✅ [Postman collection](./SIMS.postman_collection.json)
- ✅ [Frontend](Grafana.md/#frontend)
- ✅ [Customer Sign Up process](Grafana.md/#customer-sign-up-process)
- ✅ [User management](Grafana.md/#user-management)
- ⚙️ Device management - not necessary as we have only one device
- ⚙️ Is the solution secure? - IOT Hub Options could be fine fine-tuned further.
- ✅ [IoT Device Configuration (e.g. Azure Device Twins)](https://portal.azure.com/#@levchenkoden20gmail.onmicrosoft.com/resource/subscriptions/bcf62ce6-f30a-4cb1-9485-85a712cc619d/resourceGroups/SIMS/providers/Microsoft.Devices/IotHubs/iot-practicehub/DeviceExplorer)
- ✅ [Are there any 3rd party services used? (not Azure/AWS) - Terraform](./deploy/README.md)
- ✅ [Number of connected](https://portal.azure.com/#@levchenkoden20gmail.onmicrosoft.com/resource/subscriptions/bcf62ce6-f30a-4cb1-9485-85a712cc619d/resourceGroups/SIMS/providers/Microsoft.Devices/IotHubs/iot-practicehub/DeviceExplorer)
- ✅ [IoT devices (or simulators)](https://azure-samples.github.io/raspberry-pi-web-simulator/)
- ✅ [Number of used AWS/Azure services - 3 ](https://portal.azure.com/#@levchenkoden20gmail.onmicrosoft.com/resource/subscriptions/bcf62ce6-f30a-4cb1-9485-85a712cc619d/resourceGroups/SIMS/overview)

## Try it out!

1. Install [Rust](https://rustup.rs/)
2. Install [Docker](https://docs.docker.com/engine/install/)
3. Setup [secrets](./secret/README.md) for the database connection
4. Start the container:
```bash
$ make up
```
5. Open `http://127.0.0.1:8080/api/get_data` in your browser to access sensor data
6. Open `http://127.0.0.1:8080/api/alerts` in your browser to poll for alerts

### Business context description

**Topic**: Smart inventory management system (SIMS)

**Overview**

The Smart Inventory Management System (SIMS) is an IoT-powered solution for monitoring and managing warehouse inventory. It helps companies optimize storage, reduce waste, and ensure efficient operations through real-time tracking and data insights.

**Business Description**

SIMS benefits sectors like retail, pharmaceuticals, and food storage by automating inventory tracking, reducing manual labor, and enhancing product safety. Its sensors monitor stock levels, shelf occupancy, temperature, and humidity to prevent loss and maintain optimal conditions. Alerts notify staff about critical changes, supporting swift action and cost savings.

**Features**

- Real-Time Monitoring: Tracks stock levels, environmental conditions, and shelf occupancy.
- Automated Alerts: Issues alerts when inventory is low or conditions are out of range.
- Data Dashboard: A web-based dashboard displays sensor data for easy oversight.
- Role-Based Access: Differentiates permissions for staff and administrators.
- Sensors:
  - Weight Sensors: Track item quantities on shelves.
  - Temperature Sensors: Ensure conditions for sensitive products.
  - Humidity Sensors: Regulate storage conditions to prevent damage.

**Userstories As a Warehouse Staff Member**

- ⚙️ I want to receive notifications about critical changes in temperature or humidity so that I can take immediate action to protect sensitive products.
- ✅ I want to monitor inventory levels and environmental conditions in real-time so that I can ensure optimal storage conditions and product safety.

**Userstories As an Administrator**

- ✅ I want to receive real-time alerts when inventory levels are low so that I can coordinate with suppliers to reorder stock promptly and avoid shortages.
- ✅ I want to access a data dashboard that provides an overview of stock levels and environmental conditions so that I can make informed decisions about inventory management.
- ✅ I want to set role-based access permissions for my team so that sensitive data is only accessible to authorized personnel.
- ⚙️ I want to manage user access and permissions within the system so that data security is upheld.
- ✅ I want to ensure that the system integrates seamlessly with existing IT infrastructure so that operations are not disrupted.
