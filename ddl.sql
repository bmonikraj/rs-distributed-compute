CREATE TABLE public.analytics (
	id uuid NOT NULL,
	algorithm text NOT NULL,
	"parameter" text NOT NULL,
	"result" text NOT NULL,
	created_at timestamp NOT NULL,
	updated_at timestamp NOT NULL,
	CONSTRAINT analytics_pkey PRIMARY KEY (id)
);