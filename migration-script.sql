-- Create extension for UUID generation (optional if using DB-side generation)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create table for assets
CREATE TABLE assets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    address JSONB NOT NULL
);