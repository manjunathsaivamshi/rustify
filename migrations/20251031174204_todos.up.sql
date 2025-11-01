-- Add migration script here
--drop table if exists "todos";

create table if not exists "todos" (
    id uuid primary key default uuid_generate_v4(),
    title text default null,
    description text default null,
    is_completed boolean default false,
    created_by text default null,
    created_at timestamptz not null default current_timestamp,
    updated_at timestamptz not null default current_timestamp
);

create index if not exists idx_todo_title on "todos"(title);

select manage_updated_at('todos');
