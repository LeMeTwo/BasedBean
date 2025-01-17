#!/bin/bash

# Generowanie mongodb-keyfile i tworzenie sekretu
echo "Generowanie mongodb-keyfile..."
sudo bash -c "openssl rand -base64 756 > mongodb-keyfile"
sudo chmod 766 mongodb-keyfile

echo "Tworzenie sekretu Kubernetes..."
kubectl create secret generic mongodb-keyfile --from-file=mongodb-keyfile

echo "Tworzenie podstawowego deploymentu"
kubectl apply -f keg.yaml
kubectl apply -f server.yaml
kubectl apply -f keydb.yaml

# Tworzenie zasobów Kubernetes z manifestu
echo "Tworzenie zasobów z manifestu..."
kubectl apply -f mongodb-ha.yaml

# Oczekiwanie na StatefulSet
echo "Czekam, aż StatefulSet mongodb będzie gotowy..."
kubectl rollout status statefulset/mongodb --timeout=180s

echo "Pakowanie kluczy do keyDB"
kubectl exec keydb-0 -- sh -c 'seq 1000 9999 | sed "s/\(\S*\)/sadd \1.avail \"\"/" | keydb-cli > /dev/null'

# Oczekiwanie na pody StatefulSet
echo "Czekam, aż wszystkie pody StatefulSet mongodb będą gotowe..."
for pod in mongodb-0 mongodb-1 mongodb-2; do
  kubectl wait --for=condition=ready pod/$pod --timeout=180s
done

# Inicjalizacja repliki MongoDB
echo "Inicjalizacja repliki MongoDB..."
kubectl exec -it mongodb-0 -- mongosh --eval '
rs.initiate({
  _id: "rs0",
  members: [
    { _id: 0, host: "mongodb-0.mongodb-service:27017", priority: 2 },
    { _id: 1, host: "mongodb-1.mongodb-service:27017", priority: 1 },
    { _id: 2, host: "mongodb-2.mongodb-service:27017", priority: 1 }
  ]
})
'

kubectl apply -f frontend.yaml

echo "Replika MongoDB została zainicjowana."
