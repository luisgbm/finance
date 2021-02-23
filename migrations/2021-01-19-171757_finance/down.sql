DROP EXTENSION pgcrypto;

DROP TABLE accounts CASCADE;
DROP TABLE categories CASCADE;
DROP TABLE app_users CASCADE;
DROP TABLE scheduled_transactions;
DROP TABLE transactions CASCADE;
DROP TABLE transfers CASCADE;

DROP TYPE category_types CASCADE;
DROP TYPE repeat_frequencies CASCADE;
DROP TYPE scheduled_transacion_kind CASCADE;

DROP SEQUENCE transactions_transfers_id_seq;
