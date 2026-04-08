# Evolith Kubernetes Deployment

## Architecture

```
                    ┌─────────────┐
                    │   Ingress   │  (nginx-ingress-controller)
                    │  evolith.io │
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
        /api/*, /mcp/*   /health      /*
              │            │            │
       ┌──────┴──────┐    │     ┌──────┴──────┐
       │   Backend   │◄───┘     │  Frontend   │
       │  (2-8 pods) │          │  (2-6 pods)  │
       │  :8080      │          │  :3000       │
       └──────┬──────┘          └─────────────┘
              │
       ┌──────┴──────┐
       │   Postgres  │          ┌─────────────┐
       │  StatefulSet│          │    Redis     │
       │  :5432      │          │  Deployment  │
       └─────────────┘          │  :6379       │
                                └─────────────┘
```

## Prerequisites

- Kubernetes 1.28+
- kubectl
- NGINX Ingress Controller installed
- cert-manager (for TLS, optional)

## Quick Start

```bash
# 1. Create secrets (fill in real values first)
cp deploy/k8s/secrets.yaml.example deploy/k8s/secrets.yaml
# Edit secrets.yaml with real passwords/keys
kubectl apply -f deploy/k8s/secrets.yaml

# 2. Deploy everything
kubectl apply -k deploy/k8s/

# 3. Watch pods come up
kubectl get pods -n evolith -w

# 4. Check services
kubectl get svc -n evolith

# 5. Verify ingress
kubectl get ingress -n evolith
```

## File Structure

| File | Description |
|:---|:---|
| `namespace.yaml` | Namespace `evolith` |
| `configmap.yaml` | Non-sensitive env vars (CORS, rate limits, sandbox config) |
| `secrets.yaml.example` | Secret template (DB password, JWT, Stripe, SMTP) |
| `postgres.yaml` | PostgreSQL 16 StatefulSet + PVC (10Gi) |
| `postgres-service.yaml` | PostgreSQL ClusterIP Service |
| `redis.yaml` | Redis 7 Deployment + PVC (1Gi) |
| `redis-service.yaml` | Redis ClusterIP Service |
| `backend.yaml` | Backend Deployment (2 replicas) + Service + HPA (2-8) |
| `frontend.yaml` | Frontend Deployment (2 replicas) + Service + HPA (2-6) |
| `ingress-class.yaml` | NGINX IngressClass |
| `ingress.yaml` | Ingress rules (evolith.io, app.evolith.io) |
| `kustomization.yaml` | Kustomize orchestration |

## Resource Sizing

| Component | Request CPU | Request Memory | Limit CPU | Limit Memory | Replicas |
|:---|:---:|:---:|:---:|:---:|:---:|
| Backend | 100m | 128Mi | 1000m | 512Mi | 2-8 (HPA) |
| Frontend | 50m | 64Mi | 500m | 256Mi | 2-6 (HPA) |
| PostgreSQL | 100m | 256Mi | 500m | 1Gi | 1 |
| Redis | 50m | 64Mi | 250m | 512Mi | 1 |

## TLS Setup

### With cert-manager

```bash
# Install cert-manager (if not already installed)
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.14.4/cert-manager.yaml

# Create ClusterIssuer
kubectl apply -f - <<EOF
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt-prod
spec:
  acme:
    server: https://acme-v02.api.letsencrypt.org/directory
    email: admin@evolith.io
    privateKeySecretRef:
      name: letsencrypt-prod
    solvers:
      - http01:
          ingress:
            class: nginx
EOF

# Add to ingress.yaml annotations:
#   cert-manager.io/cluster-issuer: letsencrypt-prod
#   cert-manager.io/private-key-destination: "keystorepem"
```

### Manual TLS

```bash
kubectl create secret tls evolith-tls \
  --cert=path/to/fullchain.pem \
  --key=path/to/privkey.pem \
  -n evolith
```

## Common Operations

```bash
# Scale manually
kubectl scale deployment backend --replicas=4 -n evolith

# View logs
kubectl logs -f deployment/backend -n evolith
kubectl logs -f deployment/frontend -n evolith

# Restart rollout
kubectl rollout restart deployment/backend -n evolith

# Check rollout status
kubectl rollout status deployment/backend -n evolith

# Rollback
kubectl rollout undo deployment/backend -n evolith

# Delete everything
kubectl delete -k deploy/k8s/
```

## Differences from Docker Compose

| Aspect | Docker Compose | Kubernetes |
|:---|:---|:---|
| Reverse proxy | Nginx container | Ingress Controller |
| SSL termination | Nginx in container | Ingress annotations |
| Service discovery | Docker DNS | K8s Service DNS |
| Scaling | Manual / compose scale | HPA (auto) |
| Health checks | Docker HEALTHCHECK | Liveness/Readiness probes |
| Secrets | .env file | K8s Secret |
| Config | Environment in compose | ConfigMap + envFrom |
| Volumes | Docker named volumes | PVC + StorageClass |
| Anti-affinity | N/A | podAntiAffinity (spread pods) |
