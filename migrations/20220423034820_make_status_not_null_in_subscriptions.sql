-- Wrap the migration in a transaction to make sure it
-- succeeds or fails atomically
begin;
  -- Backfill `status`
  update subscriptions
  set status = 'confirmed'
  where status is null;

  -- Make `status` mandatory
  alter table subscriptions alter column status set not null;
commit;
