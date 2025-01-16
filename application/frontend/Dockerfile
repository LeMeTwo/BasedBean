# Etap 1: Budowanie aplikacji
FROM node:18-alpine AS builder

WORKDIR /app
COPY webpage/vite.config.ts /app/vite.config.ts
COPY webpage/package*.json ./
RUN npm install
RUN npm i --save-dev @types/node
COPY webpage/ ./
RUN npm run build

# Etap 2: Serwowanie aplikacji za pomocą Nginx
FROM nginx:1.25-alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY webpage/vite.config.ts /app/vite.config.ts
# Kopiowanie pliku konfiguracyjnego Nginx - WAŻNE
COPY webpage/nginx.conf /etc/nginx/nginx.conf

# Kopiowanie skryptu uruchamiającego
COPY webpage/docker-entrypoint.sh /docker-entrypoint.sh
RUN chmod +x /docker-entrypoint.sh

# Usuwanie domyślnego pliku konfiguracyjnego Nginx
RUN rm /etc/nginx/conf.d/default.conf

# Kopiowanie własnego pliku nginx.conf
COPY webpage/nginx.conf /etc/nginx/nginx.conf

# Ustawienie zmiennych środowiskowych
ENV VITE_SERVER_IP=localhost
ENV VITE_SERVER_PORT=8090

EXPOSE 80

# Uruchomienie skryptu startowego
ENTRYPOINT ["/docker-entrypoint.sh"]
