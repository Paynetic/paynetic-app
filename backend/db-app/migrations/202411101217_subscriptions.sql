CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $body$
BEGIN
    NEW."updated_at" = now();
    RETURN NEW;
END;
$body$ LANGUAGE 'plpgsql';

CREATE TABLE subscriptions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id),
    product_id uuid NOT NULL REFERENCES products(id),
    contract_address TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL,
    fee_percent INT NOT NULL,
    start_time BIGINT NOT NULL,
    current_start BIGINT NOT NULL,
    current_end BIGINT NOT NULL,
    grace BIGINT NOT NULL,
    blockchain_status TEXT NOT NULL,
    transaction_hash TEXT,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER subscriptions_modified_column
BEFORE UPDATE ON subscriptions FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();

CREATE TABLE subscription_prices (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    subscription_id uuid NOT NULL REFERENCES subscriptions(id),
    price_id uuid NOT NULL REFERENCES prices (id)
);
