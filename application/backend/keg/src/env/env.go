package env

import (
    "os"
)

func GetString(name string, fallback string) string {
    val, ok := os.LookupEnv(name)
    if ok {
        return val
    } else {
        return fallback
    }
}
