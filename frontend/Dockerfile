# frontend/Dockerfile
FROM node:22

WORKDIR /app

# 先に依存関係だけインストール（ビルド高速化のため）
COPY package*.json ./
RUN npm install

# ソースコードは docker-compose でマウントするので COPY は不要（開発用）

# React の開発サーバーを起動
CMD ["npm", "run","dev","--","--host"]