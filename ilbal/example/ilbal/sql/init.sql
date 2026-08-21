-- pgDog setup: Create publication and slot
CREATE PUBLICATION sequin_pub FOR ALL TABLES WITH (publish_via_partition_root = true);
SELECT pg_create_logical_replication_slot('sequin_slot', 'pgoutput');

-- Create roles for web app access
CREATE ROLE authenticator WITH LOGIN NOINHERIT NOCREATEDB NOCREATEROLE NOSUPERUSER PASSWORD 'quack123';
CREATE ROLE webuser WITH NOLOGIN;
GRANT webuser TO authenticator;

-- Create schema 'extension' and prepare it for use
CREATE SCHEMA extensions;
ALTER DATABASE app SET search_path = "$user", public, extensions;
GRANT USAGE ON SCHEMA extensions TO webuser;

-- Create extensions
CREATE EXTENSION citext SCHEMA extensions;
CREATE EXTENSION "uuid-ossp" SCHEMA extensions;
CREATE EXTENSION pgcrypto SCHEMA extensions;
CREATE EXTENSION pgjwt SCHEMA extensions;
CREATE EXTENSION postgis SCHEMA extensions;
