CREATE TABLE users
(
    id int PRIMARY KEY AUTO_INCREMENT,
    username varchar(191) NOT NULL UNIQUE,
    email varchar(191) NOT NULL UNIQUE,
    password_hash varchar(191) NOT NULL,
    display_name varchar(191) NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);