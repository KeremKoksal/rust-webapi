create extension if not exists "uuid-ossp";


create table departments (
  id smallserial primary key, 
  name varchar not null unique, 
  short_name varchar not null unique, 
  external_id int null
);
comment on column departments.external_id is 'This column can be used for synchronization with other applications';

create table users (
  id uuid default uuid_generate_v4() primary key, 
  username varchar not null unique, 
  staff_title varchar null, 
  education_title varchar null, 
  email varchar not null unique, 
  password varchar null, 
  first_name varchar null, 
  last_name varchar null, 
  bio varchar null, 
  image varchar null, 
  department_id smallint null, 
  email_verified boolean not null default false, 
  active boolean not null default true, 
  roles text [] not null default array[]::text[], 
  created_at timestamp not null default current_timestamp, 
  updated_at timestamp not null default current_timestamp, 
  constraint fk_user_department_id
    foreign key(department_id) 
    references departments(id)
);

comment on column users.roles is 
  'Short name of users departments will be added as a role.
  Users will see every ticket on their departments and can take them upon themselves. 
  A user can assign a ticket to another user, if (s)he is an admin of that department. 
  ie: roles = ["bidb_admin"]';

