/*
-- SEQUIN SETUP
-- Create user with a secure password
CREATE USER sequin_user WITH PASSWORD '${POSTGRES_PASSWORD}';

-- Grant connect permission
GRANT CONNECT ON DATABASE ${POSTGRES_DB} to sequin_user;

-- Grant permission to create replication tables
GRANT CREATE ON SCHEMA public TO sequin_user;
GRANT USAGE ON SCHEMA public TO sequin_user;

-- Grant select permission on tables you want to replicate
-- grant select on table table1, table2, table3 to sequin_user;
-- OR grant select on all tables in a schema
GRANT SELECT ON ALL TABLES IN SCHEMA public to sequin_user;

-- Grant replication permission
ALTER USER sequin_user WITH REPLICATION;
*/

-- Create publication and slot
CREATE PUBLICATION sequin_pub FOR ALL TABLES WITH (publish_via_partition_root = true);
SELECT pg_create_logical_replication_slot('sequin_slot', 'pgoutput');

-- PGDOG SETUP
CREATE ROLE pgdog LOGIN NOINHERIT NOCREATEDB NOCREATEROLE NOSUPERUSER WITH PASSWORD '${POSTGRES_PASSWORD}';
CREATE ROLE authenticator LOGIN NOINHERIT NOCREATEDB NOCREATEROLE NOSUPERUSER;
CREATE ROLE anonymous NOLOGIN;
CREATE ROLE webuser NOLOGIN;
