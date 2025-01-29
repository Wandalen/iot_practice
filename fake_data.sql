DO $$
    DECLARE
        d DATE;
        i INT;
        device TEXT;
        temp FLOAT;
        hum FLOAT;
        wght NUMERIC(7,3);
        ts TIMESTAMP;
        devices TEXT[] := ARRAY['warehouse-01-sensor-01'];
    BEGIN
        -- Loop through the last 7 days
        FOR d IN (SELECT generate_series(CURRENT_DATE - INTERVAL '7 days', CURRENT_DATE - INTERVAL '1 day', INTERVAL '1 day')) LOOP
                FOR i IN 1..20 LOOP
                        -- Pick a random device from the list
                        device := devices[1 + floor(random() * array_length(devices, 1))];

                        -- Generate random weight (between 1000 and 4000)
                        wght := round((1000 + random() * 3000)::NUMERIC, 3);

                        -- Generate random temperature (between 10 and 30 degrees)
                        temp := round((10 + random() * 20)::NUMERIC, 3);

                        -- Generate random humidity (between 30 and 60 percent)
                        hum := round((30 + random() * 30)::NUMERIC, 3);

                        -- Generate random timestamp within the day
                        ts := d + (random() * INTERVAL '24 hours');

                        -- Insert into the table
                        INSERT INTO iot_sensors_data (device_id, temperature, humidity, timestamp, weight)
                        VALUES (device, temp, hum, ts, wght);
                    END LOOP;
            END LOOP;
    END $$;
