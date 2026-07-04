#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
KUBE_DIR="$SCRIPT_DIR/kube"
COMPOSE_FILE="$SCRIPT_DIR/compose.yml"
HELM_DIR="$KUBE_DIR/helm"
PODMAN_FILE="$KUBE_DIR/podman.yml"
ENV_FILE="$KUBE_DIR/.env"
VALUES_FILE="$KUBE_DIR/values.yaml"

# --- Prerequisites ---
if ! command -v kompose &>/dev/null; then
  echo "Error: 'kompose' is required. Download from https://kompose.io/"
  exit 1
fi

if ! command -v helm &>/dev/null; then
  echo "Error: 'helm' is required. Download from https://helm.sh/"
  exit 1
fi

# --- Ensure output directory exists ---
mkdir -p "$KUBE_DIR"

# --- Step 1: Generate env file from placeholders if missing ---
if [ ! -f "$ENV_FILE" ]; then
  cat > "$ENV_FILE" <<-EOF
PROJECT_NAME=my-project
POSTGRES_USER=duck
POSTGRES_PASSWORD=quack
POSTGRES_DB=my-project
PG_PORT=33330
PGADMIN_PORT=33331
PGADMIN_EMAIL=admin@ilbal.dev
PGADMIN_PASSWORD=admin
PGDOG_PORT=33332
EOF
  echo "Created $ENV_FILE with default values. Edit it before re-running."
fi

# --- Step 2: Load env vars and convert compose.yml to Helm chart ---
echo "Converting compose.yml to Helm chart..."

# Source env vars so kompose can interpolate values
set -a
# shellcheck disable=SC1090
. "$ENV_FILE"
set +a

# Pre-resolve env vars with perl (avoids variable-in-YAML-keys issues with kompose)
# Write resolved compose to a temp file to keep the original intact
RESOLVED_COMPOSE=$(mktemp)
perl -pe 's/\$\{(\w+)\}/$ENV{$1}/g' < "$COMPOSE_FILE" > "$RESOLVED_COMPOSE"

rm -rf "$HELM_DIR"
kompose convert -f "$RESOLVED_COMPOSE" -c -o "$HELM_DIR"
rm -f "$RESOLVED_COMPOSE"

echo "Helm chart created at $HELM_DIR"

# Fix chart name — kompose uses the full path, but helm rejects it
CHART_NAME=$(basename "$HELM_DIR")
perl -i -pe "s/^name: .*/name: $CHART_NAME/; s/^description: .*/description: A generated Helm Chart for $CHART_NAME/;" "$HELM_DIR/Chart.yaml"
# Remove dangling list items under keywords (perl -i can't easily delete multi-line blocks)
perl -i -0 -pe "s/keywords:\n(  .*\n)*/keywords: []\n/" "$HELM_DIR/Chart.yaml"

# --- Step 3: Render Helm chart to Kubernetes YAML (podman-ready) ---
echo "Rendering Helm chart to $PODMAN_FILE..."
# Convert .env (KEY=VALUE) to YAML for helm values
perl -pe 's/^([A-Z_]+)=(.*)/\L$1: $2/' < "$ENV_FILE" > "$VALUES_FILE"
helm template ilbal-release "$HELM_DIR" -f "$VALUES_FILE" > "$PODMAN_FILE"

# Clean up temporary values file
rm -f "$VALUES_FILE"

echo "Done. Run with: podman play kube $PODMAN_FILE"
