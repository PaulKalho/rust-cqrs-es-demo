CREATE TABLE events
(
    aggregate_type text NOT NULL,
    aggregate_id text NOT NULL,
    sequence bigint CHECK (sequence >= 0) NOT NULL,
    event_type text NOT NULL,
    event_version text NOT NULL,
    payload json NOT NULL,
    metadata json NOT NULL,
    PRIMARY KEY (aggregate_type, aggregate_id, sequence)
);

CREATE TABLE project_view
(
    view_id text NOT NULL,
    version bigint CHECK (version >= 0) NOT NULL,
    payload json NOT NULL,
    PRIMARY KEY (view_id)
);

GRANT ALL PRIVILEGES ON DATABASE postgres TO postgres;

