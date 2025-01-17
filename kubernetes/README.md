Build docker images from corresponding locations:
docker build --tag local/frontend-app:v16 ../application/frontend/
docker build --tag local/basedbeanserver:v1 ../application/backend/server
docker build --tag local/basedbeankeg:v1 ../application/backend/keg

Then, run setUpCluster.sh

Expose svc/frontend-app-service either using LB service, or via port-forwarding:
kubectl port-forward svc/frontend-app-service 8090:80