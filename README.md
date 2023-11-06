## Rust on Nails

Built with the [Rust on Nails](https://rust-on-nails.com/) architecture for secure full stack web applications.

# SETUP on LOCALHOST

## INSTALLs

- brew install postgresql(@14) dbmate cornucopia
(mold does not work on Mac. No need to install it. See below)

## IMPORTANT STEPs

- export DATABASE_URL=postgresql://sohail@localhost:5432/db_assistantai?sslmode=disable
- alias cw='mold -run cargo watch --no-gitignore -i "*.scss" -i "*.ts" -i node_modules -x run'
(Mac: alias cw='cargo watch --no-gitignore -i "*.scss" -i "*.ts" -i node_modules -x run')

## NPM RUN (asset-pipeline)

- npm install && npm run start
