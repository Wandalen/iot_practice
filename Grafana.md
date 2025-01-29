# Grafana

The Grafana dashboard is designed to visualize data from the database, providing real-time insights and monitoring capabilities.

## Frontend

For the frontend, we use Grafana, which is accessible via [this link](https://iot-grafana-e3ewgph5cabrc6at.weu.grafana.azure.com/d/cebgcjrdq7togc/iot-dashboard).

Grafana provides dynamic and interactive dashboards that allow real-time monitoring of data from your database. It can also can trigger alerts based on predefined conditions and notify via email, other integrations when critical events occur.

![grafana dashboard](GrafanaDashboard.png "Dashboard")

## User management

To add read access to your managed Grafana in Azure:

1. **Open Azure Managed Grafana**: Go to your Azure Managed Grafana instance in the Azure portal.
2. **Access Control (IAM)**: In the left menu, select **Access control (IAM)**.
   ![guide step 2](azure_iam_grafana/1.png "step 2")
3. **Add Role Assignment**: Click on **Add role assignment**.
   ![guide step 3](azure_iam_grafana/2.png "step 3")
4. **Select Role**: Choose the **Grafana Viewer** role for read-only access to dashboards. Alternatively, you can choose **Grafana Editor** for read-write access if needed.
   ![guide step 4](azure_iam_grafana/3.png "step 4")
6. **Assign Access**: Select the user, group, or service principal you want to grant access to. Click **Select members**, pick the members, and then confirm with **Select**.
   ![guide step 5](azure_iam_grafana/4.png "step 5")
7. **Review and Assign**: Click **Next**, review the details, and then click **Review + assign** to complete the role assignment.

This will grant the selected users read-only access to your Grafana dashboards.

## Customer Sign Up process

Users are added manually and are intended for internal use only. 
They do not have access to the system as anonymous users. 
This ensures that only authorized personnel can interact with the data.