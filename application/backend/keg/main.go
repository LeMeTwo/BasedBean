package main

import (
	"keg/src/app"
	"keg/src/cache"
	"keg/src/db"
	"keg/src/env"

	"github.com/redis/go-redis/v9"
	"go.uber.org/zap"
)

const DB_ADDR string = "DB_ADDR"
const DB_PW string = "DB_PW"

const APP_ADDR string = "APP_ADDR"

func main() {
    dbCfg := db.DbConfig{
        Addr: env.GetString(DB_ADDR, "localhost:6379"),
        Password: env.GetString(DB_PW, ""),
    }
    client := redis.NewClient(&redis.Options{
        Addr: dbCfg.Addr,
        Password: dbCfg.Password,
        DB: 0,
        Protocol: 2,
    })
    defer client.Close()
    storage := db.NewKeyDbStorage(client)

    cache := cache.LocalKeyCache{}

    logger := zap.Must(zap.NewProduction()).Sugar()
    defer logger.Sync()

    cfg := app.Config{
        Addr: env.GetString(APP_ADDR, ":8080"),
    };

    app := &app.Application{
        Config: cfg,
        Logger: logger,
        Storage: storage,
        Cache: cache,
    }

    logger.Fatal(app.Run(app.Mount()))
}
