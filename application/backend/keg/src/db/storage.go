package db

import (
    "context"
    "fmt"

    "github.com/redis/go-redis/v9"
)

type DbConfig struct {
    Addr string
    Password string
}

const ACQUIRE_TIMEOUT int = 5
const LOCK_TIMEOUT int = 5

type KeyDbStorage struct {
    db KeyDatabase
    lock Lock
}

func NewKeyDbStorage(cfg DbConfig) KeyDbStorage {
    client := redis.NewClient(&redis.Options{
        Addr: cfg.Addr,
        Password: cfg.Password,
        DB: 0,
        Protocol: 2,
    })
    return KeyDbStorage{
        db: KeyDb{client},
        lock: NewRedisExpiringLock(client, ACQUIRE_TIMEOUT, LOCK_TIMEOUT),
    }
}

func (kds KeyDbStorage) FetchBatchReservedKeys(ctx context.Context, size int) (keys []string, err error) {
    acquired, err := kds.lock.Acquire(ctx)
    if acquired {
        defer kds.lock.Release(ctx)

        keys, err := kds.db.ScanKeys(ctx, size)
        if err != nil {
            return nil, err
        }
        kds.db.ReserveKeys(ctx, keys)

        return keys, nil
    } else {
        return nil, fmt.Errorf("Could not acquire lock: %v", err)
    }
}
