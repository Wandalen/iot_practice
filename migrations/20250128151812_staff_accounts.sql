CREATE TYPE staff_role AS ENUM ('manager', 'staff');

CREATE TABLE staff_accounts (
    id SERIAL PRIMARY KEY,
    login VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role staff_role NOT NULL
);