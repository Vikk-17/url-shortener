# GET routes
curl -X GET http://localhost:8080/

# curl -X GET http://localhost:8080/api/v1/{slug}

# POST routes
curl -X POST http://localhost:8080/api/v1/data/shorten 

echo 

curl -H "Content-Type: application/json" \
-d '{"longurl": "https://www.google.com/"}' \
-X POST \
http://localhost:8080/api/v1/data/shorten

