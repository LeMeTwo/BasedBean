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

type KeyDatabase interface {
    ScanKeys(ctx context.Context) (keys []string, err error)
    ReserveKeys(ctx context.Context, keys []string) (err error)
}

type Lock interface {
    Acquire(ctx context.Context) (bool, error)
    Release(ctx context.Context) (bool, error)
}

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
        lock: RedisFairLock{client},
    }
}

func (kds KeyDbStorage) FetchBatchReservedKeys(ctx context.Context) (keys []string, err error) {
    acquired, err := kds.lock.Acquire(ctx)
    if acquired {
        defer kds.lock.Release(ctx)

        keys, err := kds.db.ScanKeys(ctx)
        if err != nil {
            return nil, err
        }
        kds.db.ReserveKeys(ctx, keys)

        return keys, nil
    } else {
        return nil, fmt.Errorf("Could not acquire lock: %v", err)
    }
}
