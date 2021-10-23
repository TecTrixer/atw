CREATE TABLE questions (
	id UUID PRIMARY KEY,
	question VARCHAR NOT NULL,
	created_at TIMESTAMP NOT NULL,
	votes_yes INT NOT NULL,
	votes_no INT NOT NULL,
	created_by VARCHAR NOT NULL,
	active BOOL NOT NULL
);
