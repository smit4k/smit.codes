#!/bin/bash
set -e

echo "🔨 Building LambdaVim documentation..."

# Navigate to submodule
cd lambdavim-website

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
  echo "📦 Installing dependencies..."
  bun install
fi

# Build the site
echo "📦 Building static site..."
bun run build

# Navigate back to root
cd ..

# Create static/lambdavim directory
mkdir -p frontend/static/lambdavim

# Copy build output to static folder
echo "📋 Copying to frontend/static/lambdavim..."
rm -rf frontend/static/lambdavim/*
cp -r lambdavim-website/dist/* frontend/static/lambdavim/

echo "✅ LambdaVim docs built and copied successfully!"
echo "   Available at: /lambdavim in your site"
