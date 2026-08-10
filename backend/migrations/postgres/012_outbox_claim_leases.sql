ALTER TABLE outbox_events ADD COLUMN claim_token UUID;
ALTER TABLE outbox_events ADD COLUMN lease_expires_at TIMESTAMPTZ;
CREATE INDEX IF NOT EXISTS idx_outbox_claim_lease
    ON outbox_events(status, lease_expires_at);
