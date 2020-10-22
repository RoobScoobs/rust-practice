/**

    The corresponding down.sql file should perform whatever transformations are
    necessary to undue what happens in up.sql

     In this case as the up migration is creating a table,
     we can drop the table in our down migration.

**/


DROP TABLE users