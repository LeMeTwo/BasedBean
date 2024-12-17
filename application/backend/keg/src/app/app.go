package app

import (
    // "fmt"
    "errors"
    "net/http"
    "time"

    "github.com/go-chi/chi/v5"
    "github.com/go-chi/chi/v5/middleware"
    "github.com/go-chi/cors"
    "go.uber.org/zap"
)

type Application struct {
    Config Config
    Logger *zap.SugaredLogger
}

type Config struct {
    Addr string
}

func (app *Application) Mount() http.Handler {
    r := chi.NewRouter()

    r.Use(middleware.RequestID)
    r.Use(middleware.RealIP)
    r.Use(middleware.Logger)
    r.Use(middleware.Recoverer)
    r.Use(cors.Handler(cors.Options{
        // AllowedOrigins:   []string{env.GetString("CORS_ALLOWED_ORIGIN", "http://localhost:5174")},
        AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
        AllowedHeaders:   []string{"Accept", "Authorization", "Content-Type", "X-CSRF-Token"},
        ExposedHeaders:   []string{"Link"},
        AllowCredentials: false,
        MaxAge:           300,
    }))

    r.Use(middleware.Timeout(60 * time.Second))

    r.Route("/v1", func(r chi.Router) {
        r.Get("/key", app.getKey)
        r.Delete("/key", app.deleteKey)
    })

    return r
}

func (app *Application) Run(mux http.Handler) error {
    srv := &http.Server{
        Addr:         app.Config.Addr,
        Handler:      mux,
        WriteTimeout: time.Second * 30,
        ReadTimeout:  time.Second * 10,
        IdleTimeout:  time.Minute,
    }

    app.Logger.Infow("server has started")
    err := srv.ListenAndServe()
    if !errors.Is(err, http.ErrServerClosed) {
        return err
    }
    app.Logger.Infow("server has stopped")

    return nil
}
