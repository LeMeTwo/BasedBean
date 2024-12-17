package db

import (
    "context"
    "fmt"
    "github.com/redis/go-redis/v9"
)

type KeyDb struct {
    client *redis.Client
    ctx context.Context
}

type DbConfig struct {
    Addr string
    Password string
}

func Connect(cfg DbConfig) *KeyDb {
    client := redis.NewClient(&redis.Options{
        Addr: cfg.Addr,
        Password: cfg.Password,
        DB: 0,
        Protocol: 2,
    })

    ctx := context.Background()

    db := &KeyDb{
        client: client,
        ctx: ctx,
    }

    return db
}

func (kdb *KeyDb) ScanKeys() (keys []string, err error) {
    var cursor uint64
    for {
        var kdbKeys []string
        kdbKeys, cursor, err = kdb.client.Scan(kdb.ctx, cursor, AVAIL_WILDCARD, 100).Result()
        if err != nil {
            panic(err)
        }
        keys = append(keys, kdbKeys[:]...)
        if cursor == 0 || len(keys) >= 10{
            break
        }
    }

    fmt.Println(keys)
    return keys, nil
}

func (kdb *KeyDb) ReserveKeys(keys []string) (err error) {
    for _, key := range keys {
        _, err = kdb.client.Rename(kdb.ctx, key, getKeyUsedName(key)).Result()
        if err != nil {
            panic(err)
        }
    }
    return
}
