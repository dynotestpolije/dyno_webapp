

## REQUIREMENT
- compiler (rust)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- databases
```bash
# if using sqlite
sudo apt install sqlite3 libsqlite3-0 libsqlite3-dev
# if using postgresql
sudo apt install libpq-dev
# if using mysql
sudo apt install libmysqlclient-dev

cargo install diesel_cli --no-default-features --features sqlite
``` 


## TEST REQUEST
login: curl -d'{"nim":"e32201406","password":"password123"}' -H 'Content-Type: application/json' -X POST http://localhost:8080/api/auth/login -v
