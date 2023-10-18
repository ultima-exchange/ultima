-- Your SQL goes here
DROP ROLE IF EXISTS web_anon;
CREATE ROLE web_anon nologin;


CREATE SCHEMA api;


CREATE VIEW
  api.market_registration_events AS
SELECT
  *
FROM
  market_registration_events;


GRANT usage ON SCHEMA api TO web_anon;


GRANT
SELECT
  ON api.market_registration_events TO web_anon;


CREATE EXTENSION pgjwt CASCADE;


CREATE FUNCTION api.jwt (json) RETURNS TEXT AS $$
  SELECT sign((CONCAT(CONCAT('{"mode": "r","channels": ', $1->>'channels'::text),'}'))::json, 'econia_0000000000000000000000000')
$$ LANGUAGE SQL;