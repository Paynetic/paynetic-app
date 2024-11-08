# Paynetic CI Cluster

K3D clusters are run in CI for API integration and Playwright E2E tests.

The cluster setup is defined in `cluster-config.yaml`, and k8s deployments/services/configs are in the subfolders of this directoring.

## Run

Follow these steps to run the CI cluster locally. Docker Desktop and K3D must be available.

All steps start from the repository root.

**Build apps for production**

```bash
npm run prod:build
# Copy amd64 build to release folder
rm -rf backend/target/release
cp -rf backend/target/aarch64-unknown-linux-gnu/release backend/target
```

**Build Docker images**

```bash
./tools/k8s/ci/build-all.sh
```

**Create cluster**

```bash
k3d cluster create paynetic-ci-cluster --config=./tools/k8s/ci/cluster-config.yaml
```

**Import images to cluster**

```bash
k3d image import paynetic-api.prod -c paynetic-ci-cluster --verbose
k3d image import jobs-test-helper.prod -c paynetic-ci-cluster --verbose
k3d image import api-test-helper.prod -c paynetic-ci-cluster --verbose
k3d image import db-app.prod -c paynetic-ci-cluster --verbose

# List images
docker exec -it k3d-paynetic-ci-cluster-server-0 crictl images
```

**Apply kubeconfigs**

```bash
kubectl apply -f ./tools/k8s/ci/shared
kubectl apply -f ./tools/k8s/ci/db
kubectl apply -f ./tools/k8s/ci/paynetic-api
kubectl get events
kubectl get pods,deploy,services,pv,pvc -o wide
```

**Wait for cluster**

```bash
kubectl wait deployment paynetic-api --for condition=Available=True --timeout=90s
kubectl logs deploy/paynetic-api
```

**Run integration tests**

```bash
EXEC_ENV=ci npm run backend-test
```

**Destroy cluster**

```bash
k3d cluster delete paynetic-ci-cluster
```