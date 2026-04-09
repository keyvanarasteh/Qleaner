#!/usr/bin/env bash

# Exit on any error
set -e

echo "🚀 Starting Website Deployment Process for Qleaner..."

# Load .env variables
if [ -f .env ]; then
  source .env
elif [ -f ../.env ]; then
  source ../.env
elif [ -f ../../.env ]; then
  source ../../.env
else
  echo "❌ Error: .env file not found. Please create one with FTP/SSH credentials."
  exit 1
fi

# Ensure required SSH variables are set
if [ -z "$SSH_HOST" ] || [ -z "$SSH_USER" ] || [ -z "$SSH_TARGET_DIR" ]; then
  echo "❌ Error: Missing required SSH credentials in .env file."
  exit 1
fi

# Safety check: Ensure target dir contains 'qleaner' to prevent overwriting other host codes
if [[ "$SSH_TARGET_DIR" != *"/qleaner"* ]]; then
  echo "❌ Error: SSH_TARGET_DIR ($SSH_TARGET_DIR) doesn't seem to point to the 'qleaner' subdirectory."
  echo "Refusing to deploy to prevent overwriting other codes on the host."
  exit 1
fi

SSH_PORT=${SSH_PORT:-22}
ZIP_NAME="deploy_website.zip"

echo "📦 Building the website..."
# Ensure we run from the website directory
if [ -d "website" ]; then
  cd website
elif [ ! -f "package.json" ] && [ -f "../website/package.json" ]; then
  cd ../website
elif [ ! -f "package.json" ]; then
  echo "❌ Error: Cannot find website/package.json. Run this script from root or scripts dir."
  exit 1
fi

bun run build

echo "🗜️  Compressing the build directory..."
cd build
rm -f ../$ZIP_NAME
zip -r ../$ZIP_NAME ./*
cd ..

echo "🌐 Uploading $ZIP_NAME securely via SCP..."

SCP_CMD="scp -P $SSH_PORT $ZIP_NAME \"$SSH_USER@$SSH_HOST:$SSH_TARGET_DIR/$ZIP_NAME\""

# Strictly verify remote path before unzipping
REMOTE_EXTRACT_CMD="
  if [ ! -d \"$SSH_TARGET_DIR\" ]; then
    echo 'Creating remote directory...';
    mkdir -p \"$SSH_TARGET_DIR\";
  fi;
  cd \"$SSH_TARGET_DIR\" || exit 1;
  # Extra safety wrapper check
  if [[ \"\$(pwd)\" != *\"qleaner\"* ]]; then
    echo '❌ Remote check failed: Not in a qleaner related path. Aborting!';
    exit 1;
  fi;
  echo 'Extracting $ZIP_NAME...';
  unzip -o $ZIP_NAME;
  rm $ZIP_NAME;
  echo '🎉 Extraction complete!';
"

SSH_CMD="ssh -p $SSH_PORT \"$SSH_USER@$SSH_HOST\" \"$REMOTE_EXTRACT_CMD\""

if [ ! -z "$SSH_PASSWORD" ] && command -v sshpass &> /dev/null; then
    echo "Using sshpass for automated password entry..."
    echo "Creating remote directory if missing..."
    sshpass -p "$SSH_PASSWORD" ssh -o StrictHostKeyChecking=no -p $SSH_PORT "$SSH_USER@$SSH_HOST" "mkdir -p $SSH_TARGET_DIR"

    sshpass -p "$SSH_PASSWORD" scp -o StrictHostKeyChecking=no -P $SSH_PORT $ZIP_NAME "$SSH_USER@$SSH_HOST:$SSH_TARGET_DIR/$ZIP_NAME"
    echo "✅ SCP Upload complete!"
    
    echo "🔑 Connecting via SSH to extract the archive safely..."
    sshpass -p "$SSH_PASSWORD" ssh -o StrictHostKeyChecking=no -p $SSH_PORT "$SSH_USER@$SSH_HOST" "$REMOTE_EXTRACT_CMD"
else
    if [ ! -z "$SSH_PASSWORD" ]; then
        echo "⚠️  SSH_PASSWORD is set but 'sshpass' is not installed (sudo apt install sshpass). Defaulting to manual password prompt..."
    fi
    echo "Creating remote directory if missing..."
    ssh -p $SSH_PORT "$SSH_USER@$SSH_HOST" "mkdir -p $SSH_TARGET_DIR"
    
    eval $SCP_CMD
    echo "✅ SCP Upload complete!"
    
    echo "🔑 Connecting via SSH to extract the archive safely..."
    eval $SSH_CMD
fi

echo "🧹 Cleaning up local zip..."
rm $ZIP_NAME

echo "🚀 Website Deployment finished successfully! Target: $WEBSITE_URL"
