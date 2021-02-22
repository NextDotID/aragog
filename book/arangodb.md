
# Set up ArangoDB

**Installation** (See official documentation [Here](https://www.arangodb.com/docs/stable/getting-started.html))

* [Download Link](https://www.arangodb.com/download)
* Run it with `/usr/local/sbin/arangod` The default installation contains one database `_system` and a user named `root`
* Create a user and database for the project with the `arangosh` shell

 ```bash
 arangosh> db._createDatabase("DB_NAME");
 arangosh> var users = require("@arangodb/users");
 arangosh> users.save("DB_USER", "DB_PASSWORD");
 arangosh> users.grantDatabase("DB_USER", "DB_NAME");
 ```
> It is a good practice to create a test db and a development db.
* you can connect to the new created db with
 ```bash
 $> arangosh --server.username $DB_USER --server.database $DB_NAME
 ```
