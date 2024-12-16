package main

import (
    "go.uber.org/zap"
    "keg/src"
)

func main() {
    cfg := app.Config{
      Addr: ":8080",
    };

    logger := zap.Must(zap.NewProduction()).Sugar()
    defer logger.Sync()

    app := &app.Application{
      Config: cfg,
      Logger: logger,
    }
    app.Run(app.Mount())
}
