# GET routes
curl -X GET http://localhost:8080/

# POST routes
curl -X POST http://localhost:8080/api/v1/data/shorten 

echo 

curl -H "Content-Type: application/json" \
-d '{"url": "something.com"}' \
-X POST \
http://localhost:8080/api/v1/data/shorten


# sqlx command
# sqlx migrate add -r init
# sqlx migrate run
# sqlx migrate revert
