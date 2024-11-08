CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $body$
BEGIN
    NEW."updated_at" = now();
    RETURN NEW;
END;
$body$ LANGUAGE 'plpgsql';

CREATE TABLE products (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id),
    name TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    blurb TEXT NOT NULL DEFAULT '',
    contract_address TEXT NOT NULL DEFAULT '',
    payout_address TEXT NOT NULL DEFAULT '',
    category TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL,
    blockchain_status TEXT NOT NULL,
    transaction_hash TEXT,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER products_modified_column
BEFORE UPDATE ON products FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();

CREATE TABLE prices (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id uuid NOT NULL REFERENCES products(id),
    active BOOLEAN DEFAULT TRUE NOT NULL,
    name TEXT NOT NULL DEFAULT '',
    price_type TEXT NOT NULL DEFAULT 'Subscription',
    amount NUMERIC(78, 0) NOT NULL,
    base_currency TEXT NOT NULL DEFAULT 'Ethereum',
    subscription_interval TEXT,
    subscription_interval_count INT,
    trial_days INT,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER prices_modified_column
BEFORE UPDATE ON prices FOR EACH ROW
EXECUTE PROCEDURE update_modified_column()
