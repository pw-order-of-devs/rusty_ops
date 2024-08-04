drop schema if exists rusty cascade;
create schema rusty;

create table if not exists rusty.users (
    id varchar(36) primary key,
    username varchar(128) unique not null,
    password varchar(128) not null
);

create table if not exists rusty.roles (
    id varchar(36) primary key,
    name text not null,
    description text,
    users varchar(36)[]
);

create table if not exists rusty.resources (
    name varchar(36) primary key,
    rights text[]
);

create table if not exists rusty.permissions (
    user_id varchar(36),
    role_id varchar(36),
    resource varchar(256) not null,
    "right" varchar(256) not null,
    item varchar(256)
);

create table if not exists rusty.agents (
    id varchar(36) primary key,
    expiry integer not null
);

create table if not exists rusty.project_groups (
    id varchar(36) primary key,
    name text not null
);

create table if not exists rusty.projects (
    id varchar(36) primary key,
    name text not null,
    url text not null,
    main_branch varchar(256) not null,
    group_id varchar(36)
);

create table if not exists rusty.jobs (
    id varchar(36) primary key,
    name text not null,
    description text,
    template text not null,
    project_id text not null,
    constraint fk_job_project
        foreign key(project_id)
            references rusty.projects(id)
);

create table if not exists rusty.pipelines (
    id varchar(36) primary key,
    number integer not null,
    branch varchar(256) not null,
    register_date text not null,
    start_date text,
    end_date text,
    status text not null,
    job_id text not null,
    agent_id text,
    constraint fk_pipeline_job
        foreign key(job_id)
            references rusty.jobs(id)
);

create table if not exists rusty.pipelineLogs (
    id varchar(36) primary key,
    entries jsonb not null,
    constraint fk_pipeline_id
        foreign key(id)
            references rusty.pipelines(id)
);
