drop table if exists granule;
drop table if exists experiment;

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
insert into experiment (title, author) values ('NaAs', 'Carl'), ('Cz', granule'Tom');
insert into granule (experiment_id, area) values (1, 10.3), (1, 13.2), (2, 4.6);
