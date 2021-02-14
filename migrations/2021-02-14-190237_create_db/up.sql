-- Your SQL goes here
create table experiment (
    id serial primary key,
    title varchar(150) not null,
    author varchar(150) not null
);

create table  granule (
    id serial primary key,
    valid boolean not null default false,
    area real,
    experiment_id integer not null,
    foreign key (experiment_id) references experiment(id)
);

-- Create an index for faster searching via lower(author)
CREATE INDEX experiment_lower_author_index ON experiment (lower(author));
