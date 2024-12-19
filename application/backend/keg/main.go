package main

import (
	"keg/src/app"
	"keg/src/cache"
	"keg/src/db"

	"github.com/redis/go-redis/v9"
	"go.uber.org/zap"
)

func main() {
    dbCfg := db.DbConfig{
        Addr: "localhost:6379",
        Password: "",
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
        Addr: ":8080",
    };

    app := &app.Application{
        Config: cfg,
        Logger: logger,
        Storage: storage,
        Cache: cache,
    }

    logger.Fatal(app.Run(app.Mount()))
}
