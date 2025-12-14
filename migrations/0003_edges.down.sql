-- Drop edges table and indexes
DROP TABLE IF EXISTS edges CASCADE;
DROP INDEX IF EXISTS idx_edges_target;
DROP INDEX IF EXISTS idx_edges_source;