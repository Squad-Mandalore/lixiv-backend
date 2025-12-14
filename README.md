# Lixiv

## Database: Postgres

Install postgres on your system and create a database.
The application uses dotenvy to get the environment varaibles.

To add your database url do (never push your .env file):

```
echo DATABASE_URL=postgres://username:password@localhost/database_name > .env
```

```
sqlx database create
sqlx migrate run
```

