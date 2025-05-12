#!/usr/bin/env bash
set -euo pipefail

source .env

# Configuration
REMOTE="$REMOTE_USER@$REMOTE_HOST"
APP_NAME="telebot"
REMOTE_APP_PATH="/home/$REMOTE_USER/webdrive"

# Step 1: Build the application locally
echo "🔧 Building the application..."
nix build 

NIX_STORE_PATH=$(realpath result)
echo "Nix path: ${NIX_STORE_PATH}"

# Step 2: Copy the closure to the remote machine
echo "📦 Copying the closure to the remote machine..."
# May need to add command=". $HOME/.nix-profile/etc/profile.d/nix.sh; if [ -n \"$SSH_ORIGINAL_COMMAND\" ]; then eval \"$SSH_ORIGINAL_COMMAND\"; else exec \"$SHELL\"; fi" to ./ssh/authorized_keys to load Nix in a non-interactive environment 
nix copy --to ssh://$REMOTE ./result

# Step 3: Kill any existing instance of the application on the remote machine
# echo "🛑 Terminating existing instances of $APP_NAME on the remote machine..."
# ssh $REMOTE "pkill -f '$APP_NAME' || true"
#
# # Step 4: Deploy and run the new instance
# echo "🚀 Deploying and starting the new instance..."
# ssh $REMOTE bash -c "'
#   set -euo pipefail
#   mkdir -p $REMOTE_APP_PATH
#   export TELOXIDE_TOKEN=${TELOXIDE_TOKEN}
#   export DATABASE_URL=${DATABASE_URL}
#   export RUST_LOG=${RUST_LOG}
#   export APCA_API_KEY_ID=${APCA_API_KEY_ID}
#   export APCA_API_SECRET_KEY=${APCA_API_SECRET_KEY}
#
#   cd $REMOTE_APP_PATH
#   nohup  $NIX_STORE_PATH/bin/$APP_NAME > $REMOTE_APP_PATH/${APP_NAME}.log 2>&1 &
#   echo \"✅ $APP_NAME started successfully.\"
# '"
