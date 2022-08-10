# MariaDB quick setup

Launch MariaDB container:

```
podman run --detach --name dict_db --env MARIADB_ROOT_PASSWORD=your_password -p 3306:3306 mariadb:latest
export DATABASE_URL="mysql://root:your_password@localhost:3306/test"
```

RUST_LOG=trace cargo run --bin telegram
