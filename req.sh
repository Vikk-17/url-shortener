for i in $(seq 1 10000);
do
    echo "Sending"
    curl -X GET http://localhost:8080/prom/
done
