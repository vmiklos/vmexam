-- Copyright (c) 2006 by Miklos Vajna <vmiklos@frugalware.org>
-- pacman.sql for Frugalware
-- distributed under GPL License

create table ct_local_conflicts
(
  package_id int(11) not null,
  conflict_id int(11) not null,
  ctype tinyint(4) default null,
  version tinyint(4) default null
);

create table ct_local_depends
(
  package_id int(11) not null,
  depend_id int(11) not null,
  dtype tinyint(4) default null,
  version varchar(255) default null
);

create table ct_local_groups
(
  group_id int(11) not null,
  package_id int(11) not null
);

create table ct_local_provides
(
  package_id int(11) not null,
  provide_id int(11) not null,
  ptype tinyint(4) default null,
  version tinyint(4) default null
);

create table ct_sync_conflicts
(
  package_id int(11) not null,
  conflict_id int(11) not null,
  ctype tinyint(4) default null,
  version tinyint(4) default null
);

create table ct_sync_depends
(
  package_id int(11) not null,
  depend_id int(11) not null,
  dtype tinyint(4) default null,
  version varchar(255) default null
);

create table ct_sync_groups
(
  group_id int(11) not null,
  package_id int(11) not null
);

create table ct_sync_provides
(
  package_id int(11) not null,
  provide_id int(11) not null,
  ptype tinyint(4) default null,
  version tinyint(4) default null
);

create table ct_sync_replaces
(
  package_id int(11) not null,
  replace_id int(11) not null,
  rtype tinyint(4) default null,
  version tinyint(4) default null
);

create table local_files
(
  path text not null,
  package_id int(11) not null,
  sha1sum varchar(40) default null
);

create table local_groups
(
  id int(11) not null,
  name varchar(255) not null,
  primary key  (id)
);

create table local_packages (
  id int(11) not null auto_increment,
  pver varchar(255) not null,
  pdesc varchar(255) not null,
  url varchar(255) not null,
  license varchar(255) default null,
  arch varchar(255) not null,
  builddate timestamp not null default '0000-00-00',
  installdate timestamp not null default current_timestamp on update current_timestamp,
  packager varchar(255) not null,
  reason tinyint(4) not null,
  psize int(11) not null,
  sha1sum varchar(40) not null,
  scriptlet text,
  changelog text,
  primary key  (id)
);

create table sync_groups
(
  id int(11) not null,
  name varchar(255) not null,
  primary key  (id)
);

create table sync_packages
(
  id int(11) not null auto_increment,
  pver varchar(255) not null,
  pdesc varchar(255) not null,
  arch varchar(255) not null,
  csize int(11) not null,
  sha1sum varchar(40) not null,
  pforce tinyint(4) not null,
  repo_id mediumint(9) not null,
  primary key  (id)
);

create table sync_repos
(
  id int(11) not null,
  name varchar(255) not null,
  primary key  (id)
);
