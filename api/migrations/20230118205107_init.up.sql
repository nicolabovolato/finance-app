CREATE TABLE users(
    id UUID PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL
);

CREATE TABLE accounts(
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) NOT NULL,
    name VARCHAR NOT NULL,
    balance NUMERIC(10,2) NOT NULL,
    currency VARCHAR NOT NULL
);

CREATE TABLE movements(
    id UUID PRIMARY KEY,
    account_id UUID REFERENCES accounts(id) NOT NULL,
    category VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    amount NUMERIC(10,2) NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL
);