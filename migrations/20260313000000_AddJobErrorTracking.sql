-- Add error tracking fields to jobs table (required by cja job system)
ALTER TABLE jobs
ADD COLUMN IF NOT EXISTS error_count INTEGER NOT NULL DEFAULT 0;

ALTER TABLE jobs
ADD COLUMN IF NOT EXISTS last_error_message TEXT;

ALTER TABLE jobs
ADD COLUMN IF NOT EXISTS last_failed_at TIMESTAMPTZ;
