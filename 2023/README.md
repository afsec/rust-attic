# README

## Sea-ORM


### Migration
```
DATABASE_URL="sqlite://./database.sqlite3?mode=rwc" sea-orm-cli migrate refresh
```

### Entity generation
```
sea-orm-cli generate entity -u "sqlite://./database.sqlite3?mode=ro" -o src/entities
```
