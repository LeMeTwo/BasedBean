package main

import (
	// "go.uber.org/zap"
	// "keg/src/app"
	// "fmt"
	"context"
	"fmt"
	"keg/src/db"
)

func main() {
    dbCfg := db.DbConfig{
        Addr: "localhost:6379",
        Password: "",
    }
    storage := db.NewKeyDbStorage(dbCfg)
    ctx := context.Background()
    _, err := storage.FetchBatchReservedKeys(ctx, 50)
    if err != nil {
        fmt.Printf("err: %v\n", err)
    }
    // cfg := app.Config{
    //   Addr: ":8080",
    // };
    //
    // logger := zap.Must(zap.NewProduction()).Sugar()
    // defer logger.Sync()
    //
    // app := &app.Application{
    //   Config: cfg,
    //   Logger: logger,
    // }
    // app.Run(app.Mount())
}
