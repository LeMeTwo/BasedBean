package db

import (
    "context"
    "github.com/redis/go-redis/v9"
)

type RedisExpiringLock struct {
    client *redis.Client
}

func (rl RedisExpiringLock) Acquire(ctx context.Context) (bool, error) {
    return true, nil
}

func (rl RedisExpiringLock) Release(ctx context.Context) (bool, error) {
    return true, nil
}
