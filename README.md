# SETUP on LOCALHOST

## INSTALLs

- brew install postgresql(@14) dbmate cornucopia
(mold does not work on Mac. No need to install it. See below)

## IMPORTANT STEPs

- export DATABASE_URL=postgresql://sohail@localhost:5432/db_assistantai?sslmode=disable
- alias cw='mold -run cargo watch --no-gitignore -i "*.scss" -i "*.ts" -i node_modules -x run'
(Mac: alias cw='cargo watch --no-gitignore -i "*.scss" -i "*.ts" -i node_modules -x run')
