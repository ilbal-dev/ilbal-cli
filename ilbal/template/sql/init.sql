-- Create roles for web app access
CREATE ROLE authenticator WITH LOGIN NOINHERIT NOCREATEDB NOCREATEROLE NOSUPERUSER PASSWORD '${POSTGRES_PASSWORD}';
CREATE ROLE webuser WITH NOLOGIN;
GRANT webuser TO authenticator;

-- Create schema 'extension' and prepare it for use
CREATE SCHEMA extensions;
ALTER DATABASE ${POSTGRES_DB} SET search_path = "$user", public, extensions;
GRANT USAGE ON SCHEMA extensions TO webuser;

-- Create extensions
CREATE EXTENSION citext SCHEMA extensions;
CREATE EXTENSION "uuid-ossp" SCHEMA extensions;
CREATE EXTENSION pgcrypto SCHEMA extensions;
CREATE EXTENSION pgjwt SCHEMA extensions;
CREATE EXTENSION postgis SCHEMA extensions;
